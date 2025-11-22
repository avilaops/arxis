pub mod byte_level;
pub mod wordpiece;
pub mod metaspace;
pub mod strip;

pub use byte_level::ByteLevelDecoder;
pub use wordpiece::WordPieceDecoder;
pub use metaspace::MetaspaceDecoder;
pub use strip::StripDecoder;

use crate::error::Result;

/// Trait for decoding tokens back to text
pub trait Decoder: Send + Sync {
    /// Decode tokens to text
    fn decode(&self, tokens: &[String]) -> Result<String>;
}
