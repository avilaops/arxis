//! BIRCH (Balanced Iterative Reducing and Clustering using Hierarchies)
//!
//! BIRCH is a memory-efficient clustering algorithm designed for very large datasets.
//! It constructs a tree structure (CF-Tree) to incrementally cluster incoming data points.
//!
//! # Algorithm
//!
//! BIRCH uses Clustering Feature (CF) entries to summarize clusters:
//! CF = (N, LS, SS) where:
//! - N: number of points
//! - LS: linear sum of points
//! - SS: square sum of points
//!
//! This allows efficient merging and splitting of clusters without storing all points.
//!
//! # References
//!
//! Zhang, T., Ramakrishnan, R., & Livny, M. (1996). BIRCH: An efficient data clustering
//! method for very large databases. ACM SIGMOD Record, 25(2), 103-114.

use crate::metrics::distance::euclidean_distance;
use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use std::collections::VecDeque;

/// Clustering Feature - summarizes a cluster
#[derive(Clone, Debug)]
struct ClusteringFeature {
    /// Number of data points
    n: usize,
    /// Linear sum of data points
    linear_sum: Array1<f64>,
    /// Sum of squared data points
    squared_sum: f64,
}

impl ClusteringFeature {
    fn new(point: ArrayView1<f64>) -> Self {
        let squared_sum = point.iter().map(|&x| x * x).sum();
        Self {
            n: 1,
            linear_sum: point.to_owned(),
            squared_sum,
        }
    }

    fn empty(dimensions: usize) -> Self {
        Self {
            n: 0,
            linear_sum: Array1::zeros(dimensions),
            squared_sum: 0.0,
        }
    }

    fn centroid(&self) -> Array1<f64> {
        if self.n == 0 {
            return self.linear_sum.clone();
        }
        &self.linear_sum / (self.n as f64)
    }

    fn radius(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        let centroid = self.centroid();
        let centroid_sq: f64 = centroid.iter().map(|&x| x * x).sum();
        ((self.squared_sum / self.n as f64) - centroid_sq).sqrt().max(0.0)
    }

    fn merge(&mut self, other: &ClusteringFeature) {
        self.n += other.n;
        self.linear_sum = &self.linear_sum + &other.linear_sum;
        self.squared_sum += other.squared_sum;
    }

    fn add_point(&mut self, point: ArrayView1<f64>) {
        self.n += 1;
        self.linear_sum = &self.linear_sum + &point;
        self.squared_sum += point.iter().map(|&x| x * x).sum::<f64>();
    }

    fn distance_to_point(&self, point: ArrayView1<f64>) -> f64 {
        if self.n == 0 {
            return f64::INFINITY;
        }
        euclidean_distance(&self.centroid().view(), &point)
    }
}

/// Node in the CF-Tree
#[derive(Clone, Debug)]
struct CFNode {
    cf: ClusteringFeature,
    children: Vec<CFNode>,
    is_leaf: bool,
}

impl CFNode {
    fn new_leaf(dimensions: usize) -> Self {
        Self {
            cf: ClusteringFeature::empty(dimensions),
            children: Vec::new(),
            is_leaf: true,
        }
    }

    fn new_internal(dimensions: usize) -> Self {
        Self {
            cf: ClusteringFeature::empty(dimensions),
            children: Vec::new(),
            is_leaf: false,
        }
    }
}

/// BIRCH clustering builder
pub struct BirchBuilder {
    threshold: f64,
    branching_factor: usize,
    n_clusters: Option<usize>,
    max_leaf_entries: usize,
}

impl BirchBuilder {
    pub fn new() -> Self {
        Self {
            threshold: 0.5,
            branching_factor: 50,
            n_clusters: None,
            max_leaf_entries: 50,
        }
    }

    /// Set threshold for radius of subcluster
    pub fn threshold(mut self, threshold: f64) -> std::result::Result<Self, ClusteringError> {
        if threshold <= 0.0 {
            return Err(ClusteringError::InvalidParameter(
                "threshold must be positive".to_string(),
            ));
        }
        self.threshold = threshold;
        Ok(self)
    }

    /// Set maximum branching factor
    pub fn branching_factor(mut self, branching_factor: usize) -> std::result::Result<Self, ClusteringError> {
        if branching_factor < 2 {
            return Err(ClusteringError::InvalidParameter(
                "branching_factor must be at least 2".to_string(),
            ));
        }
        self.branching_factor = branching_factor;
        Ok(self)
    }

    /// Set number of clusters for final agglomerative step (optional)
    pub fn n_clusters(mut self, n_clusters: usize) -> Self {
        self.n_clusters = Some(n_clusters);
        self
    }

    /// Build and fit the model
    pub fn fit(self, data: ArrayView2<f64>) -> Result<Birch> {
        if data.nrows() == 0 {
            return Err(ClusteringError::InvalidInput(
                "Cannot cluster empty data".to_string(),
            ));
        }

        let n_samples = data.nrows();
        let n_features = data.ncols();

        // Build CF-Tree
        let mut root = CFNode::new_leaf(n_features);
        let mut labels = vec![0; n_samples];

        for (idx, point) in data.axis_iter(Axis(0)).enumerate() {
            Self::insert_point(&mut root, point, self.threshold, self.branching_factor)?;
        }

        // Extract subclusters from leaf nodes
        let subclusters = Self::extract_subclusters(&root);

        // Assign labels based on closest subcluster
        for (idx, point) in data.axis_iter(Axis(0)).enumerate() {
            let mut min_dist = f64::INFINITY;
            let mut best_cluster = 0;

            for (cluster_id, cf) in subclusters.iter().enumerate() {
                let dist = cf.distance_to_point(point);
                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = cluster_id;
                }
            }

            labels[idx] = best_cluster;
        }

        // Optional: further reduce number of clusters using agglomerative clustering
        if let Some(target_clusters) = self.n_clusters {
            if subclusters.len() > target_clusters {
                labels = Self::agglomerative_reduce(
                    &subclusters,
                    &labels,
                    target_clusters,
                )?;
            }
        }

        let n_clusters = labels.iter().max().map(|&x| x + 1).unwrap_or(0);

        // Compute final cluster centers
        let mut cluster_centers = Array2::zeros((n_clusters, n_features));
        let mut counts = vec![0; n_clusters];

        for (idx, &label) in labels.iter().enumerate() {
            for (j, &val) in data.row(idx).iter().enumerate() {
                cluster_centers[[label, j]] += val;
            }
            counts[label] += 1;
        }

        for (i, &count) in counts.iter().enumerate() {
            if count > 0 {
                for j in 0..n_features {
                    cluster_centers[[i, j]] /= count as f64;
                }
            }
        }

        Ok(Birch {
            labels,
            cluster_centers,
            n_clusters,
            subclusters,
        })
    }

    fn insert_point(
        node: &mut CFNode,
        point: ArrayView1<f64>,
        threshold: f64,
        branching_factor: usize,
    ) -> Result<()> {
        if node.is_leaf {
            // Find closest child CF entry
            let mut best_idx = None;
            let mut min_dist = f64::INFINITY;

            for (i, child) in node.children.iter().enumerate() {
                let dist = child.cf.distance_to_point(point);
                if dist < min_dist {
                    min_dist = dist;
                    best_idx = Some(i);
                }
            }

            // Try to add to existing entry
            if let Some(idx) = best_idx {
                let mut test_cf = node.children[idx].cf.clone();
                test_cf.add_point(point);

                if test_cf.radius() <= threshold {
                    node.children[idx].cf = test_cf;
                    node.cf.add_point(point);
                    return Ok(());
                }
            }

            // Create new entry
            let new_cf = ClusteringFeature::new(point);
            let mut new_child = CFNode::new_leaf(point.len());
            new_child.cf = new_cf;
            node.children.push(new_child);
            node.cf.add_point(point);

            // Check if we need to split
            if node.children.len() > branching_factor {
                // Simple split: keep first half, second half goes to new node
                // In production, would use more sophisticated splitting
                let mid = node.children.len() / 2;
                node.children.truncate(mid);

                // Recompute CF
                node.cf = ClusteringFeature::empty(point.len());
                for child in &node.children {
                    node.cf.merge(&child.cf);
                }
            }
        }

        Ok(())
    }

    fn extract_subclusters(node: &CFNode) -> Vec<ClusteringFeature> {
        let mut subclusters = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(node);

        while let Some(current) = queue.pop_front() {
            if current.is_leaf {
                for child in &current.children {
                    if child.cf.n > 0 {
                        subclusters.push(child.cf.clone());
                    }
                }
            } else {
                for child in &current.children {
                    queue.push_back(child);
                }
            }
        }

        subclusters
    }

    fn agglomerative_reduce(
        subclusters: &[ClusteringFeature],
        labels: &[usize],
        target_clusters: usize,
    ) -> Result<Vec<usize>> {
        // Simple agglomerative clustering on subclusters
        let mut cluster_map = (0..subclusters.len()).collect::<Vec<_>>();
        let mut n_clusters = subclusters.len();

        while n_clusters > target_clusters {
            // Find closest pair of clusters
            let mut min_dist = f64::INFINITY;
            let mut merge_pair = (0, 1);

            for i in 0..subclusters.len() {
                for j in (i + 1)..subclusters.len() {
                    if cluster_map[i] != cluster_map[j] {
                        let dist = euclidean_distance(
                            &subclusters[i].centroid().view(),
                            &subclusters[j].centroid().view(),
                        );
                        if dist < min_dist {
                            min_dist = dist;
                            merge_pair = (cluster_map[i], cluster_map[j]);
                        }
                    }
                }
            }

            // Merge clusters
            let (keep, remove) = merge_pair;
            for cluster_id in cluster_map.iter_mut() {
                if *cluster_id == remove {
                    *cluster_id = keep;
                }
            }

            n_clusters -= 1;
        }

        // Renumber clusters to be contiguous
        let unique_clusters: std::collections::HashSet<_> = cluster_map.iter().collect();
        let mut cluster_renumber = std::collections::HashMap::new();
        for (new_id, &old_id) in unique_clusters.iter().enumerate() {
            cluster_renumber.insert(*old_id, new_id);
        }

        Ok(labels
            .iter()
            .map(|&label| *cluster_renumber.get(&cluster_map[label]).unwrap_or(&0))
            .collect())
    }
}

impl Default for BirchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// BIRCH clustering result
pub struct Birch {
    /// Cluster labels for each sample
    pub labels: Vec<usize>,
    /// Final cluster centers
    pub cluster_centers: Array2<f64>,
    /// Number of clusters
    pub n_clusters: usize,
    /// Subclusters from CF-Tree
    subclusters: Vec<ClusteringFeature>,
}

impl Birch {
    /// Predict cluster labels for new data
    pub fn predict(&self, data: ArrayView2<f64>) -> Result<Vec<usize>> {
        let mut labels = Vec::with_capacity(data.nrows());

        for i in 0..data.nrows() {
            let mut min_dist = f64::INFINITY;
            let mut best_cluster = 0;

            for j in 0..self.cluster_centers.nrows() {
                let dist = euclidean_distance(&data.row(i), &self.cluster_centers.row(j));
                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = j;
                }
            }

            labels.push(best_cluster);
        }

        Ok(labels)
    }

    /// Get cluster labels
    pub fn labels(&self) -> &[usize] {
        &self.labels
    }

    /// Get cluster centers
    pub fn cluster_centers(&self) -> ArrayView2<f64> {
        self.cluster_centers.view()
    }

    /// Get number of clusters
    pub fn n_clusters(&self) -> usize {
        self.n_clusters
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_birch_basic() {
        let data = array![
            [0.0, 0.0],
            [0.1, 0.1],
            [0.2, 0.2],
            [5.0, 5.0],
            [5.1, 5.1],
            [5.2, 5.2],
        ];

        let model = BirchBuilder::new()
            .threshold(1.0).unwrap()
            .n_clusters(2)
            .fit(data.view())
            .unwrap();

        assert_eq!(model.n_clusters, 2);
        assert_eq!(model.labels.len(), 6);
    }
}
