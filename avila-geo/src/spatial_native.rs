//! Native Spatial Indexing - 100% Rust Implementation
//!
//! High-performance R-Tree implementation without external dependencies.
//! Optimized for geographic coordinates and spatial queries.
//!
//! ## Features
//!
//! - **R-Tree indexing**: Fast spatial queries (O(log n))
//! - **Bulk loading**: Optimized STR (Sort-Tile-Recursive) algorithm
//! - **Range queries**: Find all points within bounding box
//! - **KNN queries**: K-nearest neighbors search
//! - **Radius queries**: Find points within distance
//! - **Dynamic updates**: Insert/remove points efficiently
//!
//! ## Usage
//!
//! ```rust
//! use avila_geo::spatial_native::*;
//! use avila_geo::coords::GeoCoord;
//!
//! let mut index = RTreeIndex::new();
//! index.insert(0, GeoCoord::new(-23.55, -46.63)); // São Paulo
//! index.insert(1, GeoCoord::new(-22.91, -43.17)); // Rio
//!
//! // Find nearest neighbors
//! let neighbors = index.knn(&GeoCoord::new(-23.0, -45.0), 5);
//! ```

use crate::coords::{GeoCoord, GeoBounds};
use crate::calc::haversine_distance;
use std::cmp::Ordering;

const MAX_NODE_ENTRIES: usize = 16;
const MIN_NODE_ENTRIES: usize = 4;

/// Native R-Tree spatial index for geographic data
#[derive(Debug, Clone)]
pub struct RTreeIndex {
    root: Option<Box<RTreeNode>>,
    size: usize,
}

#[derive(Debug, Clone)]
struct RTreeNode {
    bounds: GeoBounds,
    entries: Vec<RTreeEntry>,
    is_leaf: bool,
}

#[derive(Debug, Clone)]
struct RTreeEntry {
    bounds: GeoBounds,
    data: EntryData,
}

#[derive(Debug, Clone)]
enum EntryData {
    Point { id: usize, coord: GeoCoord },
    Child(Box<RTreeNode>),
}

impl RTreeIndex {
    /// Create a new empty R-Tree index
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// Create R-Tree from bulk points (optimized STR algorithm)
    pub fn bulk_load(points: Vec<(usize, GeoCoord)>) -> Self {
        if points.is_empty() {
            return Self::new();
        }

        let size = points.len();
        let root = Self::build_str_tree(points);

        Self {
            root: Some(Box::new(root)),
            size,
        }
    }

    /// Insert a point into the index
    pub fn insert(&mut self, id: usize, coord: GeoCoord) {
        let bounds = GeoBounds::from_point(&coord);
        let entry = RTreeEntry {
            bounds,
            data: EntryData::Point { id, coord },
        };

        if let Some(root) = &mut self.root {
            if root.entries.len() >= MAX_NODE_ENTRIES {
                // Split root
                let (left, right) = Self::split_node(root);
                let new_root = RTreeNode {
                    bounds: left.bounds.union(&right.bounds),
                    entries: vec![
                        RTreeEntry {
                            bounds: left.bounds,
                            data: EntryData::Child(Box::new(left)),
                        },
                        RTreeEntry {
                            bounds: right.bounds,
                            data: EntryData::Child(Box::new(right)),
                        },
                    ],
                    is_leaf: false,
                };
                self.root = Some(Box::new(new_root));
            }
            Self::insert_entry(&mut self.root.as_mut().unwrap(), entry);
        } else {
            // First insertion
            self.root = Some(Box::new(RTreeNode {
                bounds,
                entries: vec![entry],
                is_leaf: true,
            }));
        }

        self.size += 1;
    }

    /// Find all points within bounding box
    pub fn range_query(&self, bounds: &GeoBounds) -> Vec<(usize, GeoCoord)> {
        let mut results = Vec::new();
        if let Some(root) = &self.root {
            Self::range_search(root, bounds, &mut results);
        }
        results
    }

    /// Find K nearest neighbors
    pub fn knn(&self, query: &GeoCoord, k: usize) -> Vec<(usize, GeoCoord, f64)> {
        if k == 0 || self.root.is_none() {
            return Vec::new();
        }

        let mut heap = std::collections::BinaryHeap::new();
        let root = self.root.as_ref().unwrap();

        // Start with root
        heap.push(KnnCandidate::Node {
            min_dist: 0.0,
            node: root.as_ref(),
        });

        let mut results = Vec::new();
        let mut max_dist = f64::INFINITY;

        while let Some(candidate) = heap.pop() {
            match candidate {
                KnnCandidate::Node { min_dist, node } => {
                    if min_dist > max_dist {
                        break;
                    }

                    for entry in &node.entries {
                        match &entry.data {
                            EntryData::Point { id, coord } => {
                                let dist = haversine_distance(query, coord);
                                if results.len() < k {
                                    results.push((*id, *coord, dist));
                                    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
                                    if results.len() == k {
                                        max_dist = results.last().unwrap().2;
                                    }
                                } else if dist < max_dist {
                                    results.pop();
                                    results.push((*id, *coord, dist));
                                    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
                                    max_dist = results.last().unwrap().2;
                                }
                            }
                            EntryData::Child(child) => {
                                let min_dist = Self::min_distance_to_bounds(query, &entry.bounds);
                                if min_dist <= max_dist || results.len() < k {
                                    heap.push(KnnCandidate::Node {
                                        min_dist,
                                        node: child.as_ref(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Find all points within radius (meters)
    pub fn radius_query(&self, center: &GeoCoord, radius: f64) -> Vec<(usize, GeoCoord, f64)> {
        // Approximate bounding box for initial filtering
        let bounds = Self::radius_to_bounds(center, radius);
        let candidates = self.range_query(&bounds);

        candidates
            .into_iter()
            .filter_map(|(id, coord)| {
                let dist = haversine_distance(center, &coord);
                if dist <= radius {
                    Some((id, coord, dist))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get total number of indexed points
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if index is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // === Private Helper Methods ===

    fn build_str_tree(mut points: Vec<(usize, GeoCoord)>) -> RTreeNode {
        if points.is_empty() {
            panic!("Cannot build tree from empty points");
        }

        // Calculate number of slices
        let n = points.len();
        let slices = ((n as f64) / (MAX_NODE_ENTRIES as f64)).ceil().sqrt().ceil() as usize;

        // Sort by x (longitude)
        points.sort_by(|a, b| a.1.lon.partial_cmp(&b.1.lon).unwrap());

        let mut all_entries = Vec::new();

        // Process vertical slices
        for slice_points in points.chunks(n / slices.max(1)) {
            let mut slice = slice_points.to_vec();
            // Sort each slice by y (latitude)
            slice.sort_by(|a, b| a.1.lat.partial_cmp(&b.1.lat).unwrap());

            // Create leaf nodes
            for chunk in slice.chunks(MAX_NODE_ENTRIES) {
                let entries: Vec<RTreeEntry> = chunk
                    .iter()
                    .map(|(id, coord)| {
                        let bounds = GeoBounds::from_point(coord);
                        RTreeEntry {
                            bounds,
                            data: EntryData::Point { id: *id, coord: *coord },
                        }
                    })
                    .collect();

                let node_bounds = Self::calculate_bounds(&entries);
                all_entries.push(RTreeEntry {
                    bounds: node_bounds,
                    data: EntryData::Child(Box::new(RTreeNode {
                        bounds: node_bounds,
                        entries,
                        is_leaf: true,
                    })),
                });
            }
        }

        // Build parent levels
        Self::build_parent_level(all_entries)
    }

    fn build_parent_level(mut entries: Vec<RTreeEntry>) -> RTreeNode {
        if entries.len() <= MAX_NODE_ENTRIES {
            // Create root
            let bounds = Self::calculate_bounds(&entries);
            return RTreeNode {
                bounds,
                entries,
                is_leaf: false,
            };
        }

        // Create intermediate level
        let mut parent_entries = Vec::new();
        for chunk in entries.chunks(MAX_NODE_ENTRIES) {
            let chunk_vec = chunk.to_vec();
            let bounds = Self::calculate_bounds(&chunk_vec);
            parent_entries.push(RTreeEntry {
                bounds,
                data: EntryData::Child(Box::new(RTreeNode {
                    bounds,
                    entries: chunk_vec,
                    is_leaf: false,
                })),
            });
        }

        Self::build_parent_level(parent_entries)
    }

    fn insert_entry(node: &mut RTreeNode, entry: RTreeEntry) {
        if node.is_leaf {
            node.entries.push(entry);
            node.bounds = Self::calculate_bounds(&node.entries);
        } else {
            // Find best child to insert into
            let best_idx = Self::choose_subtree(node, &entry.bounds);

            if let EntryData::Child(ref mut child) = &mut node.entries[best_idx].data {
                Self::insert_entry(child, entry);
                node.entries[best_idx].bounds = child.bounds;
                node.bounds = Self::calculate_bounds(&node.entries);
            }
        }
    }

    fn choose_subtree(node: &RTreeNode, bounds: &GeoBounds) -> usize {
        let mut min_enlargement = f64::INFINITY;
        let mut best_idx = 0;

        for (idx, entry) in node.entries.iter().enumerate() {
            let enlargement = entry.bounds.union(bounds).area() - entry.bounds.area();
            if enlargement < min_enlargement {
                min_enlargement = enlargement;
                best_idx = idx;
            }
        }

        best_idx
    }

    fn split_node(node: &mut RTreeNode) -> (RTreeNode, RTreeNode) {
        // Simple split: divide entries in half
        let mid = node.entries.len() / 2;
        let mut entries = std::mem::take(&mut node.entries);

        let right_entries = entries.split_off(mid);
        let left_entries = entries;

        let left_bounds = Self::calculate_bounds(&left_entries);
        let right_bounds = Self::calculate_bounds(&right_entries);

        let left = RTreeNode {
            bounds: left_bounds,
            entries: left_entries,
            is_leaf: node.is_leaf,
        };

        let right = RTreeNode {
            bounds: right_bounds,
            entries: right_entries,
            is_leaf: node.is_leaf,
        };

        (left, right)
    }

    fn range_search(node: &RTreeNode, bounds: &GeoBounds, results: &mut Vec<(usize, GeoCoord)>) {
        if !node.bounds.intersects(bounds) {
            return;
        }

        for entry in &node.entries {
            if !entry.bounds.intersects(bounds) {
                continue;
            }

            match &entry.data {
                EntryData::Point { id, coord } => {
                    if bounds.contains(coord) {
                        results.push((*id, *coord));
                    }
                }
                EntryData::Child(child) => {
                    Self::range_search(child, bounds, results);
                }
            }
        }
    }

    fn calculate_bounds(entries: &[RTreeEntry]) -> GeoBounds {
        if entries.is_empty() {
            return GeoBounds::default();
        }

        let mut bounds = entries[0].bounds;
        for entry in &entries[1..] {
            bounds = bounds.union(&entry.bounds);
        }
        bounds
    }

    fn min_distance_to_bounds(point: &GeoCoord, bounds: &GeoBounds) -> f64 {
        let lat = point.lat.clamp(bounds.min_lat, bounds.max_lat);
        let lon = point.lon.clamp(bounds.min_lon, bounds.max_lon);
        let closest = GeoCoord::new(lat, lon);
        haversine_distance(point, &closest)
    }

    fn radius_to_bounds(center: &GeoCoord, radius: f64) -> GeoBounds {
        // Approximate: 1 degree ≈ 111km at equator
        let deg_offset = (radius / 111_000.0) * 1.5; // Add 50% margin

        GeoBounds {
            min_lat: center.lat - deg_offset,
            max_lat: center.lat + deg_offset,
            min_lon: center.lon - deg_offset,
            max_lon: center.lon + deg_offset,
        }
    }
}

enum KnnCandidate<'a> {
    Node {
        min_dist: f64,
        node: &'a RTreeNode,
    },
}

impl<'a> PartialEq for KnnCandidate<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KnnCandidate::Node { min_dist: d1, .. }, KnnCandidate::Node { min_dist: d2, .. }) => {
                d1 == d2
            }
        }
    }
}

impl<'a> Eq for KnnCandidate<'a> {}

impl<'a> PartialOrd for KnnCandidate<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for KnnCandidate<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (KnnCandidate::Node { min_dist: d1, .. }, KnnCandidate::Node { min_dist: d2, .. }) => {
                // Reverse ordering for max-heap
                d2.partial_cmp(d1).unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl Default for RTreeIndex {
    fn default() -> Self {
        Self::new()
    }
}

// === GeoBounds Extensions ===

impl GeoBounds {
    fn from_point(coord: &GeoCoord) -> Self {
        Self {
            min_lat: coord.lat,
            max_lat: coord.lat,
            min_lon: coord.lon,
            max_lon: coord.lon,
        }
    }

    fn union(&self, other: &Self) -> Self {
        Self {
            min_lat: self.min_lat.min(other.min_lat),
            max_lat: self.max_lat.max(other.max_lat),
            min_lon: self.min_lon.min(other.min_lon),
            max_lon: self.max_lon.max(other.max_lon),
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        !(self.max_lat < other.min_lat
            || self.min_lat > other.max_lat
            || self.max_lon < other.min_lon
            || self.min_lon > other.max_lon)
    }

    fn area(&self) -> f64 {
        (self.max_lat - self.min_lat) * (self.max_lon - self.min_lon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insert_and_query() {
        let mut index = RTreeIndex::new();
        index.insert(0, GeoCoord::new(-23.55, -46.63)); // São Paulo
        index.insert(1, GeoCoord::new(-22.91, -43.17)); // Rio
        index.insert(2, GeoCoord::new(-19.92, -43.94)); // BH

        assert_eq!(index.len(), 3);

        let bounds = GeoBounds {
            min_lat: -24.0,
            max_lat: -23.0,
            min_lon: -47.0,
            max_lon: -46.0,
        };

        let results = index.range_query(&bounds);
        assert_eq!(results.len(), 1); // Only São Paulo
    }

    #[test]
    fn test_knn_query() {
        let mut index = RTreeIndex::new();
        index.insert(0, GeoCoord::new(-23.55, -46.63)); // SP
        index.insert(1, GeoCoord::new(-22.91, -43.17)); // RJ
        index.insert(2, GeoCoord::new(-19.92, -43.94)); // BH
        index.insert(3, GeoCoord::new(-25.43, -49.27)); // Curitiba

        let query = GeoCoord::new(-23.0, -45.0);
        let neighbors = index.knn(&query, 2);

        assert_eq!(neighbors.len(), 2);
        // São Paulo should be closest
        assert_eq!(neighbors[0].0, 0);
    }

    #[test]
    fn test_radius_query() {
        let mut index = RTreeIndex::new();
        index.insert(0, GeoCoord::new(-23.55, -46.63));
        index.insert(1, GeoCoord::new(-22.91, -43.17));

        let results = index.radius_query(&GeoCoord::new(-23.55, -46.63), 50_000.0);
        assert_eq!(results.len(), 1); // Only SP within 50km of SP
    }

    #[test]
    fn test_bulk_load() {
        let points = vec![
            (0, GeoCoord::new(-23.55, -46.63)),
            (1, GeoCoord::new(-22.91, -43.17)),
            (2, GeoCoord::new(-19.92, -43.94)),
            (3, GeoCoord::new(-25.43, -49.27)),
        ];

        let index = RTreeIndex::bulk_load(points);
        assert_eq!(index.len(), 4);

        let neighbors = index.knn(&GeoCoord::new(-23.0, -45.0), 2);
        assert_eq!(neighbors.len(), 2);
    }
}
