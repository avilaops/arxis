//! KMedoids clustering (PAM, CLARA, CLARANS)

use crate::metrics::distance::{euclidean_distance, Metric};
use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView2};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

/// KMedoids algorithm variant
#[derive(Debug, Clone, Copy)]
pub enum KMedoidsAlgorithm {
    /// PAM (Partitioning Around Medoids) - exact but O(n²)
    PAM,
    /// CLARA (Clustering LARge Applications) - sampling-based
    CLARA {
        sample_size: usize,
        n_samples: usize,
    },
    /// CLARANS (Clustering Large Applications based on RANdomized Search)
    CLARANS {
        num_local: usize,
        max_neighbor: usize,
    },
}

/// KMedoids clustering
pub struct KMedoids {
    n_clusters: usize,
    algorithm: KMedoidsAlgorithm,
    max_iter: usize,
    metric: Metric,
    random_state: Option<u64>,
}

impl KMedoids {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            algorithm: KMedoidsAlgorithm::PAM,
            max_iter: 300,
            metric: Metric::Euclidean,
            random_state: None,
        }
    }

    pub fn algorithm(mut self, algorithm: KMedoidsAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn metric(mut self, metric: Metric) -> Self {
        self.metric = metric;
        self
    }

    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    pub fn random_state(mut self, seed: u64) -> Self {
        self.random_state = Some(seed);
        self
    }

    pub fn fit(&self, data: &ArrayView2<f64>) -> Result<KMedoidsResult> {
        match self.algorithm {
            KMedoidsAlgorithm::PAM => self.fit_pam(data),
            KMedoidsAlgorithm::CLARA {
                sample_size,
                n_samples,
            } => self.fit_clara(data, sample_size, n_samples),
            KMedoidsAlgorithm::CLARANS {
                num_local,
                max_neighbor,
            } => self.fit_clarans(data, num_local, max_neighbor),
        }
    }

    fn fit_pam(&self, data: &ArrayView2<f64>) -> Result<KMedoidsResult> {
        let n_samples = data.dim().0;
        let mut rng = self.get_rng();

        // Initialize medoids randomly
        let mut medoid_indices: Vec<usize> =
            rand::seq::index::sample(&mut rng, n_samples, self.n_clusters).into_vec();

        let mut labels = Array1::zeros(n_samples);
        let mut total_cost = f64::INFINITY;

        // PAM algorithm
        for _iter in 0..self.max_iter {
            // Assignment step
            labels = self.assign_to_medoids(data, &medoid_indices)?;
            let new_cost = self.compute_total_cost(data, &medoid_indices, &labels)?;

            // Check convergence
            if (total_cost - new_cost).abs() < 1e-6 {
                total_cost = new_cost;
                break;
            }
            total_cost = new_cost;

            // Swap step: try to find better medoids
            let mut improved = false;
            for i in 0..self.n_clusters {
                let current_medoid = medoid_indices[i];
                let mut best_medoid = current_medoid;
                let mut best_cost = total_cost;

                // Try swapping with non-medoid points
                for j in 0..n_samples {
                    if medoid_indices.contains(&j) {
                        continue;
                    }

                    medoid_indices[i] = j;
                    let new_labels = self.assign_to_medoids(data, &medoid_indices)?;
                    let swap_cost = self.compute_total_cost(data, &medoid_indices, &new_labels)?;

                    if swap_cost < best_cost {
                        best_cost = swap_cost;
                        best_medoid = j;
                        improved = true;
                    }
                }

                medoid_indices[i] = best_medoid;
            }

            if !improved {
                break;
            }
        }

        // Final assignment
        labels = self.assign_to_medoids(data, &medoid_indices)?;

        // Extract medoid coordinates
        let mut medoids = Array2::zeros((self.n_clusters, data.dim().1));
        for (i, &idx) in medoid_indices.iter().enumerate() {
            medoids.row_mut(i).assign(&data.row(idx));
        }

        Ok(KMedoidsResult {
            labels,
            medoids,
            medoid_indices,
            inertia: total_cost,
        })
    }

    fn fit_clara(
        &self,
        data: &ArrayView2<f64>,
        sample_size: usize,
        n_samples_iter: usize,
    ) -> Result<KMedoidsResult> {
        let n_samples = data.dim().0;
        let mut rng = self.get_rng();
        let mut best_result: Option<KMedoidsResult> = None;
        let mut best_cost = f64::INFINITY;

        // Run PAM on multiple samples
        for _ in 0..n_samples_iter {
            let sample_indices =
                rand::seq::index::sample(&mut rng, n_samples, sample_size.min(n_samples))
                    .into_vec();

            // Create sample dataset
            let mut sample_data = Array2::zeros((sample_indices.len(), data.dim().1));
            for (i, &idx) in sample_indices.iter().enumerate() {
                sample_data.row_mut(i).assign(&data.row(idx));
            }

            // Run PAM on sample
            let sample_result = self.fit_pam(&sample_data.view())?;

            // Evaluate on full dataset
            let medoid_indices: Vec<usize> = sample_result
                .medoid_indices
                .iter()
                .map(|&i| sample_indices[i])
                .collect();

            let labels = self.assign_to_medoids(data, &medoid_indices)?;
            let cost = self.compute_total_cost(data, &medoid_indices, &labels)?;

            if cost < best_cost {
                best_cost = cost;
                let mut medoids = Array2::zeros((self.n_clusters, data.dim().1));
                for (i, &idx) in medoid_indices.iter().enumerate() {
                    medoids.row_mut(i).assign(&data.row(idx));
                }

                best_result = Some(KMedoidsResult {
                    labels,
                    medoids,
                    medoid_indices,
                    inertia: cost,
                });
            }
        }

        best_result.ok_or_else(|| ClusteringError::ConvergenceFailure(0))
    }

    fn fit_clarans(
        &self,
        data: &ArrayView2<f64>,
        num_local: usize,
        max_neighbor: usize,
    ) -> Result<KMedoidsResult> {
        let n_samples = data.dim().0;
        let mut rng = self.get_rng();
        let mut best_result: Option<KMedoidsResult> = None;
        let mut best_cost = f64::INFINITY;

        for _ in 0..num_local {
            // Random initialization
            let mut medoid_indices: Vec<usize> =
                rand::seq::index::sample(&mut rng, n_samples, self.n_clusters).into_vec();

            let mut labels = self.assign_to_medoids(data, &medoid_indices)?;
            let mut cost = self.compute_total_cost(data, &medoid_indices, &labels)?;
            let mut neighbors_checked = 0;

            while neighbors_checked < max_neighbor {
                // Random neighbor: swap one medoid
                let i = rng.gen_range(0..self.n_clusters);
                let j = rng.gen_range(0..n_samples);

                if medoid_indices.contains(&j) {
                    continue;
                }

                let old_medoid = medoid_indices[i];
                medoid_indices[i] = j;

                let new_labels = self.assign_to_medoids(data, &medoid_indices)?;
                let new_cost = self.compute_total_cost(data, &medoid_indices, &new_labels)?;

                if new_cost < cost {
                    cost = new_cost;
                    labels = new_labels;
                    neighbors_checked = 0; // Reset counter on improvement
                } else {
                    medoid_indices[i] = old_medoid; // Revert swap
                    neighbors_checked += 1;
                }
            }

            if cost < best_cost {
                best_cost = cost;
                let mut medoids = Array2::zeros((self.n_clusters, data.dim().1));
                for (i, &idx) in medoid_indices.iter().enumerate() {
                    medoids.row_mut(i).assign(&data.row(idx));
                }

                best_result = Some(KMedoidsResult {
                    labels,
                    medoids,
                    medoid_indices,
                    inertia: cost,
                });
            }
        }

        best_result.ok_or_else(|| ClusteringError::ConvergenceFailure(0))
    }

    fn assign_to_medoids(
        &self,
        data: &ArrayView2<f64>,
        medoid_indices: &[usize],
    ) -> Result<Array1<usize>> {
        let n_samples = data.dim().0;
        let mut labels = Array1::zeros(n_samples);

        for i in 0..n_samples {
            let point = data.row(i);
            let mut min_dist = f64::INFINITY;
            let mut nearest = 0;

            for (k, &medoid_idx) in medoid_indices.iter().enumerate() {
                let medoid = data.row(medoid_idx);
                let dist = match self.metric {
                    Metric::Euclidean => euclidean_distance(&point, &medoid),
                    _ => self.metric.distance(&point, &medoid)?,
                };

                if dist < min_dist {
                    min_dist = dist;
                    nearest = k;
                }
            }

            labels[i] = nearest;
        }

        Ok(labels)
    }

    fn compute_total_cost(
        &self,
        data: &ArrayView2<f64>,
        medoid_indices: &[usize],
        labels: &Array1<usize>,
    ) -> Result<f64> {
        let n_samples = data.dim().0;
        let mut total_cost = 0.0;

        for i in 0..n_samples {
            let point = data.row(i);
            let medoid = data.row(medoid_indices[labels[i]]);
            let dist = match self.metric {
                Metric::Euclidean => euclidean_distance(&point, &medoid),
                _ => self.metric.distance(&point, &medoid)?,
            };
            total_cost += dist;
        }

        Ok(total_cost)
    }

    fn get_rng(&self) -> Xoshiro256PlusPlus {
        if let Some(seed) = self.random_state {
            Xoshiro256PlusPlus::seed_from_u64(seed)
        } else {
            Xoshiro256PlusPlus::from_entropy()
        }
    }
}

/// Result of KMedoids clustering
pub struct KMedoidsResult {
    pub labels: Array1<usize>,
    pub medoids: Array2<f64>,
    pub medoid_indices: Vec<usize>,
    pub inertia: f64,
}

impl KMedoidsResult {
    pub fn labels(&self) -> &Array1<usize> {
        &self.labels
    }

    pub fn medoids(&self) -> &Array2<f64> {
        &self.medoids
    }

    pub fn medoid_indices(&self) -> &[usize] {
        &self.medoid_indices
    }

    pub fn inertia(&self) -> f64 {
        self.inertia
    }
}
