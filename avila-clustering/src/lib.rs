//! # avila-clustering
//!
//! State-of-the-art clustering algorithms for Rust, designed to surpass
//! scikit-learn, HDBSCAN, and RAPIDS cuML in performance and capabilities.
//!
//! ## Features
//!
//! - **Partitional Clustering**: KMeans, KMedoids, Fuzzy C-Means, Mean Shift
//! - **Density-Based**: DBSCAN, HDBSCAN, OPTICS, DENCLUE
//! - **Hierarchical**: Agglomerative, Divisive, BIRCH
//! - **Model-Based**: GMM, Bayesian GMM, Dirichlet Process GMM
//! - **Graph-Based**: Spectral Clustering, Louvain, Leiden
//! - **Scientific**: 4D clustering, curved manifolds, streaming, GPU acceleration

pub mod algorithms;
pub mod gpu;
pub mod metrics;
pub mod prelude;
pub mod scientific;

pub use prelude::*;

/// Common error type for clustering operations
#[derive(Debug, thiserror::Error)]
pub enum ClusteringError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailure(usize),

    #[error("Shape mismatch: expected {expected}, got {actual}")]
    ShapeMismatch { expected: String, actual: String },

    #[error("Numerical error: {0}")]
    NumericalError(String),

    #[error("GPU error: {0}")]
    GpuError(String),
}

pub type Result<T> = std::result::Result<T, ClusteringError>;
