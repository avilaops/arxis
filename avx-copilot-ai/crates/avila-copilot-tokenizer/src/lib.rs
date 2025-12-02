// Layer 2: Tokenizer & Embeddings
// High-performance code-aware tokenization

use std::collections::HashMap;
use std::sync::Arc;

pub mod embeddings;
pub mod error;
pub mod vocabulary;

pub use error::{Result, TokenizerError};

/// Code-aware tokenizer optimized for programming languages
pub struct CopilotTokenizer {
    vocab: HashMap<String, u32>,
    reverse_vocab: HashMap<u32, String>,
    vocabulary: vocabulary::CodeVocabulary,
    max_length: usize,
}

impl CopilotTokenizer {
    /// Create new tokenizer
    pub fn new() -> Result<Self> {
        let vocabulary = vocabulary::CodeVocabulary::new();
        let (vocab, reverse_vocab) = Self::build_vocab();

        Ok(Self {
            vocab,
            reverse_vocab,
            vocabulary,
            max_length: 4096,
        })
    }

    fn build_vocab() -> (HashMap<String, u32>, HashMap<u32, String>) {
        let mut vocab = HashMap::new();
        let mut reverse = HashMap::new();

        // Add basic tokens
        let tokens = vec![
            "<PAD>", "<BOS>", "<EOS>", "<UNK>", "<MASK>",
            "fn", "let", "mut", "const", "struct", "enum", "impl", "trait",
            "for", "while", "if", "else", "match", "return",
            "pub", "async", "await", "use", "mod",
            "(", ")", "{", "}", "[", "]", ";", ":", ",", ".", "=", "+", "-", "*", "/",
        ];

        for (idx, token) in tokens.iter().enumerate() {
            vocab.insert(token.to_string(), idx as u32);
            reverse.insert(idx as u32, token.to_string());
        }

        (vocab, reverse)
    }

    /// Encode text to token IDs
    pub fn encode(&self, text: &str) -> Result<Vec<u32>> {
        // Simple word-level tokenization
        let preprocessed = self.preprocess_code(text);
        let mut tokens = Vec::new();

        for word in preprocessed.split_whitespace() {
            let token_id = self.vocab.get(word).copied().unwrap_or(3); // <UNK>
            tokens.push(token_id);
        }

        // Truncate if needed
        if tokens.len() > self.max_length {
            tokens.truncate(self.max_length);
        }

        Ok(tokens)
    }

    /// Decode token IDs to text
    pub fn decode(&self, tokens: &[u32]) -> Result<String> {
        let words: Vec<String> = tokens
            .iter()
            .filter_map(|&id| self.reverse_vocab.get(&id).cloned())
            .collect();
        Ok(words.join(" "))
    }

    /// Encode with attention mask
    pub fn encode_with_mask(&self, text: &str) -> Result<EncodedInput> {
        let tokens = self.encode(text)?;
        let attention_mask = vec![1u8; tokens.len()];

        Ok(EncodedInput {
            input_ids: tokens,
            attention_mask,
        })
    }

    /// Batch encode multiple texts
    pub fn batch_encode(&self, texts: &[&str]) -> Result<Vec<Vec<u32>>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Preprocess code for better tokenization
    fn preprocess_code(&self, code: &str) -> String {
        // Remove excessive whitespace
        let mut result = String::with_capacity(code.len());
        let mut prev_whitespace = false;

        for ch in code.chars() {
            if ch.is_whitespace() {
                if !prev_whitespace {
                    result.push(' ');
                    prev_whitespace = true;
                }
            } else {
                result.push(ch);
                prev_whitespace = false;
            }
        }

        result
    }

    /// Get special tokens for code
    fn get_special_tokens() -> Vec<String> {
        vec![
            "[PAD]".to_string(),
            "[UNK]".to_string(),
            "[CLS]".to_string(),
            "[SEP]".to_string(),
            "[MASK]".to_string(),
            "[BOS]".to_string(),
            "[EOS]".to_string(),
            // Code-specific tokens
            "[FUNC]".to_string(),
            "[CLASS]".to_string(),
            "[VAR]".to_string(),
            "[COMMENT]".to_string(),
            "[STRING]".to_string(),
            "[NUMBER]".to_string(),
        ]
    }
}

/// Encoded input with attention mask
#[derive(Debug, Clone)]
pub struct EncodedInput {
    pub input_ids: Vec<u32>,
    pub attention_mask: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_encode_decode() {
        let tokenizer = CopilotTokenizer::new().unwrap();
        let code = "fn main() { println!(\"Hello\"); }";

        let tokens = tokenizer.encode(code).unwrap();
        assert!(!tokens.is_empty());

        let decoded = tokenizer.decode(&tokens).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_batch_encode() {
        let tokenizer = CopilotTokenizer::new().unwrap();
        let codes = vec![
            "fn foo() {}",
            "fn bar() {}",
            "fn baz() {}",
        ];

        let batch = tokenizer.batch_encode(&codes).unwrap();
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_preprocess_code() {
        let tokenizer = CopilotTokenizer::new().unwrap();
        let code = "fn   main()   {  \n  println!(\"test\");  \n}";
        let preprocessed = tokenizer.preprocess_code(code);

        // Should reduce multiple spaces to single space
        assert!(!preprocessed.contains("  "));
    }
}
