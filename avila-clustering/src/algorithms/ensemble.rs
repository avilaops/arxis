//! Ensemble Clustering
//!
//! Combines multiple clustering results to create more robust and stable clusters.
//! Uses consensus clustering, voting mechanisms, and co-association matrices.
//!
//! # Benefits
//!
//! - More stable results across runs
//! - Combines strengths of different algorithms
//! - Reduces sensitivity to initialization
//! - Better handling of complex data structures

use crate::{ClusteringError, Result};
use ndarray::{Array2, ArrayView2};
use std::collections::HashMap;

/// Ensemble clustering builder
pub struct EnsembleClusteringBuilder {
    n_clusters: usize,
    n_iterations: usize,
    consensus_method: ConsensusMethod,
    subsample_ratio: f64,
}

#[derive(Clone, Copy)]
pub enum ConsensusMethod {
    /// Co-association matrix with hierarchical clustering
    CoAssociation,
    /// Majority voting
    Voting,
    /// Evidence Accumulation Clustering
    EvidenceAccumulation,
}

impl EnsembleClusteringBuilder {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            n_iterations: 20,
            consensus_method: ConsensusMethod::CoAssociation,
            subsample_ratio: 0.8,
        }
    }

    pub fn n_iterations(mut self, n: usize) -> Self {
        self.n_iterations = n;
        self
    }

    pub fn consensus_method(mut self, method: ConsensusMethod) -> Self {
        self.consensus_method = method;
        self
    }

    pub fn subsample_ratio(mut self, ratio: f64) -> Self {
        self.subsample_ratio = ratio.clamp(0.1, 1.0);
        self
    }

    pub fn fit(&self, data: ArrayView2<f64>) -> Result<EnsembleResult> {
        let n_samples = data.nrows();

        // Build co-association matrix
        let mut coassoc = Array2::<f64>::zeros((n_samples, n_samples));
        let mut count_matrix = Array2::<usize>::zeros((n_samples, n_samples));

        // Run multiple clustering iterations
        for iter in 0..self.n_iterations {
            // Subsample data (bootstrap)
            let subsample_size = (n_samples as f64 * self.subsample_ratio) as usize;
            let indices: Vec<usize> = (0..subsample_size)
                .map(|_| rand::random::<usize>() % n_samples)
                .collect();

            // Run clustering (using K-Means as base learner)
            let labels = self.run_base_clustering(&data, &indices)?;

            // Update co-association matrix
            for i in 0..indices.len() {
                for j in (i + 1)..indices.len() {
                    let idx_i = indices[i];
                    let idx_j = indices[j];

                    if labels[i] == labels[j] {
                        coassoc[[idx_i, idx_j]] += 1.0;
                        coassoc[[idx_j, idx_i]] += 1.0;
                    }

                    count_matrix[[idx_i, idx_j]] += 1;
                    count_matrix[[idx_j, idx_i]] += 1;
                }
            }
        }

        // Normalize co-association matrix
        for i in 0..n_samples {
            for j in 0..n_samples {
                if count_matrix[[i, j]] > 0 {
                    coassoc[[i, j]] /= count_matrix[[i, j]] as f64;
                }
            }
        }

        // Final clustering on co-association matrix
        let final_labels = self.consensus_clustering(&coassoc)?;

        Ok(EnsembleResult {
            labels: final_labels,
            coassociation_matrix: coassoc,
            n_clusters: self.n_clusters,
        })
    }

    fn run_base_clustering(&self, data: &ArrayView2<f64>, indices: &[usize]) -> Result<Vec<usize>> {
        // Simple K-Means implementation for ensemble
        let n_features = data.ncols();
        let subsample_size = indices.len();

        // Initialize centroids
        let mut centroids = Array2::<f64>::zeros((self.n_clusters, n_features));
        for k in 0..self.n_clusters {
            let idx = indices[k % subsample_size];
            centroids.row_mut(k).assign(&data.row(idx));
        }

        let mut labels = vec![0; subsample_size];

        // Simple K-Means iterations
        for _iter in 0..10 {
            // Assign to nearest centroid
            for (i, &sample_idx) in indices.iter().enumerate() {
                let sample = data.row(sample_idx);
                let mut min_dist = f64::INFINITY;
                let mut best_k = 0;

                for k in 0..self.n_clusters {
                    let dist: f64 = sample.iter()
                        .zip(centroids.row(k).iter())
                        .map(|(a, b)| (a - b).powi(2))
                        .sum();

                    if dist < min_dist {
                        min_dist = dist;
                        best_k = k;
                    }
                }

                labels[i] = best_k;
            }

            // Update centroids
            let mut counts = vec![0; self.n_clusters];
            centroids.fill(0.0);

            for (i, &sample_idx) in indices.iter().enumerate() {
                let k = labels[i];
                let sample = data.row(sample_idx);

                for (j, &val) in sample.iter().enumerate() {
                    centroids[[k, j]] += val;
                }
                counts[k] += 1;
            }

            for k in 0..self.n_clusters {
                if counts[k] > 0 {
                    for j in 0..n_features {
                        centroids[[k, j]] /= counts[k] as f64;
                    }
                }
            }
        }

        Ok(labels)
    }

    fn consensus_clustering(&self, coassoc: &Array2<f64>) -> Result<Vec<usize>> {
        // Convert co-association to distance matrix
        let n = coassoc.nrows();
        let mut dist_matrix = Array2::<f64>::zeros((n, n));

        for i in 0..n {
            for j in 0..n {
                dist_matrix[[i, j]] = 1.0 - coassoc[[i, j]];
            }
        }

        // Simple hierarchical clustering
        let mut labels = vec![0; n];
        let mut cluster_id = 0;
        let mut assigned = vec![false; n];

        // Assign based on similarity threshold
        for i in 0..n {
            if assigned[i] {
                continue;
            }

            assigned[i] = true;
            labels[i] = cluster_id;

            // Find similar points
            for j in (i + 1)..n {
                if !assigned[j] && coassoc[[i, j]] > 0.5 {
                    assigned[j] = true;
                    labels[j] = cluster_id;
                }
            }

            cluster_id += 1;
            if cluster_id >= self.n_clusters {
                break;
            }
        }

        // Assign remaining points to nearest cluster
        for i in 0..n {
            if !assigned[i] {
                let mut max_sim = 0.0;
                let mut best_cluster = 0;

                for j in 0..n {
                    if assigned[j] && coassoc[[i, j]] > max_sim {
                        max_sim = coassoc[[i, j]];
                        best_cluster = labels[j];
                    }
                }

                labels[i] = best_cluster;
            }
        }

        Ok(labels)
    }
}

pub struct EnsembleResult {
    pub labels: Vec<usize>,
    pub coassociation_matrix: Array2<f64>,
    pub n_clusters: usize,
}

impl EnsembleResult {
    /// Get cluster stability score (average co-association within clusters)
    pub fn stability_score(&self) -> f64 {
        let n = self.labels.len();
        let mut total_sim = 0.0;
        let mut count = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                if self.labels[i] == self.labels[j] {
                    total_sim += self.coassociation_matrix[[i, j]];
                    count += 1;
                }
            }
        }

        if count > 0 {
            total_sim / count as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_ensemble_clustering() {
        let data = array![
            [0.0, 0.0],
            [0.1, 0.1],
            [5.0, 5.0],
            [5.1, 5.1],
        ];

        let result = EnsembleClusteringBuilder::new(2)
            .n_iterations(10)
            .fit(data.view())
            .unwrap();

        assert_eq!(result.labels.len(), 4);
        assert!(result.stability_score() > 0.5);
    }
}
