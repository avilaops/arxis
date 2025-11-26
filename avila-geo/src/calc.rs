//! Geographic calculations and algorithms
//!
//! This module provides mathematical calculations for geographic data:
//! - Distance calculations (Haversine, Vincenty)
//! - Area calculations (Shoelace, Spherical)
//! - Point-in-polygon tests
//! - Line simplification (Douglas-Peucker)
//! - Bearing and navigation

use crate::coords::GeoCoord;
use std::f64::consts::PI;

/// Earth radius in meters (mean radius)
pub const EARTH_RADIUS_M: f64 = 6371000.0;

/// Haversine distance between two coordinates (in meters)
///
/// Most accurate for small distances. Uses spherical Earth approximation.
///
/// # Formula
/// ```text
/// a = sin²(Δφ/2) + cos(φ1)⋅cos(φ2)⋅sin²(Δλ/2)
/// c = 2⋅atan2(√a, √(1−a))
/// d = R⋅c
/// ```
pub fn haversine_distance(p1: &GeoCoord, p2: &GeoCoord) -> f64 {
    let r1 = p1.to_radians();
    let r2 = p2.to_radians();

    let dlat = r2.lat - r1.lat;
    let dlon = r2.lon - r1.lon;

    let a = (dlat / 2.0).sin().powi(2)
        + r1.lat.cos() * r2.lat.cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_M * c
}

/// Vincenty distance (more accurate, uses ellipsoid)
///
/// Uses WGS84 ellipsoid parameters. More accurate than Haversine
/// but computationally more expensive.
pub fn vincenty_distance(p1: &GeoCoord, p2: &GeoCoord) -> f64 {
    const A: f64 = 6378137.0; // Semi-major axis
    const B: f64 = 6356752.314245; // Semi-minor axis
    const F: f64 = 1.0 / 298.257223563; // Flattening

    let r1 = p1.to_radians();
    let r2 = p2.to_radians();

    let l = r2.lon - r1.lon;
    let u1 = ((1.0 - F) * r1.lat.tan()).atan();
    let u2 = ((1.0 - F) * r2.lat.tan()).atan();

    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    let mut lambda = l;
    let mut iter_limit = 100;
    let mut cos_sq_alpha;
    let mut sin_sigma;
    let mut cos_sigma;
    let mut sigma;
    let mut cos2_sigma_m;

    loop {
        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();

        sin_sigma = ((cos_u2 * sin_lambda).powi(2)
            + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2))
        .sqrt();

        if sin_sigma.abs() < 1e-12 {
            return 0.0; // Coincident points
        }

        cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        sigma = sin_sigma.atan2(cos_sigma);

        let sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
        cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;

        cos2_sigma_m = if cos_sq_alpha.abs() < 1e-12 {
            0.0
        } else {
            cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_sq_alpha
        };

        let c = F / 16.0 * cos_sq_alpha * (4.0 + F * (4.0 - 3.0 * cos_sq_alpha));

        let lambda_prev = lambda;
        lambda = l
            + (1.0 - c)
                * F
                * sin_alpha
                * (sigma + c * sin_sigma * (cos2_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))));

        iter_limit -= 1;
        if (lambda - lambda_prev).abs() < 1e-12 || iter_limit == 0 {
            break;
        }
    }

    let u_sq = cos_sq_alpha * (A * A - B * B) / (B * B);
    let a_coef = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b_coef = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

    let delta_sigma = b_coef
        * sin_sigma
        * (cos2_sigma_m
            + b_coef / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))
                    - b_coef / 6.0 * cos2_sigma_m * (-3.0 + 4.0 * sin_sigma.powi(2)) * (-3.0 + 4.0 * cos2_sigma_m.powi(2))));

    B * a_coef * (sigma - delta_sigma)
}

/// Calculate bearing (forward azimuth) from p1 to p2
///
/// Returns bearing in degrees (0-360), where:
/// - 0° = North
/// - 90° = East
/// - 180° = South
/// - 270° = West
pub fn bearing(p1: &GeoCoord, p2: &GeoCoord) -> f64 {
    let r1 = p1.to_radians();
    let r2 = p2.to_radians();

    let dlon = r2.lon - r1.lon;

    let y = dlon.sin() * r2.lat.cos();
    let x = r1.lat.cos() * r2.lat.sin() - r1.lat.sin() * r2.lat.cos() * dlon.cos();

    let bearing_rad = y.atan2(x);
    (bearing_rad.to_degrees() + 360.0) % 360.0
}

/// Calculate destination point given distance and bearing
pub fn destination(start: &GeoCoord, distance_m: f64, bearing_deg: f64) -> GeoCoord {
    let r = start.to_radians();
    let bearing_rad = bearing_deg.to_radians();

    let angular_distance = distance_m / EARTH_RADIUS_M;

    let lat2 = (r.lat.sin() * angular_distance.cos()
        + r.lat.cos() * angular_distance.sin() * bearing_rad.cos())
    .asin();

    let lon2 = r.lon
        + (bearing_rad.sin() * angular_distance.sin() * r.lat.cos())
            .atan2(angular_distance.cos() - r.lat.sin() * lat2.sin());

    GeoCoord::new_unchecked(lat2.to_degrees(), lon2.to_degrees())
}

/// Shoelace formula for polygon area (in square degrees)
///
/// Simple planar approximation. Use `spherical_area` for accurate results.
pub fn shoelace_area(coords: &[GeoCoord]) -> f64 {
    if coords.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        area += coords[i].lon * coords[j].lat;
        area -= coords[j].lon * coords[i].lat;
    }

    area.abs() / 2.0
}

/// Spherical area calculation (in square meters)
///
/// Uses spherical excess formula for accurate area on sphere
pub fn spherical_area(coords: &[GeoCoord]) -> f64 {
    if coords.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    let n = coords.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let p1 = coords[i].to_radians();
        let p2 = coords[j].to_radians();

        area += (p2.lon - p1.lon) * (2.0 + p1.lat.sin() + p2.lat.sin());
    }

    area = area.abs() * EARTH_RADIUS_M * EARTH_RADIUS_M / 2.0;
    area
}

/// Point-in-polygon test using ray casting algorithm
///
/// Returns true if point is inside polygon
pub fn point_in_polygon(point: &GeoCoord, polygon: &[GeoCoord]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let pi = &polygon[i];
        let pj = &polygon[j];

        if ((pi.lat > point.lat) != (pj.lat > point.lat))
            && (point.lon
                < (pj.lon - pi.lon) * (point.lat - pi.lat) / (pj.lat - pi.lat) + pi.lon)
        {
            inside = !inside;
        }

        j = i;
    }

    inside
}

/// Douglas-Peucker line simplification algorithm
///
/// Reduces number of points in a line while preserving its shape
///
/// # Arguments
/// * `points` - Input line coordinates
/// * `epsilon` - Distance tolerance (in degrees)
pub fn douglas_peucker(points: &[GeoCoord], epsilon: f64) -> Vec<GeoCoord> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    // Find point with maximum distance from line
    let mut max_dist = 0.0;
    let mut index = 0;

    for i in 1..points.len() - 1 {
        let dist = perpendicular_distance(&points[i], &points[0], &points[points.len() - 1]);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }

    // If max distance is greater than epsilon, recursively simplify
    if max_dist > epsilon {
        let mut left = douglas_peucker(&points[0..=index], epsilon);
        let right = douglas_peucker(&points[index..], epsilon);

        left.pop(); // Remove duplicate point
        left.extend(right);
        left
    } else {
        vec![points[0], points[points.len() - 1]]
    }
}

/// Calculate perpendicular distance from point to line segment
fn perpendicular_distance(point: &GeoCoord, line_start: &GeoCoord, line_end: &GeoCoord) -> f64 {
    let dx = line_end.lon - line_start.lon;
    let dy = line_end.lat - line_start.lat;

    if dx.abs() < 1e-10 && dy.abs() < 1e-10 {
        // Line start and end are the same
        return ((point.lon - line_start.lon).powi(2) + (point.lat - line_start.lat).powi(2)).sqrt();
    }

    let numerator = ((line_end.lat - line_start.lat) * point.lon
        - (line_end.lon - line_start.lon) * point.lat
        + line_end.lon * line_start.lat
        - line_end.lat * line_start.lon)
        .abs();

    let denominator = (dx * dx + dy * dy).sqrt();

    numerator / denominator
}

/// Calculate midpoint between two coordinates
pub fn midpoint(p1: &GeoCoord, p2: &GeoCoord) -> GeoCoord {
    let r1 = p1.to_radians();
    let r2 = p2.to_radians();

    let dlon = r2.lon - r1.lon;

    let bx = r2.lat.cos() * dlon.cos();
    let by = r2.lat.cos() * dlon.sin();

    let lat = (r1.lat.sin() + r2.lat.sin()).atan2(
        ((r1.lat.cos() + bx).powi(2) + by.powi(2)).sqrt()
    );

    let lon = r1.lon + by.atan2(r1.lat.cos() + bx);

    GeoCoord::new_unchecked(lat.to_degrees(), lon.to_degrees())
}

/// Interpolate point along great circle
///
/// # Arguments
/// * `p1` - Start point
/// * `p2` - End point
/// * `fraction` - Fraction along path (0.0 = p1, 1.0 = p2)
pub fn interpolate(p1: &GeoCoord, p2: &GeoCoord, fraction: f64) -> GeoCoord {
    if fraction <= 0.0 {
        return *p1;
    }
    if fraction >= 1.0 {
        return *p2;
    }

    let r1 = p1.to_radians();
    let r2 = p2.to_radians();

    let d = haversine_distance(p1, p2) / EARTH_RADIUS_M; // Angular distance

    let a = (d * (1.0 - fraction)).sin() / d.sin();
    let b = (d * fraction).sin() / d.sin();

    let x = a * r1.lat.cos() * r1.lon.cos() + b * r2.lat.cos() * r2.lon.cos();
    let y = a * r1.lat.cos() * r1.lon.sin() + b * r2.lat.cos() * r2.lon.sin();
    let z = a * r1.lat.sin() + b * r2.lat.sin();

    let lat = z.atan2((x * x + y * y).sqrt());
    let lon = y.atan2(x);

    GeoCoord::new_unchecked(lat.to_degrees(), lon.to_degrees())
}

/// Calculate cross-track distance (distance from point to great circle path)
///
/// Returns signed distance in meters. Positive means point is right of path.
pub fn cross_track_distance(point: &GeoCoord, path_start: &GeoCoord, path_end: &GeoCoord) -> f64 {
    let d13 = haversine_distance(path_start, point) / EARTH_RADIUS_M;
    let bearing13 = bearing(path_start, point).to_radians();
    let bearing12 = bearing(path_start, path_end).to_radians();

    let dxt = (d13.sin() * (bearing13 - bearing12).sin()).asin();
    dxt * EARTH_RADIUS_M
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_distance() {
        // Distance from New York to London
        let nyc = GeoCoord::new(40.7128, -74.0060);
        let london = GeoCoord::new(51.5074, -0.1278);

        let dist = haversine_distance(&nyc, &london);

        // Should be approximately 5,570 km
        assert!((dist - 5570000.0).abs() < 50000.0);
    }

    #[test]
    fn test_bearing() {
        let p1 = GeoCoord::new(0.0, 0.0);
        let p2 = GeoCoord::new(1.0, 0.0);

        let b = bearing(&p1, &p2);
        assert!((b - 0.0).abs() < 1.0); // Should be roughly north
    }

    #[test]
    fn test_point_in_polygon() {
        let polygon = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(0.0, 10.0),
            GeoCoord::new(10.0, 10.0),
            GeoCoord::new(10.0, 0.0),
        ];

        let inside = GeoCoord::new(5.0, 5.0);
        let outside = GeoCoord::new(15.0, 15.0);

        assert!(point_in_polygon(&inside, &polygon));
        assert!(!point_in_polygon(&outside, &polygon));
    }

    #[test]
    fn test_douglas_peucker() {
        let points = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(1.0, 0.1),
            GeoCoord::new(2.0, -0.1),
            GeoCoord::new(3.0, 0.0),
        ];

        let simplified = douglas_peucker(&points, 0.5);
        assert!(simplified.len() < points.len());
    }

    #[test]
    fn test_midpoint() {
        let p1 = GeoCoord::new(0.0, 0.0);
        let p2 = GeoCoord::new(10.0, 10.0);
        let mid = midpoint(&p1, &p2);

        assert!((mid.lat - 5.0).abs() < 0.5);
        assert!((mid.lon - 5.0).abs() < 0.5);
    }
}
