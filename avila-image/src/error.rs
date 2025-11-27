//! Error types for image operations

use std::fmt;

/// Result type for image operations
pub type Result<T> = std::result::Result<T, ImageError>;

/// Error types for image operations
#[derive(Debug)]
pub enum ImageError {
    /// IO error
    Io(std::io::Error),

    /// Unsupported format
    UnsupportedFormat,

    /// Invalid dimensions
    InvalidDimensions,

    /// Decoding error
    DecodingError(String),

    /// Encoding error
    EncodingError(String),

    /// Invalid operation
    InvalidOperation(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::Io(e) => write!(f, "IO error: {}", e),
            ImageError::UnsupportedFormat => write!(f, "Unsupported image format"),
            ImageError::InvalidDimensions => write!(f, "Invalid image dimensions"),
            ImageError::DecodingError(msg) => write!(f, "Decoding error: {}", msg),
            ImageError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            ImageError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for ImageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ImageError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> Self {
        ImageError::Io(err)
    }
}
