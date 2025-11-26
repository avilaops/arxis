//! Location and region definitions

use super::{Coordinate, Country};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a specific location (city/region) for business setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub country: Country,
    pub region: String,
    pub coordinate: Coordinate,

    /// Population
    pub population: u64,

    /// Area in km²
    pub area_km2: f64,

    /// Average temperature (°C)
    pub avg_temperature: f64,

    /// Properties and characteristics
    pub properties: HashMap<String, String>,
}

impl Location {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        country: Country,
        region: impl Into<String>,
        coordinate: Coordinate,
        population: u64,
        area_km2: f64,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            country,
            region: region.into(),
            coordinate,
            population,
            area_km2,
            avg_temperature: 0.0,
            properties: HashMap::new(),
        }
    }

    pub fn population_density(&self) -> f64 {
        self.population as f64 / self.area_km2
    }

    pub fn with_temperature(mut self, temp: f64) -> Self {
        self.avg_temperature = temp;
        self
    }

    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Region type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegionType {
    /// Major metropolitan area
    Metropolitan,

    /// Urban center
    Urban,

    /// Suburban area
    Suburban,

    /// Rural/Interior region
    Rural,

    /// Free economic zone
    FreeZone,
}

/// Detailed region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub location: Location,
    pub region_type: RegionType,

    /// Accessibility metrics
    pub accessibility: AccessibilityMetrics,

    /// Urban development index (0-100)
    pub urban_development_index: f64,

    /// Safety score (0-100)
    pub safety_score: f64,
}

/// Accessibility metrics for a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityMetrics {
    /// Minutes to nearest international airport
    pub airport_minutes: u32,

    /// Minutes to nearest major city
    pub major_city_minutes: u32,

    /// Public transport quality score (0-100)
    pub public_transport_score: f64,

    /// Road network quality score (0-100)
    pub road_quality_score: f64,

    /// Number of international flight connections per week
    pub international_flights_weekly: u32,
}

impl AccessibilityMetrics {
    pub fn overall_score(&self) -> f64 {
        let airport_score = (120.0 - self.airport_minutes.min(120) as f64) / 120.0 * 100.0;
        let city_score = (90.0 - self.major_city_minutes.min(90) as f64) / 90.0 * 100.0;
        let flight_score = (self.international_flights_weekly.min(100) as f64).min(100.0);

        (airport_score * 0.25 +
         city_score * 0.15 +
         self.public_transport_score * 0.25 +
         self.road_quality_score * 0.20 +
         flight_score * 0.15)
    }
}
