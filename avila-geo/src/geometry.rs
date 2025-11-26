//! Geographic geometries
//!
//! This module defines geometric primitives for cartography:
//! - Points (cities, landmarks)
//! - Lines (roads, rivers, borders)
//! - Polygons (countries, states, lakes)

use crate::coords::{GeoCoord, GeoBounds};
use std::collections::HashMap;

/// Geographic point with metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoPoint {
    pub coord: GeoCoord,
    pub properties: HashMap<String, String>,
}

impl GeoPoint {
    pub fn new(coord: GeoCoord) -> Self {
        Self {
            coord,
            properties: HashMap::new(),
        }
    }

    pub fn with_name(coord: GeoCoord, name: impl Into<String>) -> Self {
        let mut point = Self::new(coord);
        point.properties.insert("name".to_string(), name.into());
        point
    }

    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.properties.get("name").map(|s| s.as_str())
    }
}

/// Type of geographic line
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineType {
    /// Road or highway
    Road,
    /// River or stream
    River,
    /// Political border
    Border,
    /// Coastline
    Coastline,
    /// Railway
    Railway,
    /// Custom type
    Custom,
}

/// Geographic line (polyline)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoLine {
    pub points: Vec<GeoCoord>,
    pub line_type: LineType,
    pub properties: HashMap<String, String>,
}

impl GeoLine {
    pub fn new(points: Vec<GeoCoord>, line_type: LineType) -> Self {
        Self {
            points,
            line_type,
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    pub fn bounds(&self) -> Option<GeoBounds> {
        GeoBounds::from_coords(&self.points)
    }

    /// Calculate total length in degrees (approximate)
    pub fn length_degrees(&self) -> f64 {
        let mut length = 0.0;
        for i in 1..self.points.len() {
            let p1 = &self.points[i - 1];
            let p2 = &self.points[i];
            let dlat = p2.lat - p1.lat;
            let dlon = p2.lon - p1.lon;
            length += (dlat * dlat + dlon * dlon).sqrt();
        }
        length
    }

    /// Calculate total length in meters using Haversine formula
    pub fn length_meters(&self) -> f64 {
        use crate::calc::haversine_distance;
        let mut length = 0.0;
        for i in 1..self.points.len() {
            length += haversine_distance(&self.points[i - 1], &self.points[i]);
        }
        length
    }

    /// Simplify line using Douglas-Peucker algorithm
    pub fn simplify(&self, epsilon: f64) -> Self {
        use crate::calc::douglas_peucker;
        let simplified = douglas_peucker(&self.points, epsilon);
        Self {
            points: simplified,
            line_type: self.line_type,
            properties: self.properties.clone(),
        }
    }
}

/// Geographic polygon
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoPolygon {
    /// Exterior ring (clockwise or counterclockwise)
    pub exterior: Vec<GeoCoord>,
    /// Interior rings (holes)
    pub holes: Vec<Vec<GeoCoord>>,
    pub properties: HashMap<String, String>,
}

impl GeoPolygon {
    pub fn new(exterior: Vec<GeoCoord>) -> Self {
        Self {
            exterior,
            holes: Vec::new(),
            properties: HashMap::new(),
        }
    }

    pub fn with_holes(exterior: Vec<GeoCoord>, holes: Vec<Vec<GeoCoord>>) -> Self {
        Self {
            exterior,
            holes,
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    pub fn bounds(&self) -> Option<GeoBounds> {
        GeoBounds::from_coords(&self.exterior)
    }

    /// Calculate area using Shoelace formula (in square degrees)
    pub fn area_degrees(&self) -> f64 {
        use crate::calc::shoelace_area;
        let exterior_area = shoelace_area(&self.exterior);
        let holes_area: f64 = self.holes.iter().map(|h| shoelace_area(h)).sum();
        (exterior_area - holes_area).abs()
    }

    /// Calculate area in square meters using spherical excess
    pub fn area_meters(&self) -> f64 {
        use crate::calc::spherical_area;
        let exterior_area = spherical_area(&self.exterior);
        let holes_area: f64 = self.holes.iter().map(|h| spherical_area(h)).sum();
        (exterior_area - holes_area).abs()
    }

    /// Check if a point is inside the polygon
    pub fn contains(&self, point: &GeoCoord) -> bool {
        use crate::calc::point_in_polygon;

        // Must be inside exterior
        if !point_in_polygon(point, &self.exterior) {
            return false;
        }

        // Must not be inside any hole
        for hole in &self.holes {
            if point_in_polygon(point, hole) {
                return false;
            }
        }

        true
    }

    /// Get perimeter in degrees
    pub fn perimeter_degrees(&self) -> f64 {
        let line = GeoLine::new(self.exterior.clone(), LineType::Border);
        line.length_degrees()
    }

    /// Get perimeter in meters
    pub fn perimeter_meters(&self) -> f64 {
        let line = GeoLine::new(self.exterior.clone(), LineType::Border);
        line.length_meters()
    }

    /// Simplify polygon using Douglas-Peucker
    pub fn simplify(&self, epsilon: f64) -> Self {
        use crate::calc::douglas_peucker;

        let exterior = douglas_peucker(&self.exterior, epsilon);
        let holes = self.holes
            .iter()
            .map(|h| douglas_peucker(h, epsilon))
            .collect();

        Self {
            exterior,
            holes,
            properties: self.properties.clone(),
        }
    }
}

/// Multi-geometry: collection of different geometry types
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoCollection {
    pub points: Vec<GeoPoint>,
    pub lines: Vec<GeoLine>,
    pub polygons: Vec<GeoPolygon>,
    pub properties: HashMap<String, String>,
}

impl GeoCollection {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            lines: Vec::new(),
            polygons: Vec::new(),
            properties: HashMap::new(),
        }
    }

    pub fn add_point(&mut self, point: GeoPoint) {
        self.points.push(point);
    }

    pub fn add_line(&mut self, line: GeoLine) {
        self.lines.push(line);
    }

    pub fn add_polygon(&mut self, polygon: GeoPolygon) {
        self.polygons.push(polygon);
    }

    pub fn bounds(&self) -> Option<GeoBounds> {
        let mut all_coords = Vec::new();

        for point in &self.points {
            all_coords.push(point.coord);
        }

        for line in &self.lines {
            all_coords.extend(&line.points);
        }

        for polygon in &self.polygons {
            all_coords.extend(&polygon.exterior);
        }

        GeoBounds::from_coords(&all_coords)
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty() && self.lines.is_empty() && self.polygons.is_empty()
    }

    pub fn len(&self) -> usize {
        self.points.len() + self.lines.len() + self.polygons.len()
    }
}

impl Default for GeoCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create common geometries
pub mod shapes {
    use super::*;

    /// Create a rectangle polygon
    pub fn rectangle(min_lat: f64, max_lat: f64, min_lon: f64, max_lon: f64) -> GeoPolygon {
        let exterior = vec![
            GeoCoord::new(min_lat, min_lon),
            GeoCoord::new(max_lat, min_lon),
            GeoCoord::new(max_lat, max_lon),
            GeoCoord::new(min_lat, max_lon),
            GeoCoord::new(min_lat, min_lon), // Close the ring
        ];
        GeoPolygon::new(exterior)
    }

    /// Create a circle (approximated as polygon)
    pub fn circle(center: GeoCoord, radius_degrees: f64, segments: usize) -> GeoPolygon {
        use std::f64::consts::PI;

        let mut exterior = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let angle = 2.0 * PI * i as f64 / segments as f64;
            let lat = center.lat + radius_degrees * angle.cos();
            let lon = center.lon + radius_degrees * angle.sin();
            exterior.push(GeoCoord::new_unchecked(lat, lon));
        }

        GeoPolygon::new(exterior)
    }

    /// Create a grid of points
    pub fn grid(bounds: GeoBounds, rows: usize, cols: usize) -> Vec<GeoPoint> {
        let mut points = Vec::with_capacity(rows * cols);

        for i in 0..rows {
            for j in 0..cols {
                let lat = bounds.min_lat + (i as f64 / (rows - 1) as f64) * bounds.height();
                let lon = bounds.min_lon + (j as f64 / (cols - 1) as f64) * bounds.width();
                points.push(GeoPoint::new(GeoCoord::new(lat, lon)));
            }
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = GeoPoint::with_name(
            GeoCoord::new(-23.55, -46.63),
            "São Paulo"
        );
        assert_eq!(point.name(), Some("São Paulo"));
    }

    #[test]
    fn test_line_length() {
        let line = GeoLine::new(
            vec![
                GeoCoord::new(0.0, 0.0),
                GeoCoord::new(1.0, 0.0),
                GeoCoord::new(1.0, 1.0),
            ],
            LineType::Road,
        );

        let length = line.length_degrees();
        assert!(length > 0.0);
    }

    #[test]
    fn test_rectangle() {
        let rect = shapes::rectangle(-10.0, 10.0, -20.0, 20.0);
        assert_eq!(rect.exterior.len(), 5); // 4 corners + closing point
    }

    #[test]
    fn test_collection() {
        let mut collection = GeoCollection::new();
        collection.add_point(GeoPoint::new(GeoCoord::new(0.0, 0.0)));
        collection.add_line(GeoLine::new(
            vec![GeoCoord::new(0.0, 0.0), GeoCoord::new(1.0, 1.0)],
            LineType::Road,
        ));

        assert_eq!(collection.len(), 2);
        assert!(!collection.is_empty());
    }
}
