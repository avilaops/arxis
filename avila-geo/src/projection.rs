//! Map projections
//!
//! This module implements various cartographic projections for converting
//! geographic coordinates (lat/lon) to Cartesian coordinates (x/y).
//!
//! # Supported Projections
//! - **Equirectangular**: Simple linear mapping (fast but distorted at poles)
//! - **Mercator**: Cylindrical conformal projection (used in navigation)
//! - **Web Mercator**: Pseudo-Mercator used by web maps (Google, OSM)
//! - **Albers Equal Area**: Conic projection preserving area
//! - **Lambert Conformal Conic**: Conic projection preserving angles

use crate::coords::{CartesianCoord, GeoCoord, GeoBounds};
use std::f64::consts::PI;

/// Trait for map projections
pub trait Projection: Send + Sync {
    /// Project geographic coordinate to Cartesian space
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord;

    /// Inverse projection (Cartesian to geographic)
    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord;

    /// Get the bounds this projection supports
    fn bounds(&self) -> GeoBounds {
        GeoBounds::WORLD
    }

    /// Check if projection is conformal (preserves angles)
    fn is_conformal(&self) -> bool {
        false
    }

    /// Check if projection is equal-area (preserves areas)
    fn is_equal_area(&self) -> bool {
        false
    }
}

/// Equirectangular projection (Plate Carrée)
///
/// The simplest projection: linear mapping of lat/lon to x/y.
/// Fast but has significant distortion, especially near poles.
///
/// # Properties
/// - Neither conformal nor equal-area
/// - Simple and fast
/// - Good for small areas near equator
#[derive(Debug, Clone, Copy)]
pub struct Equirectangular {
    pub bounds: GeoBounds,
}

impl Equirectangular {
    pub fn new() -> Self {
        Self {
            bounds: GeoBounds::WORLD,
        }
    }

    pub fn with_bounds(bounds: GeoBounds) -> Self {
        Self { bounds }
    }
}

impl Default for Equirectangular {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for Equirectangular {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let lon_range = self.bounds.max_lon - self.bounds.min_lon;
        let lat_range = self.bounds.max_lat - self.bounds.min_lat;

        let x = (geo.lon - self.bounds.min_lon) * (width / lon_range);
        let y = (self.bounds.max_lat - geo.lat) * (height / lat_range);

        CartesianCoord::new(x, y)
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let lon_range = self.bounds.max_lon - self.bounds.min_lon;
        let lat_range = self.bounds.max_lat - self.bounds.min_lat;

        let lon = self.bounds.min_lon + (cart.x / width) * lon_range;
        let lat = self.bounds.max_lat - (cart.y / height) * lat_range;

        GeoCoord::new_unchecked(lat, lon)
    }

    fn bounds(&self) -> GeoBounds {
        self.bounds
    }
}

/// Mercator projection
///
/// Cylindrical conformal projection. Preserves angles and shapes locally
/// but severely distorts areas near poles. Cannot show poles themselves.
///
/// # Properties
/// - Conformal (preserves angles)
/// - Infinite distortion at poles
/// - Used for navigation (rhumb lines are straight)
///
/// # Formula
/// ```text
/// x = λ
/// y = ln(tan(φ) + sec(φ))
/// where φ = latitude, λ = longitude (in radians)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Mercator {
    /// Latitude limit (Mercator can't show poles)
    pub max_lat: f64,
}

impl Mercator {
    /// Create standard Mercator projection
    ///
    /// Uses ±85.051129° latitude limit (Web Mercator standard)
    pub fn new() -> Self {
        Self {
            max_lat: 85.051129,
        }
    }

    /// Create with custom latitude limit
    pub fn with_max_lat(max_lat: f64) -> Self {
        Self {
            max_lat: max_lat.min(89.9),
        }
    }

    /// Mercator y-coordinate formula
    #[inline]
    fn mercator_y(lat: f64) -> f64 {
        let lat_rad = lat.to_radians();
        ((PI / 4.0 + lat_rad / 2.0).tan()).ln()
    }

    /// Inverse Mercator y-coordinate
    #[inline]
    fn inverse_mercator_y(y: f64) -> f64 {
        (2.0 * y.exp().atan() - PI / 2.0).to_degrees()
    }
}

impl Default for Mercator {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for Mercator {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let lat = geo.lat.clamp(-self.max_lat, self.max_lat);

        let x = (geo.lon + 180.0) * (width / 360.0);
        let y_merc = Self::mercator_y(lat);
        let y_max = Self::mercator_y(self.max_lat);
        let y = (height / 2.0) - (y_merc / y_max) * (height / 2.0);

        CartesianCoord::new(x, y)
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let lon = (cart.x / width) * 360.0 - 180.0;

        let y_max = Self::mercator_y(self.max_lat);
        let y_merc = ((height / 2.0) - cart.y) * y_max / (height / 2.0);
        let lat = Self::inverse_mercator_y(y_merc).clamp(-self.max_lat, self.max_lat);

        GeoCoord::new_unchecked(lat, lon)
    }

    fn bounds(&self) -> GeoBounds {
        GeoBounds::new(-self.max_lat, self.max_lat, -180.0, 180.0)
    }

    fn is_conformal(&self) -> bool {
        true
    }
}

/// Web Mercator projection (EPSG:3857)
///
/// Pseudo-Mercator projection used by Google Maps, OpenStreetMap, etc.
/// Similar to standard Mercator but treats Earth as a sphere instead of ellipsoid.
///
/// # Properties
/// - Not truly conformal (slight distortion)
/// - Web standard (tile coordinates)
/// - Latitude limit: ±85.051129°
#[derive(Debug, Clone, Copy)]
pub struct WebMercator;

impl WebMercator {
    pub fn new() -> Self {
        Self
    }

    const MAX_LAT: f64 = 85.051129;
    const R: f64 = 6378137.0; // Earth radius in meters

    /// Convert to tile coordinates at given zoom level
    pub fn to_tile(&self, geo: &GeoCoord, zoom: u8) -> (u32, u32) {
        let n = 2.0_f64.powi(zoom as i32);

        let lat_rad = geo.lat.to_radians();
        let x = ((geo.lon + 180.0) / 360.0 * n) as u32;
        let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0 * n) as u32;

        (x, y)
    }

    /// Convert from tile coordinates
    pub fn from_tile(x: u32, y: u32, zoom: u8) -> GeoCoord {
        let n = 2.0_f64.powi(zoom as i32);

        let lon = x as f64 / n * 360.0 - 180.0;
        let lat_rad = ((1.0 - 2.0 * y as f64 / n) * PI).sinh().atan();
        let lat = lat_rad.to_degrees();

        GeoCoord::new_unchecked(lat, lon)
    }
}

impl Default for WebMercator {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for WebMercator {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let lat = geo.lat.clamp(-Self::MAX_LAT, Self::MAX_LAT);
        let lat_rad = lat.to_radians();

        let x = (geo.lon + 180.0) / 360.0;
        let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0;

        CartesianCoord::new(x * width, y * height)
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let x_norm = cart.x / width;
        let y_norm = cart.y / height;

        let lon = x_norm * 360.0 - 180.0;
        let lat_rad = ((1.0 - 2.0 * y_norm) * PI).sinh().atan();
        let lat = lat_rad.to_degrees().clamp(-Self::MAX_LAT, Self::MAX_LAT);

        GeoCoord::new_unchecked(lat, lon)
    }

    fn bounds(&self) -> GeoBounds {
        GeoBounds::new(-Self::MAX_LAT, Self::MAX_LAT, -180.0, 180.0)
    }
}

/// Albers Equal-Area Conic projection
///
/// Conic projection that preserves area. Good for mid-latitude regions
/// that are wider east-west than north-south.
///
/// # Properties
/// - Equal-area (preserves relative sizes)
/// - Good for thematic maps
/// - Distorts angles and shapes
#[derive(Debug, Clone, Copy)]
pub struct AlbersEqualArea {
    /// First standard parallel
    pub phi1: f64,
    /// Second standard parallel
    pub phi2: f64,
    /// Central meridian
    pub lambda0: f64,
    /// Reference latitude
    pub phi0: f64,

    // Precomputed constants
    n: f64,
    c: f64,
    rho0: f64,
}

impl AlbersEqualArea {
    /// Create Albers projection with standard parallels
    ///
    /// # Arguments
    /// * `phi1` - First standard parallel (degrees)
    /// * `phi2` - Second standard parallel (degrees)
    /// * `lambda0` - Central meridian (degrees)
    /// * `phi0` - Reference latitude (degrees)
    pub fn new(phi1: f64, phi2: f64, lambda0: f64, phi0: f64) -> Self {
        let phi1_rad = phi1.to_radians();
        let phi2_rad = phi2.to_radians();
        let phi0_rad = phi0.to_radians();

        let n = (phi1_rad.cos().powi(2) + phi2_rad.cos().powi(2)) / 2.0;
        let c = phi1_rad.cos().powi(2) + 2.0 * n * phi1_rad.sin();
        let rho0 = (c - 2.0 * n * phi0_rad.sin()).sqrt() / n;

        Self {
            phi1,
            phi2,
            lambda0,
            phi0,
            n,
            c,
            rho0,
        }
    }

    /// Create with defaults for continental USA
    pub fn usa() -> Self {
        Self::new(29.5, 45.5, -96.0, 37.5)
    }

    /// Create with defaults for Brazil
    pub fn brazil() -> Self {
        Self::new(-15.0, -5.0, -55.0, -10.0)
    }
}

impl Projection for AlbersEqualArea {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let phi_rad = geo.lat.to_radians();
        let lambda_rad = (geo.lon - self.lambda0).to_radians();

        let rho = (self.c - 2.0 * self.n * phi_rad.sin()).sqrt() / self.n;
        let theta = self.n * lambda_rad;

        let x = rho * theta.sin();
        let y = self.rho0 - rho * theta.cos();

        // Scale to fit in viewport
        let scale = width.min(height) / 4.0;
        CartesianCoord::new(
            width / 2.0 + x * scale,
            height / 2.0 + y * scale,
        )
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let scale = width.min(height) / 4.0;
        let x = (cart.x - width / 2.0) / scale;
        let y = (cart.y - height / 2.0) / scale;

        let rho = ((x * x) + (self.rho0 - y).powi(2)).sqrt() * self.n.signum();
        let theta = (x / (self.rho0 - y)).atan();

        let lat = ((self.c - (rho * self.n).powi(2)) / (2.0 * self.n)).asin().to_degrees();
        let lon = self.lambda0 + (theta / self.n).to_degrees();

        GeoCoord::new_unchecked(lat, lon)
    }

    fn is_equal_area(&self) -> bool {
        true
    }
}

/// Lambert Conformal Conic projection
///
/// Conic projection that preserves angles. Good for mid-latitude regions.
/// Used for aeronautical charts and many national mapping systems.
///
/// # Properties
/// - Conformal (preserves angles)
/// - Good for mid-latitude zones
/// - Used in aviation
#[derive(Debug, Clone, Copy)]
pub struct LambertConformalConic {
    pub phi1: f64,
    pub phi2: f64,
    pub lambda0: f64,
    pub phi0: f64,

    // Precomputed
    n: f64,
    f: f64,
    rho0: f64,
}

impl LambertConformalConic {
    pub fn new(phi1: f64, phi2: f64, lambda0: f64, phi0: f64) -> Self {
        let phi1_rad = phi1.to_radians();
        let phi2_rad = phi2.to_radians();
        let phi0_rad = phi0.to_radians();

        let n = (phi1_rad.cos().ln() - phi2_rad.cos().ln())
            / ((PI / 4.0 + phi2_rad / 2.0).tan().ln() - (PI / 4.0 + phi1_rad / 2.0).tan().ln());

        let f = phi1_rad.cos() * (PI / 4.0 + phi1_rad / 2.0).tan().powf(n) / n;
        let rho0 = f / (PI / 4.0 + phi0_rad / 2.0).tan().powf(n);

        Self {
            phi1,
            phi2,
            lambda0,
            phi0,
            n,
            f,
            rho0,
        }
    }

    pub fn usa() -> Self {
        Self::new(33.0, 45.0, -96.0, 39.0)
    }
}

impl Projection for LambertConformalConic {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let phi_rad = geo.lat.to_radians();
        let lambda_rad = (geo.lon - self.lambda0).to_radians();

        let rho = self.f / (PI / 4.0 + phi_rad / 2.0).tan().powf(self.n);
        let theta = self.n * lambda_rad;

        let x = rho * theta.sin();
        let y = self.rho0 - rho * theta.cos();

        let scale = width.min(height) / 4.0;
        CartesianCoord::new(
            width / 2.0 + x * scale,
            height / 2.0 + y * scale,
        )
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let scale = width.min(height) / 4.0;
        let x = (cart.x - width / 2.0) / scale;
        let y = (cart.y - height / 2.0) / scale;

        let rho = ((x * x) + (self.rho0 - y).powi(2)).sqrt() * self.n.signum();
        let theta = (x / (self.rho0 - y)).atan();

        let lat = (2.0 * (self.f / rho).powf(1.0 / self.n).atan() - PI / 2.0).to_degrees();
        let lon = self.lambda0 + (theta / self.n).to_degrees();

        GeoCoord::new_unchecked(lat, lon)
    }

    fn is_conformal(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equirectangular() {
        let proj = Equirectangular::new();
        let geo = GeoCoord::new(0.0, 0.0); // Null Island
        let cart = proj.project(&geo, 360.0, 180.0);

        // Should be center of map
        assert!((cart.x - 180.0).abs() < 0.1);
        assert!((cart.y - 90.0).abs() < 0.1);

        // Round-trip
        let geo2 = proj.unproject(&cart, 360.0, 180.0);
        assert!((geo2.lat - geo.lat).abs() < 0.01);
        assert!((geo2.lon - geo.lon).abs() < 0.01);
    }

    #[test]
    fn test_mercator() {
        let proj = Mercator::new();
        let geo = GeoCoord::new(0.0, 0.0);
        let cart = proj.project(&geo, 360.0, 180.0);

        // Round-trip
        let geo2 = proj.unproject(&cart, 360.0, 180.0);
        assert!((geo2.lat - geo.lat).abs() < 0.01);
        assert!((geo2.lon - geo.lon).abs() < 0.01);
    }

    #[test]
    fn test_web_mercator_tiles() {
        let wm = WebMercator::new();
        let geo = GeoCoord::new(0.0, 0.0);

        // At zoom 0, should be tile (0, 0)
        let (x, y) = wm.to_tile(&geo, 0);
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }
}
