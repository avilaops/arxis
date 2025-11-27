//! Buffer Analysis - Zona de Influência
//!
//! Cria buffers (zonas de influência) ao redor de geometrias.
//! Similar ao ArcGIS Buffer tool.

use crate::coords::GeoCoord;
use crate::geometry::{GeoPoint, GeoLine, GeoPolygon};
use crate::calc::{haversine_distance, bearing, destination};

/// Cria buffer circular ao redor de um ponto
pub fn buffer_point(center: &GeoCoord, radius_meters: f64, segments: usize) -> Vec<GeoCoord> {
    let mut points = Vec::with_capacity(segments);
    let angle_step = 360.0 / segments as f64;

    for i in 0..segments {
        let angle = angle_step * i as f64;
        let point = destination(center, angle, radius_meters);
        points.push(point);
    }

    // Close the polygon
    if let Some(first) = points.first().cloned() {
        points.push(first);
    }

    points
}

/// Cria buffer ao redor de uma linha
pub fn buffer_line(line: &[GeoCoord], radius_meters: f64, segments_per_cap: usize) -> Vec<GeoCoord> {
    if line.len() < 2 {
        return Vec::new();
    }

    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    // Create offset parallel lines
    for window in line.windows(2) {
        let p1 = &window[0];
        let p2 = &window[1];

        let brng = bearing(p1, p2);
        let left_bearing = (brng - 90.0).rem_euclid(360.0);
        let right_bearing = (brng + 90.0).rem_euclid(360.0);

        let left1 = destination(p1, left_bearing, radius_meters);
        let right1 = destination(p1, right_bearing, radius_meters);

        left_side.push(left1);
        right_side.push(right1);
    }

    // Add end cap
    let last = line.last().unwrap();
    let left_last = destination(last, bearing(&line[line.len()-2], last) - 90.0, radius_meters);
    let right_last = destination(last, bearing(&line[line.len()-2], last) + 90.0, radius_meters);

    left_side.push(left_last);
    right_side.push(right_last);

    // Create semi-circle caps
    let start_cap = create_semicircle(&line[0], radius_meters, segments_per_cap, bearing(&line[1], &line[0]));
    let end_cap = create_semicircle(last, radius_meters, segments_per_cap, bearing(&line[line.len()-2], last));

    // Combine all parts
    let mut buffer = start_cap;
    buffer.extend(left_side);
    buffer.extend(end_cap);
    buffer.extend(right_side.into_iter().rev());

    // Close polygon
    if let Some(first) = buffer.first().cloned() {
        buffer.push(first);
    }

    buffer
}

/// Cria buffer ao redor de polígono
pub fn buffer_polygon(polygon: &[GeoCoord], radius_meters: f64, segments_per_corner: usize) -> Vec<GeoCoord> {
    if polygon.len() < 3 {
        return Vec::new();
    }

    let mut result = Vec::new();

    for i in 0..polygon.len() - 1 {
        let prev = if i == 0 { &polygon[polygon.len() - 2] } else { &polygon[i - 1] };
        let curr = &polygon[i];
        let next = &polygon[i + 1];

        // Calculate perpendicular offset
        let angle_in = bearing(prev, curr);
        let angle_out = bearing(curr, next);
        let corner_angle = (angle_in + angle_out) / 2.0;

        let offset_point = destination(curr, corner_angle + 90.0, radius_meters);
        result.push(offset_point);

        // Add rounded corner
        let corner = create_corner(curr, radius_meters, angle_in, angle_out, segments_per_corner);
        result.extend(corner);
    }

    // Close polygon
    if let Some(first) = result.first().cloned() {
        result.push(first);
    }

    result
}

/// Multi-ring buffer (múltiplas distâncias)
pub fn multi_ring_buffer(center: &GeoCoord, radii: &[f64], segments: usize) -> Vec<Vec<GeoCoord>> {
    radii.iter()
        .map(|&radius| buffer_point(center, radius, segments))
        .collect()
}

// === Helper Functions ===

fn create_semicircle(center: &GeoCoord, radius: f64, segments: usize, start_bearing: f64) -> Vec<GeoCoord> {
    let mut points = Vec::with_capacity(segments);
    let angle_step = 180.0 / segments as f64;

    for i in 0..=segments {
        let angle = start_bearing + angle_step * i as f64;
        points.push(destination(center, angle, radius));
    }

    points
}

fn create_corner(center: &GeoCoord, radius: f64, angle_in: f64, angle_out: f64, segments: usize) -> Vec<GeoCoord> {
    let mut points = Vec::with_capacity(segments);
    let start = angle_in + 90.0;
    let end = angle_out + 90.0;
    let angle_step = (end - start) / segments as f64;

    for i in 0..segments {
        let angle = start + angle_step * i as f64;
        points.push(destination(center, angle, radius));
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_point() {
        let center = GeoCoord::new(-23.55, -46.63);
        let buffer = buffer_point(&center, 1000.0, 32);

        assert_eq!(buffer.len(), 33); // 32 segments + closing point

        // All points should be ~1000m from center
        for point in &buffer[..32] {
            let dist = haversine_distance(&center, point);
            assert!((dist - 1000.0).abs() < 100.0); // 100m tolerance
        }
    }

    #[test]
    fn test_multi_ring_buffer() {
        let center = GeoCoord::new(0.0, 0.0);
        let radii = vec![1000.0, 2000.0, 3000.0];
        let buffers = multi_ring_buffer(&center, &radii, 16);

        assert_eq!(buffers.len(), 3);
        assert_eq!(buffers[0].len(), 17);
    }

    #[test]
    fn test_buffer_line() {
        let line = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(0.1, 0.1),
        ];
        let buffer = buffer_line(&line, 500.0, 8);

        assert!(!buffer.is_empty());
        // Buffer should form closed polygon
        assert_eq!(buffer.first(), buffer.last());
    }
}
