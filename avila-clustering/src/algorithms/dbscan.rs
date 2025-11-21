//! DBSCAN (Density-Based Spatial Clustering of Applications with Noise)

use crate::metrics::distance::{euclidean_distance, Metric};
use crate::{ClusteringError, Result};
use ndarray::{Array1, ArrayView2};

/// DBSCAN clustering
pub struct DBSCAN {
    eps: f64,
    min_samples: usize,
    metric: Metric,
}

impl DBSCAN {
    pub fn new(eps: f64, min_samples: usize) -> Self {
        Self {
            eps,
            min_samples,
            metric: Metric::Euclidean,
        }
    }

    pub fn metric(mut self, metric: Metric) -> Self {
        self.metric = metric;
        self
    }

    pub fn fit(&self, data: &ArrayView2<f64>) -> Result<DBSCANResult> {
        let n_samples = data.dim().0;
        let mut labels = Array1::from_elem(n_samples, -1i32);
        let mut visited = vec![false; n_samples];
        let mut cluster_id = 0i32;

        for i in 0..n_samples {
            if visited[i] {
                continue;
            }
            visited[i] = true;

            // Find neighbors
            let neighbors = self.range_query(data, i, self.eps);

            if neighbors.len() < self.min_samples {
                // Mark as noise
                labels[i] = -1;
            } else {
                // Start a new cluster
                self.expand_cluster(data, &mut labels, &mut visited, i, &neighbors, cluster_id)?;
                cluster_id += 1;
            }
        }

        Ok(DBSCANResult {
            labels,
            n_clusters: cluster_id as usize,
        })
    }

    fn range_query(&self, data: &ArrayView2<f64>, point_idx: usize, eps: f64) -> Vec<usize> {
        let n_samples = data.dim().0;
        let point = data.row(point_idx);
        let mut neighbors = Vec::new();

        for i in 0..n_samples {
            let other = data.row(i);
            let dist = match self.metric {
                Metric::Euclidean => euclidean_distance(&point, &other),
                _ => self
                    .metric
                    .distance(&point, &other)
                    .unwrap_or(f64::INFINITY),
            };

            if dist <= eps {
                neighbors.push(i);
            }
        }

        neighbors
    }

    fn expand_cluster(
        &self,
        data: &ArrayView2<f64>,
        labels: &mut Array1<i32>,
        visited: &mut Vec<bool>,
        point_idx: usize,
        neighbors: &[usize],
        cluster_id: i32,
    ) -> Result<()> {
        labels[point_idx] = cluster_id;
        let mut seed_set: Vec<usize> = neighbors.to_vec();
        let mut i = 0;

        while i < seed_set.len() {
            let q = seed_set[i];
            i += 1;

            if !visited[q] {
                visited[q] = true;

                // Find neighbors of q
                let q_neighbors = self.range_query(data, q, self.eps);

                if q_neighbors.len() >= self.min_samples {
                    // Add new neighbors to seed set
                    for &neighbor in &q_neighbors {
                        if !seed_set.contains(&neighbor) {
                            seed_set.push(neighbor);
                        }
                    }
                }
            }

            // Add q to cluster if not already assigned
            if labels[q] == -1 {
                labels[q] = cluster_id;
            }
        }

        Ok(())
    }
}

/// Result of DBSCAN clustering
pub struct DBSCANResult {
    pub labels: Array1<i32>, // -1 for noise points
    pub n_clusters: usize,
}

impl DBSCANResult {
    pub fn labels(&self) -> &Array1<i32> {
        &self.labels
    }

    pub fn n_clusters(&self) -> usize {
        self.n_clusters
    }

    pub fn n_noise_points(&self) -> usize {
        self.labels.iter().filter(|&&l| l == -1).count()
    }
}
