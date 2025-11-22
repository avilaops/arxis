//! Time series dimensionality reduction

use crate::{ReductionError, Result};
use ndarray::{Array2, ArrayView2};

/// Alignment method for time series
#[derive(Debug, Clone, Copy)]
pub enum AlignmentMethod {
    /// Dynamic Time Warping
    DTW,
    /// Cross-correlation
    CrossCorrelation,
}

/// Time series reduction
pub struct TimeSeriesReduction {
    n_components: usize,
    alignment_method: AlignmentMethod,
    preserve_periodicity: bool,
}

impl TimeSeriesReduction {
    pub fn new(n_components: usize) -> Self {
        Self {
            n_components,
            alignment_method: AlignmentMethod::DTW,
            preserve_periodicity: false,
        }
    }

    pub fn alignment_method(mut self, method: AlignmentMethod) -> Self {
        self.alignment_method = method;
        self
    }

    pub fn preserve_periodicity(mut self, preserve: bool) -> Self {
        self.preserve_periodicity = preserve;
        self
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<()> {
        // TODO: Implement time series reduction
        // 1. Align time series using DTW or correlation
        // 2. Apply PCA or other reduction in aligned space
        // 3. Preserve periodic features if requested
        unimplemented!("TimeSeriesReduction::fit")
    }

    pub fn transform(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>> {
        // TODO: Reduce time series dimensionality
        unimplemented!("TimeSeriesReduction::transform")
    }
}
