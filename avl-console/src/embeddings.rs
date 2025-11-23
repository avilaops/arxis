//! Embeddings and RAG (Retrieval-Augmented Generation) for AI Assistant
//!
//! This module provides:
//! - Embedding generation (stub with cosine similarity)
//! - Vector storage and retrieval
//! - Context building for RAG queries
//! - Schema metadata indexing
//!
//! Production roadmap:
//! 1. Replace stub with sentence-transformers (e.g., all-MiniLM-L6-v2)
//! 2. Integrate AvilaDB vector search for similarity queries
//! 3. Add incremental indexing of query history
//! 4. Fine-tune embeddings for SQL domain

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Vector representation (f32 for memory efficiency)
pub type Vector = Vec<f32>;

/// Document with embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedDocument {
    pub id: String,
    pub content: String,
    pub embedding: Vector,
    pub metadata: HashMap<String, String>,
}

/// Context snippet for RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSnippet {
    pub content: String,
    pub relevance_score: f32,
    pub source: String,
}

/// Stub embedding generator (deterministic hash-based for now)
pub struct EmbeddingGenerator {
    dimension: usize,
}

impl EmbeddingGenerator {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    /// Generate embedding for text (stub: character frequency + position encoding)
    pub fn embed(&self, text: &str) -> Vector {
        let mut vec = vec![0.0f32; self.dimension];
        let chars: Vec<char> = text.chars().collect();

        // Simple frequency-based embedding (deterministic)
        for (i, c) in chars.iter().enumerate() {
            let idx = (*c as usize) % self.dimension;
            vec[idx] += 1.0 / (1.0 + i as f32 * 0.01);
        }

        // Normalize to unit vector
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            vec.iter_mut().for_each(|x| *x /= norm);
        }

        vec
    }

    /// Compute cosine similarity between two vectors
    pub fn cosine_similarity(a: &Vector, b: &Vector) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a > 0.0 && norm_b > 0.0 {
            dot / (norm_a * norm_b)
        } else {
            0.0
        }
    }
}

impl Default for EmbeddingGenerator {
    fn default() -> Self {
        Self::new(128) // 128-dimensional embeddings
    }
}

/// In-memory vector store (production: use AvilaDB vector index)
pub struct VectorStore {
    pub documents: Vec<EmbeddedDocument>,
    generator: EmbeddingGenerator,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            generator: EmbeddingGenerator::default(),
        }
    }

    /// Add document with automatic embedding
    pub fn add(&mut self, id: String, content: String, metadata: HashMap<String, String>) {
        let embedding = self.generator.embed(&content);
        self.documents.push(EmbeddedDocument {
            id,
            content,
            embedding,
            metadata,
        });
    }

    /// Add pre-embedded document
    pub fn add_document(&mut self, doc: EmbeddedDocument) {
        self.documents.push(doc);
    }

    /// Search for top-k similar documents
    pub fn search(&self, query: &str, top_k: usize) -> Vec<ContextSnippet> {
        let query_embedding = self.generator.embed(query);

        let mut scored: Vec<(f32, &EmbeddedDocument)> = self
            .documents
            .iter()
            .map(|doc| {
                let score = EmbeddingGenerator::cosine_similarity(&query_embedding, &doc.embedding);
                (score, doc)
            })
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        scored
            .into_iter()
            .take(top_k)
            .map(|(score, doc)| ContextSnippet {
                content: doc.content.clone(),
                relevance_score: score,
                source: doc.metadata.get("source").cloned().unwrap_or_else(|| "unknown".to_string()),
            })
            .collect()
    }

    /// Get document count
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

impl Default for VectorStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Build RAG context from query
pub fn build_rag_context(query: &str, store: &VectorStore, max_context_length: usize) -> String {
    let snippets = store.search(query, 3); // Top 3 relevant documents

    if snippets.is_empty() {
        return String::new();
    }

    let mut context = String::from("📚 Contexto relevante:\n\n");
    let mut total_length = context.len();

    for snippet in snippets {
        let snippet_text = format!(
            "• [Relevância: {:.2}] {}\n  Fonte: {}\n\n",
            snippet.relevance_score,
            snippet.content,
            snippet.source
        );

        if total_length + snippet_text.len() > max_context_length {
            break;
        }

        context.push_str(&snippet_text);
        total_length += snippet_text.len();
    }

    context
}

/// Initialize default knowledge base with SQL patterns and AvilaDB docs
pub fn init_default_knowledge_base() -> VectorStore {
    let mut store = VectorStore::new();

    // SQL query patterns
    store.add(
        "pattern_top_n".to_string(),
        "Para encontrar os N registros principais, use ORDER BY coluna DESC LIMIT N. Exemplo: SELECT * FROM users ORDER BY created_at DESC LIMIT 10".to_string(),
        [("source".to_string(), "sql_patterns".to_string())].into(),
    );

    store.add(
        "pattern_aggregation".to_string(),
        "Para agregações por categoria, use GROUP BY com funções agregadas como COUNT, SUM, AVG. Exemplo: SELECT category, COUNT(*) as total FROM products GROUP BY category".to_string(),
        [("source".to_string(), "sql_patterns".to_string())].into(),
    );

    store.add(
        "pattern_join".to_string(),
        "Para combinar tabelas relacionadas, use JOIN. LEFT JOIN mantém registros da tabela esquerda mesmo sem match. Exemplo: SELECT u.name, COUNT(o.id) FROM users u LEFT JOIN orders o ON u.id = o.user_id GROUP BY u.name".to_string(),
        [("source".to_string(), "sql_patterns".to_string())].into(),
    );

    store.add(
        "pattern_date_filter".to_string(),
        "Para filtrar por período recente, use funções de data. Exemplo: WHERE created_at >= NOW() - INTERVAL 30 DAY ou WHERE created_at >= DATE_SUB(NOW(), INTERVAL 7 DAY)".to_string(),
        [("source".to_string(), "sql_patterns".to_string())].into(),
    );

    // AvilaDB optimizations
    store.add(
        "aviladb_partition_key".to_string(),
        "AvilaDB usa partition keys para distribuir dados. Sempre inclua partition key em queries para melhor performance. Use Hierarchical Partition Keys (HPK) para queries flexíveis.".to_string(),
        [("source".to_string(), "aviladb_docs".to_string())].into(),
    );

    store.add(
        "aviladb_indexes".to_string(),
        "Crie índices compostos em colunas frequentemente filtradas ou unidas. AvilaDB suporta índices secundários globais e locais. Use EXPLAIN para analisar uso de índices.".to_string(),
        [("source".to_string(), "aviladb_docs".to_string())].into(),
    );

    store.add(
        "aviladb_vector_search".to_string(),
        "AvilaDB tem vector search nativo para embeddings. Use para RAG, busca semântica, e recomendações. Latência <10ms em Brazil.".to_string(),
        [("source".to_string(), "aviladb_docs".to_string())].into(),
    );

    // Performance tips
    store.add(
        "perf_avoid_select_star".to_string(),
        "Evite SELECT * em produção. Especifique apenas colunas necessárias para reduzir I/O e transferência de rede. Exemplo: SELECT id, name, email FROM users ao invés de SELECT * FROM users".to_string(),
        [("source".to_string(), "performance".to_string())].into(),
    );

    store.add(
        "perf_limit_results".to_string(),
        "Use LIMIT em queries exploratórias e paginação. Evita sobrecarga de memória e melhora tempo de resposta. Combine com OFFSET para paginação: LIMIT 20 OFFSET 40".to_string(),
        [("source".to_string(), "performance".to_string())].into(),
    );

    store.add(
        "perf_covering_index".to_string(),
        "Índices covering (que incluem todas colunas da query) eliminam leitura da tabela principal. AvilaDB otimiza automaticamente para índices covering quando possível.".to_string(),
        [("source".to_string(), "performance".to_string())].into(),
    );

    store
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_generation() {
        let gen = EmbeddingGenerator::default();
        let vec = gen.embed("hello world");
        assert_eq!(vec.len(), 128);

        // Verify normalization (unit vector)
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_cosine_similarity() {
        let gen = EmbeddingGenerator::default();
        let v1 = gen.embed("sql query optimization");
        let v2 = gen.embed("optimize database queries");
        let v3 = gen.embed("weather forecast today");

        let sim_12 = EmbeddingGenerator::cosine_similarity(&v1, &v2);
        let sim_13 = EmbeddingGenerator::cosine_similarity(&v1, &v3);

        // Similar texts should have higher similarity
        assert!(sim_12 > sim_13);
        assert!(sim_12 > 0.3); // Reasonable threshold
    }

    #[test]
    fn test_vector_store() {
        let mut store = VectorStore::new();
        store.add(
            "doc1".to_string(),
            "GROUP BY aggregation queries".to_string(),
            [("source".to_string(), "test".to_string())].into(),
        );
        store.add(
            "doc2".to_string(),
            "JOIN multiple tables together".to_string(),
            [("source".to_string(), "test".to_string())].into(),
        );

        assert_eq!(store.len(), 2);

        let results = store.search("aggregate data by category", 1);
        assert_eq!(results.len(), 1);
        assert!(results[0].relevance_score > 0.0);
    }

    #[test]
    fn test_rag_context_building() {
        let mut store = VectorStore::new();
        store.add(
            "tip1".to_string(),
            "Use indexes for better performance".to_string(),
            [("source".to_string(), "docs".to_string())].into(),
        );
        store.add(
            "tip2".to_string(),
            "Partition keys distribute data evenly".to_string(),
            [("source".to_string(), "docs".to_string())].into(),
        );

        let context = build_rag_context("how to improve query speed", &store, 500);
        assert!(!context.is_empty());
        assert!(context.contains("Contexto relevante"));
        assert!(context.contains("Relevância:"));
    }

    #[test]
    fn test_default_knowledge_base() {
        let store = init_default_knowledge_base();
        assert!(store.len() >= 10);

        let results = store.search("como fazer agregação por categoria", 3);
        assert!(!results.is_empty());
        assert!(results[0].content.contains("GROUP BY") || results[0].content.contains("aggreg"));
    }
}
