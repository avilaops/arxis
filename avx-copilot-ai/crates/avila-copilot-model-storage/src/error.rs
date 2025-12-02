// Error types for model storage

use std::fmt;

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug)]
pub enum StorageError {
    ModelNotFound(String),
    IoError(std::io::Error),
    CompressionError(String),
    CacheError(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModelNotFound(name) => write!(f, "Model not found: {}", name),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            Self::CacheError(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<avila_compress::Error> for StorageError {
    fn from(e: avila_compress::Error) -> Self {
        Self::CompressionError(e.to_string())
    }
}
