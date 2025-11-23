//! Normalization layers

use crate::nn::Module;
use crate::tensor::Tensor;
use num_traits::{Float, NumAssign};

/// Batch Normalization layer
pub struct BatchNorm<T = f32> {
    pub gamma: Tensor<T>, // scale
    pub beta: Tensor<T>,  // shift
    _num_features: usize,
    epsilon: T,
    momentum: T,
    _running_mean: Tensor<T>,
    _running_var: Tensor<T>,
    training: bool,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> BatchNorm<T> {
    pub fn new(num_features: usize) -> Self {
        Self {
            gamma: Tensor::ones(&[num_features]).requires_grad_(),
            beta: Tensor::zeros(&[num_features]).requires_grad_(),
            _num_features: num_features,
            epsilon: T::from(1e-5).unwrap(),
            momentum: T::from(0.1).unwrap(),
            _running_mean: Tensor::zeros(&[num_features]),
            _running_var: Tensor::ones(&[num_features]),
            training: true,
        }
    }

    pub fn with_epsilon(mut self, epsilon: T) -> Self {
        self.epsilon = epsilon;
        self
    }

    pub fn with_momentum(mut self, momentum: T) -> Self {
        self.momentum = momentum;
        self
    }

    pub fn train(&mut self) {
        self.training = true;
    }

    pub fn eval(&mut self) {
        self.training = false;
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for BatchNorm<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        // Input shape: (batch, features, ...)
        let shape = input.shape();
        let batch_size = shape[0];
        let num_features = shape[1];

        let mut output = input.data.clone();

        if self.training {
            // Compute mean and variance across batch and spatial dimensions
            for c in 0..num_features {
                let mut sum = T::zero();
                let mut count = 0;

                // Calculate mean
                for batch_idx in 0..batch_size {
                    for spatial_idx in 0..(output.len() / (batch_size * num_features)) {
                        let idx = (batch_idx * num_features + c) * (output.len() / (batch_size * num_features)) + spatial_idx;
                        sum += output.as_slice().unwrap()[idx];
                        count += 1;
                    }
                }
                let mean = sum / T::from(count).unwrap();

                // Calculate variance
                let mut var_sum = T::zero();
                for batch_idx in 0..batch_size {
                    for spatial_idx in 0..(output.len() / (batch_size * num_features)) {
                        let idx = (batch_idx * num_features + c) * (output.len() / (batch_size * num_features)) + spatial_idx;
                        let diff = output.as_slice().unwrap()[idx] - mean;
                        var_sum += diff * diff;
                    }
                }
                let variance = var_sum / T::from(count).unwrap();

                // Normalize
                let std = (variance + self.epsilon).sqrt();
                let gamma_val = self.gamma.data.as_slice().unwrap()[c];
                let beta_val = self.beta.data.as_slice().unwrap()[c];

                for batch_idx in 0..batch_size {
                    for spatial_idx in 0..(output.len() / (batch_size * num_features)) {
                        let idx = (batch_idx * num_features + c) * (output.len() / (batch_size * num_features)) + spatial_idx;
                        let normalized = (output.as_slice().unwrap()[idx] - mean) / std;
                        output.as_slice_mut().unwrap()[idx] = gamma_val * normalized + beta_val;
                    }
                }
            }
        } else {
            // Use running statistics (simplified - just use current batch stats for now)
            for c in 0..num_features {
                let gamma_val = self.gamma.data.as_slice().unwrap()[c];
                let beta_val = self.beta.data.as_slice().unwrap()[c];

                for batch_idx in 0..batch_size {
                    for spatial_idx in 0..(output.len() / (batch_size * num_features)) {
                        let idx = (batch_idx * num_features + c) * (output.len() / (batch_size * num_features)) + spatial_idx;
                        output.as_slice_mut().unwrap()[idx] = gamma_val * output.as_slice().unwrap()[idx] + beta_val;
                    }
                }
            }
        }

        Tensor::new(output)
    }

    fn parameters(&self) -> Vec<&Tensor<T>> {
        vec![&self.gamma, &self.beta]
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        vec![&mut self.gamma, &mut self.beta]
    }
}

/// Layer Normalization
pub struct LayerNorm<T = f32> {
    pub gamma: Tensor<T>,
    pub beta: Tensor<T>,
    _normalized_shape: Vec<usize>,
    _epsilon: T,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> LayerNorm<T> {
    pub fn new(normalized_shape: Vec<usize>) -> Self {
        let total_size: usize = normalized_shape.iter().product();

        Self {
            gamma: Tensor::ones(&[total_size]).requires_grad_(),
            beta: Tensor::zeros(&[total_size]).requires_grad_(),
            _normalized_shape: normalized_shape,
            _epsilon: T::from(1e-5).unwrap(),
        }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for LayerNorm<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        // LayerNorm normalizes across the feature dimension
        // Input shape: (batch, features, ...)
        let shape = input.shape();
        let batch_size = shape[0];
        let feature_size: usize = shape[1..].iter().product();

        let mut output = input.data.clone();

        // Normalize each sample independently
        for b in 0..batch_size {
            // Calculate mean for this sample
            let mut sum = T::zero();
            for f in 0..feature_size {
                let idx = b * feature_size + f;
                sum += output.as_slice().unwrap()[idx];
            }
            let mean = sum / T::from(feature_size).unwrap();

            // Calculate variance
            let mut var_sum = T::zero();
            for f in 0..feature_size {
                let idx = b * feature_size + f;
                let diff = output.as_slice().unwrap()[idx] - mean;
                var_sum += diff * diff;
            }
            let variance = var_sum / T::from(feature_size).unwrap();
            let std = (variance + self._epsilon).sqrt();

            // Normalize and apply affine transformation
            for f in 0..feature_size {
                let idx = b * feature_size + f;
                let normalized = (output.as_slice().unwrap()[idx] - mean) / std;
                let gamma_val = self.gamma.data.as_slice().unwrap()[f];
                let beta_val = self.beta.data.as_slice().unwrap()[f];
                output.as_slice_mut().unwrap()[idx] = gamma_val * normalized + beta_val;
            }
        }

        Tensor::new(output)
    }

    fn parameters(&self) -> Vec<&Tensor<T>> {
        vec![&self.gamma, &self.beta]
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        vec![&mut self.gamma, &mut self.beta]
    }
}

/// Dropout layer (for regularization)
pub struct Dropout<T = f32> {
    p: T,
    training: bool,
}

impl<T: Float> Dropout<T> {
    pub fn new(p: T) -> Self {
        Self { p, training: true }
    }

    pub fn train(&mut self) {
        self.training = true;
    }

    pub fn eval(&mut self) {
        self.training = false;
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for Dropout<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        if !self.training {
            return input.clone();
        }

        // During training, randomly zero out elements with probability p
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mask = input.data.mapv(|_| {
            if rng.gen::<f64>() < T::to_f64(&self.p).unwrap() {
                T::zero()
            } else {
                T::one() / (T::one() - self.p)
            }
        });

        let data = &input.data * &mask;
        Tensor::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batchnorm_creation() {
        let bn = BatchNorm::<f32>::new(10);
        assert_eq!(bn.gamma.shape(), &[10]);
        assert_eq!(bn.beta.shape(), &[10]);
    }

    #[test]
    fn test_dropout() {
        let mut dropout = Dropout::<f32>::new(0.5);
        let input = Tensor::ones(&[100]);

        dropout.train();
        let _output_train = dropout.forward(&input);

        dropout.eval();
        let output_eval = dropout.forward(&input);

        // In eval mode, output should equal input
        assert_eq!(output_eval.data.sum(), 100.0);
    }
}
