//! Convenient re-exports for common reduction operations

pub use crate::{ReductionError, Result};

// Linear methods
pub use crate::linear::ica::*;
pub use crate::linear::lda::*;
pub use crate::linear::nmf::*;
pub use crate::linear::pca::*;

// Manifold learning
pub use crate::manifold::isomap::*;
pub use crate::manifold::lle::*;
pub use crate::manifold::tsne::*;
pub use crate::manifold::umap::*;

// Scientific
pub use crate::scientific::multimodal::*;
pub use crate::scientific::tensor4d::*;
pub use crate::scientific::timeseries::*;

// Streaming
pub use crate::streaming::incremental::*;

// Common types
pub use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
