//! Distance calculations for geospatial analysis
//!
//! Provides multiple distance calculation methods for different use cases:
//! - **Haversine**: Fast spherical distance (accurate for short/medium distances)
//! - **Vincenty**: High-precision ellipsoidal distance (accurate for all distances)
//! - **Euclidean**: Planar distance (only for projected coordinates)
//! - **Manhattan**: Grid-based distance

use crate::error::{validate_coord, Result};
use crate::{EARTH_RADIUS_KM, EARTH_RADIUS_M, WGS84_A, WGS84_B, WGS84_F};
use geo::Coord;

/// Calculate Haversine distance between two coordinates (in kilometers)
///
/// Uses the Haversine formula to calculate great circle distance on a sphere.
/// Suitable for most applications with error < 0.5% for distances < 500km.
///
/// # Algorithm
/// Uses the Haversine formula:
/// ```text
/// a = sin²(Δφ/2) + cos(φ1) * cos(φ2) * sin²(Δλ/2)
/// c = 2 * atan2(√a, √(1−a))
/// d = R * c
/// ```
///
/// # Complexity
/// O(1) - constant time
///
/// # Example
/// ```
/// use geospatial_analysis::distance::haversine_distance;
/// use geo::Coord;
///
/// let lisbon = Coord { x: -9.1393, y: 38.7223 };
/// let porto = Coord { x: -8.6291, y: 41.1579 };
///
/// let distance = haversine_distance(&lisbon, &porto).unwrap();
/// assert!((distance - 274.0).abs() < 1.0); // ~274 km
/// ```
///
/// # References
/// - R.W. Sinnott, "Virtues of the Haversine", Sky and Telescope, vol. 68, no. 2, 1984
pub fn haversine_distance(from: &Coord<f64>, to: &Coord<f64>) -> Result<f64> {
    validate_coord(from.x, from.y)?;
    validate_coord(to.x, to.y)?;

    let lat1 = from.y.to_radians();
    let lat2 = to.y.to_radians();
    let delta_lat = (to.y - from.y).to_radians();
    let delta_lon = (to.x - from.x).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    Ok(EARTH_RADIUS_KM * c)
}

/// Calculate Haversine distance in meters
pub fn haversine_distance_m(from: &Coord<f64>, to: &Coord<f64>) -> Result<f64> {
    Ok(haversine_distance(from, to)? * 1000.0)
}

/// Calculate Vincenty distance between two coordinates (in meters)
///
/// Uses Vincenty's formulae for calculating geodesic distance on an ellipsoid.
/// Most accurate method, suitable for all distances and critical applications.
///
/// # Algorithm
/// Iterative formula accounting for Earth's ellipsoidal shape (WGS84).
/// Converges quickly (typically < 10 iterations) to millimeter precision.
///
/// # Complexity
/// O(1) - constant time with fixed iterations
///
/// # Example
/// ```
/// use geospatial_analysis::distance::vincenty_distance;
/// use geo::Coord;
///
/// let lisbon = Coord { x: -9.1393, y: 38.7223 };
/// let porto = Coord { x: -8.6291, y: 41.1579 };
///
/// let distance = vincenty_distance(&lisbon, &porto).unwrap();
/// assert!((distance - 274_135.0).abs() < 10.0); // ~274.135 km
/// ```
///
/// # References
/// - T. Vincenty, "Direct and Inverse Solutions of Geodesics on the Ellipsoid",
///   Survey Review, vol. 23, no. 176, 1975
pub fn vincenty_distance(from: &Coord<f64>, to: &Coord<f64>) -> Result<f64> {
    validate_coord(from.x, from.y)?;
    validate_coord(to.x, to.y)?;

    let lat1 = from.y.to_radians();
    let lat2 = to.y.to_radians();
    let lon1 = from.x.to_radians();
    let lon2 = to.x.to_radians();

    let l = lon2 - lon1;
    let u1 = ((1.0 - WGS84_F) * lat1.tan()).atan();
    let u2 = ((1.0 - WGS84_F) * lat2.tan()).atan();

    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    let mut lambda = l;
    let mut lambda_prev;
    let mut iter_limit = 100;

    let (sin_sigma, cos_sigma, sigma, sin_alpha, cos_sq_alpha, cos2_sigma_m);

    loop {
        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();

        sin_sigma = ((cos_u2 * sin_lambda).powi(2)
            + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2))
        .sqrt();

        if sin_sigma == 0.0 {
            return Ok(0.0); // Co-incident points
        }

        cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        sigma = sin_sigma.atan2(cos_sigma);
        sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
        cos_sq_alpha = 1.0 - sin_alpha.powi(2);

        cos2_sigma_m = if cos_sq_alpha != 0.0 {
            cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_sq_alpha
        } else {
            0.0 // Equatorial line
        };

        let c = WGS84_F / 16.0 * cos_sq_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos_sq_alpha));

        lambda_prev = lambda;
        lambda = l
            + (1.0 - c)
                * WGS84_F
                * sin_alpha
                * (sigma + c * sin_sigma * (cos2_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))));

        iter_limit -= 1;
        if (lambda - lambda_prev).abs() < 1e-12 || iter_limit == 0 {
            break;
        }
    }

    let u_sq = cos_sq_alpha * (WGS84_A.powi(2) - WGS84_B.powi(2)) / WGS84_B.powi(2);
    let a = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

    let delta_sigma = b
        * sin_sigma
        * (cos2_sigma_m
            + b / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))
                    - b / 6.0
                        * cos2_sigma_m
                        * (-3.0 + 4.0 * sin_sigma.powi(2))
                        * (-3.0 + 4.0 * cos2_sigma_m.powi(2))));

    let distance = WGS84_B * a * (sigma - delta_sigma);

    Ok(distance)
}

/// Calculate Euclidean (planar) distance
///
/// Simple 2D distance calculation. Only use with projected coordinates (not lat/lon).
///
/// # Example
/// ```
/// use geospatial_analysis::distance::euclidean_distance;
/// use geo::Coord;
///
/// let a = Coord { x: 0.0, y: 0.0 };
/// let b = Coord { x: 3.0, y: 4.0 };
///
/// let distance = euclidean_distance(&a, &b);
/// assert_eq!(distance, 5.0);
/// ```
pub fn euclidean_distance(from: &Coord<f64>, to: &Coord<f64>) -> f64 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    (dx * dx + dy * dy).sqrt()
}

/// Calculate Manhattan (taxicab) distance
///
/// Sum of absolute differences. Useful for grid-based routing.
///
/// # Example
/// ```
/// use geospatial_analysis::distance::manhattan_distance;
/// use geo::Coord;
///
/// let a = Coord { x: 0.0, y: 0.0 };
/// let b = Coord { x: 3.0, y: 4.0 };
///
/// let distance = manhattan_distance(&a, &b);
/// assert_eq!(distance, 7.0);
/// ```
pub fn manhattan_distance(from: &Coord<f64>, to: &Coord<f64>) -> f64 {
    (to.x - from.x).abs() + (to.y - from.y).abs()
}

/// Calculate bearing (azimuth) from one point to another
///
/// Returns bearing in degrees (0-360), where 0 is North, 90 is East, etc.
///
/// # Example
/// ```
/// use geospatial_analysis::distance::bearing;
/// use geo::Coord;
///
/// let lisbon = Coord { x: -9.1393, y: 38.7223 };
/// let porto = Coord { x: -8.6291, y: 41.1579 };
///
/// let bearing_deg = bearing(&lisbon, &porto).unwrap();
/// assert!((bearing_deg - 17.0).abs() < 1.0); // ~17° (North-Northeast)
/// ```
pub fn bearing(from: &Coord<f64>, to: &Coord<f64>) -> Result<f64> {
    validate_coord(from.x, from.y)?;
    validate_coord(to.x, to.y)?;

    let lat1 = from.y.to_radians();
    let lat2 = to.y.to_radians();
    let delta_lon = (to.x - from.x).to_radians();

    let y = delta_lon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * delta_lon.cos();

    let bearing_rad = y.atan2(x);
    let bearing_deg = (bearing_rad.to_degrees() + 360.0) % 360.0;

    Ok(bearing_deg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_haversine_lisbon_porto() {
        let lisbon = Coord {
            x: -9.1393,
            y: 38.7223,
        };
        let porto = Coord {
            x: -8.6291,
            y: 41.1579,
        };

        let distance = haversine_distance(&lisbon, &porto).unwrap();
        assert_relative_eq!(distance, 274.0, epsilon = 1.0);
    }

    #[test]
    fn test_vincenty_lisbon_porto() {
        let lisbon = Coord {
            x: -9.1393,
            y: 38.7223,
        };
        let porto = Coord {
            x: -8.6291,
            y: 41.1579,
        };

        let distance = vincenty_distance(&lisbon, &porto).unwrap();
        assert_relative_eq!(distance, 274_135.0, epsilon = 100.0);
    }

    #[test]
    fn test_same_point() {
        let lisbon = Coord {
            x: -9.1393,
            y: 38.7223,
        };

        assert_eq!(haversine_distance(&lisbon, &lisbon).unwrap(), 0.0);
        assert_eq!(vincenty_distance(&lisbon, &lisbon).unwrap(), 0.0);
    }

    #[test]
    fn test_euclidean() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 3.0, y: 4.0 };
        assert_eq!(euclidean_distance(&a, &b), 5.0);
    }

    #[test]
    fn test_manhattan() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 3.0, y: 4.0 };
        assert_eq!(manhattan_distance(&a, &b), 7.0);
    }

    #[test]
    fn test_bearing_north() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 0.0, y: 1.0 };
        let bearing_deg = bearing(&a, &b).unwrap();
        assert_relative_eq!(bearing_deg, 0.0, epsilon = 0.1);
    }

    #[test]
    fn test_bearing_east() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 1.0, y: 0.0 };
        let bearing_deg = bearing(&a, &b).unwrap();
        assert_relative_eq!(bearing_deg, 90.0, epsilon = 0.1);
    }
}
