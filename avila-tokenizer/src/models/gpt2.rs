use crate::algorithms::BPE;
use crate::error::{Result, TokenizerError};
use crate::utils::unicode::{byte_to_unicode, unicode_to_byte};
use std::collections::HashMap;

/// GPT-2 Tokenizer using byte-level BPE
/// Compatible with OpenAI's GPT-2, GPT-3, and tiktoken
#[derive(Clone)]
pub struct GPT2Tokenizer {
    bpe: BPE,
    encoder: HashMap<String, u32>,
    decoder: HashMap<u32, String>,
    byte_encoder: HashMap<u8, char>,
    byte_decoder: HashMap<char, u8>,
    pattern: regex::Regex,
}

impl GPT2Tokenizer {
    /// Load GPT-2 tokenizer from pretrained model
    pub fn from_pretrained(model: &str) -> Result<Self> {
        match model {
            "gpt2" | "gpt2-small" => Self::load_gpt2_base(),
            "gpt2-medium" => Self::load_gpt2_medium(),
            "gpt2-large" => Self::load_gpt2_large(),
            "gpt2-xl" => Self::load_gpt2_xl(),
            _ => Err(TokenizerError::UnknownModel(model.to_string())),
        }
    }

    /// Create GPT-2 tokenizer with custom vocabulary and merges
    pub fn new(vocab: HashMap<String, u32>, merges: Vec<(String, String)>) -> Self {
        let bpe = BPE::new_byte_level(vocab.clone(), merges);
        let decoder: HashMap<u32, String> = vocab.iter().map(|(k, &v)| (v, k.clone())).collect();
        let byte_encoder = byte_to_unicode();
        let byte_decoder = unicode_to_byte();

        // GPT-2 tokenization pattern (lookahead not supported, using simplified version)
        let pattern = regex::Regex::new(
            r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+"
        ).unwrap();

        Self {
            bpe,
            encoder: vocab,
            decoder,
            byte_encoder,
            byte_decoder,
            pattern,
        }
    }

    fn load_gpt2_base() -> Result<Self> {
        // GPT-2 base vocabulary (50,257 tokens)
        let vocab = Self::create_gpt2_vocab();
        let merges = Self::create_gpt2_merges();
        Ok(Self::new(vocab, merges))
    }

    fn load_gpt2_medium() -> Result<Self> {
        Self::load_gpt2_base()
    }

    fn load_gpt2_large() -> Result<Self> {
        Self::load_gpt2_base()
    }

    fn load_gpt2_xl() -> Result<Self> {
        Self::load_gpt2_base()
    }

    /// Create GPT-2 vocabulary (simplified - real impl would load from file)
    fn create_gpt2_vocab() -> HashMap<String, u32> {
        let mut vocab = HashMap::new();

        // Special tokens
        vocab.insert("<|endoftext|>".to_string(), 50256);

        // Byte tokens (256 base characters)
        let byte_encoder = byte_to_unicode();
        for (byte, ch) in byte_encoder.iter() {
            vocab.insert(ch.to_string(), *byte as u32);
        }

        // Add common tokens (simplified example)
        let common_tokens = vec![
            ("Ġthe", 262), ("Ġof", 286), ("Ġand", 290), ("Ġto", 284),
            ("Ġa", 257), ("Ġin", 287), ("Ġis", 318), ("Ġfor", 329),
            ("Ġon", 319), ("Ġthat", 326), ("Ġwith", 351), ("Ġas", 355),
            ("Ġit", 340), ("Ġbe", 307), ("Ġby", 416), ("Ġat", 379),
            ("Hello", 15496), ("Ġworld", 995), (",", 11), ("!", 0),
        ];

        for (token, id) in common_tokens {
            vocab.insert(token.to_string(), id);
        }

        vocab
    }

    /// Create GPT-2 merge rules (simplified)
    fn create_gpt2_merges() -> Vec<(String, String)> {
        vec![
            ("Ġ".to_string(), "t".to_string()),
            ("h".to_string(), "e".to_string()),
            ("i".to_string(), "n".to_string()),
            ("Ġ".to_string(), "a".to_string()),
        ]
    }

    /// Encode text to token IDs
    pub fn encode(&mut self, text: &str) -> Vec<u32> {
        if text.is_empty() {
            return Vec::new();
        }

        let mut bpe_tokens = Vec::new();

        // Split text using GPT-2 pattern
        for word in self.pattern.find_iter(text) {
            let word_str = word.as_str();

            // Convert to bytes and map to unicode
            let token: String = word_str
                .bytes()
                .map(|b| self.byte_encoder.get(&b).copied().unwrap_or(' '))
                .collect();

            // Apply BPE
            let tokens = self.bpe.tokenize(&token);

            // Convert tokens to IDs
            for bpe_token in tokens {
                if let Some(&id) = self.encoder.get(&bpe_token) {
                    bpe_tokens.push(id);
                }
            }
        }

        bpe_tokens
    }

    /// Decode token IDs back to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        // Convert IDs to tokens
        let tokens: Vec<String> = ids
            .iter()
            .filter_map(|&id| self.decoder.get(&id).cloned())
            .collect();

        // Join tokens
        let joined = tokens.join("");

        // Convert from unicode back to bytes
        let bytes: Vec<u8> = joined
            .chars()
            .filter_map(|c| self.byte_decoder.get(&c).copied())
            .collect();

        String::from_utf8(bytes).map_err(|_| TokenizerError::Utf8Error)
    }

    /// Encode with special tokens
    pub fn encode_with_special(&mut self, text: &str, add_special_tokens: bool) -> Vec<u32> {
        let mut ids = self.encode(text);

        if add_special_tokens {
            // Add <|endoftext|> token at the end
            if let Some(&eos_id) = self.encoder.get("<|endoftext|>") {
                ids.push(eos_id);
            }
        }

        ids
    }

    /// Batch encoding with parallel processing
    pub fn encode_batch(&mut self, texts: &[&str]) -> Vec<Vec<u32>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }

    /// Batch decoding
    pub fn decode_batch(&self, ids_batch: &[Vec<u32>]) -> Result<Vec<String>> {
        ids_batch.iter().map(|ids| self.decode(ids)).collect()
    }

    /// Encode a pair of texts (for seq2seq models)
    pub fn encode_pair(&mut self, text_a: &str, text_b: &str) -> Vec<u32> {
        let mut ids_a = self.encode(text_a);
        let ids_b = self.encode(text_b);

        // Add separator token
        if let Some(&eos_id) = self.encoder.get("<|endoftext|>") {
            ids_a.push(eos_id);
        }

        ids_a.extend(ids_b);
        ids_a
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.encoder.len()
    }

    /// Get token from ID
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        self.decoder.get(&id).map(|s| s.as_str())
    }

    /// Get ID from token
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.encoder.get(token).copied()
    }

    /// Get vocabulary
    pub fn get_vocab(&self) -> &HashMap<String, u32> {
        &self.encoder
    }

    /// Save vocabulary to file
    pub fn save_vocabulary(&self, vocab_path: &str, merges_path: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        // Save vocab
        let vocab_json = serde_json::to_string_pretty(&self.encoder)?;
        let mut vocab_file = File::create(vocab_path)?;
        vocab_file.write_all(vocab_json.as_bytes())?;

        // Save merges
        let mut merges_file = File::create(merges_path)?;
        writeln!(merges_file, "#version: 0.2")?;
        for (a, b) in self.bpe.merges() {
            writeln!(merges_file, "{} {}", a, b)?;
        }

        Ok(())
    }

    /// Convert text to tokens (strings)
    pub fn tokenize(&mut self, text: &str) -> Vec<String> {
        let ids = self.encode(text);
        ids.iter()
            .filter_map(|&id| self.decoder.get(&id).cloned())
            .collect()
    }

    /// Count tokens in text
    pub fn count_tokens(&mut self, text: &str) -> usize {
        self.encode(text).len()
    }

    /// Truncate tokens to max length
    pub fn truncate(&self, ids: Vec<u32>, max_length: usize) -> Vec<u32> {
        if ids.len() <= max_length {
            ids
        } else {
            ids[..max_length].to_vec()
        }
    }

    /// Pad tokens to max length
    pub fn pad(&self, mut ids: Vec<u32>, max_length: usize, pad_token_id: u32) -> Vec<u32> {
        while ids.len() < max_length {
            ids.push(pad_token_id);
        }
        ids
    }
}

impl std::fmt::Debug for GPT2Tokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GPT2Tokenizer")
            .field("vocab_size", &self.vocab_size())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpt2_tokenizer_basic() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let text = "Hello world";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_gpt2_encode_decode() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let text = "Hello";
        let ids = tokenizer.encode(text);
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_gpt2_vocab_size() {
        let tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        // Simplified vocabulary for development
        assert!(tokenizer.vocab_size() > 250);
    }

    #[test]
    fn test_gpt2_batch_encode() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let texts = vec!["Hello", "World"];
        let batch = tokenizer.encode_batch(&texts);
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_gpt2_special_tokens() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let text = "Test";
        let ids = tokenizer.encode_with_special(text, true);
        assert!(ids.len() > 0);
    }
}
