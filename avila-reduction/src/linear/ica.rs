//! Independent Component Analysis (ICA)

use crate::{ReductionError, Result};
use ndarray::{Array2, ArrayView2};

/// ICA algorithm
#[derive(Debug, Clone, Copy)]
pub enum ICAAlgorithm {
    /// FastICA
    FastICA,
    /// Infomax
    Infomax,
}

/// ICA (Independent Component Analysis)
pub struct ICA {
    n_components: usize,
    algorithm: ICAAlgorithm,
    max_iter: usize,
    tolerance: f64,

    components: Option<Array2<f64>>,
    mixing_matrix: Option<Array2<f64>>,
}

impl ICA {
    pub fn new(n_components: usize) -> Self {
        Self {
            n_components,
            algorithm: ICAAlgorithm::FastICA,
            max_iter: 200,
            tolerance: 1e-4,
            components: None,
            mixing_matrix: None,
        }
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<()> {
        // TODO: Implement ICA
        // 1. Center and whiten data
        // 2. Apply FastICA algorithm
        // 3. Estimate independent components
        unimplemented!("ICA::fit")
    }

    pub fn transform(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        // TODO: Transform data to independent components
        unimplemented!("ICA::transform")
    }

    pub fn fit_transform(&mut self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        self.fit(data)?;
        self.transform(data)
    }
}
