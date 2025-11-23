//! Gaussian Mixture Models

use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use rand::Rng;

/// Covariance type for GMM
#[derive(Debug, Clone, Copy)]
pub enum CovarianceType {
    /// Full covariance matrix for each component
    Full,
    /// Tied covariance (same for all components)
    Tied,
    /// Diagonal covariance
    Diag,
    /// Spherical covariance (single variance per component)
    Spherical,
}

/// Model selection criterion
#[derive(Debug, Clone, Copy)]
pub enum Criterion {
    /// Bayesian Information Criterion
    BIC,
    /// Akaike Information Criterion
    AIC,
}

/// Gaussian Mixture Model builder
pub struct GaussianMixtureBuilder {
    n_components: usize,
    covariance_type: CovarianceType,
    em_tolerance: f64,
    max_iter: usize,
    n_init: usize,
}

impl GaussianMixtureBuilder {
    pub fn new(n_components: usize) -> Self {
        Self {
            n_components,
            covariance_type: CovarianceType::Full,
            em_tolerance: 1e-3,
            max_iter: 100,
            n_init: 5,
        }
    }

    pub fn covariance_type(mut self, cov_type: CovarianceType) -> Self {
        self.covariance_type = cov_type;
        self
    }

    pub fn em_tolerance(mut self, tol: f64) -> Self {
        self.em_tolerance = tol;
        self
    }

    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    pub fn n_init(mut self, n_init: usize) -> Self {
        self.n_init = n_init;
        self
    }

    pub fn build(self) -> GaussianMixture {
        GaussianMixture {
            n_components: self.n_components,
            covariance_type: self.covariance_type,
            em_tolerance: self.em_tolerance,
            max_iter: self.max_iter,
            n_init: self.n_init,
        }
    }
}

/// Gaussian Mixture Model
pub struct GaussianMixture {
    n_components: usize,
    covariance_type: CovarianceType,
    em_tolerance: f64,
    max_iter: usize,
    n_init: usize,
}

impl GaussianMixture {
    pub fn builder(n_components: usize) -> GaussianMixtureBuilder {
        GaussianMixtureBuilder::new(n_components)
    }

    pub fn new(n_components: usize) -> Self {
        Self::builder(n_components).build()
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<GMMResult> {
        let (n_samples, n_features) = data.dim();

        if n_samples < self.n_components {
            return Err(ClusteringError::InvalidParameter(format!(
                "n_components ({}) cannot exceed n_samples ({})",
                self.n_components, n_samples
            )));
        }

        let mut best_result: Option<GMMResult> = None;
        let mut best_log_likelihood = f64::NEG_INFINITY;

        // Multiple random initializations
        for init_idx in 0..self.n_init {
            // Initialize parameters using KMeans++
            let mut means = self.initialize_means(data)?;
            let mut covariances = self.initialize_covariances(data, &means)?;
            let mut weights = Array1::from_elem(self.n_components, 1.0 / self.n_components as f64);

            let mut converged = false;
            let mut prev_log_likelihood = f64::NEG_INFINITY;

            for iter in 0..self.max_iter {
                // E-step: Compute responsibilities
                let responsibilities = self.e_step(data, &means, &covariances, &weights)?;

                // M-step: Update parameters
                let (new_means, new_covariances, new_weights) =
                    self.m_step(data, &responsibilities)?;

                means = new_means;
                covariances = new_covariances;
                weights = new_weights;

                // Check convergence
                let log_likelihood = self.compute_log_likelihood(data, &means, &covariances, &weights)?;

                if (log_likelihood - prev_log_likelihood).abs() < self.em_tolerance {
                    converged = true;

                    if log_likelihood > best_log_likelihood {
                        best_log_likelihood = log_likelihood;
                        best_result = Some(GMMResult {
                            means: means.clone(),
                            covariances: covariances.clone(),
                            weights: weights.clone(),
                            converged,
                            n_iter: iter + 1,
                        });
                    }
                    break;
                }

                prev_log_likelihood = log_likelihood;
            }

            if !converged && best_result.is_none() {
                best_result = Some(GMMResult {
                    means,
                    covariances,
                    weights,
                    converged: false,
                    n_iter: self.max_iter,
                });
            }
        }

        best_result.ok_or_else(|| {
            ClusteringError::ConvergenceFailure(self.max_iter)
        })
    }

    fn initialize_means(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        let (n_samples, n_features) = data.dim();
        let mut means = Array2::zeros((self.n_components, n_features));

        // Use KMeans++ initialization
        let mut rng = rand::thread_rng();
        let first_idx = rng.gen_range(0..n_samples);
        means.row_mut(0).assign(&data.row(first_idx));

        for k in 1..self.n_components {
            let mut distances = Array1::zeros(n_samples);
            for i in 0..n_samples {
                let mut min_dist = f64::INFINITY;
                for j in 0..k {
                    let diff = data.row(i).to_owned() - &means.row(j);
                    let dist = diff.mapv(|x| x.powi(2)).sum();
                    min_dist = min_dist.min(dist);
                }
                distances[i] = min_dist;
            }

            let total: f64 = distances.sum();
            if total == 0.0 {
                break;
            }

            let mut cumsum = 0.0;
            let threshold: f64 = rng.gen::<f64>() * total;
            let mut selected_idx = 0;

            for i in 0..n_samples {
                cumsum += distances[i];
                if cumsum >= threshold {
                    selected_idx = i;
                    break;
                }
            }

            means.row_mut(k).assign(&data.row(selected_idx));
        }

        Ok(means)
    }

    fn initialize_covariances(&self, data: &ArrayView2<f64>, means: &Array2<f64>) -> Result<Vec<Array2<f64>>> {
        let (_, n_features) = data.dim();
        let mut covariances = Vec::with_capacity(self.n_components);

        for _ in 0..self.n_components {
            covariances.push(Array2::eye(n_features));
        }

        Ok(covariances)
    }

    fn e_step(&self, data: &ArrayView2<f64>, means: &Array2<f64>,
              covariances: &[Array2<f64>], weights: &Array1<f64>) -> Result<Array2<f64>> {
        let n_samples = data.nrows();
        let mut responsibilities = Array2::zeros((n_samples, self.n_components));

        for i in 0..n_samples {
            let sample = data.row(i);
            let mut weighted_probs = Array1::zeros(self.n_components);

            for k in 0..self.n_components {
                let prob = self.gaussian_pdf(&sample, &means.row(k), &covariances[k])?;
                weighted_probs[k] = weights[k] * prob;
            }

            let total: f64 = weighted_probs.sum();
            if total > 0.0 {
                responsibilities.row_mut(i).assign(&(weighted_probs / total));
            } else {
                responsibilities.row_mut(i).fill(1.0 / self.n_components as f64);
            }
        }

        Ok(responsibilities)
    }

    fn m_step(&self, data: &ArrayView2<f64>, responsibilities: &Array2<f64>)
              -> Result<(Array2<f64>, Vec<Array2<f64>>, Array1<f64>)> {
        let (n_samples, n_features) = data.dim();
        let n_k = responsibilities.sum_axis(ndarray::Axis(0));

        // Update means
        let mut means = Array2::zeros((self.n_components, n_features));
        for k in 0..self.n_components {
            if n_k[k] > 0.0 {
                for i in 0..n_samples {
                    means.row_mut(k).scaled_add(responsibilities[[i, k]], &data.row(i));
                }
                means.row_mut(k).mapv_inplace(|x| x / n_k[k]);
            }
        }

        // Update covariances
        let mut covariances = Vec::with_capacity(self.n_components);
        for k in 0..self.n_components {
            let mut cov = Array2::eye(n_features) * 1e-6; // Regularization

            if n_k[k] > 0.0 {
                for i in 0..n_samples {
                    let diff = data.row(i).to_owned() - &means.row(k);
                    let outer = diff.clone().insert_axis(ndarray::Axis(1))
                        .dot(&diff.clone().insert_axis(ndarray::Axis(0)));
                    cov = cov + outer * responsibilities[[i, k]];
                }
                cov.mapv_inplace(|x| x / n_k[k]);
            }

            covariances.push(cov);
        }

        // Update weights
        let weights = n_k / n_samples as f64;

        Ok((means, covariances, weights))
    }

    fn gaussian_pdf(&self, x: &ArrayView1<f64>, mean: &ArrayView1<f64>,
                    covariance: &Array2<f64>) -> Result<f64> {
        let n = x.len();
        let diff = x.to_owned() - mean;

        // Compute determinant (for diagonal/spherical case, simplified)
        let det = covariance.diag().iter().product::<f64>();
        if det <= 0.0 {
            return Ok(1e-300);
        }

        // Compute inverse (simplified for diagonal case)
        let inv_cov_diag = covariance.diag().mapv(|x| 1.0 / x.max(1e-10));
        let mahalanobis = diff.iter()
            .zip(inv_cov_diag.iter())
            .map(|(d, inv_c)| d * d * inv_c)
            .sum::<f64>();

        let norm_const = 1.0 / ((2.0 * std::f64::consts::PI).powi(n as i32 / 2) * det.sqrt());
        let mahalanobis_f64: f64 = mahalanobis;
        let prob = norm_const * (-0.5 * mahalanobis_f64).exp();

        Ok(prob.max(1e-300))
    }

    fn compute_log_likelihood(&self, data: &ArrayView2<f64>, means: &Array2<f64>,
                              covariances: &[Array2<f64>], weights: &Array1<f64>) -> Result<f64> {
        let n_samples = data.nrows();
        let mut log_likelihood = 0.0;

        for i in 0..n_samples {
            let sample = data.row(i);
            let mut prob_sum = 0.0;

            for k in 0..self.n_components {
                let prob = self.gaussian_pdf(&sample, &means.row(k), &covariances[k])?;
                prob_sum += weights[k] * prob;
            }

            log_likelihood += prob_sum.max(1e-300).ln();
        }

        Ok(log_likelihood)
    }

    pub fn predict_proba(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        // This would need to store fitted parameters
        Err(ClusteringError::InvalidParameter(
            "GMM not fitted yet. Call fit() first.".to_string()
        ))
    }

    pub fn score_samples(&self, data: &ArrayView2<f64>) -> Result<Array1<f64>> {
        // This would need to store fitted parameters
        Err(ClusteringError::InvalidParameter(
            "GMM not fitted yet. Call fit() first.".to_string()
        ))
    }
}

/// Result of GMM fitting
pub struct GMMResult {
    pub means: Array2<f64>,
    pub covariances: Vec<Array2<f64>>,
    pub weights: Array1<f64>,
    pub converged: bool,
    pub n_iter: usize,
}
