use super::DataExtractor;
use crate::types::ScraperError;
use scraper::Html;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealEstateProperty {
    pub id: String,
    pub title: String,
    pub price: f64,
    pub location: String,
    pub property_type: String,
    pub bedrooms: Option<u8>,
    pub bathrooms: Option<u8>,
    pub area_m2: Option<f64>,
    pub features: Vec<String>,
    pub description: Option<String>,
    pub images: Vec<String>,
}

pub struct IdealistaExtractor;

impl DataExtractor for IdealistaExtractor {
    type Output = Vec<RealEstateProperty>;

    fn extract(&self, html: &Html) -> Result<Vec<RealEstateProperty>, ScraperError> {
        // Idealista has anti-scraping measures
        // This would require more sophisticated extraction
        Err(ScraperError::ExtractionError(
            "Idealista extraction requires advanced anti-detection. Use API if available.".to_string()
        ))
    }

    fn validate(&self, properties: &Vec<RealEstateProperty>) -> bool {
        !properties.is_empty()
    }
}

impl IdealistaExtractor {
    pub fn new() -> Self {
        Self
    }
}
