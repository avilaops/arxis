// Error types for tokenizer

use std::fmt;

pub type Result<T> = std::result::Result<T, TokenizerError>;

#[derive(Debug)]
pub enum TokenizerError {
    EncodingError(String),
    DecodingError(String),
    VocabularyError(String),
    InvalidToken(u32),
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            Self::DecodingError(msg) => write!(f, "Decoding error: {}", msg),
            Self::VocabularyError(msg) => write!(f, "Vocabulary error: {}", msg),
            Self::InvalidToken(token) => write!(f, "Invalid token: {}", token),
        }
    }
}

impl std::error::Error for TokenizerError {}
