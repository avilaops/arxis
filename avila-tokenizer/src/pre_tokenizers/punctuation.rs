use super::PreTokenizer;
use crate::error::Result;
use crate::utils::unicode::is_punctuation;

/// Punctuation splitter
#[derive(Debug, Clone, Copy)]
pub struct PunctuationSplit;

impl PunctuationSplit {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PunctuationSplit {
    fn default() -> Self {
        Self::new()
    }
}

impl PreTokenizer for PunctuationSplit {
    fn pre_tokenize(&self, text: &str) -> Result<Vec<String>> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        for ch in text.chars() {
            if is_punctuation(ch) {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(ch.to_string());
            } else if ch.is_whitespace() {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(ch);
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punctuation_split() {
        let tokenizer = PunctuationSplit::new();
        let result = tokenizer.pre_tokenize("Hello, world!").unwrap();
        assert!(result.contains(&"Hello".to_string()));
        assert!(result.contains(&",".to_string()));
        assert!(result.contains(&"world".to_string()));
        assert!(result.contains(&"!".to_string()));
    }
}
