//! Online/Incremental Clustering
//!
//! Algorithms that can update clusters with new data points without
//! reprocessing the entire dataset. Essential for streaming data.

use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView1};
use std::collections::VecDeque;

/// Online K-Means (Mini-batch style)
pub struct OnlineKMeans {
    n_clusters: usize,
    centroids: Option<Array2<f64>>,
    cluster_counts: Vec<usize>,
    learning_rate: f64,
    batch_size: usize,
    buffer: VecDeque<Array1<f64>>,
}

impl OnlineKMeans {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            centroids: None,
            cluster_counts: vec![0; n_clusters],
            learning_rate: 0.1,
            batch_size: 100,
            buffer: VecDeque::new(),
        }
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    pub fn learning_rate(mut self, lr: f64) -> Self {
        self.learning_rate = lr.clamp(0.0, 1.0);
        self
    }

    /// Update clusters with a single new point
    pub fn partial_fit(&mut self, point: ArrayView1<f64>) -> Result<usize> {
        self.buffer.push_back(point.to_owned());

        // Process when buffer is full
        if self.buffer.len() >= self.batch_size {
            self.process_batch()?;
        }

        // Assign to nearest cluster
        self.predict_one(point)
    }

    /// Initialize or update with a batch of points
    pub fn fit_batch(&mut self, data: &Array2<f64>) -> Result<()> {
        if self.centroids.is_none() {
            // Initialize centroids
            let n_features = data.ncols();
            let mut centroids = Array2::<f64>::zeros((self.n_clusters, n_features));

            for k in 0..self.n_clusters {
                let idx = (k * data.nrows()) / self.n_clusters;
                centroids.row_mut(k).assign(&data.row(idx));
            }

            self.centroids = Some(centroids);
        }

        for i in 0..data.nrows() {
            self.partial_fit(data.row(i))?;
        }

        Ok(())
    }

    fn process_batch(&mut self) -> Result<()> {
        if self.centroids.is_none() || self.buffer.is_empty() {
            return Ok(());
        }

        while let Some(point) = self.buffer.pop_front() {
            // Find nearest centroid
            let cluster = self.predict_one(point.view())?;

            let centroids = self.centroids.as_mut().unwrap();

            // Update centroid with moving average
            let count = self.cluster_counts[cluster] as f64;
            let lr = self.learning_rate / (1.0 + count);

            for j in 0..centroids.ncols() {
                centroids[[cluster, j]] =
                    (1.0 - lr) * centroids[[cluster, j]] + lr * point[j];
            }

            self.cluster_counts[cluster] += 1;
        }

        Ok(())
    }

    fn predict_one(&self, point: ArrayView1<f64>) -> Result<usize> {
        if let Some(ref centroids) = self.centroids {
            let mut min_dist = f64::INFINITY;
            let mut best_cluster = 0;

            for k in 0..self.n_clusters {
                let dist: f64 = point.iter()
                    .zip(centroids.row(k).iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>()
                    .sqrt();

                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = k;
                }
            }

            Ok(best_cluster)
        } else {
            Err(ClusteringError::InvalidParameter(
                "Model not initialized".to_string()
            ))
        }
    }

    /// Predict cluster for new points
    pub fn predict(&self, data: &Array2<f64>) -> Result<Vec<usize>> {
        let mut labels = Vec::with_capacity(data.nrows());

        for i in 0..data.nrows() {
            labels.push(self.predict_one(data.row(i))?);
        }

        Ok(labels)
    }

    /// Get current centroids
    pub fn centroids(&self) -> Option<&Array2<f64>> {
        self.centroids.as_ref()
    }

    /// Flush buffer and process remaining points
    pub fn flush(&mut self) -> Result<()> {
        self.process_batch()
    }
}

/// BIRCH-style online clustering with CF-Tree
pub struct OnlineBIRCH {
    threshold: f64,
    max_clusters: usize,
    cluster_features: Vec<ClusterFeature>,
}

#[derive(Clone)]
struct ClusterFeature {
    n: usize,
    linear_sum: Array1<f64>,
    squared_sum: f64,
}

impl ClusterFeature {
    fn new(point: ArrayView1<f64>) -> Self {
        Self {
            n: 1,
            linear_sum: point.to_owned(),
            squared_sum: point.iter().map(|x| x * x).sum(),
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
        let centroid_sq: f64 = centroid.iter().map(|x| x * x).sum();
        ((self.squared_sum / self.n as f64) - centroid_sq).sqrt().max(0.0)
    }

    fn add_point(&mut self, point: ArrayView1<f64>) {
        self.n += 1;
        self.linear_sum = &self.linear_sum + &point;
        self.squared_sum += point.iter().map(|x| x * x).sum::<f64>();
    }

    fn distance_to(&self, point: ArrayView1<f64>) -> f64 {
        let centroid = self.centroid();
        point.iter()
            .zip(centroid.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl OnlineBIRCH {
    pub fn new(threshold: f64, max_clusters: usize) -> Self {
        Self {
            threshold,
            max_clusters,
            cluster_features: Vec::new(),
        }
    }

    /// Add a single point and update cluster structure
    pub fn partial_fit(&mut self, point: ArrayView1<f64>) -> Result<usize> {
        if self.cluster_features.is_empty() {
            // First point creates first cluster
            self.cluster_features.push(ClusterFeature::new(point));
            return Ok(0);
        }

        // Find closest cluster
        let mut min_dist = f64::INFINITY;
        let mut best_idx = 0;

        for (i, cf) in self.cluster_features.iter().enumerate() {
            let dist = cf.distance_to(point);
            if dist < min_dist {
                min_dist = dist;
                best_idx = i;
            }
        }

        // Try to add to closest cluster
        let mut test_cf = self.cluster_features[best_idx].clone();
        test_cf.add_point(point);

        if test_cf.radius() <= self.threshold {
            self.cluster_features[best_idx] = test_cf;
            Ok(best_idx)
        } else if self.cluster_features.len() < self.max_clusters {
            // Create new cluster
            self.cluster_features.push(ClusterFeature::new(point));
            Ok(self.cluster_features.len() - 1)
        } else {
            // Force add to closest (no more space for new clusters)
            self.cluster_features[best_idx].add_point(point);
            Ok(best_idx)
        }
    }

    /// Get current number of clusters
    pub fn n_clusters(&self) -> usize {
        self.cluster_features.len()
    }

    /// Get cluster centroids
    pub fn centroids(&self) -> Array2<f64> {
        if self.cluster_features.is_empty() {
            return Array2::zeros((0, 0));
        }

        let n_features = self.cluster_features[0].linear_sum.len();
        let mut centroids = Array2::zeros((self.cluster_features.len(), n_features));

        for (i, cf) in self.cluster_features.iter().enumerate() {
            centroids.row_mut(i).assign(&cf.centroid());
        }

        centroids
    }

    /// Predict cluster for new point
    pub fn predict_one(&self, point: ArrayView1<f64>) -> Result<usize> {
        if self.cluster_features.is_empty() {
            return Err(ClusteringError::InvalidParameter(
                "Model not initialized".to_string()
            ));
        }

        let mut min_dist = f64::INFINITY;
        let mut best_idx = 0;

        for (i, cf) in self.cluster_features.iter().enumerate() {
            let dist = cf.distance_to(point);
            if dist < min_dist {
                min_dist = dist;
                best_idx = i;
            }
        }

        Ok(best_idx)
    }
}

/// Sliding window clustering for concept drift detection
pub struct SlidingWindowClustering {
    window_size: usize,
    overlap: usize,
    base_clusterer: OnlineKMeans,
    windows: VecDeque<Vec<usize>>,
}

impl SlidingWindowClustering {
    pub fn new(n_clusters: usize, window_size: usize, overlap: usize) -> Self {
        Self {
            window_size,
            overlap,
            base_clusterer: OnlineKMeans::new(n_clusters),
            windows: VecDeque::new(),
        }
    }

    /// Add points and detect concept drift
    pub fn update(&mut self, points: &Array2<f64>) -> Result<ConceptDriftInfo> {
        let mut labels = Vec::new();

        for i in 0..points.nrows() {
            let label = self.base_clusterer.partial_fit(points.row(i))?;
            labels.push(label);
        }

        self.windows.push_back(labels.clone());

        // Keep only recent windows
        if self.windows.len() > self.window_size / (self.window_size - self.overlap) {
            self.windows.pop_front();
        }

        // Detect drift by comparing windows
        let drift_detected = self.detect_drift();

        Ok(ConceptDriftInfo {
            labels,
            drift_detected,
            n_windows: self.windows.len(),
        })
    }

    fn detect_drift(&self) -> bool {
        if self.windows.len() < 2 {
            return false;
        }

        // Simple drift detection: compare cluster distributions
        let recent = self.windows.back().unwrap();
        let older = self.windows.front().unwrap();

        // Calculate distribution difference
        let mut recent_dist = vec![0; self.base_clusterer.n_clusters];
        let mut older_dist = vec![0; self.base_clusterer.n_clusters];

        for &label in recent {
            recent_dist[label] += 1;
        }
        for &label in older {
            older_dist[label] += 1;
        }

        // Chi-square-like distance
        let mut distance = 0.0;
        for k in 0..self.base_clusterer.n_clusters {
            let r = recent_dist[k] as f64 / recent.len() as f64;
            let o = older_dist[k] as f64 / older.len() as f64;
            distance += (r - o).abs();
        }

        distance > 0.3 // Threshold for drift
    }
}

pub struct ConceptDriftInfo {
    pub labels: Vec<usize>,
    pub drift_detected: bool,
    pub n_windows: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_online_kmeans() {
        let mut online = OnlineKMeans::new(2);

        let batch1 = array![[0.0, 0.0], [0.1, 0.1]];
        online.fit_batch(&batch1).unwrap();

        let point = array![0.05, 0.05];
        let label = online.partial_fit(point.view()).unwrap();

        assert!(label < 2);
    }

    #[test]
    fn test_online_birch() {
        let mut birch = OnlineBIRCH::new(1.0, 10);

        birch.partial_fit(array![0.0, 0.0].view()).unwrap();
        birch.partial_fit(array![0.1, 0.1].view()).unwrap();
        birch.partial_fit(array![5.0, 5.0].view()).unwrap();

        assert!(birch.n_clusters() >= 2);
    }
}
