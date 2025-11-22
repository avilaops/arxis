use super::Normalizer;
use crate::error::Result;
use crate::utils::unicode::{strip_accents, remove_control_chars};

/// Strip accents normalizer
/// Removes combining diacritical marks
/// Example: café → cafe, São → Sao
#[derive(Debug, Clone, Copy)]
pub struct StripAccentsNormalizer;

impl StripAccentsNormalizer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StripAccentsNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Normalizer for StripAccentsNormalizer {
    fn normalize(&self, text: &str) -> Result<String> {
        Ok(strip_accents(text))
    }
}

/// Strip control characters and whitespace normalizer
#[derive(Debug, Clone)]
pub struct StripNormalizer {
    strip_left: bool,
    strip_right: bool,
    strip_control: bool,
}

impl StripNormalizer {
    pub fn new() -> Self {
        Self {
            strip_left: true,
            strip_right: true,
            strip_control: true,
        }
    }

    pub fn with_left(mut self, strip_left: bool) -> Self {
        self.strip_left = strip_left;
        self
    }

    pub fn with_right(mut self, strip_right: bool) -> Self {
        self.strip_right = strip_right;
        self
    }

    pub fn with_control(mut self, strip_control: bool) -> Self {
        self.strip_control = strip_control;
        self
    }
}

impl Default for StripNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Normalizer for StripNormalizer {
    fn normalize(&self, text: &str) -> Result<String> {
        let mut result = text.to_string();

        if self.strip_control {
            result = remove_control_chars(&result);
        }

        if self.strip_left && self.strip_right {
            result = result.trim().to_string();
        } else if self.strip_left {
            result = result.trim_start().to_string();
        } else if self.strip_right {
            result = result.trim_end().to_string();
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_accents() {
        let normalizer = StripAccentsNormalizer::new();

        let text = "café";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "cafe");
    }

    #[test]
    fn test_strip_accents_portuguese() {
        let normalizer = StripAccentsNormalizer::new();

        let text = "São Paulo";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "Sao Paulo");
    }

    #[test]
    fn test_strip_whitespace() {
        let normalizer = StripNormalizer::new();

        let text = "  hello world  ";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "hello world");
    }

    #[test]
    fn test_strip_left_only() {
        let normalizer = StripNormalizer::new().with_right(false);

        let text = "  hello  ";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "hello  ");
    }

    #[test]
    fn test_strip_right_only() {
        let normalizer = StripNormalizer::new().with_left(false);

        let text = "  hello  ";
        let normalized = normalizer.normalize(text).unwrap();
        assert_eq!(normalized, "  hello");
    }
}
