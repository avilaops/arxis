//! Error types - Simples e direto

use thiserror::Error;

pub type Result<T> = std::result::Result<T, AvilaError>;

#[derive(Error, Debug)]
pub enum AvilaError {
    #[error("Shape mismatch: {0}")]
    ShapeMismatch(String),

    #[error("Column not found: {0}")]
    ColumnNotFound(String),

    #[error("Index out of bounds: {0} (max: {1})")]
    IndexOutOfBounds(usize, usize),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Arrow error: {0}")]
    ArrowError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("{0}")]
    Generic(String),
}

impl From<arrow::error::ArrowError> for AvilaError {
    fn from(err: arrow::error::ArrowError) -> Self {
        Self::ArrowError(err.to_string())
    }
}

impl AvilaError {
    pub fn shape_mismatch(msg: impl Into<String>) -> Self {
        Self::ShapeMismatch(msg.into())
    }

    pub fn column_not_found(name: impl Into<String>) -> Self {
        Self::ColumnNotFound(name.into())
    }

    pub fn index_out_of_bounds(idx: usize, max: usize) -> Self {
        Self::IndexOutOfBounds(idx, max)
    }

    pub fn type_error(msg: impl Into<String>) -> Self {
        Self::TypeError(msg.into())
    }

    pub fn invalid_operation(msg: impl Into<String>) -> Self {
        Self::InvalidOperation(msg.into())
    }

    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::NotImplemented(msg.into())
    }

    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}
