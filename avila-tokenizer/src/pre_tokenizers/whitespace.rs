use super::PreTokenizer;
use crate::error::Result;

/// Simple whitespace pre-tokenizer
#[derive(Debug, Clone, Copy)]
pub struct WhitespaceSplit;

impl WhitespaceSplit {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WhitespaceSplit {
    fn default() -> Self {
        Self::new()
    }
}

impl PreTokenizer for WhitespaceSplit {
    fn pre_tokenize(&self, text: &str) -> Result<Vec<String>> {
        Ok(text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_split() {
        let tokenizer = WhitespaceSplit::new();
        let result = tokenizer.pre_tokenize("Hello world test").unwrap();
        assert_eq!(result, vec!["Hello", "world", "test"]);
    }
}
