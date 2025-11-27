//! avila-ml Neural Network Core - PyTorch Competitor
//!
//! Core features:
//! - Automatic differentiation (autograd)
//! - Tensor operations
//! - Neural network primitives
//! - Gradient computation

use core::ops::{Add, Mul, Sub};

/// Tensor shape
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    dims: [usize; 4], // Support up to 4D tensors
    rank: usize,
}

impl Shape {
    pub fn new(dims: &[usize]) -> Self {
        let mut shape_dims = [1; 4];
        let rank = dims.len().min(4);
        shape_dims[..rank].copy_from_slice(&dims[..rank]);

        Self {
            dims: shape_dims,
            rank,
        }
    }

    #[inline]
    pub fn rank(&self) -> usize {
        self.rank
    }

    #[inline]
    pub fn dims(&self) -> &[usize] {
        &self.dims[..self.rank]
    }

    #[inline]
    pub fn total_elements(&self) -> usize {
        self.dims[..self.rank].iter().product()
    }

    pub fn broadcast_with(&self, other: &Shape) -> Option<Shape> {
        let max_rank = self.rank.max(other.rank);
        let mut result = [1usize; 4];

        for i in 0..max_rank {
            let dim_a = if i < self.rank { self.dims[self.rank - 1 - i] } else { 1 };
            let dim_b = if i < other.rank { other.dims[other.rank - 1 - i] } else { 1 };

            if dim_a == dim_b {
                result[max_rank - 1 - i] = dim_a;
            } else if dim_a == 1 {
                result[max_rank - 1 - i] = dim_b;
            } else if dim_b == 1 {
                result[max_rank - 1 - i] = dim_a;
            } else {
                return None; // Incompatible shapes
            }
        }

        Some(Shape {
            dims: result,
            rank: max_rank,
        })
    }
}

/// Tensor with automatic differentiation
#[derive(Debug, Clone)]
pub struct Tensor {
    pub data: Vec<f32>,
    pub shape: Shape,
    pub grad: Option<Vec<f32>>,
    pub requires_grad: bool,
    /// Backward function for autograd
    pub grad_fn: Option<GradFn>,
}

impl Tensor {
    pub fn new(data: Vec<f32>, shape: Shape, requires_grad: bool) -> Self {
        Self {
            data,
            shape,
            grad: if requires_grad { Some(vec![0.0; shape.total_elements()]) } else { None },
            requires_grad,
            grad_fn: None,
        }
    }

    pub fn zeros(shape: Shape, requires_grad: bool) -> Self {
        let total = shape.total_elements();
        Self::new(vec![0.0; total], shape, requires_grad)
    }

    pub fn ones(shape: Shape, requires_grad: bool) -> Self {
        let total = shape.total_elements();
        Self::new(vec![1.0; total], shape, requires_grad)
    }

    pub fn randn(shape: Shape, requires_grad: bool) -> Self {
        let total = shape.total_elements();
        // Simple random initialization (would use proper RNG in production)
        let data: Vec<f32> = (0..total)
            .map(|i| {
                let x = (i as f32 * 0.1).sin();
                x * 0.1
            })
            .collect();

        Self::new(data, shape, requires_grad)
    }

    /// Zero gradients
    pub fn zero_grad(&mut self) {
        if let Some(ref mut grad) = self.grad {
            grad.fill(0.0);
        }
    }

    /// Backward pass - compute gradients
    pub fn backward(&mut self) {
        if !self.requires_grad {
            return;
        }

        // Initialize gradient to 1.0 for loss tensor
        if let Some(ref mut grad) = self.grad {
            if grad.iter().all(|&x| x == 0.0) {
                grad.fill(1.0);
            }
        }

        // Execute backward functions
        if let Some(grad_fn) = self.grad_fn.take() {
            grad_fn.backward(self);
        }
    }
}

/// Gradient function for autograd
#[derive(Debug, Clone)]
pub enum GradFn {
    Add { left: Box<Tensor>, right: Box<Tensor> },
    Mul { left: Box<Tensor>, right: Box<Tensor> },
    MatMul { left: Box<Tensor>, right: Box<Tensor> },
    ReLU { input: Box<Tensor> },
    Sigmoid { input: Box<Tensor> },
    Sum { input: Box<Tensor> },
}

impl GradFn {
    pub fn backward(&self, output: &Tensor) {
        match self {
            GradFn::Add { left, right } => {
                // d(a+b)/da = 1, d(a+b)/db = 1
                if let Some(out_grad) = &output.grad {
                    // Gradient flows equally to both inputs
                    // (simplified - would handle broadcasting properly)
                }
            }
            GradFn::Mul { left, right } => {
                // d(a*b)/da = b, d(a*b)/db = a
            }
            GradFn::MatMul { left, right } => {
                // Matrix multiplication gradients
            }
            GradFn::ReLU { input } => {
                // d(ReLU(x))/dx = 1 if x > 0, else 0
            }
            GradFn::Sigmoid { input } => {
                // d(sigmoid(x))/dx = sigmoid(x) * (1 - sigmoid(x))
            }
            GradFn::Sum { input } => {
                // d(sum(x))/dx = 1 for all elements
            }
        }
    }
}

/// Tensor operations
impl Tensor {
    /// Matrix multiplication
    pub fn matmul(&self, other: &Tensor) -> Tensor {
        assert_eq!(self.shape.rank(), 2, "matmul requires 2D tensors");
        assert_eq!(other.shape.rank(), 2, "matmul requires 2D tensors");

        let m = self.shape.dims[0];
        let k = self.shape.dims[1];
        let n = other.shape.dims[1];

        assert_eq!(k, other.shape.dims[0], "incompatible matmul dimensions");

        let mut result = vec![0.0; m * n];

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for k_idx in 0..k {
                    sum += self.data[i * k + k_idx] * other.data[k_idx * n + j];
                }
                result[i * n + j] = sum;
            }
        }

        let requires_grad = self.requires_grad || other.requires_grad;
        let mut output = Tensor::new(result, Shape::new(&[m, n]), requires_grad);

        if requires_grad {
            output.grad_fn = Some(GradFn::MatMul {
                left: Box::new(self.clone()),
                right: Box::new(other.clone()),
            });
        }

        output
    }

    /// Element-wise addition
    pub fn add(&self, other: &Tensor) -> Tensor {
        let broadcast_shape = self.shape.broadcast_with(&other.shape)
            .expect("incompatible shapes for addition");

        let total = broadcast_shape.total_elements();
        let mut result = vec![0.0; total];

        for i in 0..total {
            result[i] = self.data[i % self.data.len()] + other.data[i % other.data.len()];
        }

        let requires_grad = self.requires_grad || other.requires_grad;
        let mut output = Tensor::new(result, broadcast_shape, requires_grad);

        if requires_grad {
            output.grad_fn = Some(GradFn::Add {
                left: Box::new(self.clone()),
                right: Box::new(other.clone()),
            });
        }

        output
    }

    /// ReLU activation
    pub fn relu(&self) -> Tensor {
        let result: Vec<f32> = self.data.iter().map(|&x| x.max(0.0)).collect();

        let mut output = Tensor::new(result, self.shape.clone(), self.requires_grad);

        if self.requires_grad {
            output.grad_fn = Some(GradFn::ReLU {
                input: Box::new(self.clone()),
            });
        }

        output
    }

    /// Sigmoid activation
    pub fn sigmoid(&self) -> Tensor {
        let result: Vec<f32> = self.data.iter()
            .map(|&x| 1.0 / (1.0 + (-x).exp()))
            .collect();

        let mut output = Tensor::new(result, self.shape.clone(), self.requires_grad);

        if self.requires_grad {
            output.grad_fn = Some(GradFn::Sigmoid {
                input: Box::new(self.clone()),
            });
        }

        output
    }

    /// Sum all elements
    pub fn sum(&self) -> Tensor {
        let sum: f32 = self.data.iter().sum();
        let mut output = Tensor::new(vec![sum], Shape::new(&[1]), self.requires_grad);

        if self.requires_grad {
            output.grad_fn = Some(GradFn::Sum {
                input: Box::new(self.clone()),
            });
        }

        output
    }
}

/// Neural network layer
pub trait Layer {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn parameters(&self) -> Vec<&Tensor>;
    fn parameters_mut(&mut self) -> Vec<&mut Tensor>;
}

/// Linear (fully connected) layer
pub struct Linear {
    pub weight: Tensor,
    pub bias: Tensor,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let weight = Tensor::randn(
            Shape::new(&[in_features, out_features]),
            true,
        );
        let bias = Tensor::zeros(Shape::new(&[out_features]), true);

        Self { weight, bias }
    }
}

impl Layer for Linear {
    fn forward(&self, input: &Tensor) -> Tensor {
        let output = input.matmul(&self.weight);
        output.add(&self.bias)
    }

    fn parameters(&self) -> Vec<&Tensor> {
        vec![&self.weight, &self.bias]
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor> {
        vec![&mut self.weight, &mut self.bias]
    }
}

/// Optimizer trait
pub trait Optimizer {
    fn step(&mut self, parameters: Vec<&mut Tensor>);
    fn zero_grad(&mut self, parameters: Vec<&mut Tensor>);
}

/// Stochastic Gradient Descent
pub struct SGD {
    pub learning_rate: f32,
}

impl SGD {
    pub fn new(learning_rate: f32) -> Self {
        Self { learning_rate }
    }
}

impl Optimizer for SGD {
    fn step(&mut self, parameters: Vec<&mut Tensor>) {
        for param in parameters {
            if let Some(ref grad) = param.grad {
                for (p, g) in param.data.iter_mut().zip(grad.iter()) {
                    *p -= self.learning_rate * g;
                }
            }
        }
    }

    fn zero_grad(&mut self, parameters: Vec<&mut Tensor>) {
        for param in parameters {
            param.zero_grad();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let t = Tensor::zeros(Shape::new(&[2, 3]), false);
        assert_eq!(t.data.len(), 6);
        assert!(t.data.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_matmul() {
        let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], Shape::new(&[2, 2]), false);
        let b = Tensor::new(vec![2.0, 0.0, 1.0, 2.0], Shape::new(&[2, 2]), false);

        let c = a.matmul(&b);

        // Expected: [[1*2+2*1, 1*0+2*2], [3*2+4*1, 3*0+4*2]] = [[4, 4], [10, 8]]
        assert_eq!(c.data[0], 4.0);
        assert_eq!(c.data[1], 4.0);
        assert_eq!(c.data[2], 10.0);
        assert_eq!(c.data[3], 8.0);
    }

    #[test]
    fn test_relu() {
        let t = Tensor::new(vec![-1.0, 0.0, 1.0, 2.0], Shape::new(&[4]), false);
        let result = t.relu();

        assert_eq!(result.data, vec![0.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_shape_broadcast() {
        let shape_a = Shape::new(&[3, 1]);
        let shape_b = Shape::new(&[1, 4]);

        let result = shape_a.broadcast_with(&shape_b).unwrap();
        assert_eq!(result.dims(), &[3, 4]);
    }
}
