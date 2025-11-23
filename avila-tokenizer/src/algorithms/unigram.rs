use std::collections::HashMap;
use crate::error::{Result, TokenizerError};
use crate::vocab::Trie;

/// Unigram Language Model tokenizer (used in SentencePiece, T5, XLNet)
/// Uses Viterbi algorithm for tokenization and EM algorithm for training
#[derive(Debug, Clone)]
pub struct Unigram {
    /// Vocabulary pieces with their log probabilities
    /// Sorted by length (descending) for efficient matching
    pieces: Vec<(String, f64)>,
    /// Trie for efficient prefix matching
    trie: Trie,
    /// Token to ID mapping
    vocab: HashMap<String, u32>,
    /// Unknown token
    unk_token: Option<String>,
    /// Unknown token ID
    unk_id: Option<u32>,
    /// Minimum score threshold
    min_score: f64,
}

impl Unigram {
    /// Create a new Unigram tokenizer
    pub fn new(pieces: Vec<(String, f64)>) -> Self {
        // Sort pieces by length (descending) for longest-match-first
        let mut pieces = pieces;
        pieces.sort_by(|a, b| {
            b.0.len().cmp(&a.0.len())
                .then_with(|| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal))
        });

        // Build vocabulary
        let vocab: HashMap<String, u32> = pieces
            .iter()
            .enumerate()
            .map(|(i, (token, _))| (token.clone(), i as u32))
            .collect();

        // Build trie with scores
        let trie_pieces: Vec<(String, f64, u32)> = pieces
            .iter()
            .enumerate()
            .map(|(i, (token, score))| (token.clone(), *score, i as u32))
            .collect();

        let trie = Trie::from_vocab_with_scores(&trie_pieces);

        let min_score = pieces
            .iter()
            .map(|(_, score)| *score)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(f64::NEG_INFINITY);

        Self {
            pieces,
            trie,
            vocab,
            unk_token: None,
            unk_id: None,
            min_score,
        }
    }

    /// Set unknown token
    pub fn with_unk_token(mut self, unk_token: String, unk_id: u32) -> Self {
        self.unk_token = Some(unk_token);
        self.unk_id = Some(unk_id);
        self
    }

    /// Tokenize text using Viterbi algorithm
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        if text.is_empty() {
            return Vec::new();
        }

        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();

        if n == 0 {
            return Vec::new();
        }

        // Dynamic programming: best_scores[i] = best score for text[0..i]
        let mut best_scores = vec![f64::NEG_INFINITY; n + 1];
        best_scores[0] = 0.0;

        // Backpointers for reconstruction
        let mut best_token_ids = vec![None; n + 1];

        // Viterbi algorithm
        for i in 0..n {
            if best_scores[i] == f64::NEG_INFINITY {
                continue;
            }

            let suffix: String = chars[i..].iter().collect();
            let matches = self.trie.find_all_prefixes_with_scores(&suffix);

            if matches.is_empty() {
                // No match, skip single character
                let new_score = best_scores[i] + self.min_score;
                if new_score > best_scores[i + 1] {
                    best_scores[i + 1] = new_score;
                    best_token_ids[i + 1] = None; // Unknown character
                }
            } else {
                for (token, _token_id, length, score) in matches {
                    let end_pos = i + length;
                    if end_pos > n {
                        continue;
                    }

                    let new_score = best_scores[i] + score;
                    if new_score > best_scores[end_pos] {
                        best_scores[end_pos] = new_score;
                        best_token_ids[end_pos] = Some((i, token));
                    }
                }
            }
        }

        // Reconstruct path
        let mut tokens = Vec::new();
        let mut pos = n;

        while pos > 0 {
            if let Some((start, token)) = &best_token_ids[pos] {
                tokens.push(token.clone());
                pos = *start;
            } else {
                // Unknown character
                if pos > 0 {
                    if let Some(unk) = &self.unk_token {
                        tokens.push(unk.clone());
                    } else {
                        tokens.push(chars[pos - 1].to_string());
                    }
                    pos -= 1;
                }
            }
        }

        tokens.reverse();
        tokens
    }

    /// Tokenize with scores
    pub fn tokenize_with_scores(&self, text: &str) -> Vec<(String, f64)> {
        let tokens = self.tokenize(text);
        tokens
            .into_iter()
            .map(|token| {
                let score = self
                    .pieces
                    .iter()
                    .find(|(t, _)| t == &token)
                    .map(|(_, s)| *s)
                    .unwrap_or(self.min_score);
                (token, score)
            })
            .collect()
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
            .map(|token| {
                self.vocab
                    .get(token)
                    .copied()
                    .or(self.unk_id)
                    .unwrap_or(0)
            })
            .collect()
    }

    /// Decode token IDs to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let tokens = self.convert_ids_to_tokens(ids)?;
        Ok(tokens.join(""))
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

    /// Train Unigram model using Expectation-Maximization algorithm
    pub fn train(
        corpus: &[&str],
        vocab_size: usize,
        iterations: usize,
        shrinking_factor: f64,
    ) -> Result<Self> {
        // Step 1: Initialize seed vocabulary (character-level + common substrings)
        let mut candidates = Self::create_seed_vocabulary(corpus, vocab_size * 2);

        // Step 2: EM iterations
        for iteration in 0..iterations {
            // E-step: Calculate expected counts
            let expected_counts = Self::expectation_step(corpus, &candidates);

            // M-step: Update probabilities
            candidates = Self::maximization_step(&expected_counts);

            // Prune vocabulary
            if iteration % 5 == 4 {
                let target_size = (vocab_size as f64 / shrinking_factor.powi((iteration / 5) as i32)) as usize;
                candidates = Self::prune_vocabulary(candidates, target_size);
            }
        }

        // Final pruning to target size
        candidates = Self::prune_vocabulary(candidates, vocab_size);

        // Convert to log probabilities
        let total_count: f64 = candidates.iter().map(|(_, count)| count).sum();
        let pieces: Vec<(String, f64)> = candidates
            .into_iter()
            .map(|(token, count)| {
                let prob = count / total_count;
                (token, prob.ln())
            })
            .collect();

        Ok(Self::new(pieces))
    }

    /// Create seed vocabulary from corpus
    fn create_seed_vocabulary(corpus: &[&str], size: usize) -> Vec<(String, f64)> {
        let mut char_counts: HashMap<String, usize> = HashMap::new();
        let mut substring_counts: HashMap<String, usize> = HashMap::new();

        // Count characters
        for text in corpus {
            for ch in text.chars() {
                *char_counts.entry(ch.to_string()).or_insert(0) += 1;
            }

            // Count substrings (2-10 characters)
            let chars: Vec<char> = text.chars().collect();
            for i in 0..chars.len() {
                for len in 2..=10.min(chars.len() - i) {
                    let substr: String = chars[i..i + len].iter().collect();
                    *substring_counts.entry(substr).or_insert(0) += 1;
                }
            }
        }

        // Combine and sort by frequency
        let mut all_candidates: Vec<(String, usize)> = char_counts
            .into_iter()
            .chain(substring_counts)
            .collect();

        all_candidates.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top candidates
        all_candidates
            .into_iter()
            .take(size)
            .map(|(token, count)| (token, count as f64))
            .collect()
    }

    /// E-step: Calculate expected counts
    fn expectation_step(
        corpus: &[&str],
        candidates: &[(String, f64)],
    ) -> HashMap<String, f64> {
        let mut expected_counts: HashMap<String, f64> = HashMap::new();

        // Create temporary Unigram model
        let temp_model = Self::new(candidates.to_vec());

        for text in corpus {
            let tokens_with_scores = temp_model.tokenize_with_scores(text);
            let total_score: f64 = tokens_with_scores.iter().map(|(_, score)| score).sum();

            for (token, score) in tokens_with_scores {
                let expected = (score - total_score).exp();
                *expected_counts.entry(token).or_insert(0.0) += expected;
            }
        }

        expected_counts
    }

    /// M-step: Update probabilities
    fn maximization_step(expected_counts: &HashMap<String, f64>) -> Vec<(String, f64)> {
        expected_counts
            .iter()
            .map(|(token, &count)| (token.clone(), count))
            .collect()
    }

    /// Prune vocabulary to target size
    fn prune_vocabulary(mut candidates: Vec<(String, f64)>, target_size: usize) -> Vec<(String, f64)> {
        if candidates.len() <= target_size {
            return candidates;
        }

        // Sort by score (descending)
        candidates.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Keep top target_size
        candidates.truncate(target_size);
        candidates
    }

    /// Calculate token score (log probability)
    pub fn score(&self, token: &str) -> f64 {
        self.pieces
            .iter()
            .find(|(t, _)| t == token)
            .map(|(_, score)| *score)
            .unwrap_or(self.min_score)
    }

    /// Get vocabulary
    pub fn vocab(&self) -> &HashMap<String, u32> {
        &self.vocab
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Get pieces with scores
    pub fn pieces(&self) -> &[(String, f64)] {
        &self.pieces
    }

    /// Get unknown token
    pub fn unk_token(&self) -> Option<&str> {
        self.unk_token.as_deref()
    }

    /// Get unknown token ID
    pub fn unk_id(&self) -> Option<u32> {
        self.unk_id
    }

    /// Get minimum score
    pub fn min_score(&self) -> f64 {
        self.min_score
    }

    /// Calculate text log-likelihood
    pub fn log_likelihood(&self, text: &str) -> f64 {
        let tokens_with_scores = self.tokenize_with_scores(text);
        tokens_with_scores.iter().map(|(_, score)| score).sum()
    }

    /// Batch encoding
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<u32>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }

    /// Batch decoding
    pub fn decode_batch(&self, ids_batch: &[Vec<u32>]) -> Result<Vec<String>> {
        ids_batch.iter().map(|ids| self.decode(ids)).collect()
    }

    /// Sample tokenization (using scores as probabilities)
    pub fn sample(&self, text: &str, temperature: f64) -> Vec<String> {
        use rand::distributions::WeightedIndex;
        use rand::prelude::*;

        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        let mut tokens = Vec::new();
        let mut pos = 0;
        let mut rng = rand::thread_rng();

        while pos < n {
            let suffix: String = chars[pos..].iter().collect();
            let matches = self.trie.find_all_prefixes_with_scores(&suffix);

            if matches.is_empty() {
                tokens.push(chars[pos].to_string());
                pos += 1;
                continue;
            }

            // Calculate probabilities with temperature
            let probs: Vec<f64> = matches
                .iter()
                .map(|(_, _, _, score)| (score / temperature).exp())
                .collect();

            let dist = WeightedIndex::new(&probs).unwrap();
            let choice = dist.sample(&mut rng);

            let (token, _, length, _) = &matches[choice];
            tokens.push(token.clone());
            pos += length;
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_unigram() -> Unigram {
        let pieces = vec![
            ("hello".to_string(), -1.0),
            ("world".to_string(), -1.5),
            ("h".to_string(), -3.0),
            ("e".to_string(), -3.0),
            ("l".to_string(), -3.0),
            ("o".to_string(), -3.0),
            ("w".to_string(), -3.5),
            ("r".to_string(), -3.5),
            ("d".to_string(), -3.5),
        ];

        Unigram::new(pieces)
    }

    #[test]
    fn test_unigram_tokenize() {
        let unigram = create_test_unigram();
        let tokens = unigram.tokenize("hello");

        assert!(!tokens.is_empty());
        // Should prefer "hello" over character splits due to higher score
        assert!(tokens.contains(&"hello".to_string()));
    }

    #[test]
    fn test_unigram_tokenize_with_scores() {
        let unigram = create_test_unigram();
        let tokens = unigram.tokenize_with_scores("hello");

        assert!(!tokens.is_empty());
        for (token, score) in &tokens {
            assert!(score.is_finite());
            assert!(!token.is_empty());
        }
    }

    #[test]
    fn test_unigram_encode() {
        let unigram = create_test_unigram();
        let ids = unigram.encode("hello");

        assert!(!ids.is_empty());
    }

    #[test]
    fn test_unigram_encode_decode() {
        let unigram = create_test_unigram();
        let text = "hello";
        let ids = unigram.encode(text);
        let decoded = unigram.decode(&ids).unwrap();

        // Decoded text should match or be close to original
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_unigram_score() {
        let unigram = create_test_unigram();

        let score = unigram.score("hello");
        assert_eq!(score, -1.0);

        let score = unigram.score("world");
        assert_eq!(score, -1.5);
    }

    #[test]
    fn test_unigram_vocab_size() {
        let unigram = create_test_unigram();
        assert_eq!(unigram.vocab_size(), 9);
    }

    #[test]
    fn test_unigram_log_likelihood() {
        let unigram = create_test_unigram();
        let ll = unigram.log_likelihood("hello");

        assert!(ll.is_finite());
        assert!(ll < 0.0); // Log probabilities are negative
    }

    #[test]
    fn test_unigram_batch_encode() {
        let unigram = create_test_unigram();
        let texts = vec!["hello", "world"];
        let batch = unigram.encode_batch(&texts);

        assert_eq!(batch.len(), 2);
        assert!(!batch[0].is_empty());
        assert!(!batch[1].is_empty());
    }

    #[test]
    fn test_unigram_with_unk_token() {
        let unigram = create_test_unigram().with_unk_token("[UNK]".to_string(), 999);

        assert_eq!(unigram.unk_token(), Some("[UNK]"));
        assert_eq!(unigram.unk_id(), Some(999));
    }

    #[test]
    fn test_unigram_pieces_sorted() {
        let unigram = create_test_unigram();
        let pieces = unigram.pieces();

        // Should be sorted by length (descending)
        for i in 1..pieces.len() {
            assert!(pieces[i - 1].0.len() >= pieces[i].0.len());
        }
    }

    #[test]
    fn test_unigram_min_score() {
        let unigram = create_test_unigram();
        let min_score = unigram.min_score();

        assert!(min_score.is_finite());

        for (_, score) in unigram.pieces() {
            assert!(*score >= min_score);
        }
    }
}
