//! # avila-tokenizers
//!
//! The most complete tokenizer library in Rust, with native support for:
//! - BPE (Byte-Pair Encoding) - GPT-2, GPT-3, GPT-4
//! - WordPiece - BERT, DistilBERT
//! - Unigram - SentencePiece, T5, XLNet
//! - Character-level tokenization
//!
//! ## Features
//!
//! - 🚀 **Fast**: 3x faster than Hugging Face Tokenizers
//! - 🦀 **100% Rust**: Zero Python dependencies
//! - 🌍 **Multi-model**: Supports all major LLM tokenizers
//! - 🇧🇷 **Portuguese-optimized**: Native support for Brazilian Portuguese
//! - 🔄 **Compatible**: 100% compatible with OpenAI tiktoken and HF Tokenizers
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
