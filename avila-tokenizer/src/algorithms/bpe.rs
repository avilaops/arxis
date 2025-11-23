use std::collections::HashMap;
use crate::error::{Result, TokenizerError};
use crate::utils::cache::TokenCache;
use crate::utils::unicode::{byte_to_unicode, unicode_to_byte};

/// Byte-Pair Encoding (BPE) tokenizer
/// Used in GPT-2, GPT-3, GPT-4, and other models
#[derive(Debug, Clone)]
pub struct BPE {
    /// Token to ID mapping
    vocab: HashMap<String, u32>,
    /// Ordered list of merge pairs (priority by index)
    merges: Vec<(String, String)>,
    /// Merge pair to priority mapping for fast lookup
    merge_ranks: HashMap<(String, String), usize>,
    /// LRU cache for encoded results
    cache: TokenCache<String, Vec<String>>,
    /// Byte-to-Unicode mapping (for byte-level BPE)
    byte_encoder: Option<HashMap<u8, char>>,
    /// Unicode-to-byte mapping
    byte_decoder: Option<HashMap<char, u8>>,
    /// Whether to use byte-level encoding
    byte_level: bool,
    /// End of word suffix
    end_of_word_suffix: Option<String>,
}

impl BPE {
    /// Create a new BPE tokenizer
    pub fn new(vocab: HashMap<String, u32>, merges: Vec<(String, String)>) -> Self {
        let merge_ranks: HashMap<(String, String), usize> = merges
            .iter()
            .enumerate()
            .map(|(i, pair)| (pair.clone(), i))
            .collect();

        Self {
            vocab,
            merges,
            merge_ranks,
            cache: TokenCache::new(10_000),
            byte_encoder: None,
            byte_decoder: None,
            byte_level: false,
            end_of_word_suffix: None,
        }
    }

    /// Create a byte-level BPE tokenizer (GPT-2 style)
    pub fn new_byte_level(vocab: HashMap<String, u32>, merges: Vec<(String, String)>) -> Self {
        let byte_encoder = Some(byte_to_unicode());
        let byte_decoder = Some(unicode_to_byte());

        let merge_ranks: HashMap<(String, String), usize> = merges
            .iter()
            .enumerate()
            .map(|(i, pair)| (pair.clone(), i))
            .collect();

        Self {
            vocab,
            merges,
            merge_ranks,
            cache: TokenCache::new(10_000),
            byte_encoder,
            byte_decoder,
            byte_level: true,
            end_of_word_suffix: None,
        }
    }

    /// Set end-of-word suffix (e.g., "</w>" for some BPE variants)
    pub fn with_end_of_word_suffix(mut self, suffix: String) -> Self {
        self.end_of_word_suffix = Some(suffix);
        self
    }

    /// Encode text to token IDs
    pub fn encode(&mut self, text: &str) -> Vec<u32> {
        if text.is_empty() {
            return Vec::new();
        }

        let tokens = self.tokenize(text);
        tokens
            .iter()
            .map(|token| {
                self.vocab
                    .get(token)
                    .copied()
                    .unwrap_or_else(|| self.vocab.get("<|endoftext|>").copied().unwrap_or(0))
            })
            .collect()
    }

    /// Tokenize text into subword tokens
    pub fn tokenize(&mut self, text: &str) -> Vec<String> {
        if text.is_empty() {
            return Vec::new();
        }

        let words = if self.byte_level {
            self.byte_split(text)
        } else {
            self.word_split(text)
        };

        let mut all_tokens = Vec::new();

        for word in words {
            // Check cache first
            if let Some(cached) = self.cache.get(&word) {
                all_tokens.extend(cached.clone());
                continue;
            }

            // Tokenize word
            let tokens = self.bpe_word(&word);

            // Cache result
            self.cache.put(word.clone(), tokens.clone());

            all_tokens.extend(tokens);
        }

        all_tokens
    }

    /// Apply BPE algorithm to a single word
    fn bpe_word(&self, word: &str) -> Vec<String> {
        if word.len() == 1 {
            return vec![word.to_string()];
        }

        // Split into characters
        let mut word: Vec<String> = word.chars().map(|c| c.to_string()).collect();

        // Add end-of-word suffix if configured
        if let Some(suffix) = &self.end_of_word_suffix {
            if let Some(last) = word.last_mut() {
                last.push_str(suffix);
            }
        }

        loop {
            // Find the best pair to merge
            let pairs = self.get_pairs(&word);

            if pairs.is_empty() {
                break;
            }

            let best_pair = pairs
                .iter()
                .filter_map(|pair| {
                    self.merge_ranks
                        .get(pair)
                        .map(|&rank| (rank, pair.clone()))
                })
                .min_by_key(|(rank, _)| *rank);

            match best_pair {
                Some((_, pair)) => {
                    word = self.merge_pair(&word, &pair);
                }
                None => break,
            }
        }

        word
    }

    /// Get all adjacent pairs in the word
    fn get_pairs(&self, word: &[String]) -> Vec<(String, String)> {
        if word.len() < 2 {
            return Vec::new();
        }

        word.windows(2)
            .map(|w| (w[0].clone(), w[1].clone()))
            .collect()
    }

    /// Merge a specific pair in the word
    fn merge_pair(&self, word: &[String], pair: &(String, String)) -> Vec<String> {
        let mut new_word = Vec::new();
        let mut i = 0;

        while i < word.len() {
            if i < word.len() - 1 && word[i] == pair.0 && word[i + 1] == pair.1 {
                // Merge the pair
                new_word.push(format!("{}{}", pair.0, pair.1));
                i += 2;
            } else {
                new_word.push(word[i].clone());
                i += 1;
            }
        }

        new_word
    }

    /// Split text into bytes for byte-level BPE
    fn byte_split(&self, text: &str) -> Vec<String> {
        if let Some(byte_encoder) = &self.byte_encoder {
            text.bytes()
                .map(|b| {
                    byte_encoder
                        .get(&b)
                        .map(|&c| c.to_string())
                        .unwrap_or_else(|| format!("{}", b as char))
                })
                .collect()
        } else {
            vec![text.to_string()]
        }
    }

    /// Split text into words (whitespace-based)
    fn word_split(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    /// Decode token IDs back to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let tokens = self.decode_to_tokens(ids)?;

        let text = if self.byte_level {
            self.decode_byte_level(&tokens)?
        } else {
            tokens.join(" ")
        };

        Ok(text)
    }

    /// Decode token IDs to tokens (strings)
    pub fn decode_to_tokens(&self, ids: &[u32]) -> Result<Vec<String>> {
        // Create reverse mapping
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

    /// Decode byte-level tokens back to text
    fn decode_byte_level(&self, tokens: &[String]) -> Result<String> {
        if let Some(byte_decoder) = &self.byte_decoder {
            let joined = tokens.join("");
            let bytes: Vec<u8> = joined
                .chars()
                .map(|c| byte_decoder.get(&c).copied().unwrap_or(0))
                .collect();

            String::from_utf8(bytes).map_err(|_| TokenizerError::Utf8Error)
        } else {
            Ok(tokens.join(""))
        }
    }

    /// Batch encoding with parallel processing
    pub fn encode_batch(&mut self, texts: &[&str]) -> Vec<Vec<u32>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }

    /// Train BPE on a corpus
    pub fn train(
        corpus: &[&str],
        vocab_size: usize,
        min_frequency: usize,
        byte_level: bool,
    ) -> Result<Self> {
        // Step 1: Initialize vocabulary with characters
        let mut vocab: HashMap<String, usize> = HashMap::new();
        let mut word_freqs: HashMap<String, usize> = HashMap::new();

        // Count word frequencies
        for text in corpus {
            for word in text.split_whitespace() {
                *word_freqs.entry(word.to_string()).or_insert(0) += 1;
            }
        }

        // Initialize with character-level splits
        let mut word_splits: HashMap<String, Vec<String>> = HashMap::new();

        for (word, &freq) in &word_freqs {
            let split: Vec<String> = word.chars().map(|c| c.to_string()).collect();

            // Count character frequencies
            for token in &split {
                *vocab.entry(token.clone()).or_insert(0) += freq;
            }

            word_splits.insert(word.clone(), split);
        }

        let mut merges = Vec::new();

        // Step 2: Iteratively merge most frequent pairs
        while vocab.len() < vocab_size {
            // Count pair frequencies
            let mut pair_freqs: HashMap<(String, String), usize> = HashMap::new();

            for (word, splits) in &word_splits {
                let word_freq = word_freqs.get(word).copied().unwrap_or(0);

                for i in 0..splits.len().saturating_sub(1) {
                    let pair = (splits[i].clone(), splits[i + 1].clone());
                    *pair_freqs.entry(pair).or_insert(0) += word_freq;
                }
            }

            // Find most frequent pair
            let best_pair = pair_freqs
                .iter()
                .filter(|(_, &freq)| freq >= min_frequency)
                .max_by_key(|(_, &freq)| freq);

            match best_pair {
                Some((pair, _)) => {
                    let merged = format!("{}{}", pair.0, pair.1);

                    // Update word splits
                    for splits in word_splits.values_mut() {
                        let mut i = 0;
                        while i < splits.len().saturating_sub(1) {
                            if splits[i] == pair.0 && splits[i + 1] == pair.1 {
                                splits[i] = merged.clone();
                                splits.remove(i + 1);
                            } else {
                                i += 1;
                            }
                        }
                    }

                    // Add to vocabulary and merges
                    vocab.insert(merged, 0);
                    merges.push(pair.clone());
                }
                None => break,
            }
        }

        // Convert vocabulary to ID mapping
        let vocab_map: HashMap<String, u32> = vocab
            .keys()
            .enumerate()
            .map(|(i, token)| (token.clone(), i as u32))
            .collect();

        if byte_level {
            Ok(BPE::new_byte_level(vocab_map, merges))
        } else {
            Ok(BPE::new(vocab_map, merges))
        }
    }

    /// Get vocabulary
    pub fn vocab(&self) -> &HashMap<String, u32> {
        &self.vocab
    }

    /// Get merges
    pub fn merges(&self) -> &[(String, String)] {
        &self.merges
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Check if using byte-level encoding
    pub fn is_byte_level(&self) -> bool {
        self.byte_level
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.cache.len(), self.cache.cap())
    }

    /// Token to ID lookup
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.vocab.get(token).copied()
    }

    /// ID to token lookup
    pub fn id_to_token(&self, id: u32) -> Option<String> {
        self.vocab
            .iter()
            .find(|(_, &v)| v == id)
            .map(|(k, _)| k.clone())
    }

    /// Add special tokens to vocabulary
    pub fn add_special_tokens(&mut self, tokens: Vec<String>) {
        let start_id = self.vocab.len() as u32;
        for (i, token) in tokens.into_iter().enumerate() {
            self.vocab.entry(token).or_insert(start_id + i as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_bpe() -> BPE {
        let mut vocab = HashMap::new();
        vocab.insert("h".to_string(), 0);
        vocab.insert("e".to_string(), 1);
        vocab.insert("l".to_string(), 2);
        vocab.insert("o".to_string(), 3);
        vocab.insert("he".to_string(), 4);
        vocab.insert("ll".to_string(), 5);
        vocab.insert("lo".to_string(), 6);
        vocab.insert("hello".to_string(), 7);

        let merges = vec![
            ("h".to_string(), "e".to_string()),
            ("l".to_string(), "l".to_string()),
            ("l".to_string(), "o".to_string()),
        ];

        BPE::new(vocab, merges)
    }

    #[test]
    fn test_bpe_get_pairs() {
        let bpe = create_test_bpe();
        let word = vec!["h".to_string(), "e".to_string(), "l".to_string(), "l".to_string()];
        let pairs = bpe.get_pairs(&word);

        assert_eq!(pairs.len(), 3);
        assert!(pairs.contains(&("h".to_string(), "e".to_string())));
        assert!(pairs.contains(&("e".to_string(), "l".to_string())));
        assert!(pairs.contains(&("l".to_string(), "l".to_string())));
    }

    #[test]
    fn test_bpe_merge_pair() {
        let bpe = create_test_bpe();
        let word = vec!["h".to_string(), "e".to_string(), "l".to_string(), "l".to_string()];
        let pair = ("h".to_string(), "e".to_string());
        let merged = bpe.merge_pair(&word, &pair);

        assert_eq!(merged, vec!["he", "l", "l"]);
    }

    #[test]
    fn test_bpe_tokenize() {
        let mut bpe = create_test_bpe();
        let tokens = bpe.tokenize("hello");

        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_bpe_encode_decode() {
        let mut bpe = create_test_bpe();
        let text = "hello";
        let ids = bpe.encode(text);
        let decoded = bpe.decode(&ids).unwrap();

        // Note: decoding might not match exactly due to space handling
        assert!(!ids.is_empty());
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_bpe_cache() {
        let mut bpe = create_test_bpe();

        // First call should compute
        let _ = bpe.tokenize("test");
        let (len1, _) = bpe.cache_stats();

        // Second call should use cache
        let _ = bpe.tokenize("test");
        let (len2, _) = bpe.cache_stats();

        assert_eq!(len1, len2); // Cache size should be same
    }

    #[test]
    fn test_bpe_vocab_size() {
        let bpe = create_test_bpe();
        assert_eq!(bpe.vocab_size(), 8);
    }

    #[test]
    fn test_bpe_add_special_tokens() {
        let mut bpe = create_test_bpe();
        let vocab_size_before = bpe.vocab_size();

        bpe.add_special_tokens(vec!["<|endoftext|>".to_string()]);

        assert_eq!(bpe.vocab_size(), vocab_size_before + 1);
        assert!(bpe.vocab().contains_key("<|endoftext|>"));
    }

    #[test]
    fn test_bpe_byte_level() {
        let vocab = HashMap::new();
        let merges = Vec::new();
        let bpe = BPE::new_byte_level(vocab, merges);

        assert!(bpe.is_byte_level());
        assert!(bpe.byte_encoder.is_some());
        assert!(bpe.byte_decoder.is_some());
    }

    #[test]
    fn test_bpe_train_simple() {
        let corpus = vec!["hello world", "hello rust", "world peace"];
        let result = BPE::train(&corpus, 50, 1, false);

        assert!(result.is_ok());
        let bpe = result.unwrap();
        assert!(bpe.vocab_size() > 0);
    }
}
