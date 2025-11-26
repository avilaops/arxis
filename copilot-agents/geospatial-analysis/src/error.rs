//! Error types for geospatial operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, GeoError>;

#[derive(Error, Debug)]
pub enum GeoError {
    #[error("Invalid longitude: {0} (must be between -180 and 180)")]
    InvalidLongitude(f64),

    #[error("Invalid latitude: {0} (must be between -90 and 90)")]
    InvalidLatitude(f64),

    #[error("Invalid coordinate: ({lon}, {lat})")]
    InvalidCoordinate { lon: f64, lat: f64 },

    #[error("Empty geometry")]
    EmptyGeometry,

    #[error("Invalid polygon: {0}")]
    InvalidPolygon(String),

    #[error("Invalid network: {0}")]
    InvalidNetwork(String),

    #[error("Coordinate transformation failed: {0}")]
    TransformationError(String),

    #[error("Algorithm convergence failed: {0}")]
    ConvergenceError(String),

    #[error("Spatial index error: {0}")]
    IndexError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Validate longitude value
pub fn validate_longitude(lon: f64) -> Result<()> {
    if !lon.is_finite() || lon < -180.0 || lon > 180.0 {
        return Err(GeoError::InvalidLongitude(lon));
    }
    Ok(())
}

/// Validate latitude value
pub fn validate_latitude(lat: f64) -> Result<()> {
    if !lat.is_finite() || lat < -90.0 || lat > 90.0 {
        return Err(GeoError::InvalidLatitude(lat));
    }
    Ok(())
}

/// Validate coordinate pair
pub fn validate_coord(lon: f64, lat: f64) -> Result<()> {
    validate_longitude(lon)?;
    validate_latitude(lat)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_longitude() {
        assert!(validate_longitude(0.0).is_ok());
        assert!(validate_longitude(-180.0).is_ok());
        assert!(validate_longitude(180.0).is_ok());
        assert!(validate_longitude(-181.0).is_err());
        assert!(validate_longitude(181.0).is_err());
        assert!(validate_longitude(f64::NAN).is_err());
        assert!(validate_longitude(f64::INFINITY).is_err());
    }

    #[test]
    fn test_validate_latitude() {
        assert!(validate_latitude(0.0).is_ok());
        assert!(validate_latitude(-90.0).is_ok());
        assert!(validate_latitude(90.0).is_ok());
        assert!(validate_latitude(-91.0).is_err());
        assert!(validate_latitude(91.0).is_err());
        assert!(validate_latitude(f64::NAN).is_err());
    }

    #[test]
    fn test_validate_coord() {
        assert!(validate_coord(-9.1393, 38.7223).is_ok()); // Lisbon
        assert!(validate_coord(-8.6291, 41.1579).is_ok()); // Porto
        assert!(validate_coord(200.0, 38.7223).is_err());
        assert!(validate_coord(-9.1393, 100.0).is_err());
    }
}
