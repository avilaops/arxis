pub mod nfc;
pub mod nfkc;
pub mod lowercase;
pub mod strip;
pub mod replace;

pub use nfc::NFCNormalizer;
pub use nfkc::NFKCNormalizer;
pub use lowercase::LowercaseNormalizer;
pub use strip::{StripAccentsNormalizer, StripNormalizer};
pub use replace::ReplaceNormalizer;

use crate::error::Result;

/// Trait for text normalization
pub trait Normalizer: Send + Sync {
    /// Normalize text
    fn normalize(&self, text: &str) -> Result<String>;
}

/// Chain multiple normalizers
pub struct SequenceNormalizer {
    normalizers: Vec<Box<dyn Normalizer>>,
}

impl SequenceNormalizer {
    pub fn new(normalizers: Vec<Box<dyn Normalizer>>) -> Self {
        Self { normalizers }
    }
}

impl Normalizer for SequenceNormalizer {
    fn normalize(&self, text: &str) -> Result<String> {
        let mut result = text.to_string();
        for normalizer in &self.normalizers {
            result = normalizer.normalize(&result)?;
        }
        Ok(result)
    }
}
