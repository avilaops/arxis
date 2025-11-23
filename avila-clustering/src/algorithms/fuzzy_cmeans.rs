//! Fuzzy C-Means clustering implementation
//!
//! Fuzzy C-Means (FCM) is a soft clustering algorithm where each data point
//! can belong to multiple clusters with varying degrees of membership.
//!
//! # Algorithm
//!
//! FCM minimizes the objective function:
//! J = Σᵢ Σⱼ (uᵢⱼ)ᵐ ||xᵢ - cⱼ||²
//!
//! where:
//! - uᵢⱼ is the membership degree of point i to cluster j
//! - m is the fuzziness parameter (typically 2.0)
//! - xᵢ is the i-th data point
//! - cⱼ is the j-th cluster center
//!
//! # References
//!
//! Bezdek, J.C. (1981). Pattern Recognition with Fuzzy Objective Function Algorithms

use crate::metrics::distance::euclidean_distance;
use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView2, Axis};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

/// Fuzzy C-Means clustering builder
pub struct FuzzyCMeansBuilder {
    n_clusters: usize,
    fuzziness: f64,
    max_iter: usize,
    tolerance: f64,
    random_state: Option<u64>,
    parallel: bool,
}

impl FuzzyCMeansBuilder {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            fuzziness: 2.0,
            max_iter: 150,
            tolerance: 1e-4,
            random_state: None,
            parallel: true,
        }
    }

    /// Set fuzziness parameter (m). Values > 1, typically 2.0
    pub fn fuzziness(mut self, fuzziness: f64) -> Self {
        self.fuzziness = fuzziness;
        self
    }

    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn random_state(mut self, seed: u64) -> Self {
        self.random_state = Some(seed);
        self
    }

    pub fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    pub fn build(self) -> FuzzyCMeans {
        FuzzyCMeans {
            n_clusters: self.n_clusters,
            fuzziness: self.fuzziness,
            max_iter: self.max_iter,
            tolerance: self.tolerance,
            random_state: self.random_state,
            parallel: self.parallel,
            centroids: None,
            membership: None,
            n_iter: 0,
        }
    }
}

/// Fuzzy C-Means clustering
pub struct FuzzyCMeans {
    n_clusters: usize,
    fuzziness: f64,
    max_iter: usize,
    tolerance: f64,
    random_state: Option<u64>,
    parallel: bool,
    centroids: Option<Array2<f64>>,
    membership: Option<Array2<f64>>,
    n_iter: usize,
}

impl FuzzyCMeans {
    pub fn new(n_clusters: usize) -> FuzzyCMeansBuilder {
        FuzzyCMeansBuilder::new(n_clusters)
    }

    /// Fit the model to the data
    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<()> {
        let (n_samples, n_features) = data.dim();

        if n_samples < self.n_clusters {
            return Err(ClusteringError::InvalidParameter(
                "Number of samples must be >= number of clusters".to_string(),
            ));
        }

        if self.fuzziness <= 1.0 {
            return Err(ClusteringError::InvalidParameter(
                "Fuzziness parameter must be > 1.0".to_string(),
            ));
        }

        // Initialize membership matrix randomly
        let mut membership = self.initialize_membership(n_samples)?;

        let mut centroids = Array2::zeros((self.n_clusters, n_features));

        for iter in 0..self.max_iter {
            // Update centroids
            self.update_centroids(data, &membership, &mut centroids);

            // Update membership matrix
            let new_membership = self.update_membership(data, &centroids)?;

            // Check convergence
            let diff = (&new_membership - &membership)
                .mapv(|x| x.abs())
                .sum();

            membership = new_membership;

            if diff < self.tolerance {
                self.n_iter = iter + 1;
                break;
            }

            if iter == self.max_iter - 1 {
                self.n_iter = self.max_iter;
            }
        }

        self.centroids = Some(centroids);
        self.membership = Some(membership);

        Ok(())
    }

    /// Initialize membership matrix with random values that sum to 1
    fn initialize_membership(&self, n_samples: usize) -> Result<Array2<f64>> {
        let seed = self.random_state.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let mut membership = Array2::zeros((n_samples, self.n_clusters));

        for i in 0..n_samples {
            // Generate random values
            let mut row_sum = 0.0;
            for j in 0..self.n_clusters {
                let val = rng.gen::<f64>();
                membership[[i, j]] = val;
                row_sum += val;
            }

            // Normalize to sum to 1
            for j in 0..self.n_clusters {
                membership[[i, j]] /= row_sum;
            }
        }

        Ok(membership)
    }

    /// Update cluster centroids based on membership matrix
    fn update_centroids(
        &self,
        data: &ArrayView2<f64>,
        membership: &Array2<f64>,
        centroids: &mut Array2<f64>,
    ) {
        let (n_samples, n_features) = data.dim();

        for k in 0..self.n_clusters {
            let mut numerator = Array1::zeros(n_features);
            let mut denominator = 0.0;

            for i in 0..n_samples {
                let u_ik = membership[[i, k]].powf(self.fuzziness);
                numerator = numerator + &(data.row(i).to_owned() * u_ik);
                denominator += u_ik;
            }

            if denominator > 0.0 {
                centroids.row_mut(k).assign(&(numerator / denominator));
            }
        }
    }

    /// Update membership matrix based on distances to centroids
    fn update_membership(
        &self,
        data: &ArrayView2<f64>,
        centroids: &Array2<f64>,
    ) -> Result<Array2<f64>> {
        let (n_samples, _) = data.dim();
        let mut membership = Array2::zeros((n_samples, self.n_clusters));

        let exponent = 2.0 / (self.fuzziness - 1.0);

        if self.parallel {
            membership
                .axis_iter_mut(Axis(0))
                .into_par_iter()
                .zip(data.axis_iter(Axis(0)).into_par_iter())
                .for_each(|(mut mem_row, data_point)| {
                    self.compute_membership_row(&data_point.insert_axis(Axis(0)), centroids, &mut mem_row, exponent);
                });
        } else {
            for (mut mem_row, data_point) in membership
                .axis_iter_mut(Axis(0))
                .zip(data.axis_iter(Axis(0)))
            {
                self.compute_membership_row(&data_point.insert_axis(Axis(0)), centroids, &mut mem_row, exponent);
            }
        }

        Ok(membership)
    }

    /// Compute membership values for a single data point
    fn compute_membership_row(
        &self,
        data_point: &ArrayView2<f64>,
        centroids: &Array2<f64>,
        mem_row: &mut ndarray::ArrayViewMut1<f64>,
        exponent: f64,
    ) {
        // Compute distances to all centroids
        let mut distances = Vec::with_capacity(self.n_clusters);
        let point_1d = data_point.row(0);
        for k in 0..self.n_clusters {
            let dist = euclidean_distance(&point_1d, &centroids.row(k));
            distances.push(if dist < 1e-10 { 1e-10 } else { dist });
        }

        // Update membership values
        for j in 0..self.n_clusters {
            let mut sum = 0.0;
            for k in 0..self.n_clusters {
                sum += (distances[j] / distances[k]).powf(exponent);
            }
            mem_row[j] = 1.0 / sum;
        }
    }

    /// Predict cluster labels (hard assignment to most likely cluster)
    pub fn predict(&self, data: &ArrayView2<f64>) -> Result<Array1<usize>> {
        let centroids = self.centroids.as_ref()
            .ok_or_else(|| ClusteringError::InvalidParameter("Model not fitted".to_string()))?;

        let (n_samples, _) = data.dim();
        let mut labels = Array1::zeros(n_samples);

        for (i, sample) in data.axis_iter(Axis(0)).enumerate() {
            let mut min_dist = f64::INFINITY;
            let mut best_cluster = 0;

            for (k, centroid) in centroids.axis_iter(Axis(0)).enumerate() {
                let dist = euclidean_distance(&sample.into_owned().view(), &centroid.into_owned().view());
                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = k;
                }
            }

            labels[i] = best_cluster;
        }

        Ok(labels)
    }

    /// Get cluster centroids
    pub fn centroids(&self) -> Option<&Array2<f64>> {
        self.centroids.as_ref()
    }

    /// Get membership matrix (soft assignments)
    pub fn membership(&self) -> Option<&Array2<f64>> {
        self.membership.as_ref()
    }

    /// Get number of iterations performed
    pub fn n_iter(&self) -> usize {
        self.n_iter
    }

    /// Get fuzzy partition coefficient (FPC) - measure of clustering quality
    /// FPC ranges from 1/c to 1, where higher is better
    pub fn fuzzy_partition_coefficient(&self) -> Result<f64> {
        let membership = self.membership.as_ref()
            .ok_or_else(|| ClusteringError::InvalidParameter("Model not fitted".to_string()))?;

        let (n_samples, n_clusters) = membership.dim();
        let mut fpc = 0.0;

        for i in 0..n_samples {
            for k in 0..n_clusters {
                fpc += membership[[i, k]].powi(2);
            }
        }

        Ok(fpc / n_samples as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_fuzzy_cmeans_simple() {
        // Simple 2D data with 2 clusters
        let data = array![
            [1.0, 1.0],
            [1.5, 2.0],
            [3.0, 4.0],
            [5.0, 7.0],
            [3.5, 5.0],
            [4.5, 5.0],
            [3.5, 4.5],
        ];

        let mut fcm = FuzzyCMeans::new(2)
            .random_state(42)
            .build();

        fcm.fit(&data.view()).unwrap();

        let labels = fcm.predict(&data.view()).unwrap();
        assert_eq!(labels.len(), 7);

        let centroids = fcm.centroids().unwrap();
        assert_eq!(centroids.dim(), (2, 2));

        let fpc = fcm.fuzzy_partition_coefficient().unwrap();
        assert!(fpc > 0.5 && fpc <= 1.0);
    }

    #[test]
    fn test_membership_sum_to_one() {
        let data = array![
            [1.0, 1.0],
            [2.0, 2.0],
            [10.0, 10.0],
        ];

        let mut fcm = FuzzyCMeans::new(2)
            .random_state(42)
            .build();

        fcm.fit(&data.view()).unwrap();

        let membership = fcm.membership().unwrap();

        // Check that each row sums to 1
        for i in 0..membership.nrows() {
            let row_sum: f64 = membership.row(i).sum();
            assert!((row_sum - 1.0).abs() < 1e-6);
        }
    }
}
