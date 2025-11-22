use super::Decoder;
use crate::error::Result;

/// Metaspace decoder
/// Converts metaspace character (▁) back to regular space
#[derive(Debug, Clone)]
pub struct MetaspaceDecoder {
    replacement: char,
    add_prefix_space: bool,
}

impl MetaspaceDecoder {
    pub fn new() -> Self {
        Self {
            replacement: '▁',
            add_prefix_space: true,
        }
    }

    pub fn with_replacement(mut self, replacement: char) -> Self {
        self.replacement = replacement;
        self
    }

    pub fn with_prefix_space(mut self, add_prefix_space: bool) -> Self {
        self.add_prefix_space = add_prefix_space;
        self
    }
}

impl Default for MetaspaceDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for MetaspaceDecoder {
    fn decode(&self, tokens: &[String]) -> Result<String> {
        let joined = tokens.join("");
        let mut result = joined.replace(self.replacement, " ");

        if self.add_prefix_space && result.starts_with(' ') {
            result = result[1..].to_string();
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metaspace_decoder() {
        let decoder = MetaspaceDecoder::new();
        let tokens = vec!["▁Hello".to_string(), "▁world".to_string()];
        let result = decoder.decode(&tokens).unwrap();
        assert_eq!(result.trim(), "Hello world");
    }
}
