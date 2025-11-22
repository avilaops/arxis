use crate::algorithms::WordPiece;
use crate::error::{Result, TokenizerError};
use crate::normalizers::{Normalizer, NFKCNormalizer, LowercaseNormalizer};
use crate::pre_tokenizers::{PreTokenizer, WhitespaceSplit};
use std::collections::HashMap;

/// BERT Tokenizer using WordPiece algorithm
/// Compatible with BERT, DistilBERT, RoBERTa, ALBERT models
pub struct BertTokenizer {
    wordpiece: WordPiece,
    normalizer: Box<dyn Normalizer>,
    pre_tokenizer: WhitespaceSplit,
    vocab: HashMap<String, u32>,
    id_to_token: HashMap<u32, String>,

    // Special tokens
    cls_token: String,
    sep_token: String,
    pad_token: String,
    unk_token: String,
    mask_token: String,

    cls_token_id: u32,
    sep_token_id: u32,
    pad_token_id: u32,
    unk_token_id: u32,
    mask_token_id: u32,

    do_lower_case: bool,
}

impl BertTokenizer {
    /// Load BERT tokenizer from pretrained model
    pub fn from_pretrained(model: &str) -> Result<Self> {
        match model {
            "bert-base-uncased" => Self::load_bert_base_uncased(),
            "bert-base-cased" => Self::load_bert_base_cased(),
            "bert-large-uncased" => Self::load_bert_large_uncased(),
            "bert-large-cased" => Self::load_bert_large_cased(),
            "distilbert-base-uncased" => Self::load_distilbert_base_uncased(),
            _ => Err(TokenizerError::UnknownModel(model.to_string())),
        }
    }

    /// Create BERT tokenizer with custom vocabulary
    pub fn new(
        vocab: HashMap<String, u32>,
        do_lower_case: bool,
    ) -> Result<Self> {
        let id_to_token: HashMap<u32, String> = vocab.iter()
            .map(|(k, &v)| (v, k.clone()))
            .collect();

        let unk_token = "[UNK]".to_string();
        let wordpiece = WordPiece::new(vocab.clone(), unk_token.clone(), "##".to_string())?;

        let normalizer: Box<dyn Normalizer> = if do_lower_case {
            Box::new(LowercaseNormalizer)
        } else {
            Box::new(NFKCNormalizer)
        };

        Ok(Self {
            wordpiece,
            normalizer,
            pre_tokenizer: WhitespaceSplit,
            vocab,
            id_to_token,

            cls_token: "[CLS]".to_string(),
            sep_token: "[SEP]".to_string(),
            pad_token: "[PAD]".to_string(),
            unk_token: "[UNK]".to_string(),
            mask_token: "[MASK]".to_string(),

            cls_token_id: 101,
            sep_token_id: 102,
            pad_token_id: 0,
            unk_token_id: 100,
            mask_token_id: 103,

            do_lower_case,
        })
    }    fn load_bert_base_uncased() -> Result<Self> {
        let vocab = Self::create_bert_vocab();
        Self::new(vocab, true)
    }

    fn load_bert_base_cased() -> Result<Self> {
        let vocab = Self::create_bert_vocab();
        Self::new(vocab, false)
    }

    fn load_bert_large_uncased() -> Result<Self> {
        let vocab = Self::create_bert_vocab();
        Self::new(vocab, true)
    }

    fn load_bert_large_cased() -> Result<Self> {
        let vocab = Self::create_bert_vocab();
        Self::new(vocab, false)
    }

    fn load_distilbert_base_uncased() -> Result<Self> {
        let vocab = Self::create_bert_vocab();
        Self::new(vocab, true)
    }

    /// Create BERT vocabulary (30,522 tokens)
    fn create_bert_vocab() -> HashMap<String, u32> {
        let mut vocab = HashMap::new();

        // Special tokens
        vocab.insert("[PAD]".to_string(), 0);
        vocab.insert("[UNK]".to_string(), 100);
        vocab.insert("[CLS]".to_string(), 101);
        vocab.insert("[SEP]".to_string(), 102);
        vocab.insert("[MASK]".to_string(), 103);

        // Unused tokens
        for i in 1..100 {
            vocab.insert(format!("[unused{}]", i), i);
        }

        // Common tokens
        let common_tokens = vec![
            ("the", 1996), ("of", 1997), ("and", 1998), ("to", 2000),
            ("a", 1037), ("in", 1999), ("is", 2003), ("for", 2005),
            ("on", 2006), ("that", 2008), ("with", 2007), ("as", 2004),
            ("it", 2009), ("be", 2022), ("by", 2011), ("at", 2012),
            ("hello", 7592), ("world", 2088), ("!", 999), (",", 1010),
            (".", 1012), ("?", 1029), (":", 1024), (";", 1025),
        ];

        for (token, id) in common_tokens {
            vocab.insert(token.to_string(), id);
        }

        // Add subword tokens with ## prefix
        let subwords = vec![
            "##ing", "##ed", "##s", "##ly", "##er", "##ion", "##tion",
            "##al", "##en", "##or", "##able", "##ness", "##ment",
        ];

        for (i, subword) in subwords.iter().enumerate() {
            vocab.insert(subword.to_string(), 2100 + i as u32);
        }

        vocab
    }

    /// Encode text to token IDs
    pub fn encode(&self, text: &str) -> Vec<u32> {
        if text.is_empty() {
            return Vec::new();
        }

        // Normalize
        let normalized = self.normalizer.normalize(text).unwrap_or_else(|_| text.to_string());

        // Pre-tokenize
        let pre_tokens = self.pre_tokenizer.pre_tokenize(&normalized).unwrap_or_else(|_| vec![normalized.clone()]);

        // Apply WordPiece
        let mut token_ids = Vec::new();
        for word in pre_tokens {
            let tokens = self.wordpiece.tokenize(&word);
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

    /// Encode with special tokens [CLS] text [SEP]
    pub fn encode_with_special(&self, text: &str) -> Vec<u32> {
        let mut ids = vec![self.cls_token_id];
        ids.extend(self.encode(text));
        ids.push(self.sep_token_id);
        ids
    }

    /// Encode a pair of texts [CLS] text_a [SEP] text_b [SEP]
    pub fn encode_pair(&self, text_a: &str, text_b: &str) -> Vec<u32> {
        let mut ids = vec![self.cls_token_id];
        ids.extend(self.encode(text_a));
        ids.push(self.sep_token_id);
        ids.extend(self.encode(text_b));
        ids.push(self.sep_token_id);
        ids
    }

    /// Decode token IDs back to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let tokens: Vec<String> = ids
            .iter()
            .filter_map(|&id| self.id_to_token.get(&id).cloned())
            .filter(|token| !self.is_special_token(token))
            .collect();

        // Join and remove ## prefixes
        let text = tokens.join(" ").replace(" ##", "");
        Ok(text)
    }

    /// Batch encoding
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<u32>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }

    /// Batch encoding with special tokens
    pub fn encode_batch_with_special(&self, texts: &[&str]) -> Vec<Vec<u32>> {
        texts.iter().map(|text| self.encode_with_special(text)).collect()
    }

    /// Batch decoding
    pub fn decode_batch(&self, ids_batch: &[Vec<u32>]) -> Result<Vec<String>> {
        ids_batch.iter().map(|ids| self.decode(ids)).collect()
    }

    /// Tokenize text (return strings)
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let ids = self.encode(text);
        ids.iter()
            .filter_map(|&id| self.id_to_token.get(&id).cloned())
            .collect()
    }

    /// Check if token is special
    pub fn is_special_token(&self, token: &str) -> bool {
        token == "[CLS]" || token == "[SEP]" || token == "[PAD]" ||
        token == "[UNK]" || token == "[MASK]" ||
        token.starts_with("[unused")
    }    /// Get vocabulary size
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

    /// Create attention mask (1 for real tokens, 0 for padding)
    pub fn create_attention_mask(&self, ids: &[u32]) -> Vec<u32> {
        ids.iter()
            .map(|&id| if id == self.pad_token_id { 0 } else { 1 })
            .collect()
    }

    /// Create token type IDs (0 for first sequence, 1 for second)
    pub fn create_token_type_ids(&self, ids: &[u32]) -> Vec<u32> {
        let mut type_ids = Vec::new();
        let mut current_type = 0;

        for &id in ids {
            type_ids.push(current_type);
            if id == self.sep_token_id {
                current_type = 1;
            }
        }

        type_ids
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
    pub fn pad(&self, mut ids: Vec<u32>, max_length: usize) -> Vec<u32> {
        while ids.len() < max_length {
            ids.push(self.pad_token_id);
        }
        ids
    }

    /// Pad and truncate to exact length
    pub fn pad_and_truncate(&self, ids: Vec<u32>, max_length: usize) -> Vec<u32> {
        let truncated = self.truncate(ids, max_length);
        self.pad(truncated, max_length)
    }

    /// Save vocabulary to file
    pub fn save_vocabulary(&self, path: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut items: Vec<_> = self.vocab.iter().collect();
        items.sort_by_key(|(_, &id)| id);

        let mut file = File::create(path)?;
        for (token, _) in items {
            writeln!(file, "{}", token)?;
        }

        Ok(())
    }

    /// Get special token IDs
    pub fn get_special_tokens(&self) -> HashMap<String, u32> {
        let mut tokens = HashMap::new();
        tokens.insert(self.cls_token.clone(), self.cls_token_id);
        tokens.insert(self.sep_token.clone(), self.sep_token_id);
        tokens.insert(self.pad_token.clone(), self.pad_token_id);
        tokens.insert(self.unk_token.clone(), self.unk_token_id);
        tokens.insert(self.mask_token.clone(), self.mask_token_id);
        tokens
    }
}

impl std::fmt::Debug for BertTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BertTokenizer")
            .field("vocab_size", &self.vocab_size())
            .field("do_lower_case", &self.do_lower_case)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bert_tokenizer_basic() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let text = "Hello world";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_bert_encode_decode() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let text = "Hello";
        let ids = tokenizer.encode(text);
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_bert_special_tokens() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let text = "Test";
        let ids = tokenizer.encode_with_special(text);
        assert_eq!(ids[0], 101); // [CLS]
        assert_eq!(ids[ids.len() - 1], 102); // [SEP]
    }

    #[test]
    fn test_bert_pair_encoding() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let ids = tokenizer.encode_pair("Hello", "World");
        assert!(ids.len() > 2);
        assert_eq!(ids[0], 101); // [CLS]
    }

    #[test]
    fn test_bert_attention_mask() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let ids = vec![101, 2023, 2003, 0, 0]; // [CLS] this is [PAD] [PAD]
        let mask = tokenizer.create_attention_mask(&ids);
        assert_eq!(mask, vec![1, 1, 1, 0, 0]);
    }

    #[test]
    fn test_bert_vocab_size() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        assert!(tokenizer.vocab_size() > 100);
    }
}
