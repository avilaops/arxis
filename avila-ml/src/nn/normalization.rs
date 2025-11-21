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
        if self.training {
            // Compute batch statistics
            // TODO: Implement proper batch normalization with running stats
            input.clone()
        } else {
            // Use running statistics
            // normalized = (input - running_mean) / sqrt(running_var + epsilon)
            // output = gamma * normalized + beta
            input.clone()
        }
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
        // TODO: Implement proper layer normalization
        input.clone()
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
        let output_train = dropout.forward(&input);

        dropout.eval();
        let output_eval = dropout.forward(&input);

        // In eval mode, output should equal input
        assert_eq!(output_eval.data.sum(), 100.0);
    }
}
