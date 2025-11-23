//! Activation functions

use crate::autograd::{ComputeNode, Operation};
use crate::nn::Module;
use crate::tensor::Tensor;
use num_traits::{Float, NumAssign};
use std::sync::{Arc, Mutex};

/// ReLU activation function
#[derive(Default)]
pub struct ReLU;

impl ReLU {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T> for ReLU {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        let data = input.data.mapv(|x| x.max(T::zero()));
        let mut output = Tensor::new(data);

        if input.requires_grad {
            output.requires_grad = true;

            let node = ComputeNode::new(Operation::ReLU, vec![input.clone()]);
            output.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        output
    }
}

/// Sigmoid activation: 1 / (1 + e^(-x))
#[derive(Default)]
pub struct Sigmoid;

impl Sigmoid {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T> for Sigmoid {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        let data = input.data.mapv(|x| T::one() / (T::one() + (-x).exp()));
        let mut output = Tensor::new(data);

        if input.requires_grad {
            output.requires_grad = true;

            let node = ComputeNode::new(Operation::Sigmoid, vec![input.clone()]);
            output.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        output
    }
}

/// Tanh activation: (e^x - e^(-x)) / (e^x + e^(-x))
#[derive(Default)]
pub struct Tanh;

impl Tanh {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T> for Tanh {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        let data = input.data.mapv(|x| x.tanh());
        let mut output = Tensor::new(data);

        if input.requires_grad {
            output.requires_grad = true;

            let node = ComputeNode::new(Operation::Tanh, vec![input.clone()]);
            output.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        output
    }
}

/// LogSoftmax activation
pub struct Softmax {
    _dim: isize,
}

impl Softmax {
    pub fn new(dim: isize) -> Self {
        Self { _dim: dim }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T> for Softmax {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        // Numerically stable softmax: subtract max for stability
        let max_val = input.data.iter().cloned().fold(T::neg_infinity(), T::max);

        let exp_data = input.data.mapv(|x| (x - max_val).exp());
        let sum = exp_data.sum();
        let data = exp_data.mapv(|x| x / sum);

        let mut output = Tensor::new(data);

        if input.requires_grad {
            output.requires_grad = true;

            let node = ComputeNode::new(Operation::Softmax, vec![input.clone()]);
            output.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        output
    }
}

/// Leaky ReLU activation: max(alpha * x, x)
pub struct LeakyReLU<T = f32> {
    alpha: T,
}

impl<T: Float> LeakyReLU<T> {
    pub fn new(alpha: T) -> Self {
        Self { alpha }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for LeakyReLU<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        let data = input
            .data
            .mapv(|x| if x > T::zero() { x } else { self.alpha * x });
        Tensor::new(data)
    }
}

/// ELU activation: x if x > 0 else alpha * (exp(x) - 1)
pub struct ELU<T = f32> {
    alpha: T,
}

impl<T: Float> ELU<T> {
    pub fn new(alpha: T) -> Self {
        Self { alpha }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T> for ELU<T> {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        let data = input.data.mapv(|x| {
            if x > T::zero() {
                x
            } else {
                self.alpha * (x.exp() - T::one())
            }
        });
        Tensor::new(data)
    }
}

/// GELU activation (Gaussian Error Linear Unit)
#[derive(Default)]
pub struct GELU;

impl GELU {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T> for GELU {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        // GELU(x) = 0.5 * x * (1 + tanh(sqrt(2/π) * (x + 0.044715 * x^3)))
        let sqrt_2_pi = T::from(0.7978845608).unwrap(); // sqrt(2/π)
        let coeff = T::from(0.044715).unwrap();

        let data = input.data.mapv(|x| {
            let half = T::from(0.5).unwrap();
            let x3 = x * x * x;
            let inner = sqrt_2_pi * (x + coeff * x3);
            half * x * (T::one() + inner.tanh())
        });

        Tensor::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        let relu = ReLU::new();
        let input = Tensor::new(ndarray::arr1(&[-1.0_f32, 0.0, 1.0, 2.0]).into_dyn());
        let output = relu.forward(&input);

        assert_eq!(output.data[[0]], 0.0);
        assert_eq!(output.data[[1]], 0.0);
        assert_eq!(output.data[[2]], 1.0);
        assert_eq!(output.data[[3]], 2.0);
    }

    #[test]
    fn test_sigmoid() {
        let sigmoid = Sigmoid::new();
        let input = Tensor::new(ndarray::arr1(&[0.0_f32]).into_dyn());
        let output = sigmoid.forward(&input);

        assert!((output.data[[0]] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_softmax() {
        let softmax = Softmax::new(-1);
        let input = Tensor::new(ndarray::arr1(&[1.0_f32, 2.0, 3.0]).into_dyn());
        let output = softmax.forward(&input);

        let sum: f32 = output.data.sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
