use std::collections::HashMap;
use crate::error::Result;

/// Character-level tokenizer (used in ByT5 and character-level models)
#[derive(Debug, Clone)]
pub struct CharTokenizer {
    /// Character to ID mapping
    vocab: HashMap<char, u32>,
    /// ID to character mapping
    id_to_char: HashMap<u32, char>,
    /// Unknown character ID
    unk_id: Option<u32>,
}

impl CharTokenizer {
    /// Create a new character tokenizer
    pub fn new() -> Self {
        Self {
            vocab: HashMap::new(),
            id_to_char: HashMap::new(),
            unk_id: None,
        }
    }

    /// Create from character set
    pub fn from_chars(chars: Vec<char>) -> Self {
        let vocab: HashMap<char, u32> = chars
            .iter()
            .enumerate()
            .map(|(i, &ch)| (ch, i as u32))
            .collect();

        let id_to_char: HashMap<u32, char> = vocab
            .iter()
            .map(|(&ch, &id)| (id, ch))
            .collect();

        Self {
            vocab,
            id_to_char,
            unk_id: None,
        }
    }

    /// Create with unknown character
    pub fn with_unk_id(mut self, unk_id: u32) -> Self {
        self.unk_id = Some(unk_id);
        self
    }

    /// Tokenize text to characters
    pub fn tokenize(&self, text: &str) -> Vec<char> {
        text.chars().collect()
    }

    /// Encode text to character IDs
    pub fn encode(&self, text: &str) -> Vec<u32> {
        text.chars()
            .map(|ch| {
                self.vocab
                    .get(&ch)
                    .copied()
                    .or(self.unk_id)
                    .unwrap_or(0)
            })
            .collect()
    }

    /// Decode character IDs to text
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        let chars: Vec<char> = ids
            .iter()
            .filter_map(|&id| self.id_to_char.get(&id).copied())
            .collect();

        Ok(chars.into_iter().collect())
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Build vocabulary from corpus
    pub fn build_vocab(corpus: &[&str]) -> Self {
        let mut chars = std::collections::HashSet::new();

        for text in corpus {
            for ch in text.chars() {
                chars.insert(ch);
            }
        }

        let mut char_vec: Vec<char> = chars.into_iter().collect();
        char_vec.sort();

        Self::from_chars(char_vec)
    }
}

impl Default for CharTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_tokenizer_basic() {
        let chars = vec!['a', 'b', 'c', 'd', 'e'];
        let tokenizer = CharTokenizer::from_chars(chars);

        let text = "abcde";
        let ids = tokenizer.encode(text);
        let decoded = tokenizer.decode(&ids).unwrap();

        assert_eq!(decoded, text);
    }

    #[test]
    fn test_char_tokenizer_build_vocab() {
        let corpus = vec!["hello", "world"];
        let tokenizer = CharTokenizer::build_vocab(&corpus);

        assert!(tokenizer.vocab_size() > 0);
    }
}
