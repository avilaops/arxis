// Main Copilot engine - orchestrates all layers

use crate::{CopilotConfig, CopilotError, Result, MAX_LATENCY_MS};
use avila_copilot_context::ContextManager;
use avila_copilot_inference::InferenceEngine;
use avila_copilot_intelligence::CodeIntelligence;
use avila_copilot_model_storage::ModelStorage;
use avila_copilot_tokenizer::CopilotTokenizer;
use std::sync::Arc;
use std::time::Instant;

// Re-export types from intelligence
pub use avila_copilot_intelligence::{Bug, BugSeverity, Refactoring, RefactoringKind};

/// Main Copilot engine coordinating all 7 layers
pub struct CopilotEngine {
    config: CopilotConfig,
    model_storage: Arc<ModelStorage>,
    tokenizer: Arc<CopilotTokenizer>,
    context_manager: Arc<ContextManager>,
    inference_engine: Arc<InferenceEngine>,
    code_intelligence: Arc<CodeIntelligence>,
}

impl CopilotEngine {
    /// Create new Copilot engine with default configuration
    pub async fn new() -> Result<Self> {
        Self::with_config(CopilotConfig::default()).await
    }

    /// Create new Copilot engine with custom configuration
    pub async fn with_config(config: CopilotConfig) -> Result<Self> {
        // Layer 1: Model Storage
        let model_storage = Arc::new(
            ModelStorage::new(&config.model_path, config.cache_size_mb)
                .await
                .map_err(|e| CopilotError::ModelLoadError(e.to_string()))?,
        );

        // Layer 2: Tokenizer
        let tokenizer = Arc::new(
            CopilotTokenizer::new()
                .map_err(|e| CopilotError::TokenizationError(e.to_string()))?,
        );

        // Layer 3: Context Manager
        let context_manager = Arc::new(
            ContextManager::new(config.max_context_tokens)
                .await
                .map_err(|e| CopilotError::ContextError(e.to_string()))?,
        );

        // Layer 4: ML Inference Engine
        let inference_engine = Arc::new(
            InferenceEngine::new(Arc::clone(&model_storage), Arc::clone(&tokenizer))
                .await
                .map_err(|e| CopilotError::InferenceError(e.to_string()))?,
        );

        // Layer 5: Code Intelligence
        let code_intelligence = Arc::new(
            CodeIntelligence::new(Arc::clone(&context_manager))
                .await
                .map_err(|e| CopilotError::InferenceError(e.to_string()))?,
        );

        Ok(Self {
            config,
            model_storage,
            tokenizer,
            context_manager,
            inference_engine,
            code_intelligence,
        })
    }

    /// Generate code completion with latency guarantee
    pub async fn complete(&self, input: &str, cursor_position: usize) -> Result<Completion> {
        let start = Instant::now();

        // Get context
        let context = self.context_manager.get_context(input, cursor_position).await;

        // Tokenize
        let tokens = self.tokenizer.encode(&context)?;

        // Run inference
        let output = self.inference_engine.infer(&tokens).await?;

        // Decode result
        let completion_text = self.tokenizer.decode(&output)?;

        let latency_ms = start.elapsed().as_millis() as u64;

        // Enforce latency SLA
        if latency_ms > MAX_LATENCY_MS {
            return Err(CopilotError::LatencyExceeded {
                actual_ms: latency_ms,
                max_ms: MAX_LATENCY_MS,
            });
        }

        Ok(Completion {
            text: completion_text,
            latency_ms,
            confidence: 0.95,
        })
    }

    /// Detect bugs in code
    pub async fn detect_bugs(&self, code: &str) -> Result<Vec<Bug>> {
        Ok(self.code_intelligence.detect_bugs(code).await?)
    }

    /// Generate documentation
    pub async fn generate_docs(&self, code: &str) -> Result<String> {
        Ok(self.code_intelligence.generate_documentation(code).await?)
    }

    /// Generate tests
    pub async fn generate_tests(&self, code: &str) -> Result<String> {
        Ok(self.code_intelligence.generate_tests(code).await?)
    }

    /// Suggest refactorings
    pub async fn suggest_refactorings(&self, code: &str) -> Result<Vec<Refactoring>> {
        Ok(self.code_intelligence.suggest_refactorings(code).await?)
    }
}

/// Code completion result
#[derive(Debug, Clone)]
pub struct Completion {
    pub text: String,
    pub latency_ms: u64,
    pub confidence: f32,
}
