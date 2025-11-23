//! Affinity Propagation clustering implementation
//!
//! Affinity Propagation is a clustering algorithm that identifies exemplars
//! (representative points) among data points and forms clusters around them.
//! Unlike K-Means, it does not require the number of clusters to be specified.
//!
//! # Algorithm
//!
//! The algorithm passes real-valued messages between data points until a high-quality
//! set of exemplars and corresponding clusters emerges.
//!
//! Two types of messages are exchanged:
//! - Responsibility r(i,k): how well-suited point k is to be exemplar for point i
//! - Availability a(i,k): how appropriate it would be for i to choose k as exemplar
//!
//! # References
//!
//! Frey, B. J., & Dueck, D. (2007). Clustering by passing messages between data points.
//! Science, 315(5814), 972-976.

use crate::metrics::distance::euclidean_distance;
use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView2, Axis};
use rayon::prelude::*;

/// Affinity Propagation clustering builder
pub struct AffinityPropagationBuilder {
    damping: f64,
    max_iter: usize,
    convergence_iter: usize,
    preference: Option<f64>,
    parallel: bool,
}

impl AffinityPropagationBuilder {
    pub fn new() -> Self {
        Self {
            damping: 0.5,
            max_iter: 200,
            convergence_iter: 15,
            preference: None,
            parallel: true,
        }
    }

    /// Set damping factor (0.5 to 1.0). Higher values increase numerical stability
    pub fn damping(mut self, damping: f64) -> std::result::Result<Self, ClusteringError> {
        if !(0.5..=1.0).contains(&damping) {
            return Err(ClusteringError::InvalidParameter(
                "damping must be between 0.5 and 1.0".to_string(),
            ));
        }
        self.damping = damping;
        Ok(self)
    }

    /// Set maximum number of iterations
    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Set number of iterations with no change to consider convergence
    pub fn convergence_iter(mut self, convergence_iter: usize) -> Self {
        self.convergence_iter = convergence_iter;
        self
    }

    /// Set preference value for each point (affects number of clusters)
    /// If None, will be set to median of similarities
    pub fn preference(mut self, preference: f64) -> Self {
        self.preference = Some(preference);
        self
    }

    /// Enable or disable parallel processing
    pub fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    /// Build and fit the model
    pub fn fit(self, data: ArrayView2<f64>) -> Result<AffinityPropagation> {
        if data.nrows() < 2 {
            return Err(ClusteringError::InvalidInput(
                "Need at least 2 samples for clustering".to_string(),
            ));
        }

        let n_samples = data.nrows();

        // Compute similarity matrix (negative squared Euclidean distance)
        let mut similarity = Array2::<f64>::zeros((n_samples, n_samples));

        for i in 0..n_samples {
            for j in 0..n_samples {
                if i != j {
                    let dist = euclidean_distance(&data.row(i), &data.row(j));
                    similarity[[i, j]] = -dist * dist;
                }
            }
        }

        // Set preference (diagonal of similarity matrix)
        let preference = self.preference.unwrap_or_else(|| {
            let mut values: Vec<f64> = similarity
                .iter()
                .copied()
                .filter(|&x| x.is_finite())
                .collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            values[values.len() / 2] // median
        });

        for i in 0..n_samples {
            similarity[[i, i]] = preference;
        }

        // Initialize responsibility and availability matrices
        let mut responsibility = Array2::<f64>::zeros((n_samples, n_samples));
        let mut availability = Array2::<f64>::zeros((n_samples, n_samples));

        let mut exemplars = Vec::new();
        let mut no_change_count = 0;

        // Main message-passing loop
        for iter in 0..self.max_iter {
            let old_exemplars = exemplars.clone();

            // Update responsibility
            let old_responsibility = responsibility.clone();
            for i in 0..n_samples {
                for k in 0..n_samples {
                    // Find max(a(i,k') + s(i,k')) for k' != k
                    let mut max_val = f64::NEG_INFINITY;
                    for kk in 0..n_samples {
                        if kk != k {
                            let val = availability[[i, kk]] + similarity[[i, kk]];
                            if val > max_val {
                                max_val = val;
                            }
                        }
                    }

                    let new_r = similarity[[i, k]] - max_val;
                    responsibility[[i, k]] =
                        self.damping * old_responsibility[[i, k]] + (1.0 - self.damping) * new_r;
                }
            }

            // Update availability
            let old_availability = availability.clone();
            for i in 0..n_samples {
                for k in 0..n_samples {
                    if i != k {
                        // sum(max(0, r(i',k))) for i' != i and i' != k
                        let mut sum = 0.0;
                        for ii in 0..n_samples {
                            if ii != i && ii != k {
                                sum += responsibility[[ii, k]].max(0.0);
                            }
                        }

                        let new_a = (responsibility[[k, k]] + sum).min(0.0);
                        availability[[i, k]] =
                            self.damping * old_availability[[i, k]] + (1.0 - self.damping) * new_a;
                    } else {
                        // For diagonal: sum(max(0, r(i',k))) for i' != k
                        let mut sum = 0.0;
                        for ii in 0..n_samples {
                            if ii != k {
                                sum += responsibility[[ii, k]].max(0.0);
                            }
                        }

                        availability[[k, k]] =
                            self.damping * old_availability[[k, k]] + (1.0 - self.damping) * sum;
                    }
                }
            }

            // Identify exemplars
            exemplars.clear();
            for i in 0..n_samples {
                if responsibility[[i, i]] + availability[[i, i]] > 0.0 {
                    exemplars.push(i);
                }
            }

            // Check convergence
            if exemplars == old_exemplars {
                no_change_count += 1;
                if no_change_count >= self.convergence_iter {
                    break;
                }
            } else {
                no_change_count = 0;
            }
        }

        // Assign labels
        let mut labels = vec![-1i32; n_samples];
        let mut cluster_centers = Vec::new();

        if exemplars.is_empty() {
            // If no exemplars found, treat all points as noise
            return Ok(AffinityPropagation {
                labels,
                cluster_centers: Array2::zeros((0, data.ncols())),
                n_clusters: 0,
                n_iter: self.max_iter,
            });
        }

        // Assign cluster IDs to exemplars
        for (cluster_id, &exemplar) in exemplars.iter().enumerate() {
            labels[exemplar] = cluster_id as i32;
        }

        // Assign remaining points to nearest exemplar
        for i in 0..n_samples {
            if labels[i] == -1 {
                let mut best_exemplar = 0;
                let mut best_score = f64::NEG_INFINITY;

                for &exemplar in &exemplars {
                    let score = similarity[[i, exemplar]];
                    if score > best_score {
                        best_score = score;
                        best_exemplar = exemplar;
                    }
                }

                labels[i] = labels[best_exemplar];
            }
        }

        // Extract cluster centers
        for &exemplar in &exemplars {
            cluster_centers.push(data.row(exemplar).to_owned());
        }

        let centers = Array2::from_shape_fn(
            (cluster_centers.len(), data.ncols()),
            |(i, j)| cluster_centers[i][j],
        );

        Ok(AffinityPropagation {
            labels,
            cluster_centers: centers,
            n_clusters: exemplars.len(),
            n_iter: self.max_iter,
        })
    }
}

impl Default for AffinityPropagationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Affinity Propagation clustering result
pub struct AffinityPropagation {
    /// Cluster labels for each sample (-1 for noise)
    pub labels: Vec<i32>,
    /// Cluster centers (exemplars)
    pub cluster_centers: Array2<f64>,
    /// Number of clusters found
    pub n_clusters: usize,
    /// Number of iterations performed
    pub n_iter: usize,
}

impl AffinityPropagation {
    /// Predict cluster labels for new data
    pub fn predict(&self, data: ArrayView2<f64>) -> Result<Vec<i32>> {
        if self.cluster_centers.nrows() == 0 {
            return Ok(vec![-1; data.nrows()]);
        }

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

            labels.push(best_cluster as i32);
        }

        Ok(labels)
    }

    /// Get cluster labels
    pub fn labels(&self) -> &[i32] {
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
    fn test_affinity_propagation_basic() {
        let data = array![
            [0.0, 0.0],
            [0.1, 0.1],
            [5.0, 5.0],
            [5.1, 5.1],
        ];

        let model = AffinityPropagationBuilder::new()
            .max_iter(100)
            .fit(data.view())
            .unwrap();

        assert!(model.n_clusters >= 2);
        assert!(model.n_clusters <= 4);
    }

    #[test]
    fn test_affinity_propagation_predict() {
        let data = array![
            [0.0, 0.0],
            [0.1, 0.1],
            [5.0, 5.0],
            [5.1, 5.1],
        ];

        let model = AffinityPropagationBuilder::new().fit(data.view()).unwrap();

        let new_data = array![[0.05, 0.05], [5.05, 5.05]];
        let predictions = model.predict(new_data.view()).unwrap();

        assert_eq!(predictions.len(), 2);
    }
}
