use std::collections::HashMap;

/// A Trie (prefix tree) data structure for efficient token lookup
/// Used for longest-match-first strategies in WordPiece and Unigram
#[derive(Debug, Clone)]
pub struct TrieNode {
    /// Token ID if this node represents a complete token
    pub token_id: Option<u32>,
    /// Child nodes indexed by character
    pub children: HashMap<char, Box<TrieNode>>,
    /// The complete token string at this node
    pub token: Option<String>,
    /// Score for Unigram model
    pub score: Option<f64>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            token_id: None,
            children: HashMap::new(),
            token: None,
            score: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.token_id.is_some()
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

/// Trie data structure for vocabulary
#[derive(Debug, Clone)]
pub struct Trie {
    root: TrieNode,
    size: usize,
}

impl Trie {
    /// Create a new empty Trie
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
            size: 0,
        }
    }

    /// Insert a token into the trie
    pub fn insert(&mut self, token: &str, token_id: u32) {
        self.insert_with_score(token, token_id, None);
    }

    /// Insert a token with score (for Unigram)
    pub fn insert_with_score(&mut self, token: &str, token_id: u32, score: Option<f64>) {
        let mut current = &mut self.root;

        for ch in token.chars() {
            current = current
                .children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }

        if current.token_id.is_none() {
            self.size += 1;
        }

        current.token_id = Some(token_id);
        current.token = Some(token.to_string());
        current.score = score;
    }

    /// Search for a token in the trie
    pub fn search(&self, token: &str) -> Option<u32> {
        let mut current = &self.root;

        for ch in token.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return None,
            }
        }

        current.token_id
    }

    /// Check if token exists in trie
    pub fn contains(&self, token: &str) -> bool {
        self.search(token).is_some()
    }

    /// Find all tokens that are prefixes of the given text
    /// Returns (token, token_id, end_position)
    pub fn find_all_prefixes(&self, text: &str) -> Vec<(String, u32, usize)> {
        let mut results = Vec::new();
        let mut current = &self.root;
        let chars: Vec<char> = text.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            match current.children.get(&ch) {
                Some(node) => {
                    current = node;
                    if let Some(token_id) = current.token_id {
                        if let Some(token) = &current.token {
                            results.push((token.clone(), token_id, i + 1));
                        }
                    }
                }
                None => break,
            }
        }

        results
    }

    /// Find the longest token that matches the beginning of text
    /// Returns (token, token_id, length)
    pub fn find_longest_prefix(&self, text: &str) -> Option<(String, u32, usize)> {
        let prefixes = self.find_all_prefixes(text);
        prefixes.into_iter().max_by_key(|(_, _, len)| *len)
    }

    /// Find all tokens with their scores (for Unigram)
    pub fn find_all_prefixes_with_scores(&self, text: &str) -> Vec<(String, u32, usize, f64)> {
        let mut results = Vec::new();
        let mut current = &self.root;
        let chars: Vec<char> = text.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            match current.children.get(&ch) {
                Some(node) => {
                    current = node;
                    if let Some(token_id) = current.token_id {
                        if let Some(token) = &current.token {
                            let score = current.score.unwrap_or(0.0);
                            results.push((token.clone(), token_id, i + 1, score));
                        }
                    }
                }
                None => break,
            }
        }

        results
    }

    /// Get the size of the trie (number of tokens)
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if trie is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Clear the trie
    pub fn clear(&mut self) {
        self.root = TrieNode::new();
        self.size = 0;
    }

    /// Build trie from vocabulary
    pub fn from_vocab(vocab: &HashMap<String, u32>) -> Self {
        let mut trie = Trie::new();
        for (token, &id) in vocab.iter() {
            trie.insert(token, id);
        }
        trie
    }

    /// Build trie from vocabulary with scores
    pub fn from_vocab_with_scores(pieces: &[(String, f64, u32)]) -> Self {
        let mut trie = Trie::new();
        for (token, score, id) in pieces.iter() {
            trie.insert_with_score(token, *id, Some(*score));
        }
        trie
    }

    /// Get all tokens in the trie
    pub fn get_all_tokens(&self) -> Vec<(String, u32)> {
        let mut tokens = Vec::new();
        Self::collect_tokens_from(&self.root, String::new(), &mut tokens);
        tokens
    }

    fn collect_tokens_from(node: &TrieNode, prefix: String, tokens: &mut Vec<(String, u32)>) {
        if let Some(token_id) = node.token_id {
            tokens.push((prefix.clone(), token_id));
        }

        for (&ch, child) in &node.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(ch);
            Self::collect_tokens_from(child, new_prefix, tokens);
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert_and_search() {
        let mut trie = Trie::new();

        trie.insert("hello", 1);
        trie.insert("world", 2);
        trie.insert("help", 3);

        assert_eq!(trie.search("hello"), Some(1));
        assert_eq!(trie.search("world"), Some(2));
        assert_eq!(trie.search("help"), Some(3));
        assert_eq!(trie.search("not_found"), None);
        assert_eq!(trie.len(), 3);
    }

    #[test]
    fn test_trie_find_all_prefixes() {
        let mut trie = Trie::new();

        trie.insert("h", 1);
        trie.insert("he", 2);
        trie.insert("hel", 3);
        trie.insert("hello", 4);

        let prefixes = trie.find_all_prefixes("hello world");
        assert_eq!(prefixes.len(), 4);
        assert!(prefixes.iter().any(|(t, _, _)| t == "h"));
        assert!(prefixes.iter().any(|(t, _, _)| t == "he"));
        assert!(prefixes.iter().any(|(t, _, _)| t == "hel"));
        assert!(prefixes.iter().any(|(t, _, _)| t == "hello"));
    }

    #[test]
    fn test_trie_find_longest_prefix() {
        let mut trie = Trie::new();

        trie.insert("h", 1);
        trie.insert("he", 2);
        trie.insert("hello", 3);

        let longest = trie.find_longest_prefix("hello world");
        assert_eq!(longest, Some(("hello".to_string(), 3, 5)));
    }

    #[test]
    fn test_trie_contains() {
        let mut trie = Trie::new();

        trie.insert("test", 1);

        assert!(trie.contains("test"));
        assert!(!trie.contains("testing"));
        assert!(!trie.contains("tes"));
    }

    #[test]
    fn test_trie_from_vocab() {
        let mut vocab = HashMap::new();
        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);

        let trie = Trie::from_vocab(&vocab);

        assert_eq!(trie.len(), 2);
        assert_eq!(trie.search("hello"), Some(1));
        assert_eq!(trie.search("world"), Some(2));
    }

    #[test]
    fn test_trie_with_scores() {
        let mut trie = Trie::new();

        trie.insert_with_score("hello", 1, Some(-2.5));
        trie.insert_with_score("world", 2, Some(-3.0));

        let prefixes = trie.find_all_prefixes_with_scores("hello");
        assert_eq!(prefixes.len(), 1);
        assert_eq!(prefixes[0].0, "hello");
        assert_eq!(prefixes[0].3, -2.5);
    }

    #[test]
    fn test_trie_clear() {
        let mut trie = Trie::new();

        trie.insert("test", 1);
        assert_eq!(trie.len(), 1);

        trie.clear();
        assert_eq!(trie.len(), 0);
        assert!(trie.is_empty());
    }

    #[test]
    fn test_trie_get_all_tokens() {
        let mut trie = Trie::new();

        trie.insert("hello", 1);
        trie.insert("world", 2);

        let tokens = trie.get_all_tokens();
        assert_eq!(tokens.len(), 2);
        assert!(tokens.iter().any(|(t, id)| t == "hello" && *id == 1));
        assert!(tokens.iter().any(|(t, id)| t == "world" && *id == 2));
    }
}
