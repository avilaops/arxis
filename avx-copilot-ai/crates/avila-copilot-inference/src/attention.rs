// Attention mechanisms with optimizations

/// Multi-head attention
pub struct MultiHeadAttention {
    num_heads: usize,
    head_dim: usize,
}

impl MultiHeadAttention {
    pub fn new(num_heads: usize, hidden_size: usize) -> Self {
        let head_dim = hidden_size / num_heads;
        Self { num_heads, head_dim }
    }

    /// Compute attention scores (SIMD optimized)
    pub fn compute_scores(&self, q: &[f32], k: &[f32]) -> Vec<f32> {
        let seq_len = q.len() / (self.num_heads * self.head_dim);
        let mut scores = vec![0.0; seq_len * seq_len];

        // Dot product for each head
        for head in 0..self.num_heads {
            for i in 0..seq_len {
                for j in 0..seq_len {
                    let score = self.dot_product_simd(
                        &q[head * self.head_dim..(head + 1) * self.head_dim],
                        &k[head * self.head_dim..(head + 1) * self.head_dim],
                    );
                    scores[i * seq_len + j] = score;
                }
            }
        }

        scores
    }

    /// SIMD-optimized dot product
    fn dot_product_simd(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        // Simple dot product (will be optimized with avila-simd)
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    /// Apply softmax to attention scores
    pub fn softmax(&self, scores: &[f32]) -> Vec<f32> {
        let max_score = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);

        let exp_scores: Vec<f32> = scores.iter().map(|&s| (s - max_score).exp()).collect();
        let sum: f32 = exp_scores.iter().sum();

        exp_scores.iter().map(|&s| s / sum).collect()
    }

    /// Apply attention to values
    pub fn apply_attention(&self, attention_weights: &[f32], values: &[f32]) -> Vec<f32> {
        let seq_len = attention_weights.len();
        let mut output = vec![0.0; values.len()];

        for i in 0..seq_len {
            for j in 0..self.head_dim {
                output[i * self.head_dim + j] += attention_weights[i] * values[i * self.head_dim + j];
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_creation() {
        let attention = MultiHeadAttention::new(12, 768);
        assert_eq!(attention.num_heads, 12);
        assert_eq!(attention.head_dim, 64);
    }

    #[test]
    fn test_softmax() {
        let attention = MultiHeadAttention::new(1, 64);
        let scores = vec![1.0, 2.0, 3.0];
        let probs = attention.softmax(&scores);

        let sum: f32 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
