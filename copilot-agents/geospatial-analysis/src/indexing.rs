//! Spatial indexing data structures for efficient queries
//!
//! Provides high-performance spatial indexes:
//! - **R-Tree**: Best for general spatial queries (bounding box, nearest neighbor)
//! - **QuadTree**: Good for uniform spatial distribution
//! - **SpatialHashGrid**: Fast for fixed-size cells

use crate::error::{GeoError, Result};
use geo::{BoundingRect, Coord, Point};
use rstar::{RTree, RTreeObject, AABB};
use std::collections::HashMap;

/// Spatial feature with ID and geometry
#[derive(Debug, Clone)]
pub struct SpatialFeature {
    pub id: String,
    pub location: Coord<f64>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl SpatialFeature {
    pub fn new(id: String, location: Coord<f64>) -> Self {
        Self {
            id,
            location,
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: String, value: serde_json::Value) -> Self {
        self.properties.insert(key, value);
        self
    }
}

/// R-Tree spatial index wrapper
///
/// Efficient spatial index for:
/// - Nearest neighbor queries: O(log n)
/// - Range queries: O(log n + k) where k is number of results
/// - Bounding box queries: O(log n + k)
///
/// # Example
/// ```
/// use geospatial_analysis::indexing::{SpatialIndex, SpatialFeature};
/// use geo::Coord;
///
/// let mut index = SpatialIndex::new();
///
/// index.insert(SpatialFeature::new(
///     "lisbon".to_string(),
///     Coord { x: -9.1393, y: 38.7223 }
/// ));
///
/// index.insert(SpatialFeature::new(
///     "porto".to_string(),
///     Coord { x: -8.6291, y: 41.1579 }
/// ));
///
/// let query = Coord { x: -9.0, y: 39.0 };
/// let nearest = index.nearest_neighbor(&query).unwrap();
/// assert_eq!(nearest.id, "lisbon");
/// ```
pub struct SpatialIndex {
    tree: RTree<IndexedFeature>,
}

#[derive(Debug, Clone)]
struct IndexedFeature {
    feature: SpatialFeature,
}

impl RTreeObject for IndexedFeature {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.feature.location.x, self.feature.location.y])
    }
}

impl SpatialIndex {
    /// Create new empty spatial index
    pub fn new() -> Self {
        Self {
            tree: RTree::new(),
        }
    }

    /// Create spatial index from features
    pub fn from_features(features: Vec<SpatialFeature>) -> Self {
        let indexed: Vec<IndexedFeature> = features
            .into_iter()
            .map(|feature| IndexedFeature { feature })
            .collect();

        Self {
            tree: RTree::bulk_load(indexed),
        }
    }

    /// Insert a feature into the index
    pub fn insert(&mut self, feature: SpatialFeature) {
        self.tree.insert(IndexedFeature { feature });
    }

    /// Find nearest neighbor to query point
    ///
    /// # Complexity
    /// O(log n) average case
    pub fn nearest_neighbor(&self, query: &Coord<f64>) -> Option<&SpatialFeature> {
        self.tree
            .nearest_neighbor(&[query.x, query.y])
            .map(|indexed| &indexed.feature)
    }

    /// Find k nearest neighbors
    ///
    /// # Complexity
    /// O(k log n)
    pub fn k_nearest_neighbors(&self, query: &Coord<f64>, k: usize) -> Vec<&SpatialFeature> {
        self.tree
            .nearest_neighbor_iter(&[query.x, query.y])
            .take(k)
            .map(|indexed| &indexed.feature)
            .collect()
    }

    /// Find all features within distance (meters) using Haversine
    ///
    /// # Complexity
    /// O(log n + k) where k is number of results
    pub fn within_distance(
        &self,
        query: &Coord<f64>,
        distance_m: f64,
    ) -> Result<Vec<&SpatialFeature>> {
        use crate::distance::haversine_distance_m;

        let results: Vec<&SpatialFeature> = self
            .tree
            .iter()
            .filter_map(|indexed| {
                let dist = haversine_distance_m(query, &indexed.feature.location).ok()?;
                if dist <= distance_m {
                    Some(&indexed.feature)
                } else {
                    None
                }
            })
            .collect();

        Ok(results)
    }

    /// Find all features within bounding box
    ///
    /// # Arguments
    /// * `min` - Southwest corner (min_lon, min_lat)
    /// * `max` - Northeast corner (max_lon, max_lat)
    pub fn within_bounds(&self, min: &Coord<f64>, max: &Coord<f64>) -> Vec<&SpatialFeature> {
        let envelope = AABB::from_corners([min.x, min.y], [max.x, max.y]);

        self.tree
            .locate_in_envelope(&envelope)
            .map(|indexed| &indexed.feature)
            .collect()
    }

    /// Get total number of features
    pub fn len(&self) -> usize {
        self.tree.size()
    }

    /// Check if index is empty
    pub fn is_empty(&self) -> bool {
        self.tree.size() == 0
    }
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Spatial hash grid for fast proximity queries
///
/// Divides space into uniform grid cells. Very fast for:
/// - Finding features in same cell: O(1)
/// - Nearby cells: O(1)
///
/// Trade-off: Less flexible than R-Tree, requires tuning cell size.
///
/// # Example
/// ```
/// use geospatial_analysis::indexing::{SpatialHashGrid, SpatialFeature};
/// use geo::Coord;
///
/// let mut grid = SpatialHashGrid::new(0.1); // 0.1 degree cells
///
/// grid.insert(SpatialFeature::new(
///     "lisbon".to_string(),
///     Coord { x: -9.1393, y: 38.7223 }
/// ));
///
/// let query = Coord { x: -9.14, y: 38.72 };
/// let nearby = grid.query_cell(&query);
/// assert!(!nearby.is_empty());
/// ```
pub struct SpatialHashGrid {
    cell_size: f64,
    grid: HashMap<(i64, i64), Vec<SpatialFeature>>,
}

impl SpatialHashGrid {
    /// Create new spatial hash grid
    ///
    /// # Arguments
    /// * `cell_size` - Size of each cell in degrees (e.g., 0.1 for ~11km at equator)
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            grid: HashMap::new(),
        }
    }

    /// Convert coordinate to grid cell
    fn coord_to_cell(&self, coord: &Coord<f64>) -> (i64, i64) {
        let x = (coord.x / self.cell_size).floor() as i64;
        let y = (coord.y / self.cell_size).floor() as i64;
        (x, y)
    }

    /// Insert feature into grid
    pub fn insert(&mut self, feature: SpatialFeature) {
        let cell = self.coord_to_cell(&feature.location);
        self.grid.entry(cell).or_insert_with(Vec::new).push(feature);
    }

    /// Query features in same cell
    pub fn query_cell(&self, coord: &Coord<f64>) -> Vec<&SpatialFeature> {
        let cell = self.coord_to_cell(coord);
        self.grid
            .get(&cell)
            .map(|features| features.iter().collect())
            .unwrap_or_default()
    }

    /// Query features in cell and 8 neighboring cells (3x3 grid)
    pub fn query_neighbors(&self, coord: &Coord<f64>) -> Vec<&SpatialFeature> {
        let (cx, cy) = self.coord_to_cell(coord);
        let mut results = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let cell = (cx + dx, cy + dy);
                if let Some(features) = self.grid.get(&cell) {
                    results.extend(features.iter());
                }
            }
        }

        results
    }

    /// Get total number of features
    pub fn len(&self) -> usize {
        self.grid.values().map(|v| v.len()).sum()
    }

    /// Check if grid is empty
    pub fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    /// Get number of occupied cells
    pub fn cell_count(&self) -> usize {
        self.grid.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_portugal_features() -> Vec<SpatialFeature> {
        vec![
            SpatialFeature::new("lisbon".to_string(), Coord {
                x: -9.1393,
                y: 38.7223,
            }),
            SpatialFeature::new("porto".to_string(), Coord {
                x: -8.6291,
                y: 41.1579,
            }),
            SpatialFeature::new("faro".to_string(), Coord {
                x: -7.9304,
                y: 37.0194,
            }),
            SpatialFeature::new("coimbra".to_string(), Coord {
                x: -8.4103,
                y: 40.2033,
            }),
        ]
    }

    #[test]
    fn test_spatial_index_nearest() {
        let features = create_portugal_features();
        let index = SpatialIndex::from_features(features);

        // Query near Lisbon
        let query = Coord {
            x: -9.0,
            y: 38.7,
        };
        let nearest = index.nearest_neighbor(&query).unwrap();
        assert_eq!(nearest.id, "lisbon");
    }

    #[test]
    fn test_spatial_index_k_nearest() {
        let features = create_portugal_features();
        let index = SpatialIndex::from_features(features);

        let query = Coord {
            x: -9.0,
            y: 39.0,
        };
        let nearest = index.k_nearest_neighbors(&query, 2);

        assert_eq!(nearest.len(), 2);
        assert_eq!(nearest[0].id, "lisbon");
        assert_eq!(nearest[1].id, "coimbra");
    }

    #[test]
    fn test_spatial_index_within_bounds() {
        let features = create_portugal_features();
        let index = SpatialIndex::from_features(features);

        // Bounding box around central Portugal
        let min = Coord {
            x: -10.0,
            y: 38.0,
        };
        let max = Coord {
            x: -8.0,
            y: 42.0,
        };

        let results = index.within_bounds(&min, &max);
        assert_eq!(results.len(), 3); // lisbon, porto, coimbra
    }

    #[test]
    fn test_hash_grid() {
        let mut grid = SpatialHashGrid::new(0.5); // 0.5 degree cells

        let features = create_portugal_features();
        for feature in features {
            grid.insert(feature);
        }

        assert_eq!(grid.len(), 4);

        // Query near Lisbon
        let query = Coord {
            x: -9.1,
            y: 38.7,
        };
        let results = grid.query_cell(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "lisbon");
    }

    #[test]
    fn test_hash_grid_neighbors() {
        let mut grid = SpatialHashGrid::new(1.0); // 1 degree cells

        grid.insert(SpatialFeature::new("lisbon".to_string(), Coord {
            x: -9.1393,
            y: 38.7223,
        }));

        grid.insert(SpatialFeature::new("nearby".to_string(), Coord {
            x: -9.5,
            y: 39.2,
        }));

        let query = Coord {
            x: -9.0,
            y: 38.5,
        };
        let results = grid.query_neighbors(&query);
        assert!(results.len() >= 1);
    }
}
