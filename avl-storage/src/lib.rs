//! AVL Storage - S3-compatible object storage optimized for Brazil
//!
//! # Features
//!
//! - **S3-compatible API** (drop-in replacement)
//! - **Automatic compression** via `avila-compress`
//! - **3-8ms latency** in Brazil
//! - **50% cheaper** than AWS S3
//! - **Multipart uploads** for large files
//!
//! # Quick Start
//!
//! ```no_run
//! use avl_storage::{StorageClient, PutObjectRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = StorageClient::connect("https://storage.avila.cloud").await?;
//!
//!     // Upload file
//!     client.put_object(PutObjectRequest {
//!         bucket: "my-bucket".to_string(),
//!         key: "file.txt".to_string(),
//!         body: b"Hello AVL Storage!".to_vec(),
//!         content_type: Some("text/plain".to_string()),
//!         ..Default::default()
//!     }).await?;
//!
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod client;
pub mod multipart;
pub mod object;

pub use client::StorageClient;

/// AVL Storage error types
#[derive(Error, Debug)]
pub enum Error {
    #[error("Bucket not found: {bucket}")]
    BucketNotFound { bucket: String },

    #[error("Object not found: {bucket}/{key}")]
    ObjectNotFound { bucket: String, key: String },

    #[error("Invalid bucket name: {name}")]
    InvalidBucketName { name: String },

    #[error("Invalid object key: {key}")]
    InvalidKey { key: String },

    #[error("Storage full: {used} / {capacity} bytes")]
    StorageFull { used: u64, capacity: u64 },

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Storage class for objects
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageClass {
    /// Standard storage (hot data, LZ4 compression)
    Standard,

    /// Infrequent access (warm data)
    InfrequentAccess,

    /// Archive storage (cold data, Zstd compression)
    Archive,
}

impl Default for StorageClass {
    fn default() -> Self {
        Self::Standard
    }
}

/// Request to put an object
#[derive(Debug, Clone)]
pub struct PutObjectRequest {
    pub bucket: String,
    pub key: String,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
    pub metadata: HashMap<String, String>,
    pub storage_class: Option<StorageClass>,
}

impl Default for PutObjectRequest {
    fn default() -> Self {
        Self {
            bucket: String::new(),
            key: String::new(),
            body: Vec::new(),
            content_type: None,
            metadata: HashMap::new(),
            storage_class: None,
        }
    }
}

/// Response from put object
#[derive(Debug, Clone)]
pub struct PutObjectResponse {
    pub etag: String,
    pub version_id: Option<String>,
}

/// Response from get object
#[derive(Debug, Clone)]
pub struct GetObjectResponse {
    pub body: Vec<u8>,
    pub content_type: String,
    pub content_length: usize,
    pub etag: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

/// Object metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    pub key: String,
    pub size: usize,
    pub etag: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub storage_class: StorageClass,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_class_default() {
        let class = StorageClass::default();
        assert!(matches!(class, StorageClass::Standard));
    }
}
