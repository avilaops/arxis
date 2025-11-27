# AvilaDB Rust SDK - Guia Completo

> SDK oficial do AvilaDB para Rust - Database distribuído otimizado para Brasil e LATAM

[![Crates.io](https://img.shields.io/crates/v/aviladb.svg)](https://crates.io/crates/aviladb)
[![Documentation](https://docs.rs/aviladb/badge.svg)](https://docs.rs/aviladb)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

---

## 📋 Índice

1. [Instalação](#instalação)
2. [Quick Start](#quick-start)
3. [Conceitos Fundamentais](#conceitos-fundamentais)
4. [Operações CRUD](#operações-crud)
5. [Queries Avançadas](#queries-avançadas)
6. [Vector Search & RAG](#vector-search--rag)
7. [Partition Keys & Modelagem](#partition-keys--modelagem)
8. [Performance & Otimização](#performance--otimização)
9. [Exemplos Completos](#exemplos-completos)
10. [Referência da API](#referência-da-api)

---

## 🚀 Instalação

### Adicionar ao Cargo.toml

```toml
[dependencies]
aviladb = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Instalar CLI (opcional)

```bash
curl -sSL https://avila.inc/install.sh | sh
```

### Emulador Local (Docker)

```bash
docker run -p 8000:8000 avilacloud/aviladb-emulator:latest
```

---

## ⚡ Quick Start

### Conexão Básica

```rust
use aviladb::{AvilaClient, Document};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conectar ao AvilaDB
    let client = AvilaClient::connect("http://localhost:8000").await?;

    // Obter database
    let db = client.database("mydb").await?;

    // Obter collection
    let users = db.collection("users").await?;

    Ok(())
}
```

### Inserir Documento

```rust
let user = Document::new()
    .set("userId", "user123")
    .set("name", "João Silva")
    .set("email", "joao@example.com")
    .set("age", 28);

let result = users.insert(user).await?;
println!("Inserted: {}", result.id);
```

### Consultar Documentos

```rust
let results = users
    .query("SELECT * FROM users WHERE age > @min_age")
    .param("min_age", 25)
    .execute()
    .await?;

for doc in results.documents {
    let name: String = doc.get("name")?;
    println!("Found: {}", name);
}
```

---

## 🎯 Conceitos Fundamentais

### Hierarquia de Recursos

```
AvilaClient
  └─ Database
      └─ Collection
          └─ Document
```

### Document

- **Limite**: 4 MB por documento (2x maior que DynamoDB)
- **Formato**: JSON com schema flexível
- **Validação**: Automática antes de insert
- **Compressão**: Automática via `avila-compress` (LZ4)

```rust
let doc = Document::new()
    .set("field1", "value")
    .set("field2", 42)
    .set("nested", json!({
        "key": "value"
    }));
```

### Collection

- **Partition Size**: 50 GB por partição lógica
- **Partition Key**: Define distribuição de dados
- **Indexes**: Suporta vector indexes (HNSW)

---

## 📝 Operações CRUD

### Create (Insert)

#### Insert Único

```rust
let doc = Document::new()
    .set("userId", "user001")
    .set("name", "Maria Santos");

let result = collection.insert(doc).await?;

println!("ID: {}", result.id);
println!("Size: {} bytes", result.size_bytes);
println!("Compression: {:.2}x", result.compression_ratio);
println!("Latency: {}ms", result.latency_ms);
```

#### Batch Insert

```rust
let docs = vec![
    Document::new().set("userId", "user001").set("name", "João"),
    Document::new().set("userId", "user002").set("name", "Maria"),
    Document::new().set("userId", "user003").set("name", "Pedro"),
];

let results = collection.insert_batch(docs).await?;
println!("Inserted {} documents", results.len());
```

### Read (Get)

#### Get por ID

```rust
if let Some(doc) = collection.get("doc-id-123").await? {
    let name: String = doc.get("name")?;
    let age: i32 = doc.get("age")?;

    println!("Found: {} (age: {})", name, age);
}
```

#### Get com Option

```rust
let name: Option<String> = doc.get_opt("name");
let email: Option<String> = doc.get_opt("email");
```

### Update

```rust
let updated = collection.update()
    .set("status", "active")
    .set("lastLogin", chrono::Utc::now().timestamp())
    .where_eq("userId", "user123")
    .execute()
    .await?;

println!("Updated {} documents", updated);
```

### Delete

```rust
let deleted = collection.delete()
    .where_eq("userId", "user123")
    .execute()
    .await?;

println!("Deleted {} documents", deleted);
```

⚠️ **Importante**: DELETE sem WHERE clause é bloqueado para prevenir acidentes!

---

## 🔍 Queries Avançadas

### SQL-like Syntax

```rust
// Filtros simples
let results = collection
    .query("SELECT * FROM users WHERE active = true")
    .execute()
    .await?;

// Comparações
let results = collection
    .query("SELECT * FROM users WHERE age > @min AND age < @max")
    .param("min", 18)
    .param("max", 65)
    .execute()
    .await?;

// ORDER BY e LIMIT
let results = collection
    .query("SELECT * FROM users ORDER BY createdAt DESC LIMIT 10")
    .execute()
    .await?;
```

### Parâmetros Tipados

```rust
// String
.param("name", "João")

// Números
.param("age", 25)
.param("price", 99.99)

// Boolean
.param("active", true)

// Arrays
.param("tags", vec!["rust", "developer"])

// JSON objects
.param("metadata", json!({
    "key": "value"
}))
```

### Operadores Suportados

- `=`, `!=` - Igualdade
- `>`, `<`, `>=`, `<=` - Comparação
- `IN`, `NOT IN` - Conjuntos
- `LIKE` - Pattern matching
- `AND`, `OR` - Lógica booleana

---

## 🧠 Vector Search & RAG

### Criar Vector Index

```rust
// Criar índice HNSW
collection.create_vector_index(
    "embedding",  // nome do campo
    1536,         // dimensão (ex: OpenAI ada-002)
    "cosine"      // métrica: cosine, euclidean, dot
).await?;
```

### Inserir com Embeddings

```rust
// Embedding de 1536 dimensões (OpenAI)
let embedding: Vec<f32> = get_embedding_from_openai(text).await?;

let doc = Document::new()
    .set("text", "Explain quantum physics")
    .set("embedding", embedding)
    .set("metadata", json!({
        "source": "user_query",
        "userId": "user123"
    }));

collection.insert(doc).await?;
```

### Semantic Search

```rust
let query_embedding = get_embedding_from_openai("What is AI?").await?;

let similar = collection
    .vector_search("embedding", query_embedding)
    .top_k(5)                    // Top 5 resultados
    .min_similarity(0.7)         // Threshold mínimo
    .execute()
    .await?;

for doc in similar {
    let text: String = doc.get("text")?;
    let score = doc.similarity_score();
    println!("Score: {:.3} - {}", score, text);
}
```

### RAG Pattern (Retrieval-Augmented Generation)

```rust
// 1. Obter embedding da pergunta do usuário
let user_question = "Explain database partitioning";
let question_embedding = get_embedding(user_question).await?;

// 2. Buscar contexto relevante
let context_docs = memories
    .vector_search("embedding", question_embedding)
    .top_k(3)
    .execute()
    .await?;

// 3. Construir prompt com contexto
let mut context = String::new();
for doc in context_docs {
    let text: String = doc.get("text")?;
    context.push_str(&format!("- {}\n", text));
}

let prompt = format!(
    "Context:\n{}\n\nQuestion: {}\n\nAnswer:",
    context, user_question
);

// 4. Enviar para LLM (GPT-4, Claude, etc.)
let answer = llm.generate(prompt).await?;
```

---

## 🔑 Partition Keys & Modelagem

### Escolhendo Partition Key

✅ **Boas escolhas** (alta cardinalidade):
- `userId`
- `tenantId`
- `deviceId`
- `sessionId`

❌ **Más escolhas** (baixa cardinalidade):
- `status` (apenas 2-3 valores)
- `country` (poucos valores)
- `type` (poucos valores)

### Hierarchical Partition Keys (HPK)

Para datasets grandes (>50 GB por partição):

```rust
use aviladb::HierarchicalPartitionKey;

// Criar collection com HPK
let orders = db.create_collection("orders")
    .partition_key(HierarchicalPartitionKey::new()
        .add_level("tenantId")   // Nível 1: Tenant
        .add_level("userId")     // Nível 2: Usuário
        .add_level("orderId")    // Nível 3: Pedido
    )
    .build()
    .await?;

// Query otimizada por tenant
let tenant_orders = orders
    .query("SELECT * FROM orders WHERE tenantId = @tenant")
    .param("tenant", "acme-corp")
    .execute()
    .await?;

// Query otimizada por tenant + usuário
let user_orders = orders
    .query("SELECT * FROM orders WHERE tenantId = @tenant AND userId = @user")
    .param("tenant", "acme-corp")
    .param("user", "user123")
    .execute()
    .await?;
```

### Data Modeling Best Practices

#### ✅ Embedding (melhor para dados sempre acessados juntos)

```rust
let user = Document::new()
    .set("userId", "user123")
    .set("profile", json!({
        "name": "João Silva",
        "email": "joao@example.com",
        "phone": "+55 11 98765-4321"
    }))
    .set("preferences", json!({
        "theme": "dark",
        "language": "pt-BR"
    }));
```

#### ❌ Normalização (evite se dados são sempre acessados juntos)

```rust
// Evite se sempre precisa dos 3 documentos juntos:
// users -> profiles -> preferences (3 queries!)
```

---

## ⚡ Performance & Otimização

### 1. Reusar AvilaClient

```rust
// ✅ BOM: Singleton client
lazy_static! {
    static ref CLIENT: AvilaClient = {
        AvilaClient::connect("http://localhost:8000")
            .await
            .expect("Failed to connect")
    };
}

// ❌ RUIM: Criar cliente repetidamente
for _ in 0..100 {
    let client = AvilaClient::connect("http://localhost:8000").await?;
    // ...
}
```

### 2. Batch Operations

```rust
// ✅ BOM: Batch insert
let docs = vec![doc1, doc2, doc3, /* ... */];
collection.insert_batch(docs).await?;

// ❌ RUIM: Inserts individuais
for doc in docs {
    collection.insert(doc).await?;  // Muitas round-trips!
}
```

### 3. Compressão

```rust
use aviladb::Config;

let config = Config {
    endpoint: "http://localhost:8000".to_string(),
    enable_compression: true,      // Habilitar compressão
    compression_level: 6,           // 0-11 (padrão: 6)
    ..Default::default()
};

let client = AvilaClient::with_config(config).await?;
```

### 4. Connection Pooling

```rust
let config = Config {
    max_connections: 100,           // Pool size
    request_timeout: 30,            // Timeout (segundos)
    max_cache_entries: 1000,        // Cache de queries
    cache_ttl: 300,                 // TTL do cache (segundos)
    ..Default::default()
};
```

### 5. Diagnostics & Monitoring

```rust
let result = collection.query("SELECT * FROM users").execute().await?;

// Checar latência
if result.latency_ms() > 100 {
    tracing::warn!(
        "High latency: {} ms",
        result.latency_ms()
    );
}

// Estatísticas do cliente
let stats = client.stats().await;
println!("Requests: {}", stats.http_requests);
println!("Avg latency: {}ms", stats.avg_latency_ms);
println!("Cache hit rate: {:.2}%", stats.cache_hit_rate * 100.0);
```

---

## 📚 Exemplos Completos

### 1. Basic CRUD

```bash
cargo run --example basic_crud
```

Demonstra operações CRUD completas com AvilaDB.

### 2. Vector Search

```bash
cargo run --example vector_search
```

Busca semântica com embeddings e pattern RAG para AI.

### 3. Game Leaderboard

```bash
cargo run --example game_leaderboard
```

Sistema de ranking de jogadores com queries otimizadas.

---

## 📖 Referência da API

### AvilaClient

```rust
// Conectar
AvilaClient::connect(endpoint: &str) -> Result<AvilaClient>
AvilaClient::with_config(config: Config) -> Result<AvilaClient>

// Databases
client.database(name: &str) -> Result<Database>
client.create_database(name: &str) -> Result<Database>
client.list_databases() -> Result<Vec<String>>
client.delete_database(name: &str) -> Result<()>

// Stats
client.stats() -> ClientStats
```

### Database

```rust
// Collections
db.collection(name: &str) -> Result<Collection>
db.create_collection(name: &str, partition_key: &str) -> Result<Collection>
db.list_collections() -> Result<Vec<String>>
db.delete_collection(name: &str) -> Result<()>
```

### Collection

```rust
// Insert
collection.insert(doc: Document) -> Result<InsertResult>
collection.insert_batch(docs: Vec<Document>) -> Result<Vec<InsertResult>>

// Read
collection.get(id: &str) -> Result<Option<Document>>
collection.query(sql: &str) -> Query

// Update
collection.update() -> UpdateBuilder
  .set(field, value)
  .where_eq(field, value)
  .execute() -> Result<usize>

// Delete
collection.delete() -> DeleteBuilder
  .where_eq(field, value)
  .execute() -> Result<usize>

// Vector Search
collection.create_vector_index(field: &str, dim: usize, metric: &str) -> Result<()>
collection.vector_search(field: &str, vector: Vec<f32>) -> VectorSearchBuilder
  .top_k(k: usize)
  .min_similarity(threshold: f32)
  .execute() -> Result<Vec<Document>>
```

### Document

```rust
// Criar
Document::new()
  .set(key, value)  // Builder pattern

// Ler
doc.get<T>(key: &str) -> Result<T>
doc.get_opt<T>(key: &str) -> Option<T>

// Serialização
doc.to_json() -> Result<String>
Document::from_json(json: &str) -> Result<Document>

// Validação
doc.validate() -> Result<()>
doc.size_bytes() -> usize
```

### Query

```rust
query.param(name: &str, value: T) -> Query
query.execute() -> Result<QueryResult>

// QueryResult
result.documents: Vec<Document>
result.total_count: usize
result.latency_ms: u128
result.compression_ratio: f64
```

---

## 🔧 Configuração Avançada

### Config Completo

```rust
use aviladb::Config;

let config = Config {
    // Conexão
    endpoint: "https://aviladb.sa-east-1.avila.cloud".to_string(),

    // Timeout
    request_timeout: 30,            // segundos

    // Connection Pool
    max_connections: 100,

    // Compressão
    enable_compression: true,
    compression_level: 6,           // 0-11

    // Cache
    max_cache_entries: 1000,
    cache_ttl: 300,                 // segundos

    // Retry
    max_retries: 3,
    retry_backoff_ms: 100,
};

let client = AvilaClient::with_config(config).await?;
```

---

## 🛡️ Error Handling

```rust
use aviladb::AvilaError;

match collection.insert(doc).await {
    Ok(result) => println!("Inserted: {}", result.id),
    Err(AvilaError::Validation(msg)) => {
        eprintln!("Validation error: {}", msg);
    }
    Err(AvilaError::Network(msg)) => {
        eprintln!("Network error: {}", msg);
    }
    Err(AvilaError::NotFound(msg)) => {
        eprintln!("Not found: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

---

## 📊 Limites e Quotas

| Recurso | Limite |
|---------|--------|
| Tamanho do documento | 4 MB |
| Tamanho da partição | 50 GB |
| Dimensões do vetor | 2048 |
| Batch insert | 100 docs |
| Query result set | 1 MB |
| Timeout padrão | 30s |

---

## 🌍 Multi-Region

```rust
let config = Config {
    endpoint: "https://aviladb.sa-east-1.avila.cloud".to_string(),
    preferred_regions: vec![
        "sa-east-1",    // São Paulo (primary)
        "sa-east-2",    // Rio de Janeiro (failover)
        "us-east-1",    // N. Virginia (global)
    ],
    ..Default::default()
};
```

---

## 💰 Pricing (Brasil)

- **Operações**: R$ 0,50 por 1M operations
- **Storage**: R$ 0,20 por GB/mês
- **Multi-region writes**: **GRÁTIS** ✅
- **Vector search**: **GRÁTIS** ✅

**40-60% mais barato que AWS/Azure!** 🇧🇷

---

## 📞 Suporte

- **Email**: nicolas@avila.inc
- **WhatsApp**: +55 17 99781-1471
- **Docs**: https://docs.avila.inc/aviladb
- **GitHub**: https://github.com/avilaops/arxis

---

## 📜 Licença

Dual-licensed: MIT OR Apache-2.0

---

**AvilaDB Rust SDK** - Built with ❤️ in Rust for Brazil 🇧🇷
