//! Coordinate system transformations and projections

use crate::error::{validate_coord, GeoError, Result};
use crate::{WGS84_A, WGS84_B};
use geo::Coord;
use std::f64::consts::PI;

/// Web Mercator projection (EPSG:3857)
///
/// Used by Google Maps, OpenStreetMap, and most web mapping applications.
/// Valid for latitudes between -85.0511° and 85.0511°.
#[derive(Debug, Clone)]
pub struct WebMercator;

impl WebMercator {
    /// Maximum latitude for Web Mercator projection
    pub const MAX_LAT: f64 = 85.051129;

    /// Project WGS84 coordinates to Web Mercator (EPSG:3857)
    ///
    /// # Example
    /// ```
    /// use geospatial_analysis::coords::WebMercator;
    /// use geo::Coord;
    ///
    /// let lisbon = Coord { x: -9.1393, y: 38.7223 };
    /// let projected = WebMercator::project(&lisbon).unwrap();
    ///
    /// assert!((projected.x - (-1_017_876.0)).abs() < 1.0);
    /// assert!((projected.y - 4_692_197.0)).abs() < 1.0);
    /// ```
    pub fn project(coord: &Coord<f64>) -> Result<Coord<f64>> {
        validate_coord(coord.x, coord.y)?;

        if coord.y.abs() > Self::MAX_LAT {
            return Err(GeoError::InvalidLatitude(coord.y));
        }

        let x = WGS84_A * coord.x.to_radians();
        let y = WGS84_A * ((PI / 4.0) + (coord.y.to_radians() / 2.0)).tan().ln();

        Ok(Coord { x, y })
    }

    /// Unproject Web Mercator coordinates to WGS84
    ///
    /// # Example
    /// ```
    /// use geospatial_analysis::coords::WebMercator;
    /// use geo::Coord;
    ///
    /// let mercator = Coord { x: -1_017_876.0, y: 4_692_197.0 };
    /// let wgs84 = WebMercator::unproject(&mercator).unwrap();
    ///
    /// assert!((wgs84.x - (-9.1393)).abs() < 0.01);
    /// assert!((wgs84.y - 38.7223).abs() < 0.01);
    /// ```
    pub fn unproject(coord: &Coord<f64>) -> Result<Coord<f64>> {
        let lon = coord.x.to_degrees() / WGS84_A;
        let lat = (2.0 * (coord.y / WGS84_A).exp().atan() - PI / 2.0).to_degrees();

        validate_coord(lon, lat)?;

        Ok(Coord { x: lon, y: lat })
    }
}

/// UTM (Universal Transverse Mercator) zone calculator
#[derive(Debug, Clone)]
pub struct UTM;

impl UTM {
    /// Calculate UTM zone number from longitude
    ///
    /// # Example
    /// ```
    /// use geospatial_analysis::coords::UTM;
    ///
    /// let zone = UTM::zone_from_lon(-9.1393); // Lisbon
    /// assert_eq!(zone, 29);
    ///
    /// let zone = UTM::zone_from_lon(-8.6291); // Porto
    /// assert_eq!(zone, 29);
    /// ```
    pub fn zone_from_lon(lon: f64) -> i32 {
        ((lon + 180.0) / 6.0).floor() as i32 + 1
    }

    /// Determine if coordinate is in northern hemisphere
    pub fn is_northern(lat: f64) -> bool {
        lat >= 0.0
    }

    /// Get UTM zone designation (e.g., "29N" for Lisbon)
    ///
    /// # Example
    /// ```
    /// use geospatial_analysis::coords::UTM;
    /// use geo::Coord;
    ///
    /// let lisbon = Coord { x: -9.1393, y: 38.7223 };
    /// let zone = UTM::zone_designation(&lisbon).unwrap();
    /// assert_eq!(zone, "29N");
    /// ```
    pub fn zone_designation(coord: &Coord<f64>) -> Result<String> {
        validate_coord(coord.x, coord.y)?;

        let zone = Self::zone_from_lon(coord.x);
        let hemisphere = if Self::is_northern(coord.y) {
            "N"
        } else {
            "S"
        };

        Ok(format!("{}{}", zone, hemisphere))
    }
}

/// Calculate destination point given distance and bearing
///
/// # Arguments
/// * `origin` - Starting coordinate (WGS84)
/// * `distance_m` - Distance in meters
/// * `bearing_deg` - Bearing in degrees (0 = North, 90 = East)
///
/// # Example
/// ```
/// use geospatial_analysis::coords::destination_point;
/// use geo::Coord;
///
/// let lisbon = Coord { x: -9.1393, y: 38.7223 };
/// let dest = destination_point(&lisbon, 10000.0, 0.0).unwrap();
///
/// // Should be ~10km north of Lisbon
/// assert!((dest.y - 38.81).abs() < 0.01);
/// ```
pub fn destination_point(origin: &Coord<f64>, distance_m: f64, bearing_deg: f64) -> Result<Coord<f64>> {
    validate_coord(origin.x, origin.y)?;

    let lat1 = origin.y.to_radians();
    let lon1 = origin.x.to_radians();
    let bearing_rad = bearing_deg.to_radians();
    let angular_dist = distance_m / WGS84_A;

    let lat2 = (lat1.sin() * angular_dist.cos()
        + lat1.cos() * angular_dist.sin() * bearing_rad.cos())
    .asin();

    let lon2 = lon1
        + (bearing_rad.sin() * angular_dist.sin() * lat1.cos())
            .atan2(angular_dist.cos() - lat1.sin() * lat2.sin());

    let lat2_deg = lat2.to_degrees();
    let lon2_deg = ((lon2.to_degrees() + 540.0) % 360.0) - 180.0;

    validate_coord(lon2_deg, lat2_deg)?;

    Ok(Coord {
        x: lon2_deg,
        y: lat2_deg,
    })
}

/// Convert degrees to decimal degrees from DMS (degrees, minutes, seconds)
///
/// # Example
/// ```
/// use geospatial_analysis::coords::dms_to_decimal;
///
/// let lat = dms_to_decimal(38, 43, 20.28).unwrap();
/// assert!((lat - 38.7223).abs() < 0.0001);
/// ```
pub fn dms_to_decimal(degrees: i32, minutes: u32, seconds: f64) -> Result<f64> {
    if minutes >= 60 {
        return Err(GeoError::InvalidParameter(format!(
            "Invalid minutes: {}",
            minutes
        )));
    }
    if seconds >= 60.0 {
        return Err(GeoError::InvalidParameter(format!(
            "Invalid seconds: {}",
            seconds
        )));
    }

    let sign = if degrees < 0 { -1.0 } else { 1.0 };
    let decimal = degrees.abs() as f64 + (minutes as f64 / 60.0) + (seconds / 3600.0);

    Ok(sign * decimal)
}

/// Convert decimal degrees to DMS (degrees, minutes, seconds)
///
/// Returns (degrees, minutes, seconds)
///
/// # Example
/// ```
/// use geospatial_analysis::coords::decimal_to_dms;
///
/// let (deg, min, sec) = decimal_to_dms(38.7223);
/// assert_eq!(deg, 38);
/// assert_eq!(min, 43);
/// assert!((sec - 20.28).abs() < 0.01);
/// ```
pub fn decimal_to_dms(decimal: f64) -> (i32, u32, f64) {
    let sign = if decimal < 0.0 { -1 } else { 1 };
    let abs_decimal = decimal.abs();

    let degrees = abs_decimal.floor() as i32 * sign;
    let minutes_decimal = (abs_decimal - abs_decimal.floor()) * 60.0;
    let minutes = minutes_decimal.floor() as u32;
    let seconds = (minutes_decimal - minutes_decimal.floor()) * 60.0;

    (degrees, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_web_mercator_project_lisbon() {
        let lisbon = Coord {
            x: -9.1393,
            y: 38.7223,
        };
        let projected = WebMercator::project(&lisbon).unwrap();

        assert_relative_eq!(projected.x, -1_017_876.0, epsilon = 1.0);
        assert_relative_eq!(projected.y, 4_692_197.0, epsilon = 1.0);
    }

    #[test]
    fn test_web_mercator_roundtrip() {
        let original = Coord {
            x: -9.1393,
            y: 38.7223,
        };
        let projected = WebMercator::project(&original).unwrap();
        let unprojected = WebMercator::unproject(&projected).unwrap();

        assert_relative_eq!(unprojected.x, original.x, epsilon = 0.0001);
        assert_relative_eq!(unprojected.y, original.y, epsilon = 0.0001);
    }

    #[test]
    fn test_utm_zone_portugal() {
        let lisbon = Coord {
            x: -9.1393,
            y: 38.7223,
        };
        let porto = Coord {
            x: -8.6291,
            y: 41.1579,
        };

        assert_eq!(UTM::zone_from_lon(lisbon.x), 29);
        assert_eq!(UTM::zone_from_lon(porto.x), 29);

        assert_eq!(UTM::zone_designation(&lisbon).unwrap(), "29N");
        assert_eq!(UTM::zone_designation(&porto).unwrap(), "29N");
    }

    #[test]
    fn test_destination_point() {
        let lisbon = Coord {
            x: -9.1393,
            y: 38.7223,
        };

        // 10km north
        let dest = destination_point(&lisbon, 10000.0, 0.0).unwrap();
        assert_relative_eq!(dest.x, lisbon.x, epsilon = 0.01);
        assert!(dest.y > lisbon.y);

        // 10km east
        let dest = destination_point(&lisbon, 10000.0, 90.0).unwrap();
        assert!(dest.x > lisbon.x);
        assert_relative_eq!(dest.y, lisbon.y, epsilon = 0.01);
    }

    #[test]
    fn test_dms_conversion() {
        let decimal = dms_to_decimal(38, 43, 20.28).unwrap();
        assert_relative_eq!(decimal, 38.7223, epsilon = 0.0001);

        let (deg, min, sec) = decimal_to_dms(38.7223);
        assert_eq!(deg, 38);
        assert_eq!(min, 43);
        assert_relative_eq!(sec, 20.28, epsilon = 0.01);
    }

    #[test]
    fn test_dms_negative() {
        let decimal = dms_to_decimal(-9, 8, 21.48).unwrap();
        assert_relative_eq!(decimal, -9.1393, epsilon = 0.0001);
    }
}
