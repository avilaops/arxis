use super::PreTokenizer;
use crate::error::Result;

/// Metaspace pre-tokenizer (SentencePiece style)
/// Replaces spaces with a special character (▁)
#[derive(Debug, Clone)]
pub struct Metaspace {
    replacement: char,
    add_prefix_space: bool,
}

impl Metaspace {
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

impl Default for Metaspace {
    fn default() -> Self {
        Self::new()
    }
}

impl PreTokenizer for Metaspace {
    fn pre_tokenize(&self, text: &str) -> Result<Vec<String>> {
        let mut result = text.replace(' ', &self.replacement.to_string());

        if self.add_prefix_space && !result.starts_with(self.replacement) {
            result = format!("{}{}", self.replacement, result);
        }

        Ok(vec![result])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metaspace() {
        let tokenizer = Metaspace::new();
        let result = tokenizer.pre_tokenize("Hello world").unwrap();
        assert_eq!(result[0], "▁Hello▁world");
    }
}
