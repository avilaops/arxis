use super::Decoder;
use crate::error::Result;

/// WordPiece decoder
/// Removes ## prefixes and joins tokens
#[derive(Debug, Clone)]
pub struct WordPieceDecoder {
    prefix: String,
    cleanup: bool,
}

impl WordPieceDecoder {
    pub fn new() -> Self {
        Self {
            prefix: "##".to_string(),
            cleanup: true,
        }
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn with_cleanup(mut self, cleanup: bool) -> Self {
        self.cleanup = cleanup;
        self
    }
}

impl Default for WordPieceDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for WordPieceDecoder {
    fn decode(&self, tokens: &[String]) -> Result<String> {
        let mut result = String::new();

        for (i, token) in tokens.iter().enumerate() {
            if token.starts_with(&self.prefix) {
                result.push_str(&token[self.prefix.len()..]);
            } else {
                if i > 0 && self.cleanup {
                    result.push(' ');
                }
                result.push_str(token);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordpiece_decoder() {
        let decoder = WordPieceDecoder::new();
        let tokens = vec!["hello".to_string(), "##ing".to_string(), "world".to_string()];
        let result = decoder.decode(&tokens).unwrap();
        assert_eq!(result, "helloing world");
    }
}
