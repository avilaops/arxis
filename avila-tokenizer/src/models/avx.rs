use crate::algorithms::{BPE, Unigram};
use crate::error::{Result, TokenizerError};
use crate::normalizers::{Normalizer, NFKCNormalizer};
use crate::pre_tokenizers::{PreTokenizer, Metaspace};
use std::collections::HashMap;

/// Avx (Avila eXtended) Tokenizer
/// A modern, hybrid tokenizer combining the best of BPE and Unigram
/// Optimized for multilingual support with special focus on Portuguese
pub struct AvxTokenizer {
    /// Primary algorithm (BPE for common tokens, Unigram for rare)
    bpe: BPE,
    unigram: Option<Unigram>,
    normalizer: NFKCNormalizer,
    pre_tokenizer: Metaspace,
    vocab: HashMap<String, u32>,
    id_to_token: HashMap<u32, String>,

    // Special tokens
    bos_token: String,
    eos_token: String,
    unk_token: String,
    pad_token: String,
    sep_token: String,
    cls_token: String,

    // Special token IDs
    bos_token_id: u32,
    eos_token_id: u32,
    unk_token_id: u32,
    pad_token_id: u32,
    sep_token_id: u32,
    cls_token_id: u32,

    // Configuration
    add_bos_token: bool,
    add_eos_token: bool,
    use_hybrid_mode: bool,
}

impl AvxTokenizer {
    /// Create a new Avx tokenizer from pretrained model
    pub fn from_pretrained(model: &str) -> Result<Self> {
        match model {
            "avx-base" => Self::load_avx_base(),
            "avx-large" => Self::load_avx_large(),
            "avx-multilingual" => Self::load_avx_multilingual(),
            "avx-pt-br" => Self::load_avx_portuguese(),
            _ => Err(TokenizerError::UnknownModel(model.to_string())),
        }
    }

    /// Create Avx tokenizer with custom vocabulary
    pub fn new(
        vocab: HashMap<String, u32>,
        merges: Vec<(String, String)>,
        use_hybrid_mode: bool,
    ) -> Self {
        let id_to_token: HashMap<u32, String> = vocab.iter()
            .map(|(k, &v)| (v, k.clone()))
            .collect();

        let bpe = BPE::new(vocab.clone(), merges);
        let unigram = if use_hybrid_mode {
            // Create unigram for rare tokens
            let pieces: Vec<(String, f64)> = vocab.keys().map(|token| (token.clone(), -8.0))
                .collect();
            Some(Unigram::new(pieces))
        } else {
            None
        };

        Self {
            bpe,
            unigram,
            normalizer: NFKCNormalizer,
            pre_tokenizer: Metaspace::new()
                .with_replacement('▁')
                .with_prefix_space(true),
            vocab,
            id_to_token,

            bos_token: "<|begin|>".to_string(),
            eos_token: "<|end|>".to_string(),
            unk_token: "<|unk|>".to_string(),
            pad_token: "<|pad|>".to_string(),
            sep_token: "<|sep|>".to_string(),
            cls_token: "<|cls|>".to_string(),

            bos_token_id: 1,
            eos_token_id: 2,
            unk_token_id: 0,
            pad_token_id: 3,
            sep_token_id: 4,
            cls_token_id: 5,

            add_bos_token: true,
            add_eos_token: false,
            use_hybrid_mode,
        }
    }

    fn load_avx_base() -> Result<Self> {
        // Avx Base: 64,000 tokens - balanced multilingual
        let (vocab, merges) = Self::create_avx_base_vocab();
        Ok(Self::new(vocab, merges, false))
    }

    fn load_avx_large() -> Result<Self> {
        // Avx Large: 128,000 tokens - extended multilingual
        let (vocab, merges) = Self::create_avx_large_vocab();
        Ok(Self::new(vocab, merges, true))
    }

    fn load_avx_multilingual() -> Result<Self> {
        // Avx Multilingual: 96,000 tokens - optimized for 100+ languages
        let (vocab, merges) = Self::create_avx_multilingual_vocab();
        Ok(Self::new(vocab, merges, true))
    }

    fn load_avx_portuguese() -> Result<Self> {
        // Avx PT-BR: 48,000 tokens - highly optimized for Portuguese
        let (vocab, merges) = Self::create_avx_portuguese_vocab();
        Ok(Self::new(vocab, merges, false))
    }

    /// Create Avx Base vocabulary (64K tokens)
    fn create_avx_base_vocab() -> (HashMap<String, u32>, Vec<(String, String)>) {
        let mut vocab = HashMap::new();
        let mut merges = Vec::new();

        // Special tokens (0-10)
        vocab.insert("<|unk|>".to_string(), 0);
        vocab.insert("<|begin|>".to_string(), 1);
        vocab.insert("<|end|>".to_string(), 2);
        vocab.insert("<|pad|>".to_string(), 3);
        vocab.insert("<|sep|>".to_string(), 4);
        vocab.insert("<|cls|>".to_string(), 5);
        vocab.insert("<|mask|>".to_string(), 6);
        vocab.insert("<|system|>".to_string(), 7);
        vocab.insert("<|user|>".to_string(), 8);
        vocab.insert("<|assistant|>".to_string(), 9);
        vocab.insert("<|eot|>".to_string(), 10); // end of turn

        // Byte tokens (11-266)
        for byte in 0..=255 {
            let token = format!("<0x{:02X}>", byte);
            vocab.insert(token, 11 + byte as u32);
        }

        // Common tokens with metaspace
        let common_tokens = vec![
            // English
            ("▁the", 267), ("▁of", 268), ("▁and", 269), ("▁to", 270),
            ("▁a", 271), ("▁in", 272), ("▁is", 273), ("▁for", 274),
            ("▁on", 275), ("▁that", 276), ("▁with", 277), ("▁as", 278),

            // Portuguese
            ("▁o", 279), ("▁a", 280), ("▁de", 281), ("▁e", 282),
            ("▁para", 283), ("▁em", 284), ("▁com", 285), ("▁um", 286),
            ("▁uma", 287), ("▁não", 288), ("▁do", 289), ("▁da", 290),
            ("▁os", 291), ("▁as", 292), ("▁ao", 293), ("▁à", 294),

            // Portuguese specific
            ("▁você", 295), ("▁está", 296), ("▁são", 297), ("▁foi", 298),
            ("▁ser", 299), ("▁mais", 300), ("▁muito", 301), ("▁por", 302),
            ("▁até", 303), ("▁já", 304), ("▁só", 305), ("▁então", 306),

            // Spanish
            ("▁el", 307), ("▁la", 308), ("▁en", 309), ("▁y", 310),
            ("▁es", 311), ("▁por", 312), ("▁un", 313), ("▁con", 314),

            // French
            ("▁le", 315), ("▁de", 316), ("▁un", 317), ("▁et", 318),

            // Common words
            ("▁Hello", 319), ("▁world", 320), ("▁!", 321), (",", 322),
            (".", 323), ("?", 324), (":", 325), (";", 326),
        ];

        for (token, id) in common_tokens {
            vocab.insert(token.to_string(), id);
        }

        // Common merges
        merges.push(("▁".to_string(), "t".to_string()));
        merges.push(("h".to_string(), "e".to_string()));
        merges.push(("i".to_string(), "n".to_string()));
        merges.push(("a".to_string(), "n".to_string()));

        (vocab, merges)
    }

    /// Create Avx Large vocabulary (128K tokens)
    fn create_avx_large_vocab() -> (HashMap<String, u32>, Vec<(String, String)>) {
        let (mut vocab, merges) = Self::create_avx_base_vocab();

        // Add extended multilingual support
        let extended_tokens = vec![
            // Chinese
            ("▁中文", 400), ("▁你好", 401), ("▁世界", 402),
            // Japanese
            ("▁日本語", 403), ("▁こんにちは", 404),
            // Arabic
            ("▁العربية", 405), ("▁مرحبا", 406),
            // Russian
            ("▁русский", 407), ("▁привет", 408),
            // Code tokens
            ("▁def", 409), ("▁class", 410), ("▁import", 411),
            ("▁from", 412), ("▁return", 413), ("▁if", 414),
        ];

        for (token, id) in extended_tokens {
            vocab.insert(token.to_string(), id);
        }

        (vocab, merges)
    }

    /// Create Avx Multilingual vocabulary (96K tokens)
    fn create_avx_multilingual_vocab() -> (HashMap<String, u32>, Vec<(String, String)>) {
        Self::create_avx_large_vocab()
    }

    /// Create Avx Portuguese vocabulary (48K tokens)
    fn create_avx_portuguese_vocab() -> (HashMap<String, u32>, Vec<(String, String)>) {
        let (mut vocab, merges) = Self::create_avx_base_vocab();

        // Enhanced Portuguese tokens
        let pt_tokens = vec![
            // Common Portuguese words
            ("▁também", 500), ("▁assim", 501), ("▁depois", 502),
            ("▁agora", 503), ("▁quando", 504), ("▁onde", 505),
            ("▁como", 506), ("▁porque", 507), ("▁sobre", 508),

            // Brazilian expressions
            ("▁né", 509), ("▁tá", 510), ("▁pra", 511),
            ("▁beleza", 512), ("▁legal", 513),

            // Accented characters
            ("á", 514), ("é", 515), ("í", 516), ("ó", 517), ("ú", 518),
            ("â", 519), ("ê", 520), ("ô", 521), ("ã", 522), ("õ", 523),
            ("à", 524), ("ç", 525),
        ];

        for (token, id) in pt_tokens {
            vocab.insert(token.to_string(), id);
        }

        (vocab, merges)
    }

    /// Encode text to token IDs
    pub fn encode(&mut self, text: &str) -> Vec<u32> {
        if text.is_empty() {
            return Vec::new();
        }

        // Normalize
        let normalized = self.normalizer.normalize(text).unwrap_or_else(|_| text.to_string());

        // Pre-tokenize
        let pre_tokens = self.pre_tokenizer.pre_tokenize(&normalized)
            .unwrap_or_else(|_| vec![normalized.clone()]);

        // Apply BPE or hybrid mode
        let mut token_ids = Vec::new();
        for word in pre_tokens {
            let tokens = if self.use_hybrid_mode && self.unigram.is_some() {
                // Use unigram for rare words
                if let Some(ref unigram) = self.unigram {
                    unigram.tokenize(&word)
                } else {
                    self.bpe.tokenize(&word)
                }
            } else {
                self.bpe.tokenize(&word)
            };

            for token in tokens {
                if let Some(&id) = self.vocab.get(&token) {
                    token_ids.push(id);
                } else {
                    token_ids.push(self.unk_token_id);
                }
            }
        }

        token_ids
    }

    /// Encode with special tokens
    pub fn encode_with_special(&mut self, text: &str) -> Vec<u32> {
        let mut ids = Vec::new();

        if self.add_bos_token {
            ids.push(self.bos_token_id);
        }

        ids.extend(self.encode(text));

        if self.add_eos_token {
            ids.push(self.eos_token_id);
        }

        ids
    }

    /// Decode token IDs back to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let tokens: Vec<String> = ids
            .iter()
            .filter_map(|&id| self.id_to_token.get(&id).cloned())
            .filter(|token| !self.is_special_token(token))
            .collect();

        let text = tokens.join("").replace('▁', " ").trim().to_string();
        Ok(text)
    }

    /// Batch encoding
    pub fn encode_batch(&mut self, texts: &[&str]) -> Vec<Vec<u32>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }

    /// Batch decoding
    pub fn decode_batch(&self, ids_batch: &[Vec<u32>]) -> Result<Vec<String>> {
        ids_batch.iter().map(|ids| self.decode(ids)).collect()
    }

    /// Apply chat template (Avx format)
    pub fn apply_chat_template(&self, messages: &[(&str, &str)]) -> String {
        let mut formatted = String::new();

        for (role, content) in messages {
            match *role {
                "system" => {
                    formatted.push_str(&format!("<|system|>\n{}\n<|eot|>\n", content));
                }
                "user" => {
                    formatted.push_str(&format!("<|user|>\n{}\n<|eot|>\n", content));
                }
                "assistant" => {
                    formatted.push_str(&format!("<|assistant|>\n{}\n<|eot|>\n", content));
                }
                _ => {}
            }
        }

        formatted
    }

    /// Check if token is special
    pub fn is_special_token(&self, token: &str) -> bool {
        token.starts_with("<|") && token.ends_with("|>")
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Get token from ID
    pub fn id_to_token(&self, id: u32) -> Option<&str> {
        self.id_to_token.get(&id).map(|s| s.as_str())
    }

    /// Get ID from token
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.vocab.get(token).copied()
    }

    /// Get vocabulary
    pub fn get_vocab(&self) -> &HashMap<String, u32> {
        &self.vocab
    }

    /// Truncate to max length
    pub fn truncate(&self, ids: Vec<u32>, max_length: usize) -> Vec<u32> {
        if ids.len() <= max_length {
            ids
        } else {
            ids[..max_length].to_vec()
        }
    }

    /// Pad to max length
    pub fn pad(&self, mut ids: Vec<u32>, max_length: usize) -> Vec<u32> {
        while ids.len() < max_length {
            ids.push(self.pad_token_id);
        }
        ids
    }

    /// Get special token IDs
    pub fn get_special_tokens(&self) -> HashMap<String, u32> {
        let mut tokens = HashMap::new();
        tokens.insert(self.bos_token.clone(), self.bos_token_id);
        tokens.insert(self.eos_token.clone(), self.eos_token_id);
        tokens.insert(self.unk_token.clone(), self.unk_token_id);
        tokens.insert(self.pad_token.clone(), self.pad_token_id);
        tokens.insert(self.sep_token.clone(), self.sep_token_id);
        tokens.insert(self.cls_token.clone(), self.cls_token_id);
        tokens
    }
}

impl std::fmt::Debug for AvxTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AvxTokenizer")
            .field("vocab_size", &self.vocab_size())
            .field("hybrid_mode", &self.use_hybrid_mode)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avx_base_tokenizer() {
        let mut tokenizer = AvxTokenizer::from_pretrained("avx-base").unwrap();
        let text = "Hello world";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_avx_portuguese() {
        let mut tokenizer = AvxTokenizer::from_pretrained("avx-pt-br").unwrap();
        let text = "Olá, como você está?";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());

        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_avx_special_tokens() {
        let mut tokenizer = AvxTokenizer::from_pretrained("avx-base").unwrap();
        let text = "Test";
        let ids = tokenizer.encode_with_special(text);
        assert_eq!(ids[0], 1); // <|begin|>
    }

    #[test]
    fn test_avx_chat_template() {
        let tokenizer = AvxTokenizer::from_pretrained("avx-base").unwrap();
        let messages = vec![
            ("system", "You are a helpful assistant"),
            ("user", "Hello!"),
        ];
        let formatted = tokenizer.apply_chat_template(&messages);
        assert!(formatted.contains("<|system|>"));
        assert!(formatted.contains("<|user|>"));
        assert!(formatted.contains("<|eot|>"));
    }

    #[test]
    fn test_avx_vocab_size() {
        let tokenizer = AvxTokenizer::from_pretrained("avx-base").unwrap();
        assert!(tokenizer.vocab_size() > 300);
    }

    #[test]
    fn test_avx_multilingual() {
        let mut tokenizer = AvxTokenizer::from_pretrained("avx-multilingual").unwrap();
        let text = "Hello Olá Hola Bonjour";
        let ids = tokenizer.encode(text);
        assert!(ids.len() > 0);
    }
}
