// Error types for inference engine

use std::fmt;

pub type Result<T> = std::result::Result<T, InferenceError>;

#[derive(Debug)]
pub enum InferenceError {
    ModelLoadError(String),
    InvalidInput(String),
    InferenceError(String),
    CacheError(String),
    LatencyExceeded { actual_ms: u64, max_ms: u64 },
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModelLoadError(msg) => write!(f, "Model load error: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::InferenceError(msg) => write!(f, "Inference error: {}", msg),
            Self::CacheError(msg) => write!(f, "Cache error: {}", msg),
            Self::LatencyExceeded { actual_ms, max_ms } => {
                write!(f, "Latency exceeded: {}ms > {}ms", actual_ms, max_ms)
            }
        }
    }
}

impl std::error::Error for InferenceError {}
