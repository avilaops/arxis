//! Additional map projections
//!
//! This module provides more specialized projections for different use cases.

use crate::coords::{CartesianCoord, GeoCoord, GeoBounds};
use crate::projection::Projection;
use std::f64::consts::PI;

/// Robinson projection
///
/// Popular compromise projection used by National Geographic and Rand McNally.
/// Neither conformal nor equal-area, but provides a pleasant appearance.
///
/// # Properties
/// - Pseudo-cylindrical
/// - Good for world maps
/// - Used by National Geographic (1988-1998)
#[derive(Debug, Clone, Copy)]
pub struct Robinson;

impl Robinson {
    pub fn new() -> Self {
        Self
    }

    // Robinson projection coefficients
    const COEFF: [(f64, f64, f64); 19] = [
        (0.0, 1.0000, 0.0000),
        (5.0, 0.9986, 0.0620),
        (10.0, 0.9954, 0.1240),
        (15.0, 0.9900, 0.1860),
        (20.0, 0.9822, 0.2480),
        (25.0, 0.9730, 0.3100),
        (30.0, 0.9600, 0.3720),
        (35.0, 0.9427, 0.4340),
        (40.0, 0.9216, 0.4958),
        (45.0, 0.8962, 0.5571),
        (50.0, 0.8679, 0.6176),
        (55.0, 0.8350, 0.6769),
        (60.0, 0.7986, 0.7346),
        (65.0, 0.7597, 0.7903),
        (70.0, 0.7186, 0.8435),
        (75.0, 0.6732, 0.8936),
        (80.0, 0.6213, 0.9394),
        (85.0, 0.5722, 0.9761),
        (90.0, 0.5322, 1.0000),
    ];

    fn interpolate(lat_abs: f64) -> (f64, f64) {
        let idx = (lat_abs / 5.0).floor() as usize;
        if idx >= Self::COEFF.len() - 1 {
            return (Self::COEFF[Self::COEFF.len() - 1].1, Self::COEFF[Self::COEFF.len() - 1].2);
        }

        let t = (lat_abs - Self::COEFF[idx].0) / 5.0;
        let x_coeff = Self::COEFF[idx].1 + (Self::COEFF[idx + 1].1 - Self::COEFF[idx].1) * t;
        let y_coeff = Self::COEFF[idx].2 + (Self::COEFF[idx + 1].2 - Self::COEFF[idx].2] * t;

        (x_coeff, y_coeff)
    }
}

impl Default for Robinson {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for Robinson {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let lat_abs = geo.lat.abs();
        let (x_coeff, y_coeff) = Self::interpolate(lat_abs);

        let x = x_coeff * geo.lon.to_radians() * width / (2.0 * PI);
        let y = y_coeff * geo.lat.signum() * height / 2.0;

        CartesianCoord::new(
            width / 2.0 + x,
            height / 2.0 - y,
        )
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        // Robinson inverse is approximated (no closed form)
        let x_norm = (cart.x - width / 2.0) / width * (2.0 * PI);
        let y_norm = -(cart.y - height / 2.0) / height * 2.0;

        let lat = y_norm.signum() * y_norm.abs() * 90.0;
        let lon = x_norm.to_degrees();

        GeoCoord::new_unchecked(lat.clamp(-90.0, 90.0), lon.clamp(-180.0, 180.0))
    }
}

/// Winkel Tripel projection
///
/// Compromise projection that minimizes three distortions: area, direction, and distance.
/// Used by National Geographic since 1998.
///
/// # Properties
/// - Modified azimuthal
/// - Standard parallel at 50.467°
/// - Good for world maps
#[derive(Debug, Clone, Copy)]
pub struct WinkelTripel {
    pub phi1: f64, // Standard parallel (usually 50.467°)
}

impl WinkelTripel {
    pub fn new() -> Self {
        Self {
            phi1: 50.467_f64.to_radians(),
        }
    }

    pub fn with_standard_parallel(phi1_deg: f64) -> Self {
        Self {
            phi1: phi1_deg.to_radians(),
        }
    }
}

impl Default for WinkelTripel {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for WinkelTripel {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let phi = geo.lat.to_radians();
        let lambda = geo.lon.to_radians();

        let cos_phi1 = self.phi1.cos();

        // Aitoff projection part
        let alpha = (lambda / 2.0).cos() * phi.cos();
        let sinc_alpha = if alpha.abs() < 1e-10 {
            1.0
        } else {
            alpha.sin() / alpha
        };

        let x_aitoff = 2.0 * (lambda / 2.0).cos() * phi.cos() * (lambda / 2.0).sin() / sinc_alpha;
        let y_aitoff = phi.sin() / sinc_alpha;

        // Equirectangular part
        let x_equi = lambda * cos_phi1;
        let y_equi = phi;

        // Winkel Tripel is average of the two
        let x = (x_aitoff + x_equi) / 2.0;
        let y = (y_aitoff + y_equi) / 2.0;

        let scale = width.min(height) / (2.0 * PI);

        CartesianCoord::new(
            width / 2.0 + x * scale,
            height / 2.0 - y * scale,
        )
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        // Approximate inverse (no closed form)
        let scale = width.min(height) / (2.0 * PI);
        let x = (cart.x - width / 2.0) / scale;
        let y = -(cart.y - height / 2.0) / scale;

        let lat = y.to_degrees();
        let lon = x.to_degrees();

        GeoCoord::new_unchecked(lat.clamp(-90.0, 90.0), lon.clamp(-180.0, 180.0))
    }
}

/// Mollweide projection (Homalographic)
///
/// Equal-area pseudo-cylindrical projection.
/// Elliptical shape, popular for world maps and thematic maps.
///
/// # Properties
/// - Equal-area (preserves areas)
/// - Elliptical boundary
/// - Good for distribution maps
#[derive(Debug, Clone, Copy)]
pub struct Mollweide;

impl Mollweide {
    pub fn new() -> Self {
        Self
    }

    fn solve_theta(phi: f64) -> f64 {
        // Newton-Raphson iteration to solve: 2θ + sin(2θ) = π sin(φ)
        let mut theta = phi;
        let target = PI * phi.sin();

        for _ in 0..10 {
            let f = 2.0 * theta + (2.0 * theta).sin() - target;
            let fp = 2.0 + 2.0 * (2.0 * theta).cos();

            let delta = f / fp;
            theta -= delta;

            if delta.abs() < 1e-7 {
                break;
            }
        }

        theta
    }
}

impl Default for Mollweide {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for Mollweide {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let phi = geo.lat.to_radians();
        let lambda = geo.lon.to_radians();

        let theta = Self::solve_theta(phi);

        let x = 2.0 * 2.0_f64.sqrt() / PI * lambda * theta.cos();
        let y = 2.0_f64.sqrt() * theta.sin();

        let scale = width / (4.0 * 2.0_f64.sqrt());

        CartesianCoord::new(
            width / 2.0 + x * scale,
            height / 2.0 - y * scale,
        )
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let scale = width / (4.0 * 2.0_f64.sqrt());
        let x = (cart.x - width / 2.0) / scale;
        let y = -(cart.y - height / 2.0) / scale;

        let theta = (y / 2.0_f64.sqrt()).asin();
        let phi = (2.0 * theta + (2.0 * theta).sin()) / PI;
        let lambda = PI * x / (2.0 * 2.0_f64.sqrt() * theta.cos());

        GeoCoord::new_unchecked(
            phi.asin().to_degrees().clamp(-90.0, 90.0),
            lambda.to_degrees().clamp(-180.0, 180.0),
        )
    }

    fn is_equal_area(&self) -> bool {
        true
    }
}

/// Universal Transverse Mercator (UTM) projection
///
/// Divides the world into 60 zones of 6° longitude each.
/// Conformal projection used for topographic maps and surveying.
///
/// # Properties
/// - Conformal (preserves angles)
/// - Low distortion within each zone
/// - Standard for surveying and GIS
#[derive(Debug, Clone, Copy)]
pub struct UTM {
    pub zone: u8,        // UTM zone (1-60)
    pub northern: bool,  // Northern hemisphere?
}

impl UTM {
    pub fn new(zone: u8, northern: bool) -> Self {
        assert!(zone >= 1 && zone <= 60, "UTM zone must be 1-60");
        Self { zone, northern }
    }

    pub fn from_longitude(lon: f64, lat: f64) -> Self {
        let zone = (((lon + 180.0) / 6.0) as u8 + 1).clamp(1, 60);
        let northern = lat >= 0.0;
        Self::new(zone, northern)
    }

    fn central_meridian(&self) -> f64 {
        ((self.zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians()
    }

    const K0: f64 = 0.9996; // Scale factor
    const E: f64 = 0.0818191908426; // Eccentricity WGS84
    const E2: f64 = 0.00669437999014; // E²
    const A: f64 = 6378137.0; // Semi-major axis WGS84
}

impl Projection for UTM {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let phi = geo.lat.to_radians();
        let lambda = geo.lon.to_radians() - self.central_meridian();

        let n = Self::A / (1.0 - Self::E2 * phi.sin().powi(2)).sqrt();
        let t = phi.tan().powi(2);
        let c = Self::E2 * phi.cos().powi(2) / (1.0 - Self::E2);
        let a = phi.cos() * lambda;

        let m = Self::A * ((1.0 - Self::E2 / 4.0 - 3.0 * Self::E2.powi(2) / 64.0) * phi
            - (3.0 * Self::E2 / 8.0 + 3.0 * Self::E2.powi(2) / 32.0) * (2.0 * phi).sin()
            + (15.0 * Self::E2.powi(2) / 256.0) * (4.0 * phi).sin());

        let x = Self::K0 * n * (a + (1.0 - t + c) * a.powi(3) / 6.0
            + (5.0 - 18.0 * t + t.powi(2) + 72.0 * c - 58.0) * a.powi(5) / 120.0)
            + 500000.0; // False easting

        let y = Self::K0 * (m + n * phi.tan() * (a.powi(2) / 2.0
            + (5.0 - t + 9.0 * c + 4.0 * c.powi(2)) * a.powi(4) / 24.0
            + (61.0 - 58.0 * t + t.powi(2) + 600.0 * c - 330.0) * a.powi(6) / 720.0));

        let y = if self.northern {
            y
        } else {
            y + 10000000.0 // False northing for southern hemisphere
        };

        // Scale to viewport
        let scale = width.min(height) / 1000000.0;

        CartesianCoord::new(
            (x - 500000.0) * scale + width / 2.0,
            height / 2.0 - y * scale,
        )
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        // UTM inverse is complex; this is a simplified version
        // Full implementation would use series expansion

        let scale = width.min(height) / 1000000.0;
        let x = (cart.x - width / 2.0) / scale + 500000.0;
        let y = -(cart.y - height / 2.0) / scale;

        // Approximate inverse (proper implementation requires iteration)
        let lat = (y / (Self::K0 * Self::A)).to_degrees();
        let lon = self.central_meridian().to_degrees() + x / 100000.0;

        GeoCoord::new_unchecked(lat.clamp(-90.0, 90.0), lon.clamp(-180.0, 180.0))
    }

    fn is_conformal(&self) -> bool {
        true
    }
}

/// Stereographic projection (Azimuthal)
///
/// Conformal azimuthal projection. Excellent for polar regions.
/// Used for polar maps and some national grid systems.
///
/// # Properties
/// - Conformal (preserves angles)
/// - Azimuthal (preserves directions from center)
/// - Excellent for polar regions
#[derive(Debug, Clone, Copy)]
pub struct Stereographic {
    pub center_lat: f64,
    pub center_lon: f64,
    pub scale: f64,
}

impl Stereographic {
    pub fn new(center_lat: f64, center_lon: f64) -> Self {
        Self {
            center_lat: center_lat.to_radians(),
            center_lon: center_lon.to_radians(),
            scale: 1.0,
        }
    }

    pub fn north_pole() -> Self {
        Self::new(90.0, 0.0)
    }

    pub fn south_pole() -> Self {
        Self::new(-90.0, 0.0)
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
}

impl Projection for Stereographic {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let phi = geo.lat.to_radians();
        let lambda = geo.lon.to_radians();

        let phi0 = self.center_lat;
        let lambda0 = self.center_lon;

        let k = 2.0 * self.scale / (1.0 + phi0.sin() * phi.sin()
            + phi0.cos() * phi.cos() * (lambda - lambda0).cos());

        let x = k * phi.cos() * (lambda - lambda0).sin();
        let y = k * (phi0.cos() * phi.sin() - phi0.sin() * phi.cos() * (lambda - lambda0).cos());

        let scale = width.min(height) / 4.0;

        CartesianCoord::new(
            width / 2.0 + x * scale,
            height / 2.0 - y * scale,
        )
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let scale = width.min(height) / 4.0;
        let x = (cart.x - width / 2.0) / scale;
        let y = -(cart.y - height / 2.0) / scale;

        let phi0 = self.center_lat;
        let lambda0 = self.center_lon;

        let rho = (x * x + y * y).sqrt();
        let c = 2.0 * (rho / (2.0 * self.scale)).atan();

        let lat = (c.cos() * phi0.sin() + y * c.sin() * phi0.cos() / rho).asin();
        let lon = lambda0 + (x * c.sin()).atan2(rho * phi0.cos() * c.cos() - y * phi0.sin() * c.sin());

        GeoCoord::new_unchecked(lat.to_degrees(), lon.to_degrees())
    }

    fn is_conformal(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robinson() {
        let proj = Robinson::new();
        let coord = GeoCoord::new(0.0, 0.0);
        let cart = proj.project(&coord, 800.0, 600.0);

        // Should be near center
        assert!((cart.x - 400.0).abs() < 10.0);
        assert!((cart.y - 300.0).abs() < 10.0);
    }

    #[test]
    fn test_winkel_tripel() {
        let proj = WinkelTripel::new();
        let coord = GeoCoord::new(0.0, 0.0);
        let cart = proj.project(&coord, 800.0, 600.0);

        assert!((cart.x - 400.0).abs() < 10.0);
        assert!((cart.y - 300.0).abs() < 10.0);
    }

    #[test]
    fn test_mollweide() {
        let proj = Mollweide::new();
        let coord = GeoCoord::new(0.0, 0.0);
        let cart = proj.project(&coord, 800.0, 600.0);

        assert!((cart.x - 400.0).abs() < 10.0);
        assert!((cart.y - 300.0).abs() < 10.0);
    }

    #[test]
    fn test_utm() {
        let proj = UTM::new(23, true); // Zone 23N (covers part of Brazil)
        let coord = GeoCoord::new(-23.55, -46.63); // São Paulo

        let cart = proj.project(&coord, 800.0, 600.0);
        let back = proj.unproject(&cart, 800.0, 600.0);

        // Approximate round-trip (simplified inverse)
        assert!((back.lat - coord.lat).abs() < 5.0);
    }

    #[test]
    fn test_stereographic_poles() {
        let north = Stereographic::north_pole();
        let south = Stereographic::south_pole();

        let arctic = GeoCoord::new(85.0, 0.0);
        let antarctic = GeoCoord::new(-85.0, 0.0);

        let _ = north.project(&arctic, 800.0, 600.0);
        let _ = south.project(&antarctic, 800.0, 600.0);
    }
}
