# AVL Storage - Copilot Instructions

## Project Identity

**AVL Storage** is the **S3-compatible object storage** for the AVL Cloud Platform. It embodies the Arxis philosophy:
- **ARX (Fortress)**: Durable, replicated, encrypted object storage
- **AXIS (Engine)**: High-speed transfers optimized for Brazil

**Key Differentiators:**
- 🇧🇷 3-8ms latency in Brazil (vs 50-80ms AWS S3)
- 💰 50% cheaper than S3 for Brazilian traffic
- 🗜️ Automatic compression via `avila-compress`
- 🔄 100% S3 API compatible
- 🆓 FREE transfers between AVL services

---

## Architecture Principles

### 1. Fortress Philosophy (ARX)
```rust
// ✅ ALWAYS: 11 nines durability (99.999999999%)
// ✅ ALWAYS: 3-copy replication across zones
// ✅ ALWAYS: Encryption at rest (AES-256) and in transit (TLS 1.3)
// ✅ ALWAYS: Versioning and immutability support
```

### 2. Engine Philosophy (AXIS)
```rust
// ✅ ALWAYS: Compress with avila-compress before storage
// ✅ ALWAYS: Parallel uploads/downloads for large files
// ✅ ALWAYS: Optimize for Brazil latency (target 3-8ms)
// ✅ ALWAYS: Use multipart uploads for files > 100 MB
```

### 3. S3 Compatibility
```rust
// ✅ ALWAYS: Match AWS S3 API behavior exactly
// ✅ ALWAYS: Return S3-compatible error codes
// ✅ ALWAYS: Support standard S3 headers
// ✅ ALWAYS: Implement multipart upload protocol
```

---

## Code Style & Standards

### Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Bucket not found: {bucket}")]
    BucketNotFound { bucket: String },

    #[error("Object not found: {bucket}/{key}")]
    ObjectNotFound { bucket: String, key: String },

    #[error("Invalid object key: {key}")]
    InvalidKey { key: String },

    #[error("Storage full: {used} / {capacity} bytes")]
    StorageFull { used: u64, capacity: u64 },

    #[error("Compression error: {0}")]
    Compression(#[from] avila_compress::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, StorageError>;
```

### S3 API Types
```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PutObjectRequest {
    pub bucket: String,
    pub key: String,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
    pub metadata: HashMap<String, String>,
    pub storage_class: Option<StorageClass>,
}

#[derive(Debug, Clone)]
pub struct GetObjectResponse {
    pub body: Vec<u8>,
    pub content_type: String,
    pub content_length: usize,
    pub etag: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageClass {
    Standard,    // Hot data, LZ4 compression
    InfrequentAccess, // Warm data
    Archive,     // Cold data, Zstd compression
}
```

---

## Performance Guidelines

### 1. Compression (avila-compress)
```rust
use avila_compress::{lz4::Lz4Compressor, zstd::ZstdCompressor};

pub struct CompressionEngine {
    lz4: Lz4Compressor,
    zstd: ZstdCompressor,
}

impl CompressionEngine {
    // ✅ ALWAYS: Choose compression based on storage class
    pub fn compress(&self, data: &[u8], class: StorageClass) -> Result<Vec<u8>> {
        match class {
            StorageClass::Standard => {
                // Fast compression for hot data (LZ4)
                Ok(self.lz4.compress(data)?)
            }
            StorageClass::Archive => {
                // High ratio for cold data (Zstd)
                Ok(self.zstd.compress(data)?)
            }
            _ => Ok(data.to_vec()), // No compression
        }
    }

    pub fn decompress(&self, data: &[u8], class: StorageClass) -> Result<Vec<u8>> {
        match class {
            StorageClass::Standard => Ok(self.lz4.decompress(data)?),
            StorageClass::Archive => Ok(self.zstd.decompress(data)?),
            _ => Ok(data.to_vec()),
        }
    }
}
```

### 2. Multipart Uploads
```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct MultipartUpload {
    pub upload_id: String,
    pub bucket: String,
    pub key: String,
    pub parts: Vec<PartInfo>,
}

impl StorageClient {
    // ✅ ALWAYS: Use 5 MB chunks minimum (S3 requirement)
    const MIN_PART_SIZE: usize = 5 * 1024 * 1024;

    pub async fn upload_large_file(
        &self,
        bucket: &str,
        key: &str,
        file_path: &str,
    ) -> Result<()> {
        let mut file = File::open(file_path).await?;
        let file_size = file.metadata().await?.len();

        // Create multipart upload
        let upload = self.create_multipart_upload(bucket, key).await?;

        // Upload parts in parallel
        let mut part_number = 1;
        let mut futures = Vec::new();

        loop {
            let mut buffer = vec![0u8; Self::MIN_PART_SIZE];
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            buffer.truncate(n);

            // Spawn parallel upload
            let future = self.upload_part(
                bucket,
                key,
                &upload.upload_id,
                part_number,
                buffer,
            );
            futures.push(future);
            part_number += 1;
        }

        // Wait for all parts
        let parts: Vec<_> = futures::future::join_all(futures)
            .await
            .into_iter()
            .collect::<Result<_>>()?;

        // Complete upload
        self.complete_multipart_upload(bucket, key, &upload.upload_id, parts).await?;

        Ok(())
    }
}
```

### 3. ETag Calculation
```rust
use sha2::{Sha256, Digest};
use md5::{Md5, digest::DynDigest};

// ✅ ALWAYS: Use MD5 for S3 compatibility
pub fn calculate_etag(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

// For multipart uploads
pub fn calculate_multipart_etag(part_etags: &[String]) -> String {
    let mut hasher = Md5::new();
    for etag in part_etags {
        hasher.update(hex::decode(etag).unwrap());
    }
    let result = hasher.finalize();
    format!("{}-{}", hex::encode(result), part_etags.len())
}
```

---

## S3 API Implementation

### PUT Object
```rust
use axum::{Router, routing::put, extract::{Path, State}, body::Bytes};

pub async fn put_object(
    State(storage): State<StorageEngine>,
    Path((bucket, key)): Path<(String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse> {
    // Validate bucket exists
    if !storage.bucket_exists(&bucket).await? {
        return Err(StorageError::BucketNotFound { bucket });
    }

    // Extract metadata from headers
    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let metadata = extract_metadata(&headers);

    // Compress and store
    let compressed = storage.compress(&body, StorageClass::Standard)?;
    storage.put(&bucket, &key, compressed).await?;

    // Calculate ETag
    let etag = calculate_etag(&body);

    // Return S3-compatible response
    Ok((
        StatusCode::OK,
        [
            ("ETag", format!("\"{}\"", etag)),
            ("x-amz-server-side-encryption", "AES256".to_string()),
        ],
    ))
}
```

### GET Object
```rust
pub async fn get_object(
    State(storage): State<StorageEngine>,
    Path((bucket, key)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    // Retrieve compressed data
    let compressed = storage.get(&bucket, &key).await?
        .ok_or_else(|| StorageError::ObjectNotFound {
            bucket: bucket.clone(),
            key: key.clone()
        })?;

    // Decompress
    let data = storage.decompress(&compressed, StorageClass::Standard)?;

    // Get metadata
    let metadata = storage.get_metadata(&bucket, &key).await?;

    // Return S3-compatible response
    Ok((
        StatusCode::OK,
        [
            ("Content-Type", metadata.content_type),
            ("Content-Length", data.len().to_string()),
            ("ETag", format!("\"{}\"", metadata.etag)),
            ("Last-Modified", metadata.last_modified.to_rfc2822()),
        ],
        data,
    ))
}
```

### LIST Objects
```rust
#[derive(Deserialize)]
pub struct ListObjectsQuery {
    prefix: Option<String>,
    delimiter: Option<String>,
    marker: Option<String>,
    max_keys: Option<usize>,
}

pub async fn list_objects(
    State(storage): State<StorageEngine>,
    Path(bucket): Path<String>,
    Query(params): Query<ListObjectsQuery>,
) -> Result<impl IntoResponse> {
    let objects = storage.list_objects(
        &bucket,
        params.prefix.as_deref(),
        params.delimiter.as_deref(),
        params.marker.as_deref(),
        params.max_keys.unwrap_or(1000),
    ).await?;

    // Return S3 XML format
    let xml = format_list_objects_xml(&bucket, &objects);

    Ok((
        StatusCode::OK,
        [("Content-Type", "application/xml")],
        xml,
    ))
}
```

---

## Observability

### Telemetry Integration
```rust
use avx_telemetry::{Telemetry, Span};
use tracing::{info, warn, instrument};

#[instrument(skip(self, data))]
pub async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<()> {
    let start = std::time::Instant::now();

    // Track size
    let original_size = data.len();

    // Compress
    let compressed = self.compress(&data, StorageClass::Standard)?;
    let compressed_size = compressed.len();
    let ratio = original_size as f64 / compressed_size as f64;

    // Store
    self.storage.put(bucket, key, compressed).await?;

    // Log metrics
    let latency_ms = start.elapsed().as_millis();
    info!(
        bucket = bucket,
        key = key,
        original_size = original_size,
        compressed_size = compressed_size,
        compression_ratio = format!("{:.2}", ratio),
        latency_ms = latency_ms,
        "Object stored"
    );

    // Warn if high latency
    if latency_ms > 50 {
        warn!(
            latency_ms = latency_ms,
            "High latency for PUT operation"
        );
    }

    Ok(())
}
```

---

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_put_and_get() {
        let storage = StorageEngine::new_in_memory();

        let data = b"Hello AVL Storage!".to_vec();
        storage.put("test-bucket", "test-key", data.clone()).await.unwrap();

        let retrieved = storage.get("test-bucket", "test-key").await.unwrap().unwrap();
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_compression() {
        let storage = StorageEngine::new_in_memory();

        // Highly compressible data
        let data = vec![0u8; 1024 * 1024]; // 1 MB of zeros
        let compressed = storage.compress(&data, StorageClass::Standard).unwrap();

        // Should be significantly smaller
        assert!(compressed.len() < data.len() / 10);

        // Should decompress correctly
        let decompressed = storage.decompress(&compressed, StorageClass::Standard).unwrap();
        assert_eq!(decompressed, data);
    }

    #[tokio::test]
    async fn test_multipart_upload() {
        let client = StorageClient::new("http://localhost:9000");

        let upload = client.create_multipart_upload("bucket", "key").await.unwrap();

        // Upload 3 parts
        let part1 = client.upload_part("bucket", "key", &upload.upload_id, 1, vec![1; 5*1024*1024]).await.unwrap();
        let part2 = client.upload_part("bucket", "key", &upload.upload_id, 2, vec![2; 5*1024*1024]).await.unwrap();
        let part3 = client.upload_part("bucket", "key", &upload.upload_id, 3, vec![3; 1024*1024]).await.unwrap();

        // Complete
        client.complete_multipart_upload(
            "bucket",
            "key",
            &upload.upload_id,
            vec![(1, part1), (2, part2), (3, part3)],
        ).await.unwrap();

        // Verify object exists
        let obj = client.get_object("bucket", "key").await.unwrap();
        assert_eq!(obj.content_length, 11 * 1024 * 1024);
    }
}
```

---

## Best Practices Summary

### ✅ ALWAYS
- Compress objects with `avila-compress` before storage
- Use multipart uploads for files > 100 MB
- Return S3-compatible error codes and headers
- Calculate correct ETags (MD5 for single, multi-part for multipart)
- Log latency and compression ratio with `avx-telemetry`
- Encrypt at rest and in transit

### ❌ NEVER
- Store uncompressed objects > 1 MB
- Skip validation of bucket/key names
- Return non-standard S3 responses
- Expose internal storage paths
- Skip ETag validation on uploads

### 🇧🇷 Brazil-Specific
- Always measure latency from São Paulo
- Provide Portuguese error messages
- Price in R$ (Reais)
- Optimize for within-Brazil transfers

---

## Related Crates

- **avila-compress**: LZ4/Zstd compression (used for all objects)
- **avx-telemetry**: Observability and metrics
- **avx-gateway**: API gateway (routes S3 requests to AVL Storage)
- **aviladb**: Database (stores object metadata)

---

## Contact & Support

**Project Lead**: Nicolas Ávila
**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: https://github.com/avilaops/arxis

---

**AVL Storage** - *The Storage Fortress*
🏛️ Durable as a fortress | ⚙️ Fast as an engine | 🇧🇷 Built for Brazil
