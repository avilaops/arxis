//! Processamento de texto sequencial
//!
//! Análise de sequências de texto com embeddings e features temporais

use crate::common::ConvolutionKernel;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Processador de texto
#[derive(Debug, Clone)]
pub struct TextProcessor {
    /// Vocabulário (token -> índice)
    pub vocab: HashMap<String, usize>,
    /// Dimensão do embedding
    pub embedding_dim: usize,
    /// Matriz de embeddings
    pub embeddings: Option<Array2<f32>>,
}

impl TextProcessor {
    /// Cria novo processador
    pub fn new(embedding_dim: usize) -> Self {
        Self {
            vocab: HashMap::new(),
            embedding_dim,
            embeddings: None,
        }
    }

    /// Constrói vocabulário a partir de textos
    pub fn build_vocab(&mut self, texts: &[String]) {
        let mut vocab = HashMap::new();
        let mut idx = 0;

        for text in texts {
            for token in self.tokenize(text) {
                if !vocab.contains_key(&token) {
                    vocab.insert(token, idx);
                    idx += 1;
                }
            }
        }

        self.vocab = vocab;

        // Inicializa embeddings aleatórios (na prática, usar pré-treinados)
        let vocab_size = self.vocab.len();
        self.embeddings = Some(Array2::from_shape_fn(
            (vocab_size, self.embedding_dim),
            |(i, j)| (i as f32 * 0.01 + j as f32 * 0.001).sin(),
        ));
    }

    /// Tokeniza texto
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }

    /// Converte texto para sequência de índices
    pub fn text_to_indices(&self, text: &str) -> Vec<usize> {
        self.tokenize(text)
            .iter()
            .filter_map(|token| self.vocab.get(token).copied())
            .collect()
    }

    /// Converte texto para embeddings
    pub fn text_to_embeddings(&self, text: &str) -> Option<Array2<f32>> {
        let embeddings = self.embeddings.as_ref()?;
        let indices = self.text_to_indices(text);

        if indices.is_empty() {
            return None;
        }

        let mut result = Array2::zeros((indices.len(), self.embedding_dim));
        for (i, &idx) in indices.iter().enumerate() {
            if idx < embeddings.nrows() {
                result.row_mut(i).assign(&embeddings.row(idx));
            }
        }

        Some(result)
    }

    /// Extrai features do texto
    pub fn extract_features(&self, text: &str) -> TextFeatures {
        let tokens = self.tokenize(text);
        let indices = self.text_to_indices(text);

        // Estatísticas básicas
        let token_count = tokens.len();
        let unique_tokens = tokens.iter().collect::<std::collections::HashSet<_>>().len();
        let avg_token_length = if token_count > 0 {
            tokens.iter().map(|t| t.len()).sum::<usize>() as f32 / token_count as f32
        } else {
            0.0
        };

        // Densidade lexical (tokens únicos / total)
        let lexical_density = if token_count > 0 {
            unique_tokens as f32 / token_count as f32
        } else {
            0.0
        };

        // Embedding médio
        let avg_embedding = if let Some(embeddings) = self.text_to_embeddings(text) {
            Some(embeddings.mean_axis(ndarray::Axis(0)).unwrap())
        } else {
            None
        };

        TextFeatures {
            token_count,
            unique_tokens,
            avg_token_length,
            lexical_density,
            sequence_length: indices.len(),
            avg_embedding,
        }
    }

    /// Aplica convolução temporal no texto
    pub fn temporal_convolution(&self, text: &str, kernel: &ConvolutionKernel) -> Option<Array2<f32>> {
        let embeddings = self.text_to_embeddings(text)?;

        // Aplica convolução em cada dimensão do embedding
        let seq_len = embeddings.nrows();

        if seq_len < kernel.weights.len() {
            return None; // Sequência muito curta para o kernel
        }

        let output_len = (seq_len - kernel.weights.len()) / kernel.stride + 1;
        let mut result = Array2::zeros((output_len, self.embedding_dim));

        for dim in 0..self.embedding_dim {
            let signal = embeddings.column(dim).to_owned();
            let convolved = kernel.convolve(&signal.view());

            // Garante que o resultado tem o tamanho correto
            let copy_len = output_len.min(convolved.len());
            for i in 0..copy_len {
                result[[i, dim]] = convolved[i];
            }
        }

        Some(result)
    }
}

/// Features extraídas de texto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFeatures {
    /// Número de tokens
    pub token_count: usize,
    /// Número de tokens únicos
    pub unique_tokens: usize,
    /// Comprimento médio dos tokens
    pub avg_token_length: f32,
    /// Densidade lexical
    pub lexical_density: f32,
    /// Comprimento da sequência
    pub sequence_length: usize,
    /// Embedding médio
    #[serde(skip)]
    pub avg_embedding: Option<Array1<f32>>,
}

impl TextFeatures {
    /// Converte para vetor de features
    pub fn to_vector(&self) -> Vec<f32> {
        vec![
            self.token_count as f32,
            self.unique_tokens as f32,
            self.avg_token_length,
            self.lexical_density,
            self.sequence_length as f32,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_processor() {
        let mut processor = TextProcessor::new(8);
        let texts = vec![
            "Hello world".to_string(),
            "Hello Rust".to_string(),
        ];

        processor.build_vocab(&texts);
        assert!(processor.vocab.len() > 0);

        let indices = processor.text_to_indices("Hello world");
        assert_eq!(indices.len(), 2);
    }

    #[test]
    fn test_text_features() {
        let mut processor = TextProcessor::new(8);
        let texts = vec!["The quick brown fox jumps over the lazy dog".to_string()];
        processor.build_vocab(&texts);

        let features = processor.extract_features(&texts[0]);
        assert_eq!(features.token_count, 9);
        assert!(features.lexical_density > 0.0);
    }

    #[test]
    fn test_tokenization() {
        let processor = TextProcessor::new(8);
        let tokens = processor.tokenize("Hello, World! This is Rust.");

        assert_eq!(tokens, vec!["hello", "world", "this", "is", "rust"]);
    }
}
