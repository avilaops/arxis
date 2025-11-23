use super::PreTokenizer;
use crate::error::Result;
use crate::utils::regex::gpt2_split;

/// Byte-level pre-tokenizer (GPT-2 style)
#[derive(Debug, Clone)]
pub struct ByteLevel {
    add_prefix_space: bool,
    use_regex: bool,
}

impl ByteLevel {
    pub fn new() -> Self {
        Self {
            add_prefix_space: true,
            use_regex: true,
        }
    }

    pub fn with_prefix_space(mut self, add_prefix_space: bool) -> Self {
        self.add_prefix_space = add_prefix_space;
        self
    }

    pub fn with_regex(mut self, use_regex: bool) -> Self {
        self.use_regex = use_regex;
        self
    }
}

impl Default for ByteLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl PreTokenizer for ByteLevel {
    fn pre_tokenize(&self, text: &str) -> Result<Vec<String>> {
        let text = if self.add_prefix_space && !text.starts_with(' ') {
            format!(" {}", text)
        } else {
            text.to_string()
        };

        if self.use_regex {
            Ok(gpt2_split(&text))
        } else {
            Ok(vec![text])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_level() {
        let tokenizer = ByteLevel::new().with_regex(false);
        let result = tokenizer.pre_tokenize("Hello world").unwrap();
        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("Hello"));
    }
}
