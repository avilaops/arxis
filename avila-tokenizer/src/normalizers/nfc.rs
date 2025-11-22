use super::Normalizer;
use crate::error::Result;
use crate::utils::unicode::normalize_nfc;

/// NFC (Canonical Composition) Unicode normalization
/// Converts decomposed characters to composed form
/// Example: e + ´ (U+0065 U+0301) → é (U+00E9)
#[derive(Debug, Clone, Copy)]
pub struct NFCNormalizer;

impl NFCNormalizer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NFCNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Normalizer for NFCNormalizer {
    fn normalize(&self, text: &str) -> Result<String> {
        Ok(normalize_nfc(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfc_normalizer() {
        let normalizer = NFCNormalizer::new();

        // NFD to NFC
        let nfd = "e\u{0301}"; // e + combining acute accent
        let nfc = normalizer.normalize(nfd).unwrap();
        assert_eq!(nfc, "é");
    }

    #[test]
    fn test_nfc_preserves_composed() {
        let normalizer = NFCNormalizer::new();

        let text = "café";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "café");
    }
}
