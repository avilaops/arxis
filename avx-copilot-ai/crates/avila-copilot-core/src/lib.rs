// Core orchestration layer for Avila Copilot
// Coordinates all 7 layers for optimal performance

use avila_copilot_context::ContextManager;
use avila_copilot_inference::InferenceEngine;
use avila_copilot_intelligence::CodeIntelligence;
use avila_copilot_model_storage::ModelStorage;
use avila_copilot_tokenizer::CopilotTokenizer;

pub mod config;
pub mod engine;
pub mod error;
pub mod metrics;

pub use engine::CopilotEngine;
pub use error::{CopilotError, Result};

// Re-export types from engine (which re-exports from intelligence)
pub use engine::{Bug, BugSeverity, Completion, Refactoring, RefactoringKind};

/// Performance targets
pub const MAX_LATENCY_MS: u64 = 50;
pub const TARGET_LATENCY_MS: u64 = 30;

/// Core Copilot configuration
#[derive(Debug, Clone)]
pub struct CopilotConfig {
    /// Model storage path
    pub model_path: String,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Maximum context tokens
    pub max_context_tokens: usize,
    /// Enable SIMD optimizations
    pub enable_simd: bool,
    /// Number of worker threads
    pub num_threads: usize,
}

impl Default for CopilotConfig {
    fn default() -> Self {
        Self {
            model_path: "./models".to_string(),
            cache_size_mb: 1024,
            max_context_tokens: 100_000,
            enable_simd: true,
            num_threads: num_cpus::get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = CopilotConfig::default();
        assert!(config.cache_size_mb > 0);
        assert!(config.max_context_tokens > 0);
    }
}
