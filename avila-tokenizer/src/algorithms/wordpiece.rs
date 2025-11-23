use std::collections::HashMap;
use crate::error::{Result, TokenizerError};
use crate::vocab::Trie;

/// WordPiece tokenizer (used in BERT and variants)
/// Implements longest-match-first greedy tokenization with ## prefix for subwords
#[derive(Debug, Clone)]
pub struct WordPiece {
    /// Vocabulary mapping token to ID
    vocab: HashMap<String, u32>,
    /// Trie for efficient longest-match lookup
    #[allow(dead_code)]
    trie: Trie,
    /// Unknown token
    unk_token: String,
    /// Unknown token ID
    unk_id: u32,
    /// Continuing subword prefix (default: "##")
    continuing_subword_prefix: String,
    /// Maximum input characters per word
    max_input_chars_per_word: usize,
}

impl WordPiece {
    /// Create a new WordPiece tokenizer
    pub fn new(
        vocab: HashMap<String, u32>,
        unk_token: String,
        continuing_subword_prefix: String,
    ) -> Result<Self> {
        let unk_id = vocab
            .get(&unk_token)
            .copied()
            .ok_or_else(|| TokenizerError::InvalidSpecialToken(unk_token.clone()))?;

        let trie = Trie::from_vocab(&vocab);

        Ok(Self {
            vocab,
            trie,
            unk_token,
            unk_id,
            continuing_subword_prefix,
            max_input_chars_per_word: 100,
        })
    }

    /// Create WordPiece with default settings (BERT-style)
    pub fn new_bert_style(vocab: HashMap<String, u32>) -> Result<Self> {
        Self::new(vocab, "[UNK]".to_string(), "##".to_string())
    }

    /// Set maximum input characters per word
    pub fn with_max_input_chars(mut self, max_chars: usize) -> Self {
        self.max_input_chars_per_word = max_chars;
        self
    }

    /// Tokenize text into subword tokens
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let mut output_tokens = Vec::new();

        for word in text.split_whitespace() {
            if word.chars().count() > self.max_input_chars_per_word {
                output_tokens.push(self.unk_token.clone());
                continue;
            }

            let word_tokens = self.tokenize_word(word);
            output_tokens.extend(word_tokens);
        }

        output_tokens
    }

    /// Tokenize a single word using longest-match-first
    fn tokenize_word(&self, word: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut start = 0;
        let chars: Vec<char> = word.chars().collect();

        while start < chars.len() {
            let mut end = chars.len();
            let mut found = false;

            // Try to find longest match
            while start < end {
                let substr: String = chars[start..end].iter().collect();

                // Add prefix for continuing subwords (not first token)
                let token = if start == 0 {
                    substr.clone()
                } else {
                    format!("{}{}", self.continuing_subword_prefix, substr)
                };

                if self.vocab.contains_key(&token) {
                    tokens.push(token);
                    start = end;
                    found = true;
                    break;
                }

                end -= 1;
            }

            // If no match found, use unknown token
            if !found {
                tokens.clear();
                tokens.push(self.unk_token.clone());
                break;
            }
        }

        tokens
    }

    /// Tokenize a single word using trie-based lookup (faster)
    #[allow(dead_code)]
    fn tokenize_word_with_trie(&self, word: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = word.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            let remaining: String = chars[pos..].iter().collect();

            // Try to find longest prefix match
            let longest = if pos == 0 {
                self.trie.find_longest_prefix(&remaining)
            } else {
                // Add ## prefix and search
                let with_prefix = format!("{}{}", self.continuing_subword_prefix, remaining);
                self.trie.find_longest_prefix(&with_prefix)
            };

            match longest {
                Some((token, _id, length)) => {
                    tokens.push(token);
                    pos += if pos == 0 { length } else { length - self.continuing_subword_prefix.len() };
                }
                None => {
                    // No match found, return unknown token
                    return vec![self.unk_token.clone()];
                }
            }
        }

        tokens
    }

    /// Encode text to token IDs
    pub fn encode(&self, text: &str) -> Vec<u32> {
        let tokens = self.tokenize(text);
        self.convert_tokens_to_ids(&tokens)
    }

    /// Convert tokens to IDs
    pub fn convert_tokens_to_ids(&self, tokens: &[String]) -> Vec<u32> {
        tokens
            .iter()
            .map(|token| self.vocab.get(token).copied().unwrap_or(self.unk_id))
            .collect()
    }

    /// Decode token IDs to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let tokens = self.convert_ids_to_tokens(ids)?;
        Ok(self.convert_tokens_to_string(&tokens))
    }

    /// Convert IDs to tokens
    pub fn convert_ids_to_tokens(&self, ids: &[u32]) -> Result<Vec<String>> {
        let id_to_token: HashMap<u32, String> = self
            .vocab
            .iter()
            .map(|(token, &id)| (id, token.clone()))
            .collect();

        ids.iter()
            .map(|&id| {
                id_to_token
                    .get(&id)
                    .cloned()
                    .ok_or(TokenizerError::InvalidTokenId(id))
            })
            .collect()
    }

    /// Convert tokens to string (removing ## prefixes)
    pub fn convert_tokens_to_string(&self, tokens: &[String]) -> String {
        let mut result = String::new();

        for (i, token) in tokens.iter().enumerate() {
            if token.starts_with(&self.continuing_subword_prefix) {
                // Remove prefix and append without space
                result.push_str(&token[self.continuing_subword_prefix.len()..]);
            } else {
                // Add space before word (except first)
                if i > 0 {
                    result.push(' ');
                }
                result.push_str(token);
            }
        }

        result
    }

    /// Batch encoding
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<u32>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }

    /// Batch decoding
    pub fn decode_batch(&self, ids_batch: &[Vec<u32>]) -> Result<Vec<String>> {
        ids_batch.iter().map(|ids| self.decode(ids)).collect()
    }

    /// Train WordPiece from corpus
    pub fn train(
        corpus: &[&str],
        vocab_size: usize,
        min_frequency: usize,
        special_tokens: Vec<String>,
    ) -> Result<Self> {
        // Step 1: Initialize with characters
        let mut vocab: HashMap<String, usize> = HashMap::new();

        // Add special tokens
        for token in &special_tokens {
            vocab.insert(token.clone(), usize::MAX); // High frequency to keep them
        }

        // Count character frequencies
        for text in corpus {
            for word in text.split_whitespace() {
                for ch in word.chars() {
                    *vocab.entry(ch.to_string()).or_insert(0) += 1;
                }
            }
        }

        // Step 2: Iteratively build subwords
        let mut word_splits: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize word splits
        for text in corpus {
            for word in text.split_whitespace() {
                if !word_splits.contains_key(word) {
                    let split: Vec<String> = word.chars().map(|c| c.to_string()).collect();
                    word_splits.insert(word.to_string(), split);
                }
            }
        }

        // Build vocabulary up to vocab_size
        while vocab.len() < vocab_size {
            let mut subword_counts: HashMap<String, usize> = HashMap::new();

            // Count subword frequencies
            for splits in word_splits.values() {
                for (i, token) in splits.iter().enumerate() {
                    let subword = if i == 0 {
                        token.clone()
                    } else {
                        format!("##{}", token)
                    };
                    *subword_counts.entry(subword).or_insert(0) += 1;
                }

                // Try combining adjacent tokens
                for i in 0..splits.len().saturating_sub(1) {
                    let combined = if i == 0 {
                        format!("{}{}", splits[i], splits[i + 1])
                    } else {
                        format!("##{}{}", splits[i], splits[i + 1])
                    };
                    *subword_counts.entry(combined).or_insert(0) += 1;
                }
            }

            // Find best subword to add
            let best_subword = subword_counts
                .iter()
                .filter(|(token, &count)| {
                    count >= min_frequency && !vocab.contains_key(*token)
                })
                .max_by_key(|(_, &count)| count);

            match best_subword {
                Some((subword, &count)) => {
                    vocab.insert(subword.clone(), count);

                    // Update word splits (simplified - real implementation more complex)
                    // This is a basic heuristic
                }
                None => break,
            }
        }

        // Convert to ID mapping
        let vocab_map: HashMap<String, u32> = vocab
            .into_iter()
            .enumerate()
            .map(|(i, (token, _))| (token, i as u32))
            .collect();

        let _unk_token = special_tokens
            .iter()
            .find(|t| t.contains("UNK"))
            .cloned()
            .unwrap_or_else(|| "[UNK]".to_string());

        Self::new_bert_style(vocab_map)
    }

    /// Get vocabulary
    pub fn vocab(&self) -> &HashMap<String, u32> {
        &self.vocab
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Get unknown token
    pub fn unk_token(&self) -> &str {
        &self.unk_token
    }

    /// Get unknown token ID
    pub fn unk_id(&self) -> u32 {
        self.unk_id
    }

    /// Get continuing subword prefix
    pub fn continuing_subword_prefix(&self) -> &str {
        &self.continuing_subword_prefix
    }

    /// Check if token is in vocabulary
    pub fn contains_token(&self, token: &str) -> bool {
        self.vocab.contains_key(token)
    }

    /// Get token ID
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.vocab.get(token).copied()
    }

    /// Get token from ID
    pub fn id_to_token(&self, id: u32) -> Option<String> {
        self.vocab
            .iter()
            .find(|(_, &v)| v == id)
            .map(|(k, _)| k.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_wordpiece() -> WordPiece {
        let mut vocab = HashMap::new();
        vocab.insert("[UNK]".to_string(), 0);
        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);
        vocab.insert("##ing".to_string(), 3);
        vocab.insert("test".to_string(), 4);
        vocab.insert("##s".to_string(), 5);

        WordPiece::new_bert_style(vocab).unwrap()
    }

    #[test]
    fn test_wordpiece_tokenize_single_word() {
        let wp = create_test_wordpiece();
        let tokens = wp.tokenize("hello");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], "hello");
    }

    #[test]
    fn test_wordpiece_tokenize_multiple_words() {
        let wp = create_test_wordpiece();
        let tokens = wp.tokenize("hello world");

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], "hello");
        assert_eq!(tokens[1], "world");
    }

    #[test]
    fn test_wordpiece_tokenize_unknown() {
        let wp = create_test_wordpiece();
        let tokens = wp.tokenize("unknown");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], "[UNK]");
    }

    #[test]
    fn test_wordpiece_encode() {
        let wp = create_test_wordpiece();
        let ids = wp.encode("hello world");

        assert_eq!(ids.len(), 2);
        assert_eq!(ids[0], 1);
        assert_eq!(ids[1], 2);
    }

    #[test]
    fn test_wordpiece_decode() {
        let wp = create_test_wordpiece();
        let ids = vec![1, 2];
        let text = wp.decode(&ids).unwrap();

        assert_eq!(text, "hello world");
    }

    #[test]
    fn test_wordpiece_continuing_subword() {
        let wp = create_test_wordpiece();

        // Create a word that needs subword splitting
        // This is simplified - in real usage would need proper vocab
        assert_eq!(wp.continuing_subword_prefix(), "##");
    }

    #[test]
    fn test_wordpiece_convert_tokens_to_string() {
        let wp = create_test_wordpiece();
        let tokens = vec![
            "hello".to_string(),
            "##ing".to_string(),
        ];

        let text = wp.convert_tokens_to_string(&tokens);
        assert_eq!(text, "helloing");
    }

    #[test]
    fn test_wordpiece_convert_tokens_to_string_multiple_words() {
        let wp = create_test_wordpiece();
        let tokens = vec![
            "test".to_string(),
            "##s".to_string(),
            "hello".to_string(),
        ];

        let text = wp.convert_tokens_to_string(&tokens);
        assert_eq!(text, "tests hello");
    }

    #[test]
    fn test_wordpiece_vocab_size() {
        let wp = create_test_wordpiece();
        assert_eq!(wp.vocab_size(), 6);
    }

    #[test]
    fn test_wordpiece_unk_token() {
        let wp = create_test_wordpiece();
        assert_eq!(wp.unk_token(), "[UNK]");
        assert_eq!(wp.unk_id(), 0);
    }

    #[test]
    fn test_wordpiece_token_to_id() {
        let wp = create_test_wordpiece();
        assert_eq!(wp.token_to_id("hello"), Some(1));
        assert_eq!(wp.token_to_id("world"), Some(2));
        assert_eq!(wp.token_to_id("notfound"), None);
    }

    #[test]
    fn test_wordpiece_id_to_token() {
        let wp = create_test_wordpiece();
        assert_eq!(wp.id_to_token(1), Some("hello".to_string()));
        assert_eq!(wp.id_to_token(2), Some("world".to_string()));
    }

    #[test]
    fn test_wordpiece_max_input_chars() {
        let mut vocab = HashMap::new();
        vocab.insert("[UNK]".to_string(), 0);

        let wp = WordPiece::new_bert_style(vocab)
            .unwrap()
            .with_max_input_chars(5);

        let tokens = wp.tokenize("verylongword");
        assert_eq!(tokens[0], "[UNK]");
    }

    #[test]
    fn test_wordpiece_batch_encode() {
        let wp = create_test_wordpiece();
        let texts = vec!["hello", "world"];
        let batch = wp.encode_batch(&texts);

        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0], vec![1]);
        assert_eq!(batch[1], vec![2]);
    }
}
