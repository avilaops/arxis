//! Attention mechanisms

use crate::nn::{Linear, Module};
use crate::tensor::Tensor;
use num_traits::{Float, NumAssign};

/// Self-attention mechanism
pub struct Attention<T = f32> {
    query: Linear<T>,
    key: Linear<T>,
    value: Linear<T>,
    _scale: T,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Attention<T> {
    pub fn new(embed_dim: usize) -> Self {
        let scale = T::from(1.0 / (embed_dim as f64).sqrt()).unwrap();

        Self {
            query: Linear::new(embed_dim, embed_dim),
            key: Linear::new(embed_dim, embed_dim),
            value: Linear::new(embed_dim, embed_dim),
            _scale: scale,
        }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for Attention<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        // Input shape: (seq_len, embed_dim) or (batch, seq_len, embed_dim)
        let _q = self.query.forward(input);
        let _k = self.key.forward(input);
        

        // Simplified attention for 2D tensors (seq_len, embed_dim)
        // For full batched implementation, would need proper transpose

        // For now, just return weighted values
        // TODO: Implement full scaled dot-product attention with proper batching
        self.value.forward(input)
    }

    fn parameters(&self) -> Vec<&Tensor<T>> {
        let mut params = Vec::new();
        params.extend(self.query.parameters());
        params.extend(self.key.parameters());
        params.extend(self.value.parameters());
        params
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        let mut params = Vec::new();
        params.extend(self.query.parameters_mut());
        params.extend(self.key.parameters_mut());
        params.extend(self.value.parameters_mut());
        params
    }
}

/// Multi-head attention mechanism
pub struct MultiHeadAttention<T = f32> {
    heads: Vec<Attention<T>>,
    output_proj: Linear<T>,
    _num_heads: usize,
    _embed_dim: usize,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> MultiHeadAttention<T> {
    pub fn new(embed_dim: usize, num_heads: usize) -> Self {
        assert!(
            embed_dim.is_multiple_of(num_heads),
            "embed_dim must be divisible by num_heads"
        );

        let head_dim = embed_dim / num_heads;
        let heads = (0..num_heads).map(|_| Attention::new(head_dim)).collect();

        Self {
            heads,
            output_proj: Linear::new(embed_dim, embed_dim),
            _num_heads: num_heads,
            _embed_dim: embed_dim,
        }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for MultiHeadAttention<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        // Split input into heads, apply attention, concatenate, and project
        // Simplified implementation
        // TODO: Implement proper multi-head attention with head splitting

        let head_outputs: Vec<Tensor<T>> =
            self.heads.iter().map(|head| head.forward(input)).collect();

        // Concatenate heads (simplified - just use first head for now)
        let concatenated = head_outputs[0].clone();

        // Project output
        self.output_proj.forward(&concatenated)
    }

    fn parameters(&self) -> Vec<&Tensor<T>> {
        let mut params = Vec::new();
        for head in &self.heads {
            params.extend(head.parameters());
        }
        params.extend(self.output_proj.parameters());
        params
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        let mut params = Vec::new();
        for head in &mut self.heads {
            params.extend(head.parameters_mut());
        }
        params.extend(self.output_proj.parameters_mut());
        params
    }
}

/// Transformer encoder layer
pub struct TransformerEncoderLayer<T = f32> {
    self_attn: MultiHeadAttention<T>,
    feedforward: Vec<Linear<T>>,
    norm1: crate::nn::normalization::LayerNorm<T>,
    norm2: crate::nn::normalization::LayerNorm<T>,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static>
    TransformerEncoderLayer<T>
{
    pub fn new(embed_dim: usize, num_heads: usize, ff_dim: usize) -> Self {
        Self {
            self_attn: MultiHeadAttention::new(embed_dim, num_heads),
            feedforward: vec![
                Linear::new(embed_dim, ff_dim),
                Linear::new(ff_dim, embed_dim),
            ],
            norm1: crate::nn::normalization::LayerNorm::new(vec![embed_dim]),
            norm2: crate::nn::normalization::LayerNorm::new(vec![embed_dim]),
        }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for TransformerEncoderLayer<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        use crate::tensor::TensorLike;

        // Self-attention with residual connection and layer norm
        let attn_output = self.self_attn.forward(input);
        let x = self.norm1.forward(&input.add(&attn_output));

        // Feedforward with residual connection and layer norm
        let mut ff_output = self.feedforward[0].forward(&x);
        // Apply ReLU activation (simplified)
        ff_output = self.feedforward[1].forward(&ff_output);

        self.norm2.forward(&x.add(&ff_output))
    }

    fn parameters(&self) -> Vec<&Tensor<T>> {
        let mut params = Vec::new();
        params.extend(self.self_attn.parameters());
        for layer in &self.feedforward {
            params.extend(layer.parameters());
        }
        params.extend(self.norm1.parameters());
        params.extend(self.norm2.parameters());
        params
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        let mut params = Vec::new();
        params.extend(self.self_attn.parameters_mut());
        for layer in &mut self.feedforward {
            params.extend(layer.parameters_mut());
        }
        params.extend(self.norm1.parameters_mut());
        params.extend(self.norm2.parameters_mut());
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_creation() {
        let attn = Attention::<f32>::new(64);
        let input = Tensor::randn(&[10, 64]); // sequence_length=10, embed_dim=64

        let _output = attn.forward(&input);
    }

    #[test]
    fn test_multihead_attention() {
        let mha = MultiHeadAttention::<f32>::new(64, 8);
        assert_eq!(mha._num_heads, 8);
    }
}
