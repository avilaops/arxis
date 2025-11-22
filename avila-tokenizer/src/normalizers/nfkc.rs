use super::Normalizer;
use crate::error::Result;
use crate::utils::unicode::normalize_nfkc;

/// NFKC (Compatibility Composition) Unicode normalization
/// Converts compatibility characters to their canonical equivalents
/// Example: ﬁ (U+FB01) → fi (U+0066 U+0069)
#[derive(Debug, Clone, Copy)]
pub struct NFKCNormalizer;

impl NFKCNormalizer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NFKCNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Normalizer for NFKCNormalizer {
    fn normalize(&self, text: &str) -> Result<String> {
        Ok(normalize_nfkc(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfkc_normalizer() {
        let normalizer = NFKCNormalizer::new();

        // Ligature to regular characters
        let text = "ﬁ"; // U+FB01 (fi ligature)
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "fi");
    }

    #[test]
    fn test_nfkc_fullwidth() {
        let normalizer = NFKCNormalizer::new();

        // Fullwidth to halfwidth
        let text = "Ａ"; // U+FF21 (fullwidth A)
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "A"); // U+0041 (regular A)
    }
}
