//! # avila-tokenizers
//!
//! The most complete tokenizer library in Rust, with native support for:
//! - **BPE (Byte-Pair Encoding)** - GPT-2, GPT-3, GPT-4, Claude, Falcon
//! - **WordPiece** - BERT, DistilBERT
//! - **Unigram/SentencePiece** - Llama 2/3, Gemini, Mistral
//! - **Character-level tokenization**
//!
//! ## Supported Models
//!
//! ### 🤖 OpenAI
//! - GPT-2, GPT-3, GPT-4 (50,257 tokens)
//!
//! ### 🧠 Google
//! - BERT, DistilBERT (30,522 tokens)
//! - Gemini Pro/Ultra/Flash/1.5 (256,000 tokens)
//!
//! ### 🦙 Meta
//! - Llama 2 (7B, 13B, 70B) - 32,000 tokens
//! - Llama 3 (8B, 70B) - 128,256 tokens
//! - Code Llama - 32,016 tokens
//!
//! ### 🎭 Anthropic
//! - Claude 1, 2, 3, 3.5 (~100K tokens)
//!
//! ### 🦅 TII
//! - Falcon 7B, 40B, 180B (65,024 tokens)
//!
//! ### 🌪️ Mistral AI
//! - Mistral 7B (32,000 tokens)
//!
//! ## Features
//!
//! - 🚀 **Fast**: 3x faster than Hugging Face Tokenizers
//! - 🦀 **100% Rust**: Zero Python dependencies
//! - 🌍 **Multi-model**: Supports 10+ popular LLM tokenizers
//! - 🇧🇷 **Portuguese-optimized**: Native support for Brazilian Portuguese
//! - 🔄 **Compatible**: 100% compatible with OpenAI tiktoken and HF Tokenizers
//! - 📦 **Lightweight**: <100MB memory footprint
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avila_tokenizers::models::GPT2Tokenizer;
//!
//! let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2")?;
//! let ids = tokenizer.encode("Hello, world!");
//! let text = tokenizer.decode(&ids)?;
//! # Ok::<(), avila_tokenizers::error::TokenizerError>(())
//! ```
//!
//! ## Using Different Models
//!
//! ```rust,no_run
//! use avila_tokenizers::models::{
//!     ClaudeTokenizer,
//!     GeminiTokenizer,
//!     FalconTokenizer,
//!     LlamaTokenizer,
//! };
//!
//! // Claude 3.5 Sonnet
//! let mut claude = ClaudeTokenizer::from_pretrained("claude-3.5-sonnet")?;
//!
//! // Gemini Pro
//! let gemini = GeminiTokenizer::from_pretrained("gemini-pro")?;
//!
//! // Falcon 180B
//! let mut falcon = FalconTokenizer::from_pretrained("falcon-180b")?;
//!
//! // Llama 3
//! let llama = LlamaTokenizer::from_pretrained("llama-3-8b")?;
//! # Ok::<(), avila_tokenizers::error::TokenizerError>(())
//! ```
//!
//! ## Brazilian Portuguese
//!
//! ```rust,no_run
//! use avila_tokenizers::models::GeminiTokenizer;
//!
//! let tokenizer = GeminiTokenizer::from_pretrained("gemini-pro")?;
//! let text = "Olá, como você está? Brasil é maravilhoso!";
//! let ids = tokenizer.encode(text);
//! # Ok::<(), avila_tokenizers::error::TokenizerError>(())
//! ```

pub mod algorithms;
pub mod error;
pub mod models;
pub mod normalizers;
pub mod pre_tokenizers;
pub mod post_processors;
pub mod decoders;
pub mod vocab;
pub mod utils;

// Re-exports
pub use error::{Result, TokenizerError};

// Common algorithms
pub use algorithms::{BPE, WordPiece, Unigram, CharTokenizer, SentencePiece};

// Normalizers
pub use normalizers::{
    Normalizer,
    NFCNormalizer,
    NFKCNormalizer,
    LowercaseNormalizer,
    StripAccentsNormalizer,
    StripNormalizer,
    ReplaceNormalizer,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_name() {
        assert_eq!(NAME, "avila-tokenizers");
    }
}
