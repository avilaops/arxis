// Error types for LSP server

use std::fmt;

pub type Result<T> = std::result::Result<T, LspError>;

#[derive(Debug)]
pub enum LspError {
    IoError(std::io::Error),
    ParseError(String),
    SerdeError(serde_json::Error),
    EngineError(String),
}

impl fmt::Display for LspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::SerdeError(e) => write!(f, "Serde error: {}", e),
            Self::EngineError(msg) => write!(f, "Engine error: {}", msg),
        }
    }
}

impl std::error::Error for LspError {}

impl From<std::io::Error> for LspError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<serde_json::Error> for LspError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}
