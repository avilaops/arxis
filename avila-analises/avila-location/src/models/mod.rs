//! Core data models for location intelligence

pub mod location;
pub mod market;
pub mod company;
pub mod infrastructure;
pub mod economic;
pub mod score;

pub use location::*;
pub use market::*;
pub use company::*;
pub use infrastructure::*;
pub use economic::*;
pub use score::*;

use serde::{Deserialize, Serialize};

/// Coordinate system for geospatial data
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    /// Calculate Haversine distance to another coordinate (in km)
    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

/// Country/Region identifier
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Country {
    Portugal,
    UAE,
}

impl Country {
    pub fn name(&self) -> &'static str {
        match self {
            Country::Portugal => "Portugal",
            Country::UAE => "United Arab Emirates",
        }
    }

    pub fn currency(&self) -> &'static str {
        match self {
            Country::Portugal => "EUR",
            Country::UAE => "AED",
        }
    }

    pub fn currency_symbol(&self) -> &'static str {
        match self {
            Country::Portugal => "€",
            Country::UAE => "د.إ",
        }
    }
}
