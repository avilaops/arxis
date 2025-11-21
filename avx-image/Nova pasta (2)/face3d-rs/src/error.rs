use thiserror::Error;

#[derive(Error, Debug)]
pub enum Face3dError {
    #[error("Invalid shape parameters: expected {expected}, got {got}")]
    InvalidShapeParams { expected: usize, got: usize },

    #[error("Invalid expression parameters: expected {expected}, got {got}")]
    InvalidExpressionParams { expected: usize, got: usize },

    #[error("Invalid pose parameters: expected {expected}, got {got}")]
    InvalidPoseParams { expected: usize, got: usize },

    #[error("Invalid vertex index: {0}")]
    InvalidVertexIndex(usize),

    #[error("Model not initialized")]
    ModelNotInitialized,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[cfg(feature = "scientific-io")]
    #[error("HDF5 error: {0}")]
    Hdf5Error(String),

    #[error("Dimension mismatch: {0}")]
    DimensionMismatch(String),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Face3dError>;
