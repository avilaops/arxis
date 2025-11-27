use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyInfo {
    pub id: String,
    pub name: String,
    pub industry: String,
    pub size: CompanySize,
    pub employees: u32,
    pub location: String,
    pub website: Option<String>,
    pub description: Option<String>,
    pub founded_year: Option<u16>,
    pub source: String,
    pub scraped_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompanySize {
    Small,          // 1-50
    Medium,         // 51-200
    Large,          // 201-1000
    Enterprise,     // 1000+
    Range(u32, u32),
}

impl CompanySize {
    pub fn mid_point(&self) -> u32 {
        match self {
            CompanySize::Small => 25,
            CompanySize::Medium => 125,
            CompanySize::Large => 600,
            CompanySize::Enterprise => 2000,
            CompanySize::Range(min, max) => (min + max) / 2,
        }
    }

    pub fn from_text(text: &str) -> Self {
        let lower = text.to_lowercase();

        if lower.contains("1-50") || lower.contains("1 a 50") {
            CompanySize::Small
        } else if lower.contains("51-200") || lower.contains("51 a 200") {
            CompanySize::Medium
        } else if lower.contains("201-1000") || lower.contains("201 a 1000") {
            CompanySize::Large
        } else if lower.contains("1000+") || lower.contains("mais de 1000") {
            CompanySize::Enterprise
        } else {
            // Try to extract range
            use regex::Regex;
            let re = Regex::new(r"(\d+)\s*[-a]\s*(\d+)").unwrap();

            if let Some(caps) = re.captures(text) {
                let min: u32 = caps[1].parse().unwrap_or(0);
                let max: u32 = caps[2].parse().unwrap_or(0);
                CompanySize::Range(min, max)
            } else {
                CompanySize::Medium // Default
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub salary_range: Option<(f64, f64)>,
    pub required_skills: Vec<String>,
    pub experience_years: Option<u8>,
    pub remote: bool,
    pub posted_date: DateTime<Utc>,
    pub description: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub place_id: String,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub rating: Option<f32>,
    pub reviews_count: u32,
    pub business_type: Vec<String>,
    pub coordinates: (f64, f64),
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedAddress {
    pub street: String,
    pub city: String,
    pub postal_code: Option<String>,
    pub country: String,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Industry {
    Technology,
    Finance,
    Healthcare,
    Retail,
    Manufacturing,
    Education,
    RealEstate,
    Other(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ScraperError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    #[error("Extraction failed: {0}")]
    ExtractionError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Proxy error: {0}")]
    ProxyError(String),

    #[error("Max attempts exceeded")]
    MaxAttemptsExceeded,

    #[error("Robots.txt disallows this URL")]
    RobotsTxtDisallowed,

    #[cfg(feature = "storage")]
    #[error("Storage error: {0}")]
    StorageError(String),
}

pub type ScraperResult<T> = Result<T, ScraperError>;
