//! Deep web indexer
//!
//! Index .onion sites, full-text search, inverted index

use std::collections::{BTreeMap, BTreeSet};

/// Document in index
#[derive(Debug, Clone)]
pub struct Document {
    pub id: u64,
    pub onion_address: String,
    pub title: String,
    pub content: String,
    pub keywords: Vec<String>,
    pub indexed_at: u64,
}

/// Inverted index for full-text search
#[derive(Debug)]
pub struct InvertedIndex {
    pub index: BTreeMap<String, BTreeSet<u64>>,  // term -> document IDs
    pub documents: BTreeMap<u64, Document>,
    next_id: u64,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
            documents: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Add document to index
    pub fn add_document(&mut self, onion: String, title: String, content: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        // Tokenize and normalize
        let tokens = tokenize(&format!("{} {}", title, content));

        // Build inverted index
        for token in &tokens {
            self.index
                .entry(token.clone())
                .or_insert_with(BTreeSet::new)
                .insert(id);
        }

        // Store document
        let doc = Document {
            id,
            onion_address: onion,
            title,
            content,
            keywords: tokens,
            indexed_at: current_timestamp(),
        };

        self.documents.insert(id, doc);

        id
    }

    /// Search for documents
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let terms = tokenize(query);

        if terms.is_empty() {
            return Vec::new();
        }

        // Get documents matching ALL terms (AND query)
        let mut matching_docs: Option<BTreeSet<u64>> = None;

        for term in &terms {
            if let Some(doc_ids) = self.index.get(term) {
                matching_docs = Some(match matching_docs {
                    None => doc_ids.clone(),
                    Some(existing) => existing.intersection(doc_ids).cloned().collect(),
                });
            } else {
                // Term not found - no results
                return Vec::new();
            }
        }

        let matching_docs = matching_docs.unwrap_or_default();

        // Calculate relevance scores
        let mut results: Vec<SearchResult> = matching_docs
            .iter()
            .filter_map(|&id| {
                self.documents.get(&id).map(|doc| {
                    let score = calculate_relevance(doc, &terms);
                    SearchResult {
                        document: doc.clone(),
                        score,
                    }
                })
            })
            .collect();

        // Sort by relevance (descending)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        results
    }

    /// Search with OR (any term matches)
    pub fn search_or(&self, query: &str) -> Vec<SearchResult> {
        let terms = tokenize(query);

        if terms.is_empty() {
            return Vec::new();
        }

        // Get documents matching ANY term
        let mut matching_docs = BTreeSet::new();

        for term in &terms {
            if let Some(doc_ids) = self.index.get(term) {
                matching_docs.extend(doc_ids);
            }
        }

        // Calculate scores
        let mut results: Vec<SearchResult> = matching_docs
            .iter()
            .filter_map(|&id| {
                self.documents.get(&id).map(|doc| {
                    let score = calculate_relevance(doc, &terms);
                    SearchResult {
                        document: doc.clone(),
                        score,
                    }
                })
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        results
    }

    /// Get index statistics
    pub fn stats(&self) -> IndexStats {
        IndexStats {
            documents: self.documents.len(),
            terms: self.index.len(),
            avg_terms_per_doc: if self.documents.is_empty() {
                0.0
            } else {
                self.documents.values()
                    .map(|d| d.keywords.len())
                    .sum::<usize>() as f64 / self.documents.len() as f64
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub document: Document,
    pub score: f64,
}

#[derive(Debug)]
pub struct IndexStats {
    pub documents: usize,
    pub terms: usize,
    pub avg_terms_per_doc: f64,
}

// ============================================================================
// Text Processing
// ============================================================================

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty() && s.len() > 2)  // Skip short words
        .filter(|s| !is_stopword(s))
        .map(|s| s.to_string())
        .collect()
}

fn is_stopword(word: &str) -> bool {
    // Common English stopwords
    matches!(
        word,
        "the" | "and" | "for" | "are" | "but" | "not" | "you" | "all" | "can" | "her" | "was" | "one" | "our" | "out" | "has" | "this" | "that" | "with" | "from"
    )
}

fn calculate_relevance(doc: &Document, query_terms: &[String]) -> f64 {
    let mut score = 0.0;

    // TF (Term Frequency)
    for term in query_terms {
        let tf_title = doc.title.to_lowercase().matches(term).count() as f64;
        let tf_content = doc.content.to_lowercase().matches(term).count() as f64;

        // Title matches worth more
        score += tf_title * 3.0 + tf_content;
    }

    // Normalize by document length
    let doc_length = doc.content.len() as f64;
    if doc_length > 0.0 {
        score /= doc_length.sqrt();
    }

    score
}

fn current_timestamp() -> u64 {
    1700000000
}

/// Category-based indexer
#[derive(Debug)]
pub struct CategoryIndex {
    pub categories: BTreeMap<String, Vec<String>>,  // category -> onion addresses
}

impl CategoryIndex {
    pub fn new() -> Self {
        Self {
            categories: BTreeMap::new(),
        }
    }

    /// Add site to category
    pub fn add(&mut self, category: String, onion: String) {
        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(onion);
    }

    /// Get all sites in category
    pub fn get_category(&self, category: &str) -> Vec<String> {
        self.categories
            .get(category)
            .cloned()
            .unwrap_or_default()
    }

    /// List all categories
    pub fn list_categories(&self) -> Vec<(String, usize)> {
        self.categories
            .iter()
            .map(|(cat, sites)| (cat.clone(), sites.len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverted_index() {
        let mut index = InvertedIndex::new();

        index.add_document(
            "marketplace.onion".to_string(),
            "Dark Marketplace".to_string(),
            "Buy and sell products anonymously".to_string(),
        );

        index.add_document(
            "forum.onion".to_string(),
            "Tech Forum".to_string(),
            "Discuss technology and privacy".to_string(),
        );

        // Search
        let results = index.search("marketplace");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].document.onion_address, "marketplace.onion");
    }

    #[test]
    fn test_search_or() {
        let mut index = InvertedIndex::new();

        index.add_document("site1.onion".to_string(), "Bitcoin".to_string(), "Crypto payments".to_string());
        index.add_document("site2.onion".to_string(), "Forums".to_string(), "Privacy discussions".to_string());

        let results = index.search_or("bitcoin privacy");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello World! This is a test.");
        assert!(tokens.contains(&"hello".to_string()));
        assert!(tokens.contains(&"world".to_string()));
        assert!(tokens.contains(&"test".to_string()));
        // Should not contain stopwords
        assert!(!tokens.contains(&"is".to_string()));
    }

    #[test]
    fn test_category_index() {
        let mut index = CategoryIndex::new();

        index.add("marketplace".to_string(), "shop1.onion".to_string());
        index.add("marketplace".to_string(), "shop2.onion".to_string());
        index.add("forum".to_string(), "forum1.onion".to_string());

        let marketplaces = index.get_category("marketplace");
        assert_eq!(marketplaces.len(), 2);

        let categories = index.list_categories();
        assert_eq!(categories.len(), 2);
    }
}
