pub mod whitespace;
pub mod byte_level;
pub mod metaspace;
pub mod punctuation;
pub mod digits;

pub use whitespace::WhitespaceSplit;
pub use byte_level::ByteLevel;
pub use metaspace::Metaspace;
pub use punctuation::PunctuationSplit;
pub use digits::DigitsSplit;

use crate::error::Result;

/// Trait for pre-tokenization (splitting text before tokenization)
pub trait PreTokenizer: Send + Sync {
    /// Pre-tokenize text into words/segments
    fn pre_tokenize(&self, text: &str) -> Result<Vec<String>>;
}
