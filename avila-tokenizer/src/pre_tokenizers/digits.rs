use super::PreTokenizer;
use crate::error::Result;

/// Digits splitter - separates digit sequences
#[derive(Debug, Clone, Copy)]
pub struct DigitsSplit {
    individual_digits: bool,
}

impl DigitsSplit {
    pub fn new() -> Self {
        Self {
            individual_digits: false,
        }
    }

    pub fn with_individual_digits(mut self, individual: bool) -> Self {
        self.individual_digits = individual;
        self
    }
}

impl Default for DigitsSplit {
    fn default() -> Self {
        Self::new()
    }
}

impl PreTokenizer for DigitsSplit {
    fn pre_tokenize(&self, text: &str) -> Result<Vec<String>> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_digits = false;

        for ch in text.chars() {
            let is_digit = ch.is_ascii_digit();

            if self.individual_digits && is_digit {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(ch.to_string());
                in_digits = false;
            } else if is_digit != in_digits && !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
                current.push(ch);
                in_digits = is_digit;
            } else {
                current.push(ch);
                in_digits = is_digit;
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
    fn test_digits_split() {
        let tokenizer = DigitsSplit::new();
        let result = tokenizer.pre_tokenize("test123hello456").unwrap();
        assert!(result.contains(&"test".to_string()));
        assert!(result.contains(&"123".to_string()));
        assert!(result.contains(&"hello".to_string()));
        assert!(result.contains(&"456".to_string()));
    }

    #[test]
    fn test_individual_digits() {
        let tokenizer = DigitsSplit::new().with_individual_digits(true);
        let result = tokenizer.pre_tokenize("test123").unwrap();
        assert!(result.contains(&"1".to_string()));
        assert!(result.contains(&"2".to_string()));
        assert!(result.contains(&"3".to_string()));
    }
}
