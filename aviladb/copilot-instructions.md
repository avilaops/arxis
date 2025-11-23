# AvilaDB - Copilot Instructions

## Project Identity

**AvilaDB** is the **distributed NoSQL database** for the AVL Cloud Platform. It embodies the Arxis philosophy:
- **ARX (Fortress)**: Solid, reliable, ACID-compliant data storage
- **AXIS (Engine)**: High-performance query engine with vector search

**Key Differentiators:**
- 🇧🇷 Optimized for Brazil (5-10ms latency in São Paulo/Rio)
- 📦 4 MB documents (2x larger than competitors)
- 🔍 Native vector search (HNSW, no external services)
- 🌍 Multi-region writes FREE
- 💰 40-60% cheaper than AWS/Azure for Brazilian workloads

---

## Architecture Principles

### 1. Fortress Philosophy (ARX)
```rust
// ✅ ALWAYS: Guarantee data durability
// ✅ ALWAYS: Use ACID transactions
// ✅ ALWAYS: Encrypt at rest and in transit
// ✅ ALWAYS: Multi-region replication
```

### 2. Engine Philosophy (AXIS)
```rust
// ✅ ALWAYS: Optimize for low latency (target 5-10ms)
// ✅ ALWAYS: Use avila-compress for storage efficiency
// ✅ ALWAYS: Partition-aware query routing
// ✅ ALWAYS: Connection pooling and reuse
```

### 3. Brazil-First Design
```rust
// ✅ ALWAYS: Prioritize Brazilian data centers
// ✅ ALWAYS: Measure latency from São Paulo perspective
// ✅ ALWAYS: Price in R$ (Reais), not USD
// ✅ ALWAYS: Portuguese documentation alongside English
```

---

## Code Style & Standards

### Naming Conventions
```rust
// Types: PascalCase
pub struct AvilaClient { }
pub struct Document { }
pub struct VectorIndex { }

// Functions: snake_case
pub async fn insert_document() { }
pub async fn vector_search() { }

// Constants: SCREAMING_SNAKE_CASE
const MAX_DOCUMENT_SIZE_MB: usize = 4;
const MAX_PARTITION_SIZE_GB: usize = 50;
```

### Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AvilaDBError {
    #[error("Document too large: {size} MB (max: 4 MB)")]
    DocumentTooLarge { size: usize },

    #[error("Partition full: {size} GB (max: 50 GB)")]
    PartitionFull { size: usize },

    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AvilaDBError>;
```

### Async/Await
```rust
// ✅ ALWAYS: Use async for I/O operations
pub async fn insert(&self, doc: Document) -> Result<InsertResult> {
    // Compress before storage
    let compressed = self.compress_document(&doc)?;

    // Write to storage
    self.storage.put(&compressed).await?;

    // Update indexes
    self.update_indexes(&doc).await?;

    Ok(InsertResult { id: doc.id })
}

// ✅ ALWAYS: Provide sync alternatives when possible
pub fn insert_blocking(&self, doc: Document) -> Result<InsertResult> {
    tokio::runtime::Runtime::new()?.block_on(self.insert(doc))
}
```

---

## Performance Guidelines

### 1. Compression (avila-compress)
```rust
use avila_compress::lz4::Lz4Compressor;

// ✅ ALWAYS: Compress documents before storage
pub fn compress_document(&self, doc: &Document) -> Result<Vec<u8>> {
    let json = serde_json::to_vec(doc)?;
    let compressor = Lz4Compressor::new();
    Ok(compressor.compress(&json)?)
}

// ✅ ALWAYS: Decompress on read
pub fn decompress_document(&self, data: &[u8]) -> Result<Document> {
    let compressor = Lz4Compressor::new();
    let json = compressor.decompress(data)?;
    Ok(serde_json::from_slice(&json)?)
}
```

### 2. Batch Operations
```rust
// ✅ PREFER: Batch writes over individual writes
pub async fn insert_batch(&self, docs: Vec<Document>) -> Result<BatchResult> {
    // Compress all documents
    let compressed: Vec<_> = docs.iter()
        .map(|doc| self.compress_document(doc))
        .collect::<Result<_>>()?;

    // Single write transaction
    self.storage.batch_put(compressed).await?;

    Ok(BatchResult { count: docs.len() })
}

// ❌ AVOID: Loop with individual writes
// for doc in docs {
//     self.insert(doc).await?; // Slow!
// }
```

### 3. Query Optimization
```rust
// ✅ ALWAYS: Use partition key in queries
let results = collection
    .query("SELECT * FROM users WHERE userId = @id") // Partition key!
    .param("id", "user123")
    .execute()
    .await?;

// ❌ AVOID: Cross-partition queries without filters
// SELECT * FROM users WHERE email = @email  // Scans all partitions!
```

---

## Storage Layer

### RocksDB Integration
```rust
use rocksdb::{DB, Options, WriteBatch};

pub struct StorageEngine {
    db: DB,
    compression: Lz4Compressor,
}

impl StorageEngine {
    pub fn new(path: &str) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::None); // We use avila-compress!

        let db = DB::open(&opts, path)?;

        Ok(Self {
            db,
            compression: Lz4Compressor::new(),
        })
    }

    pub async fn put(&self, key: &str, value: &[u8]) -> Result<()> {
        // Compress with avila-compress
        let compressed = self.compression.compress(value)?;

        // Store in RocksDB
        self.db.put(key.as_bytes(), &compressed)?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        match self.db.get(key.as_bytes())? {
            Some(compressed) => {
                // Decompress
                let data = self.compression.decompress(&compressed)?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}
```

---

## Vector Search

### HNSW Index
```rust
use hnsw::{Hnsw, DistanceMetric};

pub struct VectorIndex {
    index: Hnsw<f32>,
    dimension: usize,
    metric: DistanceMetric,
}

impl VectorIndex {
    pub fn new(dimension: usize, metric: &str) -> Result<Self> {
        let metric = match metric {
            "cosine" => DistanceMetric::Cosine,
            "euclidean" => DistanceMetric::Euclidean,
            "dot" => DistanceMetric::DotProduct,
            _ => return Err(AvilaDBError::InvalidMetric(metric.to_string())),
        };

        let index = Hnsw::new(dimension, 16, 200, metric);

        Ok(Self { index, dimension, metric })
    }

    pub fn add(&mut self, id: usize, vector: &[f32]) -> Result<()> {
        if vector.len() != self.dimension {
            return Err(AvilaDBError::DimensionMismatch {
                expected: self.dimension,
                got: vector.len(),
            });
        }

        self.index.add(id, vector)?;
        Ok(())
    }

    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(usize, f32)>> {
        let results = self.index.search(query, k)?;
        Ok(results)
    }
}
```

---

## Observability

### Telemetry Integration
```rust
use avx_telemetry::{Telemetry, Span};
use tracing::{info, warn, error, instrument};

#[instrument(skip(self))]
pub async fn insert(&self, doc: Document) -> Result<InsertResult> {
    let _span = Span::current();

    // Track operation
    let start = std::time::Instant::now();

    // Perform insert
    let result = self.insert_internal(doc).await?;

    // Log metrics
    let latency_ms = start.elapsed().as_millis();
    info!(
        latency_ms = latency_ms,
        document_size = result.size_bytes,
        "Document inserted"
    );

    // Warn if high latency
    if latency_ms > 100 {
        warn!(
            latency_ms = latency_ms,
            "High latency detected for insert operation"
        );
    }

    Ok(result)
}
```

### Diagnostics
```rust
pub struct DiagnosticInfo {
    pub latency_ms: u128,
    pub storage_latency_ms: u128,
    pub index_latency_ms: u128,
    pub compression_ratio: f64,
    pub partition_key: String,
}

impl Document {
    pub fn diagnostics(&self) -> DiagnosticInfo {
        DiagnosticInfo {
            latency_ms: self.latency_ms,
            storage_latency_ms: self.storage_latency_ms,
            index_latency_ms: self.index_latency_ms,
            compression_ratio: self.compression_ratio,
            partition_key: self.partition_key.clone(),
        }
    }
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
    async fn test_insert_and_query() {
        let client = AvilaClient::connect("http://localhost:8000").await.unwrap();
        let db = client.database("testdb").await.unwrap();
        let collection = db.collection("users").await.unwrap();

        // Insert
        let doc = Document::new()
            .set("userId", "test123")
            .set("name", "Test User");

        let result = collection.insert(doc).await.unwrap();
        assert!(!result.id.is_empty());

        // Query
        let docs = collection
            .query("SELECT * FROM users WHERE userId = @id")
            .param("id", "test123")
            .execute()
            .await
            .unwrap();

        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].get::<String>("name").unwrap(), "Test User");
    }

    #[test]
    fn test_document_size_limit() {
        let mut doc = Document::new();

        // 4 MB limit
        let large_data = vec![0u8; 5 * 1024 * 1024]; // 5 MB
        doc.set("data", large_data);

        let result = doc.validate();
        assert!(matches!(result, Err(AvilaDBError::DocumentTooLarge { .. })));
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_vector_search_e2e() {
    let client = AvilaClient::connect("http://localhost:8000").await.unwrap();
    let db = client.database("vectordb").await.unwrap();
    let collection = db.collection("embeddings").await.unwrap();

    // Create index
    collection.create_vector_index("embedding", 3, "cosine").await.unwrap();

    // Insert vectors
    let docs = vec![
        Document::new()
            .set("text", "hello world")
            .set("embedding", vec![1.0, 0.0, 0.0]),
        Document::new()
            .set("text", "hello rust")
            .set("embedding", vec![0.9, 0.1, 0.0]),
    ];

    collection.insert_batch(docs).await.unwrap();

    // Search
    let query = vec![1.0, 0.0, 0.0];
    let results = collection
        .vector_search("embedding", query)
        .top_k(2)
        .execute()
        .await
        .unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].get::<String>("text").unwrap(), "hello world");
}
```

---

## Documentation

### Public API Documentation
```rust
/// AvilaDB client for connecting to the database.
///
/// # Examples
///
/// ```rust
/// use aviladb::AvilaClient;
///
/// #[tokio::main]
/// async fn main() {
///     let client = AvilaClient::connect("http://localhost:8000").await.unwrap();
///     let db = client.database("mydb").await.unwrap();
/// }
/// ```
pub struct AvilaClient { }

/// Inserts a document into the collection.
///
/// Documents are limited to 4 MB. They are automatically compressed
/// using `avila-compress` before storage.
///
/// # Errors
///
/// Returns `AvilaDBError::DocumentTooLarge` if the document exceeds 4 MB.
///
/// # Examples
///
/// ```rust
/// let doc = Document::new()
///     .set("userId", "user123")
///     .set("name", "João Silva");
///
/// let result = collection.insert(doc).await?;
/// println!("Inserted: {}", result.id);
/// ```
pub async fn insert(&self, doc: Document) -> Result<InsertResult> { }
```

---

## Best Practices Summary

### ✅ ALWAYS
- Use `avila-compress` for document compression
- Measure latency and log diagnostics when > 100ms
- Use partition keys in queries to avoid cross-partition scans
- Batch operations instead of loops with individual operations
- Encrypt data at rest and in transit
- Use `avx-telemetry` for observability

### ❌ NEVER
- Store uncompressed documents > 1 MB
- Make cross-partition queries without filters
- Store sensitive data without encryption
- Skip error handling in async code
- Hardcode credentials or endpoints

### 🇧🇷 Brazil-Specific
- Always test latency from São Paulo (primary region)
- Provide Portuguese documentation
- Price in R$ (Reais)
- Consider local holidays for maintenance windows

---

## Related Crates

- **avila-compress**: Native LZ4 compression (used for storage)
- **avx-telemetry**: Observability and tracing
- **avx-gateway**: API gateway (routes requests to AvilaDB)
- **avx-api-core**: Shared types and utilities

---

## Contact & Support

**Project Lead**: Nicolas Ávila
**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: https://github.com/avilaops/arxis

---

**AvilaDB** - *The Distributed Fortress*
🏛️ Solid as a fortress | ⚙️ Fast as an engine | 🇧🇷 Built for Brazil
