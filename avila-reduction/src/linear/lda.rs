//! Linear Discriminant Analysis (LDA)

use crate::{ReductionError, Result};
use ndarray::{Array1, Array2, ArrayView2};

/// Linear Discriminant Analysis (supervised)
pub struct LDA {
    n_components: Option<usize>,

    components: Option<Array2<f64>>,
    explained_variance_ratio: Option<Array1<f64>>,
}

impl LDA {
    pub fn new(n_components: Option<usize>) -> Self {
        Self {
            n_components,
            components: None,
            explained_variance_ratio: None,
        }
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>, labels: &[usize]) -> Result<()> {
        // TODO: Implement LDA
        // 1. Compute within-class and between-class scatter matrices
        // 2. Solve generalized eigenvalue problem
        // 3. Select discriminant components
        unimplemented!("LDA::fit")
    }

    pub fn transform(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        // TODO: Project data onto discriminant components
        unimplemented!("LDA::transform")
    }

    pub fn fit_transform(
        &mut self,
        data: &ArrayView2<f64>,
        labels: &[usize],
    ) -> Result<Array2<f64>> {
        self.fit(data, labels)?;
        self.transform(data)
    }
}
