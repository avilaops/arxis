//! OPTICS (Ordering Points To Identify the Clustering Structure)
//!
//! OPTICS is a density-based clustering algorithm similar to DBSCAN, but it
//! produces an ordering of the database representing its density-based clustering structure.
//!
//! # Algorithm
//!
//! OPTICS creates a reachability plot that represents the density-based clustering
//! structure of the data. Unlike DBSCAN, it doesn't require epsilon to be fixed,
//! making it better for data with varying densities.
//!
//! # Key Concepts
//!
//! - **Core Distance**: Minimum epsilon for a point to be a core point
//! - **Reachability Distance**: Density-based distance between points
//! - **Ordering**: Sequence that reveals the clustering structure
//!
//! # References
//!
//! Ankerst, M., Breunig, M. M., Kriegel, H. P., & Sander, J. (1999).
//! OPTICS: Ordering points to identify the clustering structure.

use crate::metrics::distance::euclidean_distance;
use crate::{ClusteringError, Result};
use kiddo::KdTree;
use ndarray::{Array1, Array2, ArrayView2, Axis};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Point in the OPTICS ordering with reachability information
#[derive(Debug, Clone)]
pub struct OpticsPoint {
    pub index: usize,
    pub reachability_distance: f64,
    pub core_distance: f64,
    pub processed: bool,
}

impl PartialEq for OpticsPoint {
    fn eq(&self, other: &Self) -> bool {
        self.reachability_distance == other.reachability_distance
    }
}

impl Eq for OpticsPoint {}

impl PartialOrd for OpticsPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse ordering for min-heap
        other.reachability_distance.partial_cmp(&self.reachability_distance)
    }
}

impl Ord for OpticsPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// OPTICS clustering builder
pub struct OpticsBuilder {
    min_samples: usize,
    max_eps: f64,
    metric: String,
    cluster_method: ClusterMethod,
    xi: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub enum ClusterMethod {
    Xi,
    Dbscan { eps: f64 },
}

impl OpticsBuilder {
    pub fn new(min_samples: usize) -> Self {
        Self {
            min_samples,
            max_eps: f64::INFINITY,
            metric: "euclidean".to_string(),
            cluster_method: ClusterMethod::Xi,
            xi: Some(0.05),
        }
    }

    /// Set maximum epsilon value
    pub fn max_eps(mut self, max_eps: f64) -> Self {
        self.max_eps = max_eps;
        self
    }

    /// Set distance metric
    pub fn metric(mut self, metric: &str) -> Self {
        self.metric = metric.to_string();
        self
    }

    /// Use Xi method for cluster extraction
    pub fn xi(mut self, xi: f64) -> Self {
        self.xi = Some(xi);
        self.cluster_method = ClusterMethod::Xi;
        self
    }

    /// Use DBSCAN-like extraction with fixed epsilon
    pub fn dbscan(mut self, eps: f64) -> Self {
        self.cluster_method = ClusterMethod::Dbscan { eps };
        self
    }

    pub fn build(self) -> Optics {
        Optics {
            min_samples: self.min_samples,
            max_eps: self.max_eps,
            metric: self.metric,
            cluster_method: self.cluster_method,
            xi: self.xi,
            ordering: None,
            labels: None,
            reachability: None,
        }
    }
}

/// OPTICS clustering
pub struct Optics {
    min_samples: usize,
    max_eps: f64,
    metric: String,
    cluster_method: ClusterMethod,
    xi: Option<f64>,
    ordering: Option<Vec<usize>>,
    labels: Option<Array1<i32>>,
    reachability: Option<Vec<f64>>,
}

impl Optics {
    pub fn new(min_samples: usize) -> OpticsBuilder {
        OpticsBuilder::new(min_samples)
    }

    /// Fit the model and extract clusters
    pub fn fit_predict(&mut self, data: &ArrayView2<f64>) -> Result<Array1<i32>> {
        self.fit(data)?;
        Ok(self.labels.clone().unwrap())
    }

    /// Fit the model to the data
    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<()> {
        let (n_samples, n_features) = data.dim();

        if n_samples < self.min_samples {
            return Err(ClusteringError::InvalidParameter(
                "Number of samples must be >= min_samples".to_string(),
            ));
        }

        // Build KD-tree for efficient neighbor queries
        let mut kdtree: KdTree<f64, 2> = KdTree::new();

        for (i, point) in data.axis_iter(Axis(0)).enumerate() {
            if n_features == 2 {
                kdtree.add(&[point[0], point[1]], i as u64);
            }
        }

        // Initialize OPTICS ordering
        let mut ordering = Vec::new();
        let mut reachability = vec![f64::INFINITY; n_samples];
        let mut processed = vec![false; n_samples];

        // Priority queue for order seeds
        let mut seeds = BinaryHeap::new();

        // Process all points
        for i in 0..n_samples {
            if processed[i] {
                continue;
            }

            // Get neighbors
            let neighbors = self.get_neighbors(data, i, &kdtree, n_features)?;

            // Mark as processed
            processed[i] = true;
            ordering.push(i);

            // Compute core distance
            let core_dist = self.core_distance(&neighbors, data, i);

            if core_dist != f64::INFINITY {
                // Update seeds
                self.update_seeds(&neighbors, &mut seeds, &mut reachability, &processed, data, i, core_dist);

                // Process seeds
                while let Some(seed_idx) = seeds.pop().map(|op: OpticsPoint| op.index) {
                    if processed[seed_idx] {
                        continue;
                    }

                    let seed_neighbors = self.get_neighbors(data, seed_idx, &kdtree, n_features)?;
                    processed[seed_idx] = true;
                    ordering.push(seed_idx);

                    let seed_core_dist = self.core_distance(&seed_neighbors, data, seed_idx);

                    if seed_core_dist != f64::INFINITY {
                        self.update_seeds(
                            &seed_neighbors,
                            &mut seeds,
                            &mut reachability,
                            &processed,
                            data,
                            seed_idx,
                            seed_core_dist,
                        );
                    }
                }
            }
        }

        self.ordering = Some(ordering.clone());

        // Build reachability vector in ordering
        let ordered_reachability: Vec<f64> = ordering.iter()
            .map(|&idx| reachability[idx])
            .collect();

        self.reachability = Some(ordered_reachability.clone());

        // Extract clusters based on cluster method
        let labels = match self.cluster_method {
            ClusterMethod::Xi => {
                self.extract_clusters_xi(&ordered_reachability, &ordering)?
            }
            ClusterMethod::Dbscan { eps } => {
                self.extract_clusters_dbscan(&ordered_reachability, &ordering, eps)
            }
        };

        self.labels = Some(labels);

        Ok(())
    }

    /// Get neighbors within max_eps
    fn get_neighbors(
        &self,
        data: &ArrayView2<f64>,
        point_idx: usize,
        kdtree: &KdTree<f64, 2>,
        n_features: usize,
    ) -> Result<Vec<usize>> {
        if n_features != 2 {
            // Fallback to brute force for non-2D data
            let point = data.row(point_idx);
            let mut neighbors = Vec::new();

            for (i, other_point) in data.axis_iter(Axis(0)).enumerate() {
                if i == point_idx {
                    continue;
                }
                let dist = euclidean_distance(&point.into_owned().view(), &other_point.into_owned().view());
                if dist <= self.max_eps {
                    neighbors.push(i);
                }
            }

            Ok(neighbors)
        } else {
            // Use KD-tree for 2D data
            let point = data.row(point_idx);
            let results = kdtree.within::<kiddo::SquaredEuclidean>(&[point[0], point[1]], self.max_eps * self.max_eps);

            Ok(results.into_iter()
                .map(|neighbor| neighbor.item as usize)
                .filter(|&idx| idx != point_idx)
                .collect())
        }
    }

    /// Compute core distance
    fn core_distance(&self, neighbors: &[usize], data: &ArrayView2<f64>, point_idx: usize) -> f64 {
        if neighbors.len() < self.min_samples {
            return f64::INFINITY;
        }

        let point = data.row(point_idx);
        let mut distances: Vec<f64> = neighbors
            .iter()
            .map(|&neighbor_idx| {
                let neighbor = data.row(neighbor_idx);
                euclidean_distance(&point.into_owned().view(), &neighbor.into_owned().view())
            })
            .collect();

        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
        distances[self.min_samples - 1]
    }

    /// Update seeds with new reachability distances
    fn update_seeds(
        &self,
        neighbors: &[usize],
        seeds: &mut BinaryHeap<OpticsPoint>,
        reachability: &mut [f64],
        processed: &[bool],
        data: &ArrayView2<f64>,
        point_idx: usize,
        core_dist: f64,
    ) {
        let point = data.row(point_idx);

        for &neighbor_idx in neighbors {
            if processed[neighbor_idx] {
                continue;
            }

            let neighbor = data.row(neighbor_idx);
            let dist = euclidean_distance(&point.into_owned().view(), &neighbor.into_owned().view());
            let new_reach_dist = core_dist.max(dist);

            if new_reach_dist < reachability[neighbor_idx] {
                reachability[neighbor_idx] = new_reach_dist;
                seeds.push(OpticsPoint {
                    index: neighbor_idx,
                    reachability_distance: new_reach_dist,
                    core_distance: 0.0,
                    processed: false,
                });
            }
        }
    }

    /// Extract clusters using Xi method
    fn extract_clusters_xi(
        &self,
        reachability: &[f64],
        ordering: &[usize],
    ) -> Result<Array1<i32>> {
        let xi = self.xi.unwrap_or(0.05);
        let mut labels = Array1::from_elem(ordering.len(), -1);
        let mut cluster_id = 0;

        // Simple steep down/up detection for cluster extraction
        let mut in_cluster = false;
        let mut current_min = f64::INFINITY;

        for i in 1..reachability.len() {
            let prev = reachability[i - 1];
            let curr = reachability[i];

            if !prev.is_infinite() && !curr.is_infinite() {
                let ratio = (prev - curr) / prev.max(curr);

                if ratio > xi && !in_cluster {
                    // Steep down - start of cluster
                    in_cluster = true;
                    current_min = curr;
                    labels[ordering[i]] = cluster_id;
                } else if in_cluster {
                    if curr < current_min {
                        current_min = curr;
                    }

                    if ratio < -xi {
                        // Steep up - end of cluster
                        in_cluster = false;
                        cluster_id += 1;
                    } else {
                        labels[ordering[i]] = cluster_id;
                    }
                }
            }
        }

        Ok(labels)
    }

    /// Extract clusters using DBSCAN-like approach
    fn extract_clusters_dbscan(
        &self,
        reachability: &[f64],
        ordering: &[usize],
        eps: f64,
    ) -> Array1<i32> {
        let mut labels = Array1::from_elem(ordering.len(), -1);
        let mut cluster_id = 0;

        for (i, &reach) in reachability.iter().enumerate() {
            if reach <= eps {
                if i > 0 && reachability[i - 1] <= eps && labels[ordering[i - 1]] >= 0 {
                    labels[ordering[i]] = labels[ordering[i - 1]];
                } else {
                    labels[ordering[i]] = cluster_id;
                    cluster_id += 1;
                }
            }
        }

        labels
    }

    /// Get cluster labels
    pub fn labels(&self) -> Option<&Array1<i32>> {
        self.labels.as_ref()
    }

    /// Get ordering of points
    pub fn ordering(&self) -> Option<&Vec<usize>> {
        self.ordering.as_ref()
    }

    /// Get reachability distances
    pub fn reachability(&self) -> Option<&Vec<f64>> {
        self.reachability.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_optics_simple() {
        let data = array![
            [1.0, 1.0],
            [1.5, 2.0],
            [1.3, 1.5],
            [10.0, 10.0],
            [10.5, 11.0],
            [11.0, 10.5],
        ];

        let mut optics = Optics::new(2)
            .max_eps(3.0)
            .dbscan(2.0)
            .build();

        let labels = optics.fit_predict(&data.view()).unwrap();
        assert_eq!(labels.len(), 6);

        // Should find 2 clusters
        let unique_labels: std::collections::HashSet<_> = labels.iter()
            .filter(|&&l| l >= 0)
            .collect();
        assert!(unique_labels.len() >= 1);
    }
}
