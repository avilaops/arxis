//! Coordinate systems and conversions
//!
//! This module provides fundamental coordinate types and conversions:
//! - Geographic coordinates (latitude/longitude)
//! - Cartesian coordinates (x/y pixels)
//! - 3D Earth-Centered Earth-Fixed (ECEF) coordinates

use std::f64::consts::PI;

/// Geographic coordinate (latitude/longitude) in degrees
///
/// # Convention
/// - Latitude: -90° (South Pole) to +90° (North Pole)
/// - Longitude: -180° (West) to +180° (East)
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoCoord {
    /// Latitude in degrees (-90 to 90)
    pub lat: f64,
    /// Longitude in degrees (-180 to 180)
    pub lon: f64,
}

impl GeoCoord {
    /// Create a new geographic coordinate
    ///
    /// # Panics
    /// Panics if latitude is outside [-90, 90] or longitude outside [-180, 180]
    #[inline]
    pub fn new(lat: f64, lon: f64) -> Self {
        assert!(
            lat >= -90.0 && lat <= 90.0,
            "Latitude must be between -90 and 90 degrees"
        );
        assert!(
            lon >= -180.0 && lon <= 180.0,
            "Longitude must be between -180 and 180 degrees"
        );
        Self { lat, lon }
    }

    /// Create coordinate without bounds checking (unsafe but faster)
    #[inline]
    pub const fn new_unchecked(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    /// Convert to radians
    #[inline]
    pub fn to_radians(&self) -> GeoCoordRadians {
        GeoCoordRadians {
            lat: self.lat.to_radians(),
            lon: self.lon.to_radians(),
        }
    }

    /// Normalize longitude to [-180, 180] range
    #[inline]
    pub fn normalize_lon(&mut self) {
        while self.lon > 180.0 {
            self.lon -= 360.0;
        }
        while self.lon < -180.0 {
            self.lon += 360.0;
        }
    }

    /// Clamp latitude to [-90, 90] range
    #[inline]
    pub fn clamp_lat(&mut self) {
        self.lat = self.lat.clamp(-90.0, 90.0);
    }

    /// Convert to ECEF (Earth-Centered Earth-Fixed) coordinates
    ///
    /// Uses WGS84 ellipsoid parameters
    pub fn to_ecef(&self) -> [f64; 3] {
        const A: f64 = 6378137.0; // Semi-major axis (meters)
        const E2: f64 = 0.00669437999014; // First eccentricity squared

        let rad = self.to_radians();
        let sin_lat = rad.lat.sin();
        let cos_lat = rad.lat.cos();
        let sin_lon = rad.lon.sin();
        let cos_lon = rad.lon.cos();

        let n = A / (1.0 - E2 * sin_lat * sin_lat).sqrt();

        [
            n * cos_lat * cos_lon,
            n * cos_lat * sin_lon,
            n * (1.0 - E2) * sin_lat,
        ]
    }
}

/// Geographic coordinate in radians (for internal calculations)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoCoordRadians {
    pub lat: f64,
    pub lon: f64,
}

impl GeoCoordRadians {
    /// Convert back to degrees
    #[inline]
    pub fn to_degrees(&self) -> GeoCoord {
        GeoCoord {
            lat: self.lat.to_degrees(),
            lon: self.lon.to_degrees(),
        }
    }
}

/// Cartesian coordinate (2D pixel/screen space)
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

impl CartesianCoord {
    /// Create a new cartesian coordinate
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Convert to integer coordinates (for pixel operations)
    #[inline]
    pub fn to_i32(&self) -> (i32, i32) {
        (self.x.round() as i32, self.y.round() as i32)
    }

    /// Convert to unsigned integer coordinates
    #[inline]
    pub fn to_u32(&self) -> Option<(u32, u32)> {
        if self.x >= 0.0 && self.y >= 0.0 {
            Some((self.x.round() as u32, self.y.round() as u32))
        } else {
            None
        }
    }

    /// Calculate distance to another point
    #[inline]
    pub fn distance(&self, other: &CartesianCoord) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate squared distance (faster, avoids sqrt)
    #[inline]
    pub fn distance_squared(&self, other: &CartesianCoord) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// Bounds for geographic regions
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoBounds {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl GeoBounds {
    /// Create new geographic bounds
    pub fn new(min_lat: f64, max_lat: f64, min_lon: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            max_lat,
            min_lon,
            max_lon,
        }
    }

    /// Create bounds from a list of coordinates
    pub fn from_coords(coords: &[GeoCoord]) -> Option<Self> {
        if coords.is_empty() {
            return None;
        }

        let mut bounds = Self {
            min_lat: coords[0].lat,
            max_lat: coords[0].lat,
            min_lon: coords[0].lon,
            max_lon: coords[0].lon,
        };

        for coord in &coords[1..] {
            bounds.extend(coord);
        }

        Some(bounds)
    }

    /// Extend bounds to include a coordinate
    pub fn extend(&mut self, coord: &GeoCoord) {
        self.min_lat = self.min_lat.min(coord.lat);
        self.max_lat = self.max_lat.max(coord.lat);
        self.min_lon = self.min_lon.min(coord.lon);
        self.max_lon = self.max_lon.max(coord.lon);
    }

    /// Check if coordinate is within bounds
    #[inline]
    pub fn contains(&self, coord: &GeoCoord) -> bool {
        coord.lat >= self.min_lat
            && coord.lat <= self.max_lat
            && coord.lon >= self.min_lon
            && coord.lon <= self.max_lon
    }

    /// Get center coordinate
    #[inline]
    pub fn center(&self) -> GeoCoord {
        GeoCoord {
            lat: (self.min_lat + self.max_lat) / 2.0,
            lon: (self.min_lon + self.max_lon) / 2.0,
        }
    }

    /// Get width in degrees
    #[inline]
    pub fn width(&self) -> f64 {
        self.max_lon - self.min_lon
    }

    /// Get height in degrees
    #[inline]
    pub fn height(&self) -> f64 {
        self.max_lat - self.min_lat
    }

    /// World bounds (entire Earth)
    pub const WORLD: Self = Self {
        min_lat: -90.0,
        max_lat: 90.0,
        min_lon: -180.0,
        max_lon: 180.0,
    };

    /// Brazil approximate bounds
    pub const BRAZIL: Self = Self {
        min_lat: -33.75,
        max_lat: 5.27,
        min_lon: -73.99,
        max_lon: -28.84,
    };

    /// USA approximate bounds
    pub const USA: Self = Self {
        min_lat: 24.52,
        max_lat: 49.38,
        min_lon: -125.0,
        max_lon: -66.93,
    };

    /// Europe approximate bounds
    pub const EUROPE: Self = Self {
        min_lat: 36.0,
        max_lat: 71.0,
        min_lon: -10.0,
        max_lon: 40.0,
    };

    /// Middle East approximate bounds
    pub const MIDDLE_EAST: Self = Self {
        min_lat: 12.0,
        max_lat: 42.0,
        min_lon: 34.0,
        max_lon: 63.0,
    };

    /// Dubai/UAE approximate bounds
    pub const DUBAI: Self = Self {
        min_lat: 22.5,
        max_lat: 26.5,
        min_lon: 51.0,
        max_lon: 56.5,
    };

    /// Gulf region (Arabian/Persian Gulf)
    pub const GULF_REGION: Self = Self {
        min_lat: 22.0,
        max_lat: 30.5,
        min_lon: 47.0,
        max_lon: 57.0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocoord_creation() {
        let coord = GeoCoord::new(-23.55, -46.63); // São Paulo
        assert_eq!(coord.lat, -23.55);
        assert_eq!(coord.lon, -46.63);
    }

    #[test]
    #[should_panic]
    fn test_invalid_latitude() {
        GeoCoord::new(91.0, 0.0);
    }

    #[test]
    fn test_normalize_lon() {
        let mut coord = GeoCoord::new_unchecked(0.0, 190.0);
        coord.normalize_lon();
        assert_eq!(coord.lon, -170.0);
    }

    #[test]
    fn test_bounds_contains() {
        let bounds = GeoBounds::BRAZIL;
        let sao_paulo = GeoCoord::new(-23.55, -46.63);
        let new_york = GeoCoord::new(40.71, -74.01);

        assert!(bounds.contains(&sao_paulo));
        assert!(!bounds.contains(&new_york));
    }

    #[test]
    fn test_cartesian_distance() {
        let p1 = CartesianCoord::new(0.0, 0.0);
        let p2 = CartesianCoord::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }
}
