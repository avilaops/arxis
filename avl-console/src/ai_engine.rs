//! Local AI Engine abstraction for AVL Console
//!
//! This module introduces an internal AI backend architecture that allows
//! the console to run without depending on external SaaS LLM providers.
//! For now we provide a lightweight, deterministic stub implementation
//! (`LocalAIDummyBackend`) that can be incrementally replaced by a real
//! inference pipeline (e.g. using candle, ggml, llama.cpp FFI, or a custom
//! Rust transformer implementation).
//!
//! Design Goals:
//! - Pluggable backends (pattern based vs local model vs remote API)
//! - Non-blocking async generation interface
//! - Streaming support readiness (returning a `Stream` of tokens in future)
//! - Deterministic fallback when model not loaded
//!
//! Roadmap for real local model integration:
//! 1. Weight loader for GGUF / safetensors
//! 2. Tokenizer integration (existing avila-tokenizer crate)
//! 3. Quantized matrix ops via avx-gpu / avila-linalg crates
//! 4. KV cache management per session
//! 5. Streaming SSE endpoint
//! 6. Persistence of fine-tune deltas

use std::sync::Arc;

/// Enumeration of backend kinds the assistant can use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIBackendKind {
    /// Existing pattern matching heuristics only.
    Pattern,
    /// Local LLM model (stub for now).
    Local,
}

impl Default for AIBackendKind {
    fn default() -> Self { Self::Pattern }
}

/// Result of an AI generation call.
#[derive(Debug, Clone)]
pub struct AIResult {
    pub text: String,
    pub explanation: Option<String>,
    pub tips: Option<Vec<String>>,
    pub sql: Option<String>,
}

/// Trait every backend must implement.
pub trait AIBackend: Send + Sync {
    fn generate(&self, prompt: &str) -> AIResult;
}

/// Dummy local backend. Deterministic and fast.
pub struct LocalAIDummyBackend;

impl LocalAIDummyBackend {
    pub fn new() -> Arc<Self> { Arc::new(Self) }
}

impl AIBackend for LocalAIDummyBackend {
    fn generate(&self, prompt: &str) -> AIResult {
        // Very naive heuristic: produce pseudo-SQL if words like SELECT / FROM appear
        let lower = prompt.to_lowercase();
        if lower.contains("schema") {
            return AIResult {
                text: "📘 Local AI: Posso ajudar descrevendo seu schema. Liste tabelas ou campos específicos para detalhes.".to_string(),
                explanation: Some("Este é um backend local stub. Para descrição de schema real iremos consultar metadados do AvilaDB.".to_string()),
                tips: Some(vec![
                    "Use 'listar tabelas' ou 'detalhar tabela <nome>'".to_string(),
                    "Integre metadados em cache para baixa latência".to_string(),
                ]),
                sql: None,
            };
        }
        if lower.contains("crie") && lower.contains("tabela") {
            return AIResult {
                text: "🛠️ Local AI: Gerando proposta de DDL para a tabela solicitada.".to_string(),
                explanation: Some("Geração de DDL básica baseada em palavras-chave. Ajuste tipos conforme necessidade.".to_string()),
                tips: Some(vec!["Adicione índices para colunas de filtro".to_string(), "Evite muitos campos NULL".to_string()]),
                sql: Some("CREATE TABLE exemplo (
    id UUID PRIMARY KEY,
    nome TEXT NOT NULL,
    criado_em TIMESTAMP DEFAULT NOW()
);".to_string()),
            };
        }
        AIResult {
            text: "🤖 Local AI stub: modelo local ainda não treinado para esta solicitação. Forneça mais contexto ou habilite o backend de padrão (pattern).".to_string(),
            explanation: Some("Backend local está em modo placeholder. Próximas versões incluirão carregamento de pesos e geração token por token.".to_string()),
            tips: Some(vec![
                "Defina objetivo claro: ex. 'Gerar SQL para vendas por mês'".to_string(),
                "Inclua entidades e filtros explícitos".to_string(),
            ]),
            sql: None,
        }
    }
}
