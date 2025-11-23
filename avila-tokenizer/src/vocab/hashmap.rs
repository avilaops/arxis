use std::collections::HashMap;
use crate::error::{Result, TokenizerError};

/// Efficient vocabulary lookup using HashMap
#[derive(Debug, Clone)]
pub struct VocabHashMap {
    /// Token to ID mapping
    token_to_id: HashMap<String, u32>,
    /// ID to token mapping (for decoding)
    id_to_token: HashMap<u32, String>,
    /// Special tokens
    special_tokens: HashMap<String, u32>,
    /// Unknown token
    unk_token: Option<String>,
    /// Unknown token ID
    unk_id: Option<u32>,
}

impl VocabHashMap {
    /// Create a new empty vocabulary
    pub fn new() -> Self {
        Self {
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            special_tokens: HashMap::new(),
            unk_token: None,
            unk_id: None,
        }
    }

    /// Create vocabulary with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            token_to_id: HashMap::with_capacity(capacity),
            id_to_token: HashMap::with_capacity(capacity),
            special_tokens: HashMap::new(),
            unk_token: None,
            unk_id: None,
        }
    }

    /// Create from HashMap
    pub fn from_hashmap(vocab: HashMap<String, u32>) -> Self {
        let id_to_token: HashMap<u32, String> = vocab
            .iter()
            .map(|(k, &v)| (v, k.clone()))
            .collect();

        Self {
            token_to_id: vocab,
            id_to_token,
            special_tokens: HashMap::new(),
            unk_token: None,
            unk_id: None,
        }
    }

    /// Insert a token into the vocabulary
    pub fn insert(&mut self, token: String, id: u32) {
        self.id_to_token.insert(id, token.clone());
        self.token_to_id.insert(token, id);
    }

    /// Insert a special token
    pub fn insert_special(&mut self, token: String, id: u32) {
        self.insert(token.clone(), id);
        self.special_tokens.insert(token, id);
    }

    /// Set unknown token
    pub fn set_unk_token(&mut self, token: String, id: u32) {
        self.insert(token.clone(), id);
        self.unk_token = Some(token);
        self.unk_id = Some(id);
    }

    /// Get token ID
    pub fn get_id(&self, token: &str) -> Option<u32> {
        self.token_to_id.get(token).copied()
    }

    /// Get token ID or unknown token ID
    pub fn get_id_or_unk(&self, token: &str) -> u32 {
        self.get_id(token)
            .or(self.unk_id)
            .unwrap_or(0)
    }

    /// Get token by ID
    pub fn get_token(&self, id: u32) -> Option<&str> {
        self.id_to_token.get(&id).map(|s| s.as_str())
    }

    /// Get token by ID or unknown token
    pub fn get_token_or_unk(&self, id: u32) -> Result<&str> {
        self.get_token(id)
            .or(self.unk_token.as_deref())
            .ok_or(TokenizerError::InvalidTokenId(id))
    }

    /// Check if token exists
    pub fn contains_token(&self, token: &str) -> bool {
        self.token_to_id.contains_key(token)
    }

    /// Check if ID exists
    pub fn contains_id(&self, id: u32) -> bool {
        self.id_to_token.contains_key(&id)
    }

    /// Check if token is special
    pub fn is_special_token(&self, token: &str) -> bool {
        self.special_tokens.contains_key(token)
    }

    /// Get vocabulary size
    pub fn len(&self) -> usize {
        self.token_to_id.len()
    }

    /// Check if vocabulary is empty
    pub fn is_empty(&self) -> bool {
        self.token_to_id.is_empty()
    }

    /// Get all tokens
    pub fn tokens(&self) -> Vec<&str> {
        self.token_to_id.keys().map(|s| s.as_str()).collect()
    }

    /// Get all IDs
    pub fn ids(&self) -> Vec<u32> {
        self.id_to_token.keys().copied().collect()
    }

    /// Get token-to-ID map reference
    pub fn token_to_id_map(&self) -> &HashMap<String, u32> {
        &self.token_to_id
    }

    /// Get ID-to-token map reference
    pub fn id_to_token_map(&self) -> &HashMap<u32, String> {
        &self.id_to_token
    }

    /// Get special tokens
    pub fn special_tokens(&self) -> &HashMap<String, u32> {
        &self.special_tokens
    }

    /// Get unknown token
    pub fn unk_token(&self) -> Option<&str> {
        self.unk_token.as_deref()
    }

    /// Get unknown token ID
    pub fn unk_id(&self) -> Option<u32> {
        self.unk_id
    }

    /// Convert tokens to IDs
    pub fn convert_tokens_to_ids(&self, tokens: &[String]) -> Vec<u32> {
        tokens.iter().map(|t| self.get_id_or_unk(t)).collect()
    }

    /// Convert IDs to tokens
    pub fn convert_ids_to_tokens(&self, ids: &[u32]) -> Result<Vec<String>> {
        ids.iter()
            .map(|&id| {
                self.get_token(id)
                    .map(|s| s.to_string())
                    .ok_or(TokenizerError::InvalidTokenId(id))
            })
            .collect()
    }

    /// Encode text to IDs (simple whitespace split)
    pub fn encode_simple(&self, text: &str) -> Vec<u32> {
        text.split_whitespace()
            .map(|token| self.get_id_or_unk(token))
            .collect()
    }

    /// Decode IDs to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let tokens = self.convert_ids_to_tokens(ids)?;
        Ok(tokens.join(" "))
    }

    /// Decode IDs to text, skipping special tokens
    pub fn decode_skip_special(&self, ids: &[u32]) -> Result<String> {
        let tokens: Result<Vec<String>> = ids.iter()
            .filter_map(|&id| {
                self.get_token(id).and_then(|token| {
                    if self.is_special_token(token) {
                        None
                    } else {
                        Some(Ok(token.to_string()))
                    }
                })
            })
            .collect();

        Ok(tokens?.join(" "))
    }

    /// Merge another vocabulary into this one
    pub fn merge(&mut self, other: &VocabHashMap) {
        for (token, &id) in &other.token_to_id {
            if !self.contains_token(token) {
                self.insert(token.clone(), id);
            }
        }
    }

    /// Extend vocabulary with new tokens
    pub fn extend(&mut self, tokens: Vec<String>) {
        let start_id = self.len() as u32;
        for (i, token) in tokens.into_iter().enumerate() {
            if !self.contains_token(&token) {
                self.insert(token, start_id + i as u32);
            }
        }
    }

    /// Clear the vocabulary
    pub fn clear(&mut self) {
        self.token_to_id.clear();
        self.id_to_token.clear();
        self.special_tokens.clear();
        self.unk_token = None;
        self.unk_id = None;
    }

    /// Get vocabulary statistics
    pub fn stats(&self) -> VocabStats {
        VocabStats {
            total_tokens: self.len(),
            special_tokens: self.special_tokens.len(),
            has_unk_token: self.unk_token.is_some(),
            max_id: self.id_to_token.keys().max().copied(),
        }
    }
}

impl Default for VocabHashMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Vocabulary statistics
#[derive(Debug, Clone)]
pub struct VocabStats {
    pub total_tokens: usize,
    pub special_tokens: usize,
    pub has_unk_token: bool,
    pub max_id: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocab_insert_and_get() {
        let mut vocab = VocabHashMap::new();

        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);

        assert_eq!(vocab.get_id("hello"), Some(1));
        assert_eq!(vocab.get_id("world"), Some(2));
        assert_eq!(vocab.get_token(1), Some("hello"));
        assert_eq!(vocab.get_token(2), Some("world"));
        assert_eq!(vocab.len(), 2);
    }

    #[test]
    fn test_vocab_special_tokens() {
        let mut vocab = VocabHashMap::new();

        vocab.insert_special("[PAD]".to_string(), 0);
        vocab.insert_special("[UNK]".to_string(), 1);
        vocab.insert("hello".to_string(), 2);

        assert!(vocab.is_special_token("[PAD]"));
        assert!(vocab.is_special_token("[UNK]"));
        assert!(!vocab.is_special_token("hello"));
        assert_eq!(vocab.special_tokens().len(), 2);
    }

    #[test]
    fn test_vocab_unk_token() {
        let mut vocab = VocabHashMap::new();

        vocab.set_unk_token("[UNK]".to_string(), 0);
        vocab.insert("hello".to_string(), 1);

        assert_eq!(vocab.get_id_or_unk("hello"), 1);
        assert_eq!(vocab.get_id_or_unk("unknown"), 0);
        assert_eq!(vocab.unk_token(), Some("[UNK]"));
        assert_eq!(vocab.unk_id(), Some(0));
    }

    #[test]
    fn test_vocab_convert_tokens() {
        let mut vocab = VocabHashMap::new();

        vocab.set_unk_token("[UNK]".to_string(), 0);
        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);

        let tokens = vec!["hello".to_string(), "world".to_string(), "unknown".to_string()];
        let ids = vocab.convert_tokens_to_ids(&tokens);

        assert_eq!(ids, vec![1, 2, 0]); // unknown -> [UNK] (0)
    }

    #[test]
    fn test_vocab_convert_ids() {
        let mut vocab = VocabHashMap::new();

        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);

        let ids = vec![1, 2];
        let tokens = vocab.convert_ids_to_tokens(&ids).unwrap();

        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_vocab_decode() {
        let mut vocab = VocabHashMap::new();

        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);

        let ids = vec![1, 2];
        let text = vocab.decode(&ids).unwrap();

        assert_eq!(text, "hello world");
    }

    #[test]
    fn test_vocab_decode_skip_special() {
        let mut vocab = VocabHashMap::new();

        vocab.insert_special("[CLS]".to_string(), 0);
        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);
        vocab.insert_special("[SEP]".to_string(), 3);

        let ids = vec![0, 1, 2, 3];
        let text = vocab.decode_skip_special(&ids).unwrap();

        assert_eq!(text, "hello world");
    }

    #[test]
    fn test_vocab_extend() {
        let mut vocab = VocabHashMap::new();

        vocab.insert("hello".to_string(), 0);

        vocab.extend(vec!["world".to_string(), "test".to_string()]);

        assert_eq!(vocab.len(), 3);
        assert!(vocab.contains_token("world"));
        assert!(vocab.contains_token("test"));
    }

    #[test]
    fn test_vocab_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("hello".to_string(), 1);
        map.insert("world".to_string(), 2);

        let vocab = VocabHashMap::from_hashmap(map);

        assert_eq!(vocab.len(), 2);
        assert_eq!(vocab.get_id("hello"), Some(1));
        assert_eq!(vocab.get_token(2), Some("world"));
    }

    #[test]
    fn test_vocab_stats() {
        let mut vocab = VocabHashMap::new();

        vocab.set_unk_token("[UNK]".to_string(), 0);
        vocab.insert_special("[PAD]".to_string(), 1);
        vocab.insert("hello".to_string(), 2);
        vocab.insert("world".to_string(), 3);

        let stats = vocab.stats();

        assert_eq!(stats.total_tokens, 4);
        assert_eq!(stats.special_tokens, 1); // Only [PAD], [UNK] is set separately
        assert!(stats.has_unk_token);
        assert_eq!(stats.max_id, Some(3));
    }
}
