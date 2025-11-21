//! Módulo comum para processamento sequencial

use ndarray::{Array1, ArrayView1};
use serde::{Deserialize, Serialize};

/// Interface para dados sequenciais 1D
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequentialData<T> {
    /// Dados da sequência
    pub data: Vec<T>,
    /// Taxa de amostragem (samples por segundo)
    pub sample_rate: f32,
    /// Metadados opcionais
    pub metadata: Option<String>,
}

impl<T: Clone> SequentialData<T> {
    /// Cria nova sequência de dados
    pub fn new(data: Vec<T>, sample_rate: f32) -> Self {
        Self {
            data,
            sample_rate,
            metadata: None,
        }
    }

    /// Define metadados
    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Retorna o comprimento da sequência
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Verifica se está vazia
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Retorna a duração em segundos
    pub fn duration(&self) -> f32 {
        self.data.len() as f32 / self.sample_rate
    }
}

/// Kernel de convolução 1D
#[derive(Debug, Clone)]
pub struct ConvolutionKernel {
    /// Pesos do kernel
    pub weights: Array1<f32>,
    /// Stride (passo)
    pub stride: usize,
    /// Padding
    pub padding: usize,
}

impl ConvolutionKernel {
    /// Cria novo kernel
    pub fn new(weights: Vec<f32>, stride: usize, padding: usize) -> Self {
        Self {
            weights: Array1::from(weights),
            stride,
            padding,
        }
    }

    /// Kernel gaussiano
    pub fn gaussian(size: usize, sigma: f32) -> Self {
        let center = size as f32 / 2.0;
        let weights: Vec<f32> = (0..size)
            .map(|i| {
                let x = i as f32 - center;
                (-x * x / (2.0 * sigma * sigma)).exp()
            })
            .collect();

        // Normalizar
        let sum: f32 = weights.iter().sum();
        let normalized: Vec<f32> = weights.iter().map(|w| w / sum).collect();

        Self::new(normalized, 1, size / 2)
    }

    /// Kernel de média móvel
    pub fn moving_average(size: usize) -> Self {
        let weight = 1.0 / size as f32;
        let weights = vec![weight; size];
        Self::new(weights, 1, size / 2)
    }

    /// Aplica convolução
    pub fn convolve(&self, signal: &ArrayView1<f32>) -> Array1<f32> {
        let kernel_size = self.weights.len();
        let signal_len = signal.len();
        let output_len = (signal_len + 2 * self.padding - kernel_size) / self.stride + 1;

        let mut output = Array1::zeros(output_len);

        for i in 0..output_len {
            let start = i * self.stride;
            let mut sum = 0.0;

            for (k, &weight) in self.weights.iter().enumerate() {
                let idx = start + k;
                if idx >= self.padding && idx < signal_len + self.padding {
                    let signal_idx = idx - self.padding;
                    sum += signal[signal_idx] * weight;
                }
            }

            output[i] = sum;
        }

        output
    }
}

/// Funções de ativação
pub mod activation {
    /// ReLU (Rectified Linear Unit)
    #[inline]
    pub fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    /// Sigmoid
    #[inline]
    pub fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }

    /// Tanh
    #[inline]
    pub fn tanh(x: f32) -> f32 {
        x.tanh()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let seq = SequentialData::new(data.clone(), 100.0);

        assert_eq!(seq.len(), 4);
        assert_eq!(seq.duration(), 0.04);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_convolution_moving_average() {
        let kernel = ConvolutionKernel::moving_average(3);
        let signal = Array1::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = kernel.convolve(&signal.view());

        // Com padding de 1 (size/2) e stride de 1, resultado tem 5 amostras
        assert_eq!(result.len(), 5);

        // Verifica que a convolução foi aplicada
        assert!(result[0] > 0.0);
        assert!(result[result.len() - 1] > 0.0);
    }
}
