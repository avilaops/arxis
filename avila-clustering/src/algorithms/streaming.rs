//! Streaming and incremental clustering

use crate::{ClusteringError, Result};
use ndarray::{Array2, ArrayView2};

/// Streaming KMeans for online learning
pub struct StreamingKMeans {
    n_clusters: usize,
    decay_factor: f64,
    centroids: Option<Array2<f64>>,
    cluster_counts: Vec<f64>,
}

impl StreamingKMeans {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            decay_factor: 0.95,
            centroids: None,
            cluster_counts: vec![0.0; n_clusters],
        }
    }

    pub fn decay_factor(mut self, decay: f64) -> Self {
        self.decay_factor = decay;
        self
    }

    pub fn partial_fit(&mut self, batch: &ArrayView2<f64>) -> Result<()> {
        // TODO: Implement incremental update
        // 1. If first batch, initialize centroids
        // 2. Assign points to nearest centroids
        // 3. Update centroids with decay
        unimplemented!("StreamingKMeans::partial_fit")
    }

    pub fn centroids(&self) -> Option<&Array2<f64>> {
        self.centroids.as_ref()
    }
}
