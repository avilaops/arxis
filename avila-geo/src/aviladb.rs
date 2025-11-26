//! AvilaDB Geospatial Integration
//!
//! Provides geospatial query capabilities for AvilaDB,
//! enabling efficient storage and retrieval of geographic data.
//!
//! ## Features
//!
//! - GeoJSON document storage in AvilaDB
//! - Spatial queries (bounding box, radius, polygon contains)
//! - Geospatial indexes for fast lookups
//! - Integration with avila-geo types
//!
//! ## Usage
//!
//! ```rust,ignore
//! use avila_geo::aviladb::{GeoDocument, GeoQuery};
//! use avila_geo::coords::GeoCoord;
//!
//! // Create a geo document
//! let doc = GeoDocument::new()
//!     .with_id("city_saopaulo")
//!     .with_name("São Paulo")
//!     .with_location(GeoCoord::new(-23.55, -46.63))
//!     .with_property("population", 12_300_000);
//!
//! // Query nearby locations
//! let query = GeoQuery::within_radius(
//!     GeoCoord::new(-23.55, -46.63),
//!     50_000.0 // 50km
//! );
//! ```

use crate::coords::{GeoCoord, GeoBounds};
use crate::geometry::{GeoPoint, GeoLine, GeoPolygon, GeoCollection};
use crate::calc::{haversine_distance, point_in_polygon};

use std::collections::HashMap;

/// Geographic document for AvilaDB storage
///
/// Represents a document with geospatial properties
#[derive(Debug, Clone)]
pub struct GeoDocument {
    pub id: String,
    pub geometry_type: GeometryType,
    pub coordinates: Vec<GeoCoord>,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Type of geometry stored in the document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
}

impl GeoDocument {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            id: String::new(),
            geometry_type: GeometryType::Point,
            coordinates: Vec::new(),
            properties: HashMap::new(),
        }
    }

    /// Set document ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Set a single point location
    pub fn with_location(mut self, coord: GeoCoord) -> Self {
        self.geometry_type = GeometryType::Point;
        self.coordinates = vec![coord];
        self
    }

    /// Set line coordinates
    pub fn with_line(mut self, coords: Vec<GeoCoord>) -> Self {
        self.geometry_type = GeometryType::LineString;
        self.coordinates = coords;
        self
    }

    /// Set polygon coordinates
    pub fn with_polygon(mut self, coords: Vec<GeoCoord>) -> Self {
        self.geometry_type = GeometryType::Polygon;
        self.coordinates = coords;
        self
    }

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Convert to GeoJSON
    pub fn to_geojson(&self) -> serde_json::Value {
        let geometry = match self.geometry_type {
            GeometryType::Point if self.coordinates.len() == 1 => {
                serde_json::json!({
                    "type": "Point",
                    "coordinates": [self.coordinates[0].lon, self.coordinates[0].lat]
                })
            }
            GeometryType::LineString => {
                let coords: Vec<[f64; 2]> = self.coordinates
                    .iter()
                    .map(|c| [c.lon, c.lat])
                    .collect();
                serde_json::json!({
                    "type": "LineString",
                    "coordinates": coords
                })
            }
            GeometryType::Polygon => {
                let coords: Vec<[f64; 2]> = self.coordinates
                    .iter()
                    .map(|c| [c.lon, c.lat])
                    .collect();
                serde_json::json!({
                    "type": "Polygon",
                    "coordinates": [coords]
                })
            }
            _ => serde_json::json!(null),
        };

        serde_json::json!({
            "type": "Feature",
            "id": self.id,
            "geometry": geometry,
            "properties": self.properties
        })
    }

    /// Parse from GeoJSON
    pub fn from_geojson(value: &serde_json::Value) -> Result<Self, String> {
        let id = value["id"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let geometry = &value["geometry"];
        let geom_type = geometry["type"]
            .as_str()
            .ok_or("Missing geometry type")?;

        let (geometry_type, coordinates) = match geom_type {
            "Point" => {
                let coords = geometry["coordinates"]
                    .as_array()
                    .ok_or("Invalid Point coordinates")?;
                let lon = coords[0].as_f64().ok_or("Invalid longitude")?;
                let lat = coords[1].as_f64().ok_or("Invalid latitude")?;
                (GeometryType::Point, vec![GeoCoord::new(lat, lon)])
            }
            "LineString" => {
                let coords = geometry["coordinates"]
                    .as_array()
                    .ok_or("Invalid LineString coordinates")?;
                let points: Result<Vec<GeoCoord>, String> = coords
                    .iter()
                    .map(|c| {
                        let arr = c.as_array().ok_or("Invalid coordinate")?;
                        let lon = arr[0].as_f64().ok_or("Invalid longitude")?;
                        let lat = arr[1].as_f64().ok_or("Invalid latitude")?;
                        Ok(GeoCoord::new(lat, lon))
                    })
                    .collect();
                (GeometryType::LineString, points?)
            }
            "Polygon" => {
                let coords = geometry["coordinates"]
                    .as_array()
                    .ok_or("Invalid Polygon coordinates")?
                    .get(0)
                    .ok_or("Empty Polygon")?
                    .as_array()
                    .ok_or("Invalid Polygon ring")?;
                let points: Result<Vec<GeoCoord>, String> = coords
                    .iter()
                    .map(|c| {
                        let arr = c.as_array().ok_or("Invalid coordinate")?;
                        let lon = arr[0].as_f64().ok_or("Invalid longitude")?;
                        let lat = arr[1].as_f64().ok_or("Invalid latitude")?;
                        Ok(GeoCoord::new(lat, lon))
                    })
                    .collect();
                (GeometryType::Polygon, points?)
            }
            _ => return Err(format!("Unsupported geometry type: {}", geom_type)),
        };

        let properties = value["properties"]
            .as_object()
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(Self {
            id,
            geometry_type,
            coordinates,
            properties,
        })
    }
}

/// Geospatial query builder
#[derive(Debug, Clone)]
pub struct GeoQuery {
    pub query_type: QueryType,
    pub filters: Vec<PropertyFilter>,
}

/// Type of geospatial query
#[derive(Debug, Clone)]
pub enum QueryType {
    /// Find all documents within a bounding box
    WithinBounds(GeoBounds),

    /// Find all documents within radius of a point
    WithinRadius {
        center: GeoCoord,
        radius_meters: f64,
    },

    /// Find all documents within a polygon
    WithinPolygon {
        polygon: Vec<GeoCoord>,
    },

    /// Find k nearest neighbors to a point
    NearestNeighbors {
        center: GeoCoord,
        k: usize,
    },
}

/// Property filter for additional constraints
#[derive(Debug, Clone)]
pub struct PropertyFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

/// Filter operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Contains,
}

impl GeoQuery {
    /// Query within bounding box
    pub fn within_bounds(bounds: GeoBounds) -> Self {
        Self {
            query_type: QueryType::WithinBounds(bounds),
            filters: Vec::new(),
        }
    }

    /// Query within radius
    pub fn within_radius(center: GeoCoord, radius_meters: f64) -> Self {
        Self {
            query_type: QueryType::WithinRadius { center, radius_meters },
            filters: Vec::new(),
        }
    }

    /// Query within polygon
    pub fn within_polygon(polygon: Vec<GeoCoord>) -> Self {
        Self {
            query_type: QueryType::WithinPolygon { polygon },
            filters: Vec::new(),
        }
    }

    /// Find k nearest neighbors
    pub fn nearest_neighbors(center: GeoCoord, k: usize) -> Self {
        Self {
            query_type: QueryType::NearestNeighbors { center, k },
            filters: Vec::new(),
        }
    }

    /// Add property filter
    pub fn with_filter(mut self, filter: PropertyFilter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Execute query on a collection of documents
    pub fn execute(&self, documents: &[GeoDocument]) -> Vec<GeoDocument> {
        let mut results: Vec<GeoDocument> = documents
            .iter()
            .filter(|doc| self.matches_geometry(doc))
            .filter(|doc| self.matches_filters(doc))
            .cloned()
            .collect();

        // Sort by distance for nearest neighbors
        if let QueryType::NearestNeighbors { center, k } = &self.query_type {
            results.sort_by(|a, b| {
                let dist_a = self.distance_to_center(a, center);
                let dist_b = self.distance_to_center(b, center);
                dist_a.partial_cmp(&dist_b).unwrap()
            });
            results.truncate(*k);
        }

        results
    }

    fn matches_geometry(&self, doc: &GeoDocument) -> bool {
        match &self.query_type {
            QueryType::WithinBounds(bounds) => {
                doc.coordinates.iter().any(|coord| bounds.contains(coord))
            }
            QueryType::WithinRadius { center, radius_meters } => {
                doc.coordinates.iter().any(|coord| {
                    haversine_distance(center, coord) <= *radius_meters
                })
            }
            QueryType::WithinPolygon { polygon } => {
                doc.coordinates.iter().any(|coord| {
                    point_in_polygon(coord, polygon)
                })
            }
            QueryType::NearestNeighbors { .. } => true, // Filter in post-processing
        }
    }

    fn matches_filters(&self, doc: &GeoDocument) -> bool {
        self.filters.iter().all(|filter| {
            if let Some(value) = doc.properties.get(&filter.field) {
                Self::compare_values(value, &filter.value, filter.operator)
            } else {
                false
            }
        })
    }

    fn compare_values(a: &serde_json::Value, b: &serde_json::Value, op: FilterOperator) -> bool {
        use FilterOperator::*;
        match op {
            Equal => a == b,
            NotEqual => a != b,
            GreaterThan => {
                if let (Some(a_num), Some(b_num)) = (a.as_f64(), b.as_f64()) {
                    a_num > b_num
                } else {
                    false
                }
            }
            LessThan => {
                if let (Some(a_num), Some(b_num)) = (a.as_f64(), b.as_f64()) {
                    a_num < b_num
                } else {
                    false
                }
            }
            GreaterOrEqual => {
                if let (Some(a_num), Some(b_num)) = (a.as_f64(), b.as_f64()) {
                    a_num >= b_num
                } else {
                    false
                }
            }
            LessOrEqual => {
                if let (Some(a_num), Some(b_num)) = (a.as_f64(), b.as_f64()) {
                    a_num <= b_num
                } else {
                    false
                }
            }
            Contains => {
                if let Some(a_str) = a.as_str() {
                    if let Some(b_str) = b.as_str() {
                        a_str.contains(b_str)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    fn distance_to_center(&self, doc: &GeoDocument, center: &GeoCoord) -> f64 {
        doc.coordinates
            .iter()
            .map(|coord| haversine_distance(center, coord))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f64::INFINITY)
    }
}

/// Geospatial index for efficient queries
///
/// Simple grid-based spatial index
pub struct GeoIndex {
    grid_size: f64,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl GeoIndex {
    /// Create a new spatial index with specified grid size (in degrees)
    pub fn new(grid_size: f64) -> Self {
        Self {
            grid_size,
            cells: HashMap::new(),
        }
    }

    /// Index a document
    pub fn insert(&mut self, doc_index: usize, coord: &GeoCoord) {
        let cell = self.coord_to_cell(coord);
        self.cells.entry(cell).or_insert_with(Vec::new).push(doc_index);
    }

    /// Find candidate documents within bounding box
    pub fn query_bounds(&self, bounds: &GeoBounds) -> Vec<usize> {
        let min_cell = self.coord_to_cell(&GeoCoord::new(bounds.min_lat, bounds.min_lon));
        let max_cell = self.coord_to_cell(&GeoCoord::new(bounds.max_lat, bounds.max_lon));

        let mut results = Vec::new();

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(docs) = self.cells.get(&(x, y)) {
                    results.extend_from_slice(docs);
                }
            }
        }

        results.sort_unstable();
        results.dedup();
        results
    }

    fn coord_to_cell(&self, coord: &GeoCoord) -> (i32, i32) {
        let x = (coord.lon / self.grid_size).floor() as i32;
        let y = (coord.lat / self.grid_size).floor() as i32;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_document() {
        let doc = GeoDocument::new()
            .with_id("sp")
            .with_location(GeoCoord::new(-23.55, -46.63))
            .with_property("name", "São Paulo")
            .with_property("population", 12_300_000);

        assert_eq!(doc.id, "sp");
        assert_eq!(doc.geometry_type, GeometryType::Point);
        assert_eq!(doc.coordinates.len(), 1);
    }

    #[test]
    fn test_within_radius_query() {
        let docs = vec![
            GeoDocument::new()
                .with_id("sp")
                .with_location(GeoCoord::new(-23.55, -46.63)),
            GeoDocument::new()
                .with_id("rj")
                .with_location(GeoCoord::new(-22.91, -43.17)),
        ];

        let query = GeoQuery::within_radius(
            GeoCoord::new(-23.55, -46.63),
            50_000.0
        );

        let results = query.execute(&docs);
        assert_eq!(results.len(), 1); // Only SP within 50km
        assert_eq!(results[0].id, "sp");
    }

    #[test]
    fn test_geo_index() {
        let mut index = GeoIndex::new(1.0);

        index.insert(0, &GeoCoord::new(-23.55, -46.63));
        index.insert(1, &GeoCoord::new(-22.91, -43.17));

        let bounds = GeoBounds {
            min_lat: -24.0,
            max_lat: -23.0,
            min_lon: -47.0,
            max_lon: -46.0,
        };

        let results = index.query_bounds(&bounds);
        assert_eq!(results, vec![0]); // Only SP in bounds
    }
}
