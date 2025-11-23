//! # Avila Convexa1D
//!
//! Processamento de dados sequenciais unidimensionais (1D) para áudio e texto.
//! Otimizado para análise temporal e extração de features.

pub mod audio;
pub mod text;
pub mod common;

pub use audio::{AudioProcessor, AudioFeatures};
pub use text::{TextProcessor, TextFeatures};
pub use common::{SequentialData, ConvolutionKernel};

/// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
