// Error types for code intelligence

use std::fmt;

pub type Result<T> = std::result::Result<T, IntelligenceError>;

#[derive(Debug)]
pub enum IntelligenceError {
    ParseError(String),
    AnalysisError(String),
    GenerationError(String),
    RefactoringError(String),
}

impl fmt::Display for IntelligenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::AnalysisError(msg) => write!(f, "Analysis error: {}", msg),
            Self::GenerationError(msg) => write!(f, "Generation error: {}", msg),
            Self::RefactoringError(msg) => write!(f, "Refactoring error: {}", msg),
        }
    }
}

impl std::error::Error for IntelligenceError {}
