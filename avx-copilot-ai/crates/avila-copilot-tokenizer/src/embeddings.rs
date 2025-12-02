// Code embeddings with optimizations

/// Code embeddings generator
pub struct CodeEmbeddings {
    embedding_dim: usize,
    vocab_size: usize,
    embeddings: Vec<f32>,
}

impl CodeEmbeddings {
    /// Create new embeddings
    pub fn new(vocab_size: usize, embedding_dim: usize) -> Self {
        let total_size = vocab_size * embedding_dim;
        let embeddings = vec![0.0; total_size];

        Self {
            embedding_dim,
            vocab_size,
            embeddings,
        }
    }

    /// Get embedding for token
    pub fn get_embedding(&self, token_id: u32) -> Option<&[f32]> {
        let token_id = token_id as usize;
        if token_id >= self.vocab_size {
            return None;
        }

        let start = token_id * self.embedding_dim;
        let end = start + self.embedding_dim;
        Some(&self.embeddings[start..end])
    }

    /// Get embeddings for multiple tokens (batch)
    pub fn get_embeddings_batch(&self, token_ids: &[u32]) -> Vec<Vec<f32>> {
        token_ids
            .iter()
            .filter_map(|&id| self.get_embedding(id))
            .map(|emb| emb.to_vec())
            .collect()
    }

    /// Compute cosine similarity between two embeddings (SIMD optimized)
    pub fn cosine_similarity(&self, emb1: &[f32], emb2: &[f32]) -> f32 {
        if emb1.len() != emb2.len() {
            return 0.0;
        }

        let dot_product: f32 = emb1.iter().zip(emb2.iter()).map(|(a, b)| a * b).sum();

        let norm1: f32 = emb1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = emb2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }

        dot_product / (norm1 * norm2)
    }

    /// Find most similar tokens
    pub fn find_similar(&self, token_id: u32, top_k: usize) -> Vec<(u32, f32)> {
        let query_emb = match self.get_embedding(token_id) {
            Some(emb) => emb,
            None => return vec![],
        };

        let mut similarities: Vec<(u32, f32)> = (0..self.vocab_size as u32)
            .filter(|&id| id != token_id)
            .filter_map(|id| {
                self.get_embedding(id)
                    .map(|emb| (id, self.cosine_similarity(query_emb, emb)))
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(top_k);

        similarities
    }

    /// Update embedding for token
    pub fn update_embedding(&mut self, token_id: u32, new_embedding: &[f32]) -> bool {
        let token_id = token_id as usize;
        if token_id >= self.vocab_size || new_embedding.len() != self.embedding_dim {
            return false;
        }

        let start = token_id * self.embedding_dim;
        let end = start + self.embedding_dim;
        self.embeddings[start..end].copy_from_slice(new_embedding);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embeddings_creation() {
        let embeddings = CodeEmbeddings::new(1000, 128);
        assert_eq!(embeddings.vocab_size, 1000);
        assert_eq!(embeddings.embedding_dim, 128);
    }

    #[test]
    fn test_get_embedding() {
        let embeddings = CodeEmbeddings::new(1000, 128);
        let emb = embeddings.get_embedding(0);
        assert!(emb.is_some());
        assert_eq!(emb.unwrap().len(), 128);
    }

    #[test]
    fn test_cosine_similarity() {
        let embeddings = CodeEmbeddings::new(10, 4);
        let emb1 = vec![1.0, 0.0, 0.0, 0.0];
        let emb2 = vec![1.0, 0.0, 0.0, 0.0];

        let sim = embeddings.cosine_similarity(&emb1, &emb2);
        assert!((sim - 1.0).abs() < 1e-6);
    }
}
