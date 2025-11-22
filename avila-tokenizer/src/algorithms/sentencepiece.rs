use crate::algorithms::Unigram;
use crate::error::Result;

/// SentencePiece tokenizer wrapper
/// Combines Unigram with metaspace preprocessing
#[derive(Debug, Clone)]
pub struct SentencePiece {
    /// Underlying Unigram model
    unigram: Unigram,
    /// Use metaspace (▁) instead of regular space
    use_metaspace: bool,
    /// Add dummy prefix space
    add_dummy_prefix: bool,
}

impl SentencePiece {
    /// Create a new SentencePiece tokenizer
    pub fn new(unigram: Unigram) -> Self {
        Self {
            unigram,
            use_metaspace: true,
            add_dummy_prefix: true,
        }
    }

    /// Disable metaspace
    pub fn without_metaspace(mut self) -> Self {
        self.use_metaspace = false;
        self
    }

    /// Disable dummy prefix
    pub fn without_dummy_prefix(mut self) -> Self {
        self.add_dummy_prefix = false;
        self
    }

    /// Preprocess text (add metaspace)
    fn preprocess(&self, text: &str) -> String {
        if !self.use_metaspace {
            return text.to_string();
        }

        let mut result = if self.add_dummy_prefix {
            "▁".to_string()
        } else {
            String::new()
        };

        result.push_str(&text.replace(' ', "▁"));
        result
    }

    /// Postprocess text (remove metaspace)
    fn postprocess(&self, text: &str) -> String {
        if !self.use_metaspace {
            return text.to_string();
        }

        let mut result = text.replace('▁', " ");

        if self.add_dummy_prefix && result.starts_with(' ') {
            result = result[1..].to_string();
        }

        result
    }

    /// Tokenize text
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let preprocessed = self.preprocess(text);
        self.unigram.tokenize(&preprocessed)
    }

    /// Encode text to IDs
    pub fn encode(&self, text: &str) -> Vec<u32> {
        let preprocessed = self.preprocess(text);
        self.unigram.encode(&preprocessed)
    }

    /// Decode IDs to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let text = self.unigram.decode(ids)?;
        Ok(self.postprocess(&text))
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.unigram.vocab_size()
    }

    /// Get underlying Unigram model
    pub fn unigram(&self) -> &Unigram {
        &self.unigram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sp() -> SentencePiece {
        let pieces = vec![
            ("▁hello".to_string(), -1.0),
            ("▁world".to_string(), -1.5),
            ("▁".to_string(), -2.0),
            ("h".to_string(), -3.0),
            ("e".to_string(), -3.0),
            ("l".to_string(), -3.0),
            ("o".to_string(), -3.0),
        ];

        let unigram = Unigram::new(pieces);
        SentencePiece::new(unigram)
    }

    #[test]
    fn test_sp_preprocess() {
        let sp = create_test_sp();
        let preprocessed = sp.preprocess("hello world");
        assert!(preprocessed.contains('▁'));
    }

    #[test]
    fn test_sp_postprocess() {
        let sp = create_test_sp();
        let postprocessed = sp.postprocess("▁hello▁world");
        assert_eq!(postprocessed.trim(), "hello world");
    }

    #[test]
    fn test_sp_encode_decode() {
        let sp = create_test_sp();
        let text = "hello";
        let ids = sp.encode(text);
        let decoded = sp.decode(&ids).unwrap();

        assert!(!ids.is_empty());
        assert!(!decoded.is_empty());
    }
}
