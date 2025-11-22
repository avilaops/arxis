//! HDBSCAN (Hierarchical Density-Based Spatial Clustering)

use crate::metrics::distance::Metric;
use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView2};

/// HDBSCAN algorithm variant
#[derive(Debug, Clone, Copy)]
pub enum HDBSCANAlgorithm {
    /// Boruvka's algorithm with KD-tree (O(n log n))
    BoruvkaKdTree,
    /// Prim's algorithm
    Prims,
    /// Generic MST algorithm
    Generic,
}

/// HDBSCAN clustering builder
pub struct HDBSCANBuilder {
    min_cluster_size: usize,
    min_samples: Option<usize>,
    cluster_selection_epsilon: f64,
    metric: Metric,
    algorithm: HDBSCANAlgorithm,
    prediction_data: bool,
}

impl HDBSCANBuilder {
    pub fn new(min_cluster_size: usize) -> Self {
        Self {
            min_cluster_size,
            min_samples: None,
            cluster_selection_epsilon: 0.0,
            metric: Metric::Euclidean,
            algorithm: HDBSCANAlgorithm::BoruvkaKdTree,
            prediction_data: false,
        }
    }

    pub fn min_samples(mut self, min_samples: usize) -> Self {
        self.min_samples = Some(min_samples);
        self
    }

    pub fn cluster_selection_epsilon(mut self, epsilon: f64) -> Self {
        self.cluster_selection_epsilon = epsilon;
        self
    }

    pub fn metric(mut self, metric: Metric) -> Self {
        self.metric = metric;
        self
    }

    pub fn algorithm(mut self, algorithm: HDBSCANAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn prediction_data(mut self, prediction_data: bool) -> Self {
        self.prediction_data = prediction_data;
        self
    }

    pub fn build(self) -> HDBSCAN {
        HDBSCAN {
            min_cluster_size: self.min_cluster_size,
            min_samples: self.min_samples.unwrap_or(self.min_cluster_size),
            cluster_selection_epsilon: self.cluster_selection_epsilon,
            metric: self.metric,
            algorithm: self.algorithm,
            prediction_data: self.prediction_data,
        }
    }
}

/// HDBSCAN clustering
pub struct HDBSCAN {
    min_cluster_size: usize,
    min_samples: usize,
    cluster_selection_epsilon: f64,
    metric: Metric,
    algorithm: HDBSCANAlgorithm,
    prediction_data: bool,
}

impl HDBSCAN {
    pub fn builder(min_cluster_size: usize) -> HDBSCANBuilder {
        HDBSCANBuilder::new(min_cluster_size)
    }

    pub fn new(min_cluster_size: usize, min_samples: usize) -> Self {
        Self::builder(min_cluster_size)
            .min_samples(min_samples)
            .build()
    }

    pub fn fit(&self, data: &ArrayView2<f64>) -> Result<HDBSCANResult> {
        // TODO: Implement HDBSCAN algorithm
        // 1. Compute mutual reachability distance
        // 2. Build minimum spanning tree
        // 3. Construct cluster hierarchy
        // 4. Extract clusters using excess of mass
        // 5. Compute stability scores
        unimplemented!("HDBSCAN::fit")
    }
}

/// Result of HDBSCAN clustering
pub struct HDBSCANResult {
    pub labels: Array1<i32>, // -1 for noise
    probabilities: Option<Array1<f64>>,
    outlier_scores: Option<Array1<f64>>,
}

impl HDBSCANResult {
    pub fn labels(&self) -> &Array1<i32> {
        &self.labels
    }

    pub fn probabilities(&self) -> Result<&Array1<f64>> {
        self.probabilities.as_ref().ok_or_else(|| {
            ClusteringError::InvalidParameter("prediction_data was not enabled".to_string())
        })
    }

    pub fn outlier_scores(&self) -> Result<&Array1<f64>> {
        self.outlier_scores.as_ref().ok_or_else(|| {
            ClusteringError::InvalidParameter("outlier scores not computed".to_string())
        })
    }
}
