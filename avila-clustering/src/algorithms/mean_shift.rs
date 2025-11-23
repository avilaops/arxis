//! Mean Shift clustering implementation
//!
//! Mean Shift is a non-parametric clustering algorithm that does not require
//! specifying the number of clusters in advance. It works by iteratively shifting
//! each point towards the mode of the density function.
//!
//! # Algorithm
//!
//! For each point:
//! 1. Define a window (sphere) around it
//! 2. Compute the mean of points within the window
//! 3. Shift the window to the mean
//! 4. Repeat until convergence
//!
//! Points that converge to the same mode belong to the same cluster.
//!
//! # References
//!
//! Comaniciu, D., & Meer, P. (2002). Mean shift: A robust approach toward
//! feature space analysis.

use crate::metrics::distance::euclidean_distance;
use crate::{ClusteringError, Result};
use kiddo::KdTree;
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use rayon::prelude::*;
use std::collections::HashMap;

/// Kernel function for Mean Shift
#[derive(Debug, Clone, Copy)]
pub enum Kernel {
    /// Flat (uniform) kernel
    Flat,
    /// Gaussian kernel
    Gaussian,
    /// Epanechnikov kernel
    Epanechnikov,
}

/// Mean Shift clustering builder
pub struct MeanShiftBuilder {
    bandwidth: Option<f64>,
    kernel: Kernel,
    max_iter: usize,
    tolerance: f64,
    bin_seeding: bool,
    min_bin_freq: usize,
    cluster_all: bool,
    parallel: bool,
}

impl MeanShiftBuilder {
    pub fn new() -> Self {
        Self {
            bandwidth: None,
            kernel: Kernel::Gaussian,
            max_iter: 300,
            tolerance: 1e-3,
            bin_seeding: true,
            min_bin_freq: 1,
            cluster_all: true,
            parallel: true,
        }
    }

    /// Set bandwidth. If None, will be estimated automatically
    pub fn bandwidth(mut self, bandwidth: f64) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// Set kernel function
    pub fn kernel(mut self, kernel: Kernel) -> Self {
        self.kernel = kernel;
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

    /// Use bin seeding for faster convergence
    pub fn bin_seeding(mut self, bin_seeding: bool) -> Self {
        self.bin_seeding = bin_seeding;
        self
    }

    /// Minimum frequency for a bin to be considered a seed
    pub fn min_bin_freq(mut self, min_bin_freq: usize) -> Self {
        self.min_bin_freq = min_bin_freq;
        self
    }

    /// Assign all points to nearest cluster (even outliers)
    pub fn cluster_all(mut self, cluster_all: bool) -> Self {
        self.cluster_all = cluster_all;
        self
    }

    pub fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    pub fn build(self) -> MeanShift {
        MeanShift {
            bandwidth: self.bandwidth,
            kernel: self.kernel,
            max_iter: self.max_iter,
            tolerance: self.tolerance,
            bin_seeding: self.bin_seeding,
            min_bin_freq: self.min_bin_freq,
            cluster_all: self.cluster_all,
            parallel: self.parallel,
            cluster_centers: None,
            labels: None,
        }
    }
}

impl Default for MeanShiftBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Mean Shift clustering
pub struct MeanShift {
    bandwidth: Option<f64>,
    kernel: Kernel,
    max_iter: usize,
    tolerance: f64,
    bin_seeding: bool,
    min_bin_freq: usize,
    cluster_all: bool,
    parallel: bool,
    cluster_centers: Option<Array2<f64>>,
    labels: Option<Array1<i32>>,
}

impl MeanShift {
    pub fn new() -> MeanShiftBuilder {
        MeanShiftBuilder::new()
    }

    /// Fit the model and predict cluster labels
    pub fn fit_predict(&mut self, data: &ArrayView2<f64>) -> Result<Array1<i32>> {
        self.fit(data)?;
        Ok(self.labels.clone().unwrap())
    }

    /// Fit the model to the data
    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<()> {
        let (n_samples, n_features) = data.dim();

        if n_samples == 0 {
            return Err(ClusteringError::InvalidParameter(
                "Empty dataset".to_string(),
            ));
        }

        // Estimate bandwidth if not provided
        let bandwidth = self.bandwidth.unwrap_or_else(|| {
            self.estimate_bandwidth(data)
        });

        if bandwidth <= 0.0 {
            return Err(ClusteringError::InvalidParameter(
                "Bandwidth must be positive".to_string(),
            ));
        }

        // Get seeds (starting points for mean shift)
        let seeds = if self.bin_seeding {
            self.get_bin_seeds(data, bandwidth)
        } else {
            // Use all data points as seeds
            data.axis_iter(Axis(0))
                .map(|row| row.to_owned())
                .collect()
        };

        // Perform mean shift for each seed
        let cluster_centers = if self.parallel {
            seeds
                .par_iter()
                .map(|seed| self.mean_shift_single(data, seed.view(), bandwidth))
                .collect::<Result<Vec<_>>>()?
        } else {
            seeds
                .iter()
                .map(|seed| self.mean_shift_single(data, seed.view(), bandwidth))
                .collect::<Result<Vec<_>>>()?
        };

        // Remove duplicates (points that converged to same mode)
        let unique_centers = self.remove_near_duplicates(cluster_centers, bandwidth);

        // Convert to Array2
        let n_clusters = unique_centers.len();
        let mut centers_array = Array2::zeros((n_clusters, n_features));
        for (i, center) in unique_centers.iter().enumerate() {
            centers_array.row_mut(i).assign(&center.view());
        }

        // Assign labels
        let mut labels = Array1::from_elem(n_samples, -1);
        for (i, point) in data.axis_iter(Axis(0)).enumerate() {
            let mut min_dist = f64::INFINITY;
            let mut best_cluster = -1;

            for (j, center) in centers_array.axis_iter(Axis(0)).enumerate() {
                let dist = euclidean_distance(&point.into_owned().view(), &center.into_owned().view());

                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = j as i32;
                }
            }

            if self.cluster_all || min_dist <= bandwidth {
                labels[i] = best_cluster;
            }
        }

        self.cluster_centers = Some(centers_array);
        self.labels = Some(labels);

        Ok(())
    }

    /// Estimate bandwidth using Scott's rule
    fn estimate_bandwidth(&self, data: &ArrayView2<f64>) -> f64 {
        let (n_samples, n_features) = data.dim();

        // Compute standard deviation for each feature
        let mut stds = Vec::new();
        for col in data.axis_iter(Axis(1)) {
            let mean = col.sum() / n_samples as f64;
            let variance = col.mapv(|x| (x - mean).powi(2)).sum() / n_samples as f64;
            stds.push(variance.sqrt());
        }

        // Scott's rule: h = n^(-1/(d+4)) * σ
        let factor = (n_samples as f64).powf(-1.0 / (n_features as f64 + 4.0));
        let avg_std = stds.iter().sum::<f64>() / stds.len() as f64;

        factor * avg_std * 3.0 // Scale up for better results
    }

    /// Get bin seeds for faster initialization
    fn get_bin_seeds(&self, data: &ArrayView2<f64>, bandwidth: f64) -> Vec<Array1<f64>> {
        let (_, n_features) = data.dim();
        let bin_size = bandwidth;

        // Create bins
        let mut bins: HashMap<Vec<i32>, Vec<Array1<f64>>> = HashMap::new();

        for point in data.axis_iter(Axis(0)) {
            let bin_coords: Vec<i32> = point
                .iter()
                .map(|&x| (x / bin_size).floor() as i32)
                .collect();

            bins.entry(bin_coords)
                .or_insert_with(Vec::new)
                .push(point.to_owned());
        }

        // Get seeds from bins with sufficient points
        let mut seeds = Vec::new();
        for (_, points) in bins {
            if points.len() >= self.min_bin_freq {
                // Use mean of points in bin as seed
                let mut seed = Array1::zeros(n_features);
                for point in &points {
                    seed = seed + point;
                }
                seed = seed / points.len() as f64;
                seeds.push(seed);
            }
        }

        if seeds.is_empty() {
            // Fallback to first point if no bins meet threshold
            seeds.push(data.row(0).to_owned());
        }

        seeds
    }

    /// Perform mean shift for a single seed point
    fn mean_shift_single(
        &self,
        data: &ArrayView2<f64>,
        seed: ArrayView1<f64>,
        bandwidth: f64,
    ) -> Result<Array1<f64>> {
        let mut current = seed.to_owned();

        for _ in 0..self.max_iter {
            // Find points within bandwidth
            let mut weighted_sum = Array1::zeros(current.len());
            let mut weight_sum = 0.0;

            for point in data.axis_iter(Axis(0)) {
                let dist = euclidean_distance(&current.view(), &point.into_owned().view());

                if dist <= bandwidth {
                    let weight = self.kernel_weight(dist, bandwidth);
                    weighted_sum = weighted_sum + &(point.to_owned() * weight);
                    weight_sum += weight;
                }
            }

            if weight_sum == 0.0 {
                // No points in bandwidth, return current position
                return Ok(current);
            }

            let new_center = weighted_sum / weight_sum;
            let shift = euclidean_distance(&current.view(), &new_center.view());

            current = new_center;

            if shift < self.tolerance {
                break;
            }
        }

        Ok(current)
    }

    /// Compute kernel weight
    fn kernel_weight(&self, distance: f64, bandwidth: f64) -> f64 {
        let x = distance / bandwidth;

        match self.kernel {
            Kernel::Flat => {
                if x <= 1.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Kernel::Gaussian => {
                (-0.5 * x * x).exp()
            }
            Kernel::Epanechnikov => {
                if x <= 1.0 {
                    1.0 - x * x
                } else {
                    0.0
                }
            }
        }
    }

    /// Remove near duplicate cluster centers
    fn remove_near_duplicates(
        &self,
        centers: Vec<Array1<f64>>,
        bandwidth: f64,
    ) -> Vec<Array1<f64>> {
        let mut unique: Vec<Array1<f64>> = Vec::new();

        for center in centers {
            let mut is_unique = true;

            for existing in &unique {
                let dist = euclidean_distance(&center.view(), &existing.view());
                if dist < bandwidth * 0.5 {
                    is_unique = false;
                    break;
                }
            }

            if is_unique {
                unique.push(center);
            }
        }

        unique
    }

    /// Get cluster centers
    pub fn cluster_centers(&self) -> Option<&Array2<f64>> {
        self.cluster_centers.as_ref()
    }

    /// Get cluster labels
    pub fn labels(&self) -> Option<&Array1<i32>> {
        self.labels.as_ref()
    }
}

impl Default for MeanShift {
    fn default() -> Self {
        Self::new().build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_mean_shift_simple() {
        let data = array![
            [1.0, 1.0],
            [1.5, 2.0],
            [1.3, 1.5],
            [10.0, 10.0],
            [10.5, 11.0],
            [11.0, 10.5],
        ];

        let mut ms = MeanShift::new()
            .bandwidth(2.0)
            .build();

        let labels = ms.fit_predict(&data.view()).unwrap();
        assert_eq!(labels.len(), 6);

        let centers = ms.cluster_centers().unwrap();
        assert!(centers.nrows() >= 1);
    }

    #[test]
    fn test_mean_shift_auto_bandwidth() {
        let data = array![
            [1.0, 1.0],
            [2.0, 2.0],
            [100.0, 100.0],
        ];

        let mut ms = MeanShift::new().build();
        let result = ms.fit(&data.view());
        assert!(result.is_ok());
    }
}
