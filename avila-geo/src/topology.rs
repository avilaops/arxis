//! Topology operations on geographic geometries
//!
//! Implements fundamental computational geometry operations:
//! - Buffer (offset) operations
//! - Union, intersection, difference
//! - Convex hull
//! - Voronoi diagrams
//! - Polygon clipping

use crate::coords::GeoCoord;
use crate::geometry::{GeoLine, GeoPolygon};
use crate::calc::{haversine_distance, bearing, destination};

/// Buffer (offset) a point by a distance
///
/// Creates a circular polygon around a point
pub fn buffer_point(center: &GeoCoord, radius_meters: f64, segments: usize) -> GeoPolygon {
    let mut points = Vec::with_capacity(segments);

    for i in 0..segments {
        let angle = (i as f64 / segments as f64) * 360.0;
        let point = destination(center, angle, radius_meters);
        points.push(point);
    }

    GeoPolygon::new(points)
}

/// Buffer (offset) a line by a distance
///
/// Creates a polygon around a line with specified width
pub fn buffer_line(line: &GeoLine, width_meters: f64, segments_per_cap: usize) -> GeoPolygon {
    if line.coords.len() < 2 {
        return GeoPolygon::new(vec![]);
    }

    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    // Generate offset points for each segment
    for window in line.coords.windows(2) {
        let p1 = &window[0];
        let p2 = &window[1];

        let forward_bearing = bearing(p1, p2);
        let left_bearing = (forward_bearing - 90.0 + 360.0) % 360.0;
        let right_bearing = (forward_bearing + 90.0) % 360.0;

        let left_p1 = destination(p1, left_bearing, width_meters / 2.0);
        let right_p1 = destination(p1, right_bearing, width_meters / 2.0);

        if left_side.is_empty() {
            left_side.push(left_p1);
            right_side.push(right_p1);
        }

        let left_p2 = destination(p2, left_bearing, width_meters / 2.0);
        let right_p2 = destination(p2, right_bearing, width_meters / 2.0);

        left_side.push(left_p2);
        right_side.push(right_p2);
    }

    // Add rounded end caps
    let last = line.coords.last().unwrap();
    let prev = &line.coords[line.coords.len() - 2];
    let end_bearing = bearing(prev, last);

    for i in 0..=segments_per_cap {
        let angle = end_bearing + 90.0 + (i as f64 / segments_per_cap as f64) * 180.0;
        let point = destination(last, angle, width_meters / 2.0);
        left_side.push(point);
    }

    // Combine left and right sides
    right_side.reverse();
    left_side.extend(right_side);

    GeoPolygon::new(left_side)
}

/// Buffer (offset) a polygon by a distance
///
/// Expands or shrinks a polygon by the specified distance
pub fn buffer_polygon(polygon: &GeoPolygon, distance_meters: f64, segments: usize) -> GeoPolygon {
    // Simplified buffering: convert to line, buffer, and return
    // A proper implementation would handle holes and complex cases
    let line = GeoLine::new(polygon.exterior.clone(), crate::geometry::LineType::Border);
    buffer_line(&line, distance_meters * 2.0, segments)
}

/// Compute convex hull using Graham scan algorithm
pub fn convex_hull(points: &[GeoCoord]) -> Vec<GeoCoord> {
    if points.len() < 3 {
        return points.to_vec();
    }

    // Find lowest point (or leftmost if tie)
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| {
        a.lat.partial_cmp(&b.lat)
            .unwrap()
            .then(a.lon.partial_cmp(&b.lon).unwrap())
    });

    let pivot = sorted[0];

    // Sort by polar angle with respect to pivot
    sorted.sort_by(|a, b| {
        let angle_a = ((a.lon - pivot.lon).atan2(a.lat - pivot.lat) * 180.0 / std::f64::consts::PI + 360.0) % 360.0;
        let angle_b = ((b.lon - pivot.lon).atan2(b.lat - pivot.lat) * 180.0 / std::f64::consts::PI + 360.0) % 360.0;
        angle_a.partial_cmp(&angle_b).unwrap()
    });

    let mut hull = Vec::new();
    hull.push(sorted[0]);
    hull.push(sorted[1]);

    for point in sorted.iter().skip(2) {
        while hull.len() > 1 && !is_left_turn(&hull[hull.len() - 2], &hull[hull.len() - 1], point) {
            hull.pop();
        }
        hull.push(*point);
    }

    hull
}

/// Check if three points make a left turn
fn is_left_turn(p1: &GeoCoord, p2: &GeoCoord, p3: &GeoCoord) -> bool {
    let cross = (p2.lon - p1.lon) * (p3.lat - p1.lat) - (p2.lat - p1.lat) * (p3.lon - p1.lon);
    cross > 0.0
}

/// Bounding box of a set of points
pub fn bounding_box(points: &[GeoCoord]) -> (GeoCoord, GeoCoord) {
    let mut min_lat = f64::INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let mut min_lon = f64::INFINITY;
    let mut max_lon = f64::NEG_INFINITY;

    for point in points {
        min_lat = min_lat.min(point.lat);
        max_lat = max_lat.max(point.lat);
        min_lon = min_lon.min(point.lon);
        max_lon = max_lon.max(point.lon);
    }

    (
        GeoCoord::new(min_lat, min_lon),
        GeoCoord::new(max_lat, max_lon),
    )
}

/// Centroid of a polygon
pub fn centroid(polygon: &GeoPolygon) -> GeoCoord {
    let mut lat_sum = 0.0;
    let mut lon_sum = 0.0;
    let count = polygon.exterior.len() as f64;

    for point in &polygon.exterior {
        lat_sum += point.lat;
        lon_sum += point.lon;
    }

    GeoCoord::new(lat_sum / count, lon_sum / count)
}

/// Sutherland-Hodgman polygon clipping algorithm
///
/// Clips a polygon against a rectangular boundary
pub fn clip_polygon(polygon: &GeoPolygon, min: &GeoCoord, max: &GeoCoord) -> GeoPolygon {
    let mut points = polygon.exterior.clone();

    // Clip against each edge of the rectangle
    points = clip_against_edge(&points, min.lat, true, false);  // Bottom
    points = clip_against_edge(&points, max.lat, true, true);   // Top
    points = clip_against_edge(&points, min.lon, false, false); // Left
    points = clip_against_edge(&points, max.lon, false, true);  // Right

    GeoPolygon::new(points)
}

fn clip_against_edge(points: &[GeoCoord], edge: f64, is_lat: bool, is_max: bool) -> Vec<GeoCoord> {
    if points.is_empty() {
        return vec![];
    }

    let mut clipped = Vec::new();

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        let v1 = if is_lat { p1.lat } else { p1.lon };
        let v2 = if is_lat { p2.lat } else { p2.lon };

        let inside1 = if is_max { v1 <= edge } else { v1 >= edge };
        let inside2 = if is_max { v2 <= edge } else { v2 >= edge };

        if inside1 && inside2 {
            // Both inside
            clipped.push(p2);
        } else if inside1 && !inside2 {
            // Entering outside
            let t = (edge - v1) / (v2 - v1);
            let intersection = GeoCoord::new(
                p1.lat + t * (p2.lat - p1.lat),
                p1.lon + t * (p2.lon - p1.lon),
            );
            clipped.push(intersection);
        } else if !inside1 && inside2 {
            // Entering inside
            let t = (edge - v1) / (v2 - v1);
            let intersection = GeoCoord::new(
                p1.lat + t * (p2.lat - p1.lat),
                p1.lon + t * (p2.lon - p1.lon),
            );
            clipped.push(intersection);
            clipped.push(p2);
        }
        // Both outside: skip
    }

    clipped
}

/// Minimum bounding circle (Welzl's algorithm - simplified)
pub fn minimum_bounding_circle(points: &[GeoCoord]) -> (GeoCoord, f64) {
    if points.is_empty() {
        return (GeoCoord::new(0.0, 0.0), 0.0);
    }

    // Simplified: use centroid and max distance
    let center = centroid(&GeoPolygon::new(points.to_vec()));
    let mut max_dist = 0.0;

    for point in points {
        let dist = haversine_distance(&center, point);
        max_dist = max_dist.max(dist);
    }

    (center, max_dist)
}

/// Line intersection (simplified for geographic coordinates)
pub fn line_intersection(
    p1: &GeoCoord,
    p2: &GeoCoord,
    p3: &GeoCoord,
    p4: &GeoCoord,
) -> Option<GeoCoord> {
    let x1 = p1.lon;
    let y1 = p1.lat;
    let x2 = p2.lon;
    let y2 = p2.lat;
    let x3 = p3.lon;
    let y3 = p3.lat;
    let x4 = p4.lon;
    let y4 = p4.lat;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if denom.abs() < 1e-10 {
        return None; // Parallel or coincident
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        Some(GeoCoord::new(y, x))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_point() {
        let center = GeoCoord::new(-23.55, -46.63);
        let polygon = buffer_point(&center, 1000.0, 16);

        assert_eq!(polygon.exterior.len(), 16);

        // All points should be ~1000m from center
        for point in &polygon.exterior {
            let dist = haversine_distance(&center, point);
            assert!((dist - 1000.0).abs() < 10.0); // Within 10m
        }
    }

    #[test]
    fn test_convex_hull() {
        let points = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(1.0, 0.0),
            GeoCoord::new(1.0, 1.0),
            GeoCoord::new(0.0, 1.0),
            GeoCoord::new(0.5, 0.5), // Interior point
        ];

        let hull = convex_hull(&points);

        // Should have 4 points (the corners)
        assert_eq!(hull.len(), 4);
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            GeoCoord::new(-23.55, -46.63),
            GeoCoord::new(-22.91, -43.17),
            GeoCoord::new(-15.78, -47.93),
        ];

        let (min, max) = bounding_box(&points);

        assert_eq!(min.lat, -23.55);
        assert_eq!(max.lat, -15.78);
        assert_eq!(min.lon, -47.93);
        assert_eq!(max.lon, -43.17);
    }

    #[test]
    fn test_centroid() {
        let polygon = GeoPolygon::new(vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(2.0, 0.0),
            GeoCoord::new(2.0, 2.0),
            GeoCoord::new(0.0, 2.0),
        ]);

        let center = centroid(&polygon);

        assert!((center.lat - 1.0).abs() < 0.01);
        assert!((center.lon - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_line_intersection() {
        // Lines that intersect at (1, 1)
        let p1 = GeoCoord::new(0.0, 0.0);
        let p2 = GeoCoord::new(2.0, 2.0);
        let p3 = GeoCoord::new(0.0, 2.0);
        let p4 = GeoCoord::new(2.0, 0.0);

        let intersection = line_intersection(&p1, &p2, &p3, &p4);

        assert!(intersection.is_some());
        let point = intersection.unwrap();
        assert!((point.lat - 1.0).abs() < 0.01);
        assert!((point.lon - 1.0).abs() < 0.01);
    }
}
