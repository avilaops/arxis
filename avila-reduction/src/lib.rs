//! # avila-reduction
//!
//! State-of-the-art dimensionality reduction algorithms for Rust, designed to surpass
//! scikit-learn, UMAP-learn, and OpenTSNE in performance and capabilities.
//!
//! ## Features
//!
//! - **Linear Methods**: PCA, LDA, ICA, NMF, Factor Analysis
//! - **Manifold Learning**: t-SNE, UMAP, Isomap, LLE, Laplacian Eigenmaps
//! - **Neural Methods**: Autoencoders, VAE, Contractive AE
//! - **Scientific**: 4D tensor reduction, physics-aware reduction, streaming

pub mod gpu;
pub mod linear;
pub mod manifold;
pub mod neural;
pub mod prelude;
pub mod scientific;
pub mod streaming;

pub use prelude::*;

/// Common error type for reduction operations
#[derive(Debug, thiserror::Error)]
pub enum ReductionError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailure(usize),

    #[error("Shape mismatch: expected {expected}, got {actual}")]
    ShapeMismatch { expected: String, actual: String },

    #[error("Numerical error: {0}")]
    NumericalError(String),

    #[error("GPU error: {0}")]
    GpuError(String),

    #[error("Feature not enabled: {0}")]
    FeatureNotEnabled(String),
}

pub type Result<T> = std::result::Result<T, ReductionError>;
