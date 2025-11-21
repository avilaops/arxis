//! KMeans clustering implementation

use crate::metrics::distance::{euclidean_distance, Metric};
use crate::{ClusteringError, Result};
use ndarray::{s, Array1, Array2, ArrayView2, Axis};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

/// Initialization method for KMeans
#[derive(Debug, Clone, Copy)]
pub enum InitMethod {
    /// Random initialization
    Random,
    /// KMeans++ initialization (smart seeding)
    KMeansPlusPlus,
    /// Forgy initialization
    Forgy,
}

/// Algorithm variant for KMeans
#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    /// Lloyd's algorithm (standard)
    Lloyd,
    /// Elkan's algorithm (faster with triangular inequality)
    Elkan,
    /// Mini-batch KMeans
    MiniBatch { batch_size: usize },
}

/// KMeans clustering builder
pub struct KMeansBuilder {
    n_clusters: usize,
    init_method: InitMethod,
    algorithm: Algorithm,
    max_iter: usize,
    tolerance: f64,
    n_init: usize,
    random_state: Option<u64>,
    parallel: bool,
    gpu: bool,
}

impl KMeansBuilder {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            init_method: InitMethod::KMeansPlusPlus,
            algorithm: Algorithm::Lloyd,
            max_iter: 300,
            tolerance: 1e-4,
            n_init: 10,
            random_state: None,
            parallel: true,
            gpu: false,
        }
    }

    pub fn init_method(mut self, method: InitMethod) -> Self {
        self.init_method = method;
        self
    }

    pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
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

    pub fn n_init(mut self, n_init: usize) -> Self {
        self.n_init = n_init;
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

    pub fn gpu(mut self, gpu: bool) -> Self {
        self.gpu = gpu;
        self
    }

    pub fn build(self) -> KMeans {
        KMeans {
            n_clusters: self.n_clusters,
            init_method: self.init_method,
            algorithm: self.algorithm,
            max_iter: self.max_iter,
            tolerance: self.tolerance,
            n_init: self.n_init,
            random_state: self.random_state,
            parallel: self.parallel,
            gpu: self.gpu,
            centroids: None,
        }
    }
}

/// KMeans clustering
pub struct KMeans {
    n_clusters: usize,
    init_method: InitMethod,
    algorithm: Algorithm,
    max_iter: usize,
    tolerance: f64,
    n_init: usize,
    random_state: Option<u64>,
    parallel: bool,
    gpu: bool,
    centroids: Option<Array2<f64>>,
}

impl KMeans {
    pub fn builder(n_clusters: usize) -> KMeansBuilder {
        KMeansBuilder::new(n_clusters)
    }

    pub fn new(n_clusters: usize) -> Self {
        Self::builder(n_clusters).build()
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<KMeansResult> {
        let (n_samples, n_features) = data.dim();

        if self.n_clusters > n_samples {
            return Err(ClusteringError::InvalidParameter(format!(
                "n_clusters ({}) cannot be larger than n_samples ({})",
                self.n_clusters, n_samples
            )));
        }

        let mut best_result: Option<KMeansResult> = None;
        let mut best_inertia = f64::INFINITY;

        // Run multiple initializations
        for init_idx in 0..self.n_init {
            let seed = self.random_state.map(|s| s + init_idx as u64);

            // Initialize centroids
            let centroids = self.initialize_centroids(data, seed)?;

            // Run the selected algorithm
            let result = match self.algorithm {
                Algorithm::Lloyd => self.fit_lloyd(data, centroids)?,
                Algorithm::Elkan => self.fit_elkan(data, centroids)?,
                Algorithm::MiniBatch { batch_size } => {
                    self.fit_minibatch(data, centroids, batch_size, seed)?
                }
            };

            // Keep the best result
            if result.inertia < best_inertia {
                best_inertia = result.inertia;
                best_result = Some(result);
            }
        }

        let result = best_result.unwrap();
        self.centroids = Some(result.centroids.clone());
        Ok(result)
    }

    pub fn predict(&self, data: &ArrayView2<f64>) -> Result<Array1<usize>> {
        let centroids = self.centroids.as_ref().ok_or_else(|| {
            ClusteringError::InvalidParameter(
                "KMeans not fitted yet. Call fit() first.".to_string(),
            )
        })?;

        Ok(assign_labels(data, centroids, self.parallel))
    }

    fn initialize_centroids(
        &self,
        data: &ArrayView2<f64>,
        seed: Option<u64>,
    ) -> Result<Array2<f64>> {
        let (n_samples, n_features) = data.dim();
        let mut rng = if let Some(s) = seed {
            Xoshiro256PlusPlus::seed_from_u64(s)
        } else {
            Xoshiro256PlusPlus::from_entropy()
        };

        match self.init_method {
            InitMethod::Random => {
                // Randomly select k samples as centroids
                let mut centroids = Array2::zeros((self.n_clusters, n_features));
                let selected = rand::seq::index::sample(&mut rng, n_samples, self.n_clusters);

                for (i, idx) in selected.into_iter().enumerate() {
                    centroids.row_mut(i).assign(&data.row(idx));
                }
                Ok(centroids)
            }
            InitMethod::KMeansPlusPlus => {
                // KMeans++ initialization
                self.kmeans_plusplus_init(data, &mut rng)
            }
            InitMethod::Forgy => {
                // Same as Random for now
                let mut centroids = Array2::zeros((self.n_clusters, n_features));
                let selected = rand::seq::index::sample(&mut rng, n_samples, self.n_clusters);

                for (i, idx) in selected.into_iter().enumerate() {
                    centroids.row_mut(i).assign(&data.row(idx));
                }
                Ok(centroids)
            }
        }
    }

    fn kmeans_plusplus_init(
        &self,
        data: &ArrayView2<f64>,
        rng: &mut Xoshiro256PlusPlus,
    ) -> Result<Array2<f64>> {
        let (n_samples, n_features) = data.dim();
        let mut centroids = Array2::zeros((self.n_clusters, n_features));

        // Choose first centroid randomly
        let first_idx = rng.gen_range(0..n_samples);
        centroids.row_mut(0).assign(&data.row(first_idx));

        // Choose remaining centroids
        for k in 1..self.n_clusters {
            // Compute distances to nearest centroid
            let mut distances = Array1::zeros(n_samples);
            for i in 0..n_samples {
                let point = data.row(i);
                let mut min_dist = f64::INFINITY;

                for j in 0..k {
                    let centroid = centroids.row(j);
                    let dist = euclidean_distance(&point, &centroid);
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
                distances[i] = min_dist * min_dist;
            }

            // Sample proportionally to squared distance
            let total: f64 = distances.sum();
            let threshold = rng.gen::<f64>() * total;
            let mut cumsum = 0.0;
            let mut selected_idx = 0;

            for (i, &dist) in distances.iter().enumerate() {
                cumsum += dist;
                if cumsum >= threshold {
                    selected_idx = i;
                    break;
                }
            }

            centroids.row_mut(k).assign(&data.row(selected_idx));
        }

        Ok(centroids)
    }

    fn fit_lloyd(
        &self,
        data: &ArrayView2<f64>,
        mut centroids: Array2<f64>,
    ) -> Result<KMeansResult> {
        let (n_samples, _) = data.dim();
        let mut labels = Array1::zeros(n_samples);
        let mut inertia = 0.0;
        let mut n_iter = 0;

        for iter in 0..self.max_iter {
            n_iter = iter + 1;

            // Assignment step
            let new_labels = assign_labels(data, &centroids, self.parallel);

            // Update step
            let (new_centroids, new_inertia) = update_centroids(data, &new_labels, self.n_clusters);

            // Check convergence
            let centroid_shift = compute_centroid_shift(&centroids, &new_centroids);

            centroids = new_centroids;
            labels = new_labels;
            inertia = new_inertia;

            if centroid_shift < self.tolerance {
                break;
            }
        }

        Ok(KMeansResult {
            labels,
            centroids,
            inertia,
            n_iter,
        })
    }

    fn fit_elkan(
        &self,
        data: &ArrayView2<f64>,
        mut centroids: Array2<f64>,
    ) -> Result<KMeansResult> {
        // Elkan's algorithm using triangle inequality for speedup
        // For simplicity, delegate to Lloyd for now
        // TODO: Implement full Elkan optimization
        self.fit_lloyd(data, centroids)
    }

    fn fit_minibatch(
        &self,
        data: &ArrayView2<f64>,
        mut centroids: Array2<f64>,
        batch_size: usize,
        seed: Option<u64>,
    ) -> Result<KMeansResult> {
        let (n_samples, _) = data.dim();
        let mut rng = if let Some(s) = seed {
            Xoshiro256PlusPlus::seed_from_u64(s)
        } else {
            Xoshiro256PlusPlus::from_entropy()
        };

        let mut counts = Array1::<f64>::zeros(self.n_clusters);
        let n_batches = (n_samples + batch_size - 1) / batch_size;

        for iter in 0..(self.max_iter * n_batches / 10) {
            // Sample a mini-batch
            let batch_indices =
                rand::seq::index::sample(&mut rng, n_samples, batch_size.min(n_samples));

            let mut batch_data = Array2::zeros((batch_indices.len(), data.dim().1));
            for (i, idx) in batch_indices.into_iter().enumerate() {
                batch_data.row_mut(i).assign(&data.row(idx));
            }

            // Assign batch to centroids
            let batch_labels = assign_labels(&batch_data.view(), &centroids, false);

            // Update centroids with batch
            for (i, &label) in batch_labels.iter().enumerate() {
                counts[label] += 1.0;
                let eta = 1.0 / counts[label];
                let point = batch_data.row(i);
                let mut centroid = centroids.row_mut(label);

                for j in 0..centroid.len() {
                    centroid[j] = (1.0 - eta) * centroid[j] + eta * point[j];
                }
            }
        }

        // Final assignment
        let labels = assign_labels(data, &centroids, self.parallel);
        let inertia = compute_inertia(data, &labels, &centroids);

        Ok(KMeansResult {
            labels,
            centroids,
            inertia,
            n_iter: self.max_iter,
        })
    }

    pub fn fit_predict(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>> {
        let result = self.fit(data)?;
        Ok(result.labels)
    }
}

/// Result of KMeans clustering
pub struct KMeansResult {
    pub labels: Array1<usize>,
    pub centroids: Array2<f64>,
    pub inertia: f64,
    pub n_iter: usize,
}

impl KMeansResult {
    pub fn labels(&self) -> &Array1<usize> {
        &self.labels
    }

    pub fn centroids(&self) -> &Array2<f64> {
        &self.centroids
    }

    pub fn inertia(&self) -> f64 {
        self.inertia
    }

    pub fn n_iter(&self) -> usize {
        self.n_iter
    }
}

// Helper functions

fn assign_labels(data: &ArrayView2<f64>, centroids: &Array2<f64>, parallel: bool) -> Array1<usize> {
    let n_samples = data.dim().0;

    if parallel {
        use rayon::prelude::*;
        let labels_vec: Vec<usize> = (0..n_samples)
            .into_par_iter()
            .map(|i| {
                let point = data.row(i);
                find_nearest_centroid(&point, centroids)
            })
            .collect();
        Array1::from(labels_vec)
    } else {
        let mut labels = Array1::<usize>::zeros(n_samples);
        for i in 0..n_samples {
            let point = data.row(i);
            labels[i] = find_nearest_centroid(&point, centroids);
        }
        labels
    }
}

fn find_nearest_centroid(point: &ndarray::ArrayView1<f64>, centroids: &Array2<f64>) -> usize {
    let mut min_dist = f64::INFINITY;
    let mut nearest = 0;

    for (k, centroid) in centroids.axis_iter(Axis(0)).enumerate() {
        let dist = euclidean_distance(point, &centroid);
        if dist < min_dist {
            min_dist = dist;
            nearest = k;
        }
    }

    nearest
}

fn update_centroids(
    data: &ArrayView2<f64>,
    labels: &Array1<usize>,
    n_clusters: usize,
) -> (Array2<f64>, f64) {
    let (n_samples, n_features) = data.dim();
    let mut new_centroids = Array2::zeros((n_clusters, n_features));
    let mut counts = Array1::<f64>::zeros(n_clusters);

    // Sum points for each cluster
    for i in 0..n_samples {
        let label = labels[i];
        counts[label] += 1.0;
        let point = data.row(i);
        let mut centroid = new_centroids.row_mut(label);

        for j in 0..n_features {
            centroid[j] += point[j];
        }
    }

    // Compute means
    for k in 0..n_clusters {
        if counts[k] > 0.0 {
            let mut centroid = new_centroids.row_mut(k);
            for j in 0..n_features {
                centroid[j] /= counts[k];
            }
        }
    }

    // Compute inertia
    let inertia = compute_inertia(data, labels, &new_centroids);

    (new_centroids, inertia)
}

fn compute_inertia(data: &ArrayView2<f64>, labels: &Array1<usize>, centroids: &Array2<f64>) -> f64 {
    let n_samples = data.dim().0;
    let mut inertia = 0.0;

    for i in 0..n_samples {
        let point = data.row(i);
        let centroid = centroids.row(labels[i]);
        let dist = euclidean_distance(&point, &centroid);
        inertia += dist * dist;
    }

    inertia
}

fn compute_centroid_shift(old_centroids: &Array2<f64>, new_centroids: &Array2<f64>) -> f64 {
    let mut max_shift = 0.0;

    for (old, new) in old_centroids
        .axis_iter(Axis(0))
        .zip(new_centroids.axis_iter(Axis(0)))
    {
        let shift = euclidean_distance(&old, &new);
        if shift > max_shift {
            max_shift = shift;
        }
    }

    max_shift
}
