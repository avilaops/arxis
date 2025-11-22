use super::Normalizer;
use crate::error::Result;

/// Lowercase normalization
#[derive(Debug, Clone, Copy)]
pub struct LowercaseNormalizer;

impl LowercaseNormalizer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LowercaseNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Normalizer for LowercaseNormalizer {
    fn normalize(&self, text: &str) -> Result<String> {
        Ok(text.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_normalizer() {
        let normalizer = LowercaseNormalizer::new();

        let text = "Hello WORLD!";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "hello world!");
    }

    #[test]
    fn test_lowercase_unicode() {
        let normalizer = LowercaseNormalizer::new();

        let text = "CAFÉ";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "café");
    }
}
