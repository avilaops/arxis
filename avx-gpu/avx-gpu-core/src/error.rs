//! Error types for AVX-GPU

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No compatible GPU device found")]
    NoDeviceFound,

    #[error("Backend initialization failed: {0}")]
    BackendInitFailed(String),

    #[error("Buffer allocation failed: {size} bytes")]
    BufferAllocationFailed { size: usize },

    #[error("Buffer size mismatch: expected {expected}, got {actual}")]
    BufferSizeMismatch { expected: usize, actual: usize },

    #[error("Kernel compilation failed: {0}")]
    KernelCompilationFailed(String),

    #[error("Kernel execution failed: {0}")]
    KernelExecutionFailed(String),

    #[error("Invalid kernel argument at index {index}: {reason}")]
    InvalidKernelArgument { index: usize, reason: String },

    #[error("Memory copy failed: {0}")]
    MemoryCopyFailed(String),

    #[error("Device synchronization failed: {0}")]
    SyncFailed(String),

    #[error("Feature not supported by backend: {0}")]
    UnsupportedFeature(String),

    #[error("Invalid device type: {0}")]
    InvalidDeviceType(String),

    #[error("Backend-specific error: {0}")]
    BackendError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn backend(msg: impl Into<String>) -> Self {
        Self::BackendError(msg.into())
    }

    pub fn unsupported(feature: impl Into<String>) -> Self {
        Self::UnsupportedFeature(feature.into())
    }
}
