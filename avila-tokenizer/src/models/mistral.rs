use crate::models::llama::LlamaTokenizer;
use crate::error::Result;

/// Mistral Tokenizer
/// Mistral AI models use the same tokenizer as Llama 2 (SentencePiece with 32,000 tokens)
/// This is a convenience wrapper for better model discovery
pub struct MistralTokenizer {
    inner: LlamaTokenizer,
}

impl MistralTokenizer {
    /// Load Mistral tokenizer from pretrained model
    pub fn from_pretrained(model: &str) -> Result<Self> {
        let inner = LlamaTokenizer::from_pretrained(model)?;
        Ok(Self { inner })
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.inner.vocab_size()
    }

    /// Encode text into token IDs
    pub fn encode(&self, text: &str) -> Vec<u32> {
        self.inner.encode(text)
    }

    /// Encode batch of texts
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<u32>> {
        self.inner.encode_batch(texts)
    }

    /// Decode token IDs back to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        self.inner.decode(ids)
    }

    /// Save vocabulary to file
    pub fn save_vocabulary(&self, vocab_path: &str, scores_path: &str) -> Result<()> {
        self.inner.save_vocabulary(vocab_path, scores_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mistral_7b_creation() {
        let tokenizer = MistralTokenizer::from_pretrained("mistral-7b");
        assert!(tokenizer.is_ok());
    }

    #[test]
    fn test_encode_simple() {
        let tokenizer = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        let ids = tokenizer.encode("Hello, world!");
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let tokenizer = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        let text = "The quick brown fox";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty(), "Encoding should produce tokens");
        
        let decoded = tokenizer.decode(&ids);
        assert!(decoded.is_ok(), "Decoding should succeed");
    }

    #[test]
    fn test_batch_encoding() {
        let tokenizer = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        let texts = vec!["Hello", "World", "Test"];
        let batch_ids = tokenizer.encode_batch(&texts);
        assert_eq!(batch_ids.len(), 3);
    }
}
