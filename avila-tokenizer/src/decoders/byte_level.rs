use super::Decoder;
use crate::error::{Result, TokenizerError};
use crate::utils::unicode::unicode_to_byte;

/// Byte-level decoder (GPT-2 style)
/// Converts Unicode characters back to bytes
#[derive(Debug, Clone)]
pub struct ByteLevelDecoder {
    byte_decoder: std::collections::HashMap<char, u8>,
}

impl ByteLevelDecoder {
    pub fn new() -> Self {
        Self {
            byte_decoder: unicode_to_byte(),
        }
    }
}

impl Default for ByteLevelDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for ByteLevelDecoder {
    fn decode(&self, tokens: &[String]) -> Result<String> {
        let joined = tokens.join("");
        let bytes: Vec<u8> = joined
            .chars()
            .map(|c| self.byte_decoder.get(&c).copied().unwrap_or(0))
            .collect();

        String::from_utf8(bytes).map_err(|_| TokenizerError::Utf8Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_level_decoder() {
        let decoder = ByteLevelDecoder::new();
        // Basic test - actual usage requires proper byte-encoded tokens
        let result = decoder.decode(&["test".to_string()]);
        assert!(result.is_ok());
    }
}
