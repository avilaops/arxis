# 🗄️ AvilaDB

### *The Distributed Fortress. The Data Engine.*

**AVILA** (fortress) + **DB** (database) = **AvilaDB**

> *Where data finds solid ground and engines drive queries*

[![Crates.io](https://img.shields.io/crates/v/aviladb.svg)](https://crates.io/crates/aviladb)
[![Documentation](https://docs.rs/aviladb/badge.svg)](https://docs.rs/aviladb)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-CE422B.svg)](https://www.rust-lang.org/)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.cloud)

**🇧🇷 Latency 5-10ms in Brazil** | **🌍 Multi-region writes FREE** | **📦 4 MB documents (2x competitors)**

---

## 🎯 What is AvilaDB?

**AvilaDB** is the distributed NoSQL database for the **AVL Cloud Platform** - built as a **fortress for your data** and an **engine for your queries**.

Like Arxis provides the mathematical citadel, AvilaDB provides the **data citadel**:
- 🏛️ **Solid Foundation**: Consistent, durable, ACID-compliant storage
- ⚙️ **Query Engine**: High-performance queries with vector search
- 🛡️ **Protection**: Encryption, backups, multi-region replication
- 🚀 **Performance**: 5-10ms latency in Brazil, sub-50ms in LATAM

---

## 🌟 Key Features

### 🇧🇷 **Optimized for Brazil & LATAM**
- **5-10ms latency** in São Paulo, Rio, Brasília
- **Sub-50ms** throughout Latin America
- **40-60% cheaper** than AWS DynamoDB or Azure Cosmos DB
- **Native Portuguese** documentation and support

### 📦 **Large Documents**
- **4 MB per document** (vs 400 KB DynamoDB, 2 MB Cosmos DB)
- **50 GB per partition** (vs 10 GB DynamoDB, 20 GB Cosmos DB)
- Hierarchical Partition Keys (HPK) overcome limits

### 🔍 **Native Vector Search**
- Built-in **HNSW index** for embeddings
- **Semantic search** without external services
- Perfect for AI/Chat/RAG applications
- **Low-cost** compared to Pinecone/Weaviate

### 🌍 **Multi-region Writes (FREE)**
- Global distribution included
- No extra cost (unlike AWS/Azure)
- Automatic conflict resolution
- Active-active replication

### ⚡ **High Performance**
- **Compression**: LZ4 via `avila-compress` (>500 MB/s)
- **Throughput Units (TUs)**: Elastic scaling
- **Connection pooling**: Reuse clients
- **Batch operations**: Bulk writes/reads

### 💰 **Pricing (Brazil)**
- **R$ 0,50** per 1M operations
- **R$ 0,20** per GB/month storage
- **FREE** multi-region writes
- **FREE** vector search (built-in)

---

## 🏗️ Architecture

AvilaDB follows the **Arxis philosophy** - solid as a fortress, powerful as an engine.

Built on top of world-class libraries from the **[Arxis ecosystem](https://github.com/avilaops/arxis)**:

### Core Dependencies

- 🗜️ **[avila-compress](https://github.com/avilaops/arxis/tree/main/avila-compress)**
  - Native LZ4/Zstd compression (100% Rust, zero external deps)
  - 3x faster than standard implementations (>500 MB/s)
  - Optimized for columnar data and time-series
  - Production-ready ✅

- 📊 **[avila-telemetry](https://github.com/avilaops/arxis/tree/main/avila-telemetry)**
  - Time series analysis, anomaly detection, forecasting
  - NASA-grade data quality metrics (≥0.95 threshold)
  - Observability and performance monitoring
  - 22 tests passing ✅

- 🧮 **[avila-math](https://github.com/avilaops/arxis/tree/main/avila-math)**
  - Mathematical kernel for vectors and tensors
  - Shared across entire AVL ecosystem
  - High-performance linear algebra
  - 26 tests passing ✅

- 📡 **[avx-http](https://github.com/avilaops/arxis/tree/main/avx-http)**
  - Native HTTP client/server for AVL Platform
  - Optimized for Brazil and LATAM latency
  - Built on Tokio + Axum

### Architecture Layers

```
┌─────────────────────────────────────────┐
│        AvilaDB - Data Citadel           │
├─────────────────────────────────────────┤
│  🏛️ Storage Layer (RocksDB)             │
│     - ACID transactions                  │
│     - LSM-tree storage                   │
│     - avila-compress (LZ4/Zstd)         │
├─────────────────────────────────────────┤
│  ⚙️ Query Engine                        │
│     - SQL-like queries                   │
│     - Vector search (HNSW + avila-math) │
│     - Partition-aware routing           │
├─────────────────────────────────────────┤
│  🛡️ Distribution Layer                  │
│     - Raft consensus                     │
│     - Multi-region replication          │
│     - Automatic failover                │
├─────────────────────────────────────────┤
│  🚀 API Layer (avx-http + Axum)         │
│     - REST + gRPC                        │
│     - WebSocket subscriptions           │
│     - avila-telemetry observability     │
└─────────────────────────────────────────┘
```

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
aviladb = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Using the CLI

```bash
# Install via curl
curl -sSL https://avila.cloud/install.sh | sh

# Connect to AvilaDB
avila db connect --account my-account

# Query data
avila db query "SELECT * FROM users WHERE active = true"
```

### Local Emulator (Docker)

```bash
docker run -p 8000:8000 avilacloud/aviladb-emulator:latest
```

---

## 🚀 Quick Start

### Basic Operations

```rust
use aviladb::{AvilaClient, Document};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to AvilaDB
    let client = AvilaClient::connect("http://localhost:8000").await?;
    let db = client.database("gamedb").await?;
    let players = db.collection("players").await?;

    // Insert document (4 MB limit)
    let player = Document::new()
        .set("userId", "player123")
        .set("username", "CoolGamer")
        .set("level", 42)
        .set("inventory", vec!["sword", "shield"])
        .set("stats", json!({
            "hp": 100,
            "mp": 50,
            "attack": 25
        }));

    let result = players.insert(player).await?;
    println!("Inserted: {}", result.id);

    // Query with filters
    let high_level = players
        .query("SELECT * FROM players WHERE level > @min_level")
        .param("min_level", 40)
        .execute()
        .await?;

    for doc in high_level {
        println!("Player: {}", doc.get("username")?);
    }

    // Update
    players.update()
        .set("level", 43)
        .where_eq("userId", "player123")
        .execute()
        .await?;

    // Delete
    players.delete()
        .where_eq("userId", "player123")
        .execute()
        .await?;

    Ok(())
}
```

### Vector Search (AI/RAG)

```rust
use aviladb::{AvilaClient, VectorIndex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AvilaClient::connect("http://localhost:8000").await?;
    let db = client.database("aidb").await?;
    let memories = db.collection("chat_memory").await?;

    // Create vector index (HNSW)
    memories.create_vector_index("embedding", 1536, "cosine").await?;

    // Store with embedding
    let memory = Document::new()
        .set("userId", "user123")
        .set("message", "Tell me about quantum physics")
        .set("embedding", vec![0.1, 0.2, 0.3, /* ... 1536 dims */])
        .set("timestamp", chrono::Utc::now());

    memories.insert(memory).await?;

    // Semantic search
    let query_embedding = vec![0.15, 0.18, 0.29, /* ... */];
    let similar = memories
        .vector_search("embedding", query_embedding)
        .top_k(5)
        .execute()
        .await?;

    for doc in similar {
        println!("Similar: {} (score: {})",
            doc.get("message")?,
            doc.similarity_score()
        );
    }

    Ok(())
}
```

### Hierarchical Partition Keys (HPK)

```rust
use aviladb::{AvilaClient, HierarchicalKey};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AvilaClient::connect("http://localhost:8000").await?;
    let db = client.database("multitenantdb").await?;

    // Create collection with HPK
    let orders = db.create_collection("orders")
        .partition_key(HierarchicalKey::new()
            .add_level("tenantId")
            .add_level("userId")
            .add_level("orderId")
        )
        .build()
        .await?;

    // Query within tenant
    let tenant_orders = orders
        .query("SELECT * FROM orders WHERE tenantId = @tenant")
        .param("tenant", "acme-corp")
        .execute()
        .await?;

    // Query specific user
    let user_orders = orders
        .query("SELECT * FROM orders WHERE tenantId = @tenant AND userId = @user")
        .param("tenant", "acme-corp")
        .param("user", "user456")
        .execute()
        .await?;

    Ok(())
}
```

---

## 📊 Use Cases

### 🎮 **Game Development**
- Player profiles and session data
- Leaderboards and rankings
- In-game chat and social features
- Real-time matchmaking queues
- **5-10ms latency** for Brazilian players

### 🤖 **AI/Chat/RAG Applications**
- Chat history with vector embeddings
- User context and memory
- Semantic search for retrieval
- Multi-user isolation
- **Native vector search** (no extra services)

### 🛒 **E-commerce**
- Product catalogs
- Shopping carts
- Order management
- User profiles and wishlists
- Real-time inventory

### 📡 **IoT & Scientific**
- Sensor data ingestion
- Device twins and profiles
- Time-series storage
- Telemetry aggregation
- LIGO/LISA data storage

---

## 🆚 Comparison with Competitors

| Feature                 | AvilaDB       | AWS DynamoDB        | Azure Cosmos DB |
| ----------------------- | ------------- | ------------------- | --------------- |
| **Max document size**   | **4 MB** ✅    | 400 KB              | 2 MB            |
| **Partition size**      | **50 GB** ✅   | 10 GB               | 20 GB           |
| **Multi-region writes** | **FREE** ✅    | Extra cost          | Extra cost      |
| **Vector search**       | **Native** ✅  | Requires OpenSearch | Limited         |
| **Brazil latency**      | **5-10ms** ✅  | 80-120ms            | 40-60ms         |
| **Pricing (1M ops)**    | **R$ 0,50** ✅ | USD 1.25            | USD 0.85        |
| **Storage (GB/month)**  | **R$ 0,20** ✅ | USD 0.25            | USD 0.25        |

**AvilaDB is 40-60% cheaper** for Brazilian workloads! 🇧🇷

---

## 🛠️ Best Practices

### Data Modeling
```rust
// ✅ GOOD: Embed related data
let user = Document::new()
    .set("userId", "user123")
    .set("profile", json!({
        "name": "João Silva",
        "email": "joao@example.com"
    }))
    .set("preferences", json!({
        "theme": "dark",
        "language": "pt-BR"
    }));

// ❌ BAD: Too many separate documents for always-together data
// users -> profiles -> preferences (3 queries instead of 1)
```

### Partition Key Choice
```rust
// ✅ GOOD: High cardinality, even distribution
.partition_key("userId")  // userId, tenantId, deviceId, sessionId

// ❌ BAD: Low cardinality, hot partitions
.partition_key("status")  // "active", "inactive" (only 2 values!)
```

### Diagnostics
```rust
// Log diagnostic info when latency is high
let result = players.query("SELECT * FROM players")
    .execute()
    .await?;

if result.latency_ms() > 100 {
    tracing::warn!(
        "High latency: {} ms\nDiagnostics: {}",
        result.latency_ms(),
        result.diagnostics()
    );
}
```

---

## 🧪 Development Tools

### CLI
```bash
# Connect to account
avila db connect --account my-account

# List databases
avila db list

# Query interactively
avila db query "SELECT * FROM users LIMIT 10"

# Create collection
avila db create-collection players --partition-key userId

# Import data
avila db import players data.json

# Export data
avila db export players --output backup.json
```

### Emulator
```bash
# Run locally (no cloud costs!)
docker run -p 8000:8000 avilacloud/aviladb-emulator:latest

# Update connection string
export AVILADB_ENDPOINT=http://localhost:8000
```

---

## 📚 Documentation

- **[Official Docs](https://docs.avila.cloud/aviladb)** - Complete guide
- **[API Reference](https://docs.rs/aviladb)** - Rust API docs
- **[Best Practices](https://docs.avila.cloud/aviladb/best-practices)** - Optimization tips
- **[Examples](./examples)** - Code samples

---

## 🏛️ Philosophy - The Data Citadel

AvilaDB embodies the **Arxis philosophy**:

### 🏛️ **ARX - Fortress**
- **Solid storage**: ACID transactions, durability guarantees
- **Protection**: Encryption at rest and in transit
- **Reliability**: Multi-region replication, automatic backups

### ⚙️ **AXIS - Engine**
- **Query power**: Fast SQL-like queries, vector search
- **Scalability**: Elastic throughput units, auto-scaling
- **Performance**: Optimized for Brazil and LATAM

### 🇧🇷 **Built for Brazil**
- **Local presence**: Data centers in São Paulo, Rio
- **Low latency**: 5-10ms for Brazilian users
- **Fair pricing**: R$ instead of USD, 40-60% cheaper
- **Portuguese-first**: Docs, support, community

---

## 🤝 Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/awesome-feature`)
3. Commit your changes (`git commit -m 'Add awesome feature'`)
4. Push to the branch (`git push origin feature/awesome-feature`)
5. Open a Pull Request

---

## 📞 Support

**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: https://github.com/avilaops/arxis
**Docs**: https://docs.avila.cloud/aviladb

---

## 📜 License

Dual-licensed under MIT OR Apache-2.0 - See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

---

## 🏛️ Built by Avila

**AvilaDB** - *The Distributed Fortress*
Part of the **AVL Cloud Platform**

🏛️ **Solid as a fortress**
⚙️ **Fast as an engine**
🇧🇷 **Built for Brazil**

Built with ❤️ in Rust for the Brazilian and LATAM tech community.
