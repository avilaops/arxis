//! Spatial indexing using R-Tree
//!
//! Provides fast spatial queries: nearest neighbors, range searches,
//! and intersection tests.

#[cfg(feature = "spatial-index")]
use rstar::{RTree, RTreeObject, AABB};

use crate::coords::{GeoCoord, GeoBounds};
use crate::geometry::{GeoPoint, GeoLine, GeoPolygon};

/// Spatial index for fast geographic queries
#[cfg(feature = "spatial-index")]
pub struct SpatialIndex {
    point_tree: RTree<IndexedPoint>,
}

#[cfg(feature = "spatial-index")]
#[derive(Debug, Clone)]
struct IndexedPoint {
    coord: GeoCoord,
    id: usize,
}

#[cfg(feature = "spatial-index")]
impl RTreeObject for IndexedPoint {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.coord.lon, self.coord.lat])
    }
}

#[cfg(feature = "spatial-index")]
impl SpatialIndex {
    /// Create empty spatial index
    pub fn new() -> Self {
        Self {
            point_tree: RTree::new(),
        }
    }

    /// Create from points
    pub fn from_points(points: &[GeoPoint]) -> Self {
        let indexed: Vec<IndexedPoint> = points
            .iter()
            .enumerate()
            .map(|(id, p)| IndexedPoint {
                coord: p.coord,
                id,
            })
            .collect();

        Self {
            point_tree: RTree::bulk_load(indexed),
        }
    }

    /// Insert a point
    pub fn insert(&mut self, point: &GeoPoint, id: usize) {
        self.point_tree.insert(IndexedPoint {
            coord: point.coord,
            id,
        });
    }

    /// Find nearest neighbor
    pub fn nearest(&self, coord: &GeoCoord) -> Option<(usize, f64)> {
        self.point_tree
            .nearest_neighbor(&[coord.lon, coord.lat])
            .map(|indexed| {
                let dist = crate::calc::haversine_distance(&indexed.coord, coord);
                (indexed.id, dist)
            })
    }

    /// Find k nearest neighbors
    pub fn k_nearest(&self, coord: &GeoCoord, k: usize) -> Vec<(usize, f64)> {
        use crate::calc::haversine_distance;

        let mut results: Vec<_> = self
            .point_tree
            .nearest_neighbor_iter(&[coord.lon, coord.lat])
            .take(k)
            .map(|indexed| {
                let dist = haversine_distance(&indexed.coord, coord);
                (indexed.id, dist)
            })
            .collect();

        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results
    }

    /// Find all points within distance (meters)
    pub fn within_distance(&self, coord: &GeoCoord, radius_m: f64) -> Vec<(usize, f64)> {
        use crate::calc::haversine_distance;

        // Approximate bounding box (rough calculation)
        let lat_delta = (radius_m / 111_000.0).to_degrees();
        let lon_delta = (radius_m / (111_000.0 * coord.lat.to_radians().cos())).to_degrees();

        let envelope = AABB::from_corners(
            [coord.lon - lon_delta, coord.lat - lat_delta],
            [coord.lon + lon_delta, coord.lat + lat_delta],
        );

        self.point_tree
            .locate_in_envelope(&envelope)
            .filter_map(|indexed| {
                let dist = haversine_distance(&indexed.coord, coord);
                if dist <= radius_m {
                    Some((indexed.id, dist))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Find all points within bounds
    pub fn within_bounds(&self, bounds: &GeoBounds) -> Vec<usize> {
        let envelope = AABB::from_corners(
            [bounds.min_lon, bounds.min_lat],
            [bounds.max_lon, bounds.max_lat],
        );

        self.point_tree
            .locate_in_envelope(&envelope)
            .map(|indexed| indexed.id)
            .collect()
    }

    /// Get total number of indexed points
    pub fn len(&self) -> usize {
        self.point_tree.size()
    }

    /// Check if index is empty
    pub fn is_empty(&self) -> bool {
        self.point_tree.size() == 0
    }
}

#[cfg(feature = "spatial-index")]
impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Query builder for spatial searches
#[cfg(feature = "spatial-index")]
pub struct SpatialQuery<'a> {
    index: &'a SpatialIndex,
    center: GeoCoord,
}

#[cfg(feature = "spatial-index")]
impl<'a> SpatialQuery<'a> {
    pub fn new(index: &'a SpatialIndex, center: GeoCoord) -> Self {
        Self { index, center }
    }

    /// Find nearest
    pub fn nearest(self) -> Option<(usize, f64)> {
        self.index.nearest(&self.center)
    }

    /// Find k nearest
    pub fn k_nearest(self, k: usize) -> Vec<(usize, f64)> {
        self.index.k_nearest(&self.center, k)
    }

    /// Find within radius
    pub fn within(self, radius_m: f64) -> Vec<(usize, f64)> {
        self.index.within_distance(&self.center, radius_m)
    }
}

#[cfg(test)]
#[cfg(feature = "spatial-index")]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_index() {
        let points = vec![
            GeoPoint::with_name(GeoCoord::new(-23.55, -46.63), "São Paulo"),
            GeoPoint::with_name(GeoCoord::new(-22.91, -43.17), "Rio de Janeiro"),
            GeoPoint::with_name(GeoCoord::new(-15.78, -47.93), "Brasília"),
        ];

        let index = SpatialIndex::from_points(&points);

        // Find nearest to São Paulo
        let query_point = GeoCoord::new(-23.5, -46.6);
        let nearest = index.nearest(&query_point);

        assert!(nearest.is_some());
        let (id, dist) = nearest.unwrap();
        assert_eq!(id, 0); // Should be São Paulo
        assert!(dist < 10_000.0); // Within 10km
    }

    #[test]
    fn test_k_nearest() {
        let points = vec![
            GeoPoint::new(GeoCoord::new(0.0, 0.0)),
            GeoPoint::new(GeoCoord::new(1.0, 1.0)),
            GeoPoint::new(GeoCoord::new(2.0, 2.0)),
            GeoPoint::new(GeoCoord::new(3.0, 3.0)),
        ];

        let index = SpatialIndex::from_points(&points);
        let origin = GeoCoord::new(0.0, 0.0);

        let nearest_3 = index.k_nearest(&origin, 3);
        assert_eq!(nearest_3.len(), 3);

        // Should be sorted by distance
        assert!(nearest_3[0].1 < nearest_3[1].1);
        assert!(nearest_3[1].1 < nearest_3[2].1);
    }

    #[test]
    fn test_within_bounds() {
        let points = vec![
            GeoPoint::new(GeoCoord::new(-23.55, -46.63)), // Inside
            GeoPoint::new(GeoCoord::new(-22.91, -43.17)), // Inside
            GeoPoint::new(GeoCoord::new(40.71, -74.01)),  // Outside (NYC)
        ];

        let index = SpatialIndex::from_points(&points);
        let brazil_bounds = GeoBounds::BRAZIL;

        let inside = index.within_bounds(&brazil_bounds);
        assert_eq!(inside.len(), 2); // Only Brazilian cities
    }
}
