pub mod csv;
pub mod json;
pub mod parquet;

use crate::{
    models::BehaviorEvent,
    storage::{EventFilter, StorageError},
};
use async_trait::async_trait;
use std::path::Path;

/// Trait para exportação de dados
#[async_trait]
pub trait DataExporter: Send + Sync {
    async fn export(
        &self,
        events: Vec<BehaviorEvent>,
        output_path: &Path,
    ) -> Result<ExportResult, ExportError>;
}

/// Resultado da exportação
#[derive(Debug)]
pub struct ExportResult {
    pub file_path: String,
    pub record_count: usize,
    pub file_size_bytes: u64,
    pub format: ExportFormat,
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Csv,
    Json,
    JsonLines,
    Parquet,
}

/// Erros de exportação
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

impl From<serde_json::Error> for ExportError {
    fn from(err: serde_json::Error) -> Self {
        ExportError::Serialization(err.to_string())
    }
}

impl From<csv::Error> for ExportError {
    fn from(err: csv::Error) -> Self {
        ExportError::Serialization(err.to_string())
    }
}
