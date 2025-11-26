use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct ScraperMonitor {
    metrics: ScraperMetrics,
    start_time: Instant,
}

#[derive(Debug, Clone)]
pub struct ScraperMetrics {
    pub urls_scraped: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub total_response_time_ms: f64,
    pub errors_by_type: HashMap<String, usize>,
}

impl Default for ScraperMetrics {
    fn default() -> Self {
        Self {
            urls_scraped: 0,
            success_count: 0,
            error_count: 0,
            total_response_time_ms: 0.0,
            errors_by_type: HashMap::new(),
        }
    }
}

impl ScraperMonitor {
    pub fn new() -> Self {
        Self {
            metrics: ScraperMetrics::default(),
            start_time: Instant::now(),
        }
    }

    /// Record a successful scrape
    pub fn record_success(&mut self, duration: Duration) {
        self.metrics.urls_scraped += 1;
        self.metrics.success_count += 1;
        self.metrics.total_response_time_ms += duration.as_millis() as f64;
    }

    /// Record a failed scrape
    pub fn record_error(&mut self, error_type: &str) {
        self.metrics.urls_scraped += 1;
        self.metrics.error_count += 1;
        *self.metrics.errors_by_type.entry(error_type.to_string()).or_insert(0) += 1;
    }

    /// Get success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        if self.metrics.urls_scraped == 0 {
            return 0.0;
        }
        self.metrics.success_count as f64 / self.metrics.urls_scraped as f64
    }

    /// Get average response time in milliseconds
    pub fn avg_response_time_ms(&self) -> f64 {
        if self.metrics.success_count == 0 {
            return 0.0;
        }
        self.metrics.total_response_time_ms / self.metrics.success_count as f64
    }

    /// Get total uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get metrics summary
    pub fn summary(&self) -> String {
        format!(
            "Scraper Stats:\n\
             - URLs Scraped: {}\n\
             - Success Rate: {:.2}%\n\
             - Avg Response Time: {:.2}ms\n\
             - Total Errors: {}\n\
             - Uptime: {:?}",
            self.metrics.urls_scraped,
            self.success_rate() * 100.0,
            self.avg_response_time_ms(),
            self.metrics.error_count,
            self.uptime()
        )
    }

    /// Check for anomalies
    pub fn check_anomalies(&self) -> Vec<String> {
        let mut alerts = Vec::new();

        if self.success_rate() < 0.8 {
            alerts.push(format!(
                "⚠️  Low success rate: {:.2}%",
                self.success_rate() * 100.0
            ));
        }

        if self.avg_response_time_ms() > 5000.0 {
            alerts.push(format!(
                "⚠️  High average response time: {:.2}ms",
                self.avg_response_time_ms()
            ));
        }

        if self.metrics.error_count > self.metrics.success_count / 2 {
            alerts.push("⚠️  Error count exceeds 50% of requests".to_string());
        }

        alerts
    }
}

/// Data quality validator
pub struct QualityValidator;

impl QualityValidator {
    /// Validate company data completeness
    pub fn validate_company(&self, company: &crate::types::CompanyInfo) -> QualityScore {
        let mut score = 0.0;
        let mut issues = Vec::new();

        // Name (required)
        if !company.name.is_empty() {
            score += 20.0;
        } else {
            issues.push("Missing company name".to_string());
        }

        // Industry
        if !company.industry.is_empty() {
            score += 15.0;
        } else {
            issues.push("Missing industry".to_string());
        }

        // Employees
        if company.employees > 0 {
            score += 15.0;
        } else {
            issues.push("Missing employee count".to_string());
        }

        // Location
        if !company.location.is_empty() && company.location != "Unknown" {
            score += 15.0;
        } else {
            issues.push("Missing or unknown location".to_string());
        }

        // Website
        if company.website.is_some() {
            score += 15.0;
        } else {
            issues.push("Missing website".to_string());
        }

        // Description
        if company.description.is_some() && company.description.as_ref().unwrap().len() > 50 {
            score += 10.0;
        } else {
            issues.push("Missing or short description".to_string());
        }

        // Founded year
        if company.founded_year.is_some() {
            score += 10.0;
        }

        QualityScore { score, issues }
    }

    /// Validate job posting data
    pub fn validate_job(&self, job: &crate::types::JobPosting) -> QualityScore {
        let mut score = 0.0;
        let mut issues = Vec::new();

        if !job.title.is_empty() {
            score += 25.0;
        } else {
            issues.push("Missing job title".to_string());
        }

        if !job.company.is_empty() {
            score += 20.0;
        } else {
            issues.push("Missing company name".to_string());
        }

        if !job.location.is_empty() {
            score += 15.0;
        } else {
            issues.push("Missing location".to_string());
        }

        if job.description.is_some() && job.description.as_ref().unwrap().len() > 100 {
            score += 20.0;
        } else {
            issues.push("Missing or short description".to_string());
        }

        if !job.required_skills.is_empty() {
            score += 10.0;
        } else {
            issues.push("No required skills listed".to_string());
        }

        if job.salary_range.is_some() {
            score += 10.0;
        }

        QualityScore { score, issues }
    }
}

#[derive(Debug)]
pub struct QualityScore {
    pub score: f64,
    pub issues: Vec<String>,
}

impl QualityScore {
    pub fn is_acceptable(&self) -> bool {
        self.score >= 60.0
    }

    pub fn grade(&self) -> &str {
        match self.score as u32 {
            90..=100 => "A",
            80..=89 => "B",
            70..=79 => "C",
            60..=69 => "D",
            _ => "F",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_monitor_metrics() {
        let mut monitor = ScraperMonitor::new();

        monitor.record_success(Duration::from_millis(100));
        monitor.record_success(Duration::from_millis(200));
        monitor.record_error("HTTP 404");

        assert_eq!(monitor.metrics.urls_scraped, 3);
        assert_eq!(monitor.metrics.success_count, 2);
        assert_eq!(monitor.metrics.error_count, 1);
        assert_eq!(monitor.success_rate(), 2.0 / 3.0);
        assert_eq!(monitor.avg_response_time_ms(), 150.0);
    }

    #[test]
    fn test_quality_validation() {
        let validator = QualityValidator;

        let company = crate::types::CompanyInfo {
            id: "test".to_string(),
            name: "Test Company".to_string(),
            industry: "Technology".to_string(),
            size: crate::types::CompanySize::Medium,
            employees: 100,
            location: "Lisboa".to_string(),
            website: Some("https://example.com".to_string()),
            description: Some("A great company with excellent products and services".to_string()),
            founded_year: Some(2020),
            source: "Test".to_string(),
            scraped_at: Utc::now(),
        };

        let score = validator.validate_company(&company);
        assert!(score.is_acceptable());
        assert_eq!(score.grade(), "A");
    }
}
