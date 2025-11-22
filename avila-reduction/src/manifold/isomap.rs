//! Isomap (Isometric Mapping)

use crate::{ReductionError, Result};
use ndarray::{Array2, ArrayView2};

/// Isomap
pub struct Isomap {
    n_components: usize,
    n_neighbors: usize,
}

impl Isomap {
    pub fn new(n_components: usize, n_neighbors: usize) -> Self {
        Self {
            n_components,
            n_neighbors,
        }
    }

    pub fn fit_transform(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        // TODO: Implement Isomap
        // 1. Construct k-NN graph
        // 2. Compute shortest paths (geodesic distances)
        // 3. Apply MDS on geodesic distance matrix
        unimplemented!("Isomap::fit_transform")
    }
}
