//! Convenient re-exports for common reduction operations

pub use crate::{ReductionError, Result};

// Linear methods
pub use crate::linear::pca::*;
pub use crate::linear::ica::*;
pub use crate::linear::nmf::*;
pub use crate::linear::lda::*;

// Manifold learning
pub use crate::manifold::tsne::*;
pub use crate::manifold::umap::*;
pub use crate::manifold::isomap::*;
pub use crate::manifold::lle::*;

// Scientific
pub use crate::scientific::tensor4d::*;
pub use crate::scientific::timeseries::*;
pub use crate::scientific::multimodal::*;

// Streaming
pub use crate::streaming::incremental::*;

// Common types
pub use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
