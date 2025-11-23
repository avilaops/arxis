//! Text and Document Clustering
//!
//! Specialized clustering for text data using TF-IDF, embeddings,
//! and semantic similarity measures.

use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use std::collections::{HashMap, HashSet};

/// TF-IDF Vectorizer for converting text to numerical features
pub struct TfidfVectorizer {
    vocabulary: HashMap<String, usize>,
    idf: Array1<f64>,
    max_features: Option<usize>,
    min_df: usize,
    max_df: f64,
}

impl TfidfVectorizer {
    pub fn new() -> Self {
        Self {
            vocabulary: HashMap::new(),
            idf: Array1::zeros(0),
            max_features: None,
            min_df: 1,
            max_df: 1.0,
        }
    }

    pub fn max_features(mut self, n: usize) -> Self {
        self.max_features = Some(n);
        self
    }

    pub fn min_df(mut self, min: usize) -> Self {
        self.min_df = min;
        self
    }

    pub fn max_df(mut self, max: f64) -> Self {
        self.max_df = max;
        self
    }

    /// Fit and transform documents to TF-IDF matrix
    pub fn fit_transform(&mut self, documents: &[String]) -> Result<Array2<f64>> {
        if documents.is_empty() {
            return Err(ClusteringError::InvalidParameter("Empty documents".to_string()));
        }

        // Build vocabulary
        let mut term_doc_count: HashMap<String, usize> = HashMap::new();
        let mut doc_terms: Vec<Vec<String>> = Vec::new();

        for doc in documents {
            let terms = self.tokenize(doc);
            let unique_terms: HashSet<_> = terms.iter().cloned().collect();

            for term in &unique_terms {
                *term_doc_count.entry(term.clone()).or_insert(0) += 1;
            }

            doc_terms.push(terms);
        }

        // Filter by document frequency
        let n_docs = documents.len() as f64;
        let mut vocab_list: Vec<(String, usize)> = term_doc_count
            .into_iter()
            .filter(|(_, count)| {
                *count >= self.min_df && (*count as f64 / n_docs) <= self.max_df
            })
            .collect();

        // Limit vocabulary size
        if let Some(max_feat) = self.max_features {
            vocab_list.sort_by(|a, b| b.1.cmp(&a.1));
            vocab_list.truncate(max_feat);
        }

        // Build vocabulary index
        self.vocabulary = vocab_list
            .iter()
            .enumerate()
            .map(|(i, (term, _))| (term.clone(), i))
            .collect();

        let vocab_size = self.vocabulary.len();

        // Compute IDF
        self.idf = Array1::zeros(vocab_size);
        for (term, idx) in &self.vocabulary {
            let df = vocab_list.iter()
                .find(|(t, _)| t == term)
                .map(|(_, count)| *count)
                .unwrap_or(1);

            self.idf[*idx] = (n_docs / df as f64).ln() + 1.0;
        }

        // Compute TF-IDF matrix
        let mut tfidf = Array2::<f64>::zeros((documents.len(), vocab_size));

        for (doc_idx, terms) in doc_terms.iter().enumerate() {
            let mut term_freq: HashMap<usize, usize> = HashMap::new();

            for term in terms {
                if let Some(&vocab_idx) = self.vocabulary.get(term) {
                    *term_freq.entry(vocab_idx).or_insert(0) += 1;
                }
            }

            let total_terms = terms.len() as f64;

            for (vocab_idx, count) in term_freq {
                let tf = count as f64 / total_terms;
                tfidf[[doc_idx, vocab_idx]] = tf * self.idf[vocab_idx];
            }

            // L2 normalization
            let norm: f64 = tfidf.row(doc_idx).iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                for i in 0..vocab_size {
                    tfidf[[doc_idx, i]] /= norm;
                }
            }
        }

        Ok(tfidf)
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|word| word.len() > 2)
            .collect()
    }
}

impl Default for TfidfVectorizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Topic modeling using clustering
pub struct TopicModeling {
    n_topics: usize,
    n_top_words: usize,
}

impl TopicModeling {
    pub fn new(n_topics: usize) -> Self {
        Self {
            n_topics,
            n_top_words: 10,
        }
    }

    pub fn n_top_words(mut self, n: usize) -> Self {
        self.n_top_words = n;
        self
    }

    /// Extract topics from documents using K-Means on TF-IDF
    pub fn fit(&self, documents: &[String]) -> Result<TopicModelResult> {
        // Convert to TF-IDF
        let mut vectorizer = TfidfVectorizer::new()
            .max_features(1000)
            .min_df(2);

        let tfidf = vectorizer.fit_transform(documents)?;

        // Cluster documents (simple K-Means)
        let labels = self.cluster_documents(&tfidf)?;

        // Extract top words per topic
        let topics = self.extract_topics(&tfidf, &labels, &vectorizer.vocabulary);

        Ok(TopicModelResult {
            labels,
            topics,
            n_topics: self.n_topics,
        })
    }

    fn cluster_documents(&self, tfidf: &Array2<f64>) -> Result<Vec<usize>> {
        // Simple K-Means clustering
        let n_docs = tfidf.nrows();
        let n_features = tfidf.ncols();

        // Initialize centroids
        let mut centroids = Array2::<f64>::zeros((self.n_topics, n_features));
        for k in 0..self.n_topics {
            centroids.row_mut(k).assign(&tfidf.row(k % n_docs));
        }

        let mut labels = vec![0; n_docs];

        for _iter in 0..20 {
            // Assign to nearest centroid (cosine similarity)
            for i in 0..n_docs {
                let doc = tfidf.row(i);
                let mut max_sim = -1.0;
                let mut best_k = 0;

                for k in 0..self.n_topics {
                    let sim = Self::cosine_similarity(doc, centroids.row(k));
                    if sim > max_sim {
                        max_sim = sim;
                        best_k = k;
                    }
                }

                labels[i] = best_k;
            }

            // Update centroids
            centroids.fill(0.0);
            let mut counts = vec![0; self.n_topics];

            for i in 0..n_docs {
                let k = labels[i];
                for j in 0..n_features {
                    centroids[[k, j]] += tfidf[[i, j]];
                }
                counts[k] += 1;
            }

            for k in 0..self.n_topics {
                if counts[k] > 0 {
                    for j in 0..n_features {
                        centroids[[k, j]] /= counts[k] as f64;
                    }
                }
            }
        }

        Ok(labels)
    }

    fn extract_topics(
        &self,
        tfidf: &Array2<f64>,
        labels: &[usize],
        vocabulary: &HashMap<String, usize>,
    ) -> Vec<Topic> {
        let mut topics = Vec::new();
        let n_features = tfidf.ncols();

        // Reverse vocabulary
        let mut idx_to_word: HashMap<usize, String> = vocabulary
            .iter()
            .map(|(word, &idx)| (idx, word.clone()))
            .collect();

        for topic_id in 0..self.n_topics {
            // Get average TF-IDF for this topic
            let mut topic_vec = Array1::<f64>::zeros(n_features);
            let mut count = 0;

            for (i, &label) in labels.iter().enumerate() {
                if label == topic_id {
                    topic_vec = &topic_vec + &tfidf.row(i).to_owned();
                    count += 1;
                }
            }

            if count > 0 {
                topic_vec /= count as f64;
            }

            // Get top words
            let mut word_scores: Vec<(String, f64)> = topic_vec
                .iter()
                .enumerate()
                .filter_map(|(idx, &score)| {
                    idx_to_word.get(&idx).map(|word| (word.clone(), score))
                })
                .collect();

            word_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            word_scores.truncate(self.n_top_words);

            topics.push(Topic {
                id: topic_id,
                words: word_scores,
            });
        }

        topics
    }

    fn cosine_similarity(a: ArrayView1<f64>, b: ArrayView1<f64>) -> f64 {
        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a > 0.0 && norm_b > 0.0 {
            dot / (norm_a * norm_b)
        } else {
            0.0
        }
    }
}

pub struct TopicModelResult {
    pub labels: Vec<usize>,
    pub topics: Vec<Topic>,
    pub n_topics: usize,
}

pub struct Topic {
    pub id: usize,
    pub words: Vec<(String, f64)>,
}

impl Topic {
    pub fn display(&self) -> String {
        let words: Vec<String> = self.words
            .iter()
            .map(|(word, score)| format!("{}({:.3})", word, score))
            .collect();

        format!("Topic {}: {}", self.id, words.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tfidf_vectorizer() {
        let documents = vec![
            "machine learning is great".to_string(),
            "deep learning is amazing".to_string(),
            "natural language processing".to_string(),
        ];

        let mut vectorizer = TfidfVectorizer::new();
        let tfidf = vectorizer.fit_transform(&documents).unwrap();

        assert_eq!(tfidf.nrows(), 3);
        assert!(tfidf.ncols() > 0);
    }
}
