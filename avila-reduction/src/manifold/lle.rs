//! LLE (Locally Linear Embedding)

use crate::{ReductionError, Result};
use ndarray::{Array2, ArrayView2};

/// LLE (Locally Linear Embedding)
pub struct LLE {
    n_components: usize,
    n_neighbors: usize,
    reg: f64,
}

impl LLE {
    pub fn new(n_components: usize, n_neighbors: usize) -> Self {
        Self {
            n_components,
            n_neighbors,
            reg: 1e-3,
        }
    }

    pub fn fit_transform(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        // TODO: Implement LLE
        // 1. Find k nearest neighbors
        // 2. Compute reconstruction weights
        // 3. Compute embedding from eigenvectors
        unimplemented!("LLE::fit_transform")
    }
}
