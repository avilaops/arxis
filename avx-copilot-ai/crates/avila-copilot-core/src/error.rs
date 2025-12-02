// Error types for Avila Copilot

use std::fmt;

pub type Result<T> = std::result::Result<T, CopilotError>;

#[derive(Debug)]
pub enum CopilotError {
    /// Model loading error
    ModelLoadError(String),
    /// Inference error
    InferenceError(String),
    /// Context error
    ContextError(String),
    /// Tokenization error
    TokenizationError(String),
    /// IO error
    IoError(std::io::Error),
    /// Configuration error
    ConfigError(String),
    /// Latency exceeded
    LatencyExceeded { actual_ms: u64, max_ms: u64 },
}

impl fmt::Display for CopilotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModelLoadError(msg) => write!(f, "Model load error: {}", msg),
            Self::InferenceError(msg) => write!(f, "Inference error: {}", msg),
            Self::ContextError(msg) => write!(f, "Context error: {}", msg),
            Self::TokenizationError(msg) => write!(f, "Tokenization error: {}", msg),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Self::LatencyExceeded { actual_ms, max_ms } => {
                write!(f, "Latency exceeded: {}ms > {}ms", actual_ms, max_ms)
            }
        }
    }
}

impl std::error::Error for CopilotError {}

impl From<std::io::Error> for CopilotError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<avila_copilot_tokenizer::TokenizerError> for CopilotError {
    fn from(e: avila_copilot_tokenizer::TokenizerError) -> Self {
        Self::TokenizationError(e.to_string())
    }
}

impl From<avila_copilot_inference::InferenceError> for CopilotError {
    fn from(e: avila_copilot_inference::InferenceError) -> Self {
        Self::InferenceError(e.to_string())
    }
}

impl From<avila_copilot_intelligence::IntelligenceError> for CopilotError {
    fn from(e: avila_copilot_intelligence::IntelligenceError) -> Self {
        Self::InferenceError(e.to_string())
    }
}
