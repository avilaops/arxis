//! AvilaDB - Globally distributed NoSQL database optimized for Brazil and LATAM
//!
//! # Features
//!
//! - **4 MB documents** (2x larger than DynamoDB)
//! - **Native vector search** (HNSW index)
//! - **Multi-region writes** (FREE)
//! - **5-10ms latency** in Brazil
//! - **Automatic compression** via `avila-compress`
//!
//! # Quick Start
//!
//! ```no_run
//! use aviladb::{AvilaClient, Document};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AvilaClient::connect("http://localhost:8000").await?;
//!     let db = client.database("gamedb").await?;
//!     let players = db.collection("players").await?;
//!
//!     // Insert document
//!     let player = Document::new()
//!         .set("userId", "player123")
//!         .set("username", "CoolGamer")
//!         .set("level", 42);
//!
//!     players.insert(player).await?;
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod client;
pub mod collection;
pub mod config;
pub mod database;
pub mod document;
pub mod error;
pub mod query;
pub mod storage;
pub mod vector;

pub use client::AvilaClient;
pub use collection::Collection;
pub use config::Config as AvilaConfig;
pub use database::Database;
pub use document::Document;
pub use error::{AvilaError, Result as AvilaResult};
pub use query::Query;

/// Maximum document size in bytes (4 MB)
pub const MAX_DOCUMENT_SIZE: usize = 4 * 1024 * 1024;

/// Maximum partition size in bytes (50 GB)
pub const MAX_PARTITION_SIZE: u64 = 50 * 1024 * 1024 * 1024;

/// AvilaDB error types
#[derive(Error, Debug)]
pub enum Error {
    #[error("Document too large: {size} bytes (max: {MAX_DOCUMENT_SIZE} bytes)")]
    DocumentTooLarge { size: usize },

    #[error("Partition full: {size} GB (max: 50 GB)")]
    PartitionFull { size: u64 },

    #[error("Bucket not found: {bucket}")]
    BucketNotFound { bucket: String },

    #[error("Collection not found: {collection}")]
    CollectionNotFound { collection: String },

    #[error("Document not found: {id}")]
    DocumentNotFound { id: String },

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Configuration for AvilaDB client
#[derive(Debug, Clone)]
pub struct Config {
    /// Endpoint URL (e.g., "http://localhost:8000")
    pub endpoint: String,

    /// Access key for authentication
    pub access_key: Option<String>,

    /// Secret key for authentication
    pub secret_key: Option<String>,

    /// Default database name
    pub default_database: Option<String>,

    /// Enable compression (default: true)
    pub compression: bool,

    /// Request timeout in seconds (default: 30)
    pub timeout_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8000".to_string(),
            access_key: None,
            secret_key: None,
            default_database: None,
            compression: true,
            timeout_secs: 30,
        }
    }
}

/// Insert result with document ID and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertResult {
    /// Unique document ID
    pub id: String,

    /// Document size in bytes
    pub size_bytes: usize,

    /// Compression ratio (original / compressed)
    pub compression_ratio: f64,

    /// Operation latency in milliseconds
    pub latency_ms: u128,
}

/// Query result with documents and metadata
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Retrieved documents
    pub documents: Vec<Document>,

    /// Total count (may be greater than documents.len() if paginated)
    pub total_count: usize,

    /// Continuation token for pagination
    pub continuation_token: Option<String>,

    /// Query latency in milliseconds
    pub latency_ms: u128,
}

/// Storage class for documents
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageClass {
    /// Standard storage with LZ4 compression (hot data)
    Standard,

    /// Infrequent access (warm data)
    InfrequentAccess,

    /// Archive storage with Zstd compression (cold data)
    Archive,
}

impl Default for StorageClass {
    fn default() -> Self {
        Self::Standard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.endpoint, "http://localhost:8000");
        assert!(config.compression);
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    fn test_max_constants() {
        assert_eq!(MAX_DOCUMENT_SIZE, 4 * 1024 * 1024); // 4 MB
        assert_eq!(MAX_PARTITION_SIZE, 50 * 1024 * 1024 * 1024); // 50 GB
    }
}
