# 🗄️ AvilaDB SDK - Official Rust Client

[![Crates.io](https://img.shields.io/crates/v/aviladb.svg)](https://crates.io/crates/aviladb)
[![Documentation](https://docs.rs/aviladb/badge.svg)](https://docs.rs/aviladb)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**🇧🇷 Latency 5-10ms in Brazil** | **🌍 Multi-region writes FREE** | **📦 4 MB documents**

---

## 📦 Installation

```toml
[dependencies]
aviladb = "0.1"
tokio = { version = "1", features = ["full"] }
```

---

## 🚀 Quick Start

```rust
use aviladb::{AvilaClient, Document};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to AvilaDB
    let client = AvilaClient::connect("http://localhost:8000").await?;
    let db = client.database("mydb").await?;
    let collection = db.collection("users").await?;

    // Insert document
    let user = Document::new()
        .set("userId", "user123")
        .set("name", "João Silva")
        .set("level", 42);

    collection.insert(user).await?;

    // Query
    let users = collection
        .query("SELECT * FROM users WHERE level > 40")
        .execute()
        .await?;

    Ok(())
}
```

---

## 🌟 Features

### ✅ Core Features (Implemented)

- ✅ **Client & Connection**: Async client with connection pooling
- ✅ **CRUD Operations**: Insert, query, update, delete
- ✅ **Batch Operations**: Bulk inserts/updates
- ✅ **Query API**: SQL-like queries with parameterization
- ✅ **Compression**: LZ4/Zstd via native Rust (no C deps)
- ✅ **Error Handling**: Comprehensive error types
- ✅ **Configuration**: Builder pattern with validation
- ✅ **Documentation**: Inline examples and rustdoc

### 🚧 In Progress

- 🚧 **Vector Search**: HNSW index for embeddings (integration pending)
- 🚧 **Telemetry**: Observability with avila-telemetry (integration pending)
- 🚧 **Cache**: Query result caching

### 📋 Planned

- 📋 **Transactions**: ACID transactions
- 📋 **Streams**: Real-time data streams
- 📋 **Geo Queries**: Geographic queries

---

## 📚 Examples

Run examples with:

```bash
# Basic operations
cargo run --example quickstart

# Game backend
cargo run --example game_backend

# AI Chat with RAG
cargo run --example ai_chat_rag

# IoT telemetry
cargo run --example iot_telemetry

# Vector search
cargo run --example vector_search
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run benchmarks
cargo bench
```

---

## 📊 Benchmarks

```bash
# Compression benchmarks
cargo bench --bench compression_bench

# Query benchmarks
cargo bench --bench query_bench
```

---

## 🏗️ Architecture

AvilaDB SDK follows Rust best practices:

- **Async/Await**: Built on Tokio
- **Zero-Copy**: Minimize allocations
- **Type Safety**: Strong typing for documents
- **Error Handling**: Result types with thiserror
- **Builder Pattern**: Fluent configuration API

---

## 📞 Support

**Email**: nicolas@avila.inc
**Docs**: https://docs.avila.inc/aviladb
**GitHub**: https://github.com/avilaops/arxis

---

## 📜 License

Dual-licensed under MIT OR Apache-2.0.

---

**Built with ❤️ in Rust for the Brazilian and LATAM tech community.**
