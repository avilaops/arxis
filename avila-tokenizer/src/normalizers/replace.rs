use super::Normalizer;
use crate::error::Result;
use regex::Regex;

/// Replace normalizer using regex patterns
#[derive(Debug, Clone)]
pub struct ReplaceNormalizer {
    pattern: Regex,
    replacement: String,
}

impl ReplaceNormalizer {
    pub fn new(pattern: &str, replacement: String) -> Result<Self> {
        let regex = Regex::new(pattern)
            .map_err(|e| crate::error::TokenizerError::NormalizationError(e.to_string()))?;

        Ok(Self {
            pattern: regex,
            replacement,
        })
    }

    pub fn pattern(&self) -> &Regex {
        &self.pattern
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

impl Normalizer for ReplaceNormalizer {
    fn normalize(&self, text: &str) -> Result<String> {
        Ok(self.pattern.replace_all(text, &self.replacement).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_normalizer() {
        let normalizer = ReplaceNormalizer::new(r"\s+", " ".to_string()).unwrap();

        let text = "hello    world\t\ntest";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "hello world test");
    }

    #[test]
    fn test_replace_digits() {
        let normalizer = ReplaceNormalizer::new(r"\d+", "0".to_string()).unwrap();

        let text = "I have 123 apples and 456 oranges";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "I have 0 apples and 0 oranges");
    }

    #[test]
    fn test_replace_punctuation() {
        let normalizer = ReplaceNormalizer::new(r"[^\w\s]", "".to_string()).unwrap();

        let text = "Hello, world! How are you?";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "Hello world How are you");
    }
}
