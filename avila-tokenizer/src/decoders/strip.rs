use super::Decoder;
use crate::error::Result;
use std::collections::HashSet;

/// Strip decoder
/// Removes special tokens from decoded text
#[derive(Debug, Clone)]
pub struct StripDecoder {
    special_tokens: HashSet<String>,
}

impl StripDecoder {
    pub fn new(special_tokens: Vec<String>) -> Self {
        Self {
            special_tokens: special_tokens.into_iter().collect(),
        }
    }
}

impl Decoder for StripDecoder {
    fn decode(&self, tokens: &[String]) -> Result<String> {
        let filtered: Vec<&String> = tokens
            .iter()
            .filter(|token| !self.special_tokens.contains(*token))
            .collect();

        Ok(filtered
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_decoder() {
        let decoder = StripDecoder::new(vec!["[CLS]".to_string(), "[SEP]".to_string()]);
        let tokens = vec![
            "[CLS]".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "[SEP]".to_string(),
        ];
        let result = decoder.decode(&tokens).unwrap();
        assert_eq!(result, "hello world");
    }
}
