// Error types for context manager

use std::fmt;

pub type Result<T> = std::result::Result<T, ContextError>;

#[derive(Debug)]
pub enum ContextError {
    IoError(std::io::Error),
    ParseError(String),
    IndexError(String),
    CacheError(String),
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::IndexError(msg) => write!(f, "Index error: {}", msg),
            Self::CacheError(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for ContextError {}

impl From<std::io::Error> for ContextError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}
