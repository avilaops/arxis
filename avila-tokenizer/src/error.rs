use thiserror::Error;

/// Main error type for avila-tokenizers
#[derive(Error, Debug, Clone)]
pub enum TokenizerError {
    #[error("Unknown model: {0}")]
    UnknownModel(String),

    #[error("Invalid vocabulary: {0}")]
    InvalidVocabulary(String),

    #[error("Token not found in vocabulary: {0}")]
    TokenNotFound(String),

    #[error("Invalid merge file: {0}")]
    InvalidMergeFile(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(String),

    #[error("Invalid UTF-8 sequence")]
    Utf8Error,

    #[error("Invalid token ID: {0}")]
    InvalidTokenId(u32),

    #[error("Training error: {0}")]
    TrainingError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Normalization error: {0}")]
    NormalizationError(String),

    #[error("Pre-tokenization error: {0}")]
    PreTokenizationError(String),

    #[error("Post-processing error: {0}")]
    PostProcessingError(String),

    #[error("Decoding error: {0}")]
    DecodingError(String),

    #[error("Empty text input")]
    EmptyInput,

    #[error("Vocabulary size mismatch: expected {expected}, got {actual}")]
    VocabSizeMismatch { expected: usize, actual: usize },

    #[error("Invalid special token: {0}")]
    InvalidSpecialToken(String),

    #[error("Maximum input length exceeded: {0} > {1}")]
    MaxLengthExceeded(usize, usize),
}

impl From<std::io::Error> for TokenizerError {
    fn from(err: std::io::Error) -> Self {
        TokenizerError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for TokenizerError {
    fn from(err: serde_json::Error) -> Self {
        TokenizerError::JsonError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for TokenizerError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        TokenizerError::Utf8Error
    }
}

impl From<std::str::Utf8Error> for TokenizerError {
    fn from(_: std::str::Utf8Error) -> Self {
        TokenizerError::Utf8Error
    }
}

/// Result type alias for avila-tokenizers
pub type Result<T> = std::result::Result<T, TokenizerError>;
