//! Tensor module with automatic differentiation support
//!
//! This module provides the core `Tensor` type that wraps ndarray arrays
//! and tracks gradients for automatic differentiation.

use ndarray::{ArrayD, IxDyn, ScalarOperand};
use num_traits::{Float, NumAssign};
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::autograd::{ComputeNode, Operation};

/// A tensor with automatic differentiation capabilities
///
/// Wraps an n-dimensional array and tracks computational graph for backpropagation.
#[derive(Clone)]
pub struct Tensor<T = f32> {
    /// The underlying data array
    pub data: ArrayD<T>,

    /// Gradient accumulated during backpropagation (shared across clones)
    pub grad: Arc<Mutex<Option<ArrayD<T>>>>,

    /// Whether this tensor requires gradient computation
    pub requires_grad: bool,

    /// Computational graph node for autograd
    pub(crate) grad_fn: Option<Arc<Mutex<ComputeNode<T>>>>,
}

impl<T: Float + NumAssign + ScalarOperand + 'static + Send + Sync> Tensor<T> {
    /// Create a new tensor from an ndarray
    pub fn new(data: ArrayD<T>) -> Self {
        Self {
            data,
            grad: Arc::new(Mutex::new(None)),
            requires_grad: false,
            grad_fn: None,
        }
    }

    /// Create a tensor that requires gradient
    pub fn with_grad(data: ArrayD<T>) -> Self {
        Self {
            data,
            grad: Arc::new(Mutex::new(Some(ArrayD::zeros(IxDyn(&[]))))),
            requires_grad: true,
            grad_fn: None,
        }
    }

    /// Create a tensor from a scalar
    pub fn scalar(value: T) -> Self {
        Self::new(ArrayD::from_elem(IxDyn(&[]), value))
    }

    /// Create a zero tensor with the given shape
    pub fn zeros(shape: &[usize]) -> Self {
        Self::new(ArrayD::zeros(IxDyn(shape)))
    }

    /// Create a one tensor with the given shape
    pub fn ones(shape: &[usize]) -> Self {
        Self::new(ArrayD::ones(IxDyn(shape)))
    }

    /// Create a tensor filled with random values from uniform distribution [0, 1)
    pub fn rand(shape: &[usize]) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let shape_dyn = IxDyn(shape);
        let size: usize = shape.iter().product();

        let data: Vec<T> = (0..size)
            .map(|_| T::from(rng.gen::<f64>()).unwrap())
            .collect();

        Self::new(ArrayD::from_shape_vec(shape_dyn, data).unwrap())
    }

    /// Create a tensor with random normal distribution (mean=0, std=1)
    pub fn randn(shape: &[usize]) -> Self {
        use rand_distr::{Distribution, Normal};
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let shape_dyn = IxDyn(shape);
        let size: usize = shape.iter().product();

        let data: Vec<T> = (0..size)
            .map(|_| T::from(normal.sample(&mut rng)).unwrap())
            .collect();

        Self::new(ArrayD::from_shape_vec(shape_dyn, data).unwrap())
    }

    /// Get the shape of the tensor
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    /// Get the number of dimensions
    pub fn ndim(&self) -> usize {
        self.data.ndim()
    }

    /// Get the total number of elements
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Reshape the tensor
    pub fn reshape(&self, shape: &[usize]) -> Self {
        let data = self.data.clone().to_shape(IxDyn(shape)).unwrap().to_owned();
        let mut result = Self::new(data);
        result.requires_grad = self.requires_grad;
        result
    }

    /// Enable gradient tracking on this tensor
    pub fn requires_grad_(mut self) -> Self {
        self.requires_grad = true;
        if self.grad.lock().unwrap().is_none() {
            *self.grad.lock().unwrap() = Some(ArrayD::zeros(self.data.raw_dim()));
        }
        self
    }

    /// Zero out the gradients
    pub fn zero_grad(&mut self) {
        if let Some(ref mut grad) = *self.grad.lock().unwrap() {
            grad.fill(T::zero());
        }
    }

    /// Perform backward pass (compute gradients)
    pub fn backward(&mut self) {
        if !self.requires_grad {
            return;
        }

        // Initialize gradient to ones for scalar output
        if self.grad.lock().unwrap().is_none() {
            *self.grad.lock().unwrap() = Some(ArrayD::ones(self.data.raw_dim()));
        }

        // Perform backward pass through computational graph
        if let Some(ref grad_fn) = self.grad_fn {
            let grad_output = self.grad.lock().unwrap().clone().unwrap();
            grad_fn.lock().unwrap().backward(grad_output);
        }
    }

    /// Detach from computational graph (stop gradient tracking)
    pub fn detach(&self) -> Self {
        let mut result = Self::new(self.data.clone());
        result.requires_grad = false;
        result.grad_fn = None;
        result
    }

    /// Get a reference to the data
    pub fn data(&self) -> &ArrayD<T> {
        &self.data
    }

    /// Get a mutable reference to the data (breaks gradient tracking)
    pub fn data_mut(&mut self) -> &mut ArrayD<T> {
        self.grad_fn = None; // Break computational graph
        &mut self.data
    }

    /// Divide tensor by a scalar value
    pub fn div_scalar(&self, scalar: T) -> Self {
        let data = self.data.mapv(|x| x / scalar);
        Self::new(data)
    }

    /// Apply softmax along specified dimension
    pub fn softmax(&self, _dim: isize) -> Self {
        // Simplified softmax - applies to last dimension
        let mut result = self.data.clone();
        let shape = result.shape();
        let last_dim = shape[shape.len() - 1];

        // Compute for each element along last dimension
        let total_size = result.len();
        let num_groups = total_size / last_dim;

        for group in 0..num_groups {
            let start_idx = group * last_dim;

            // Find max for numerical stability
            let mut max_val = T::neg_infinity();
            for i in 0..last_dim {
                let val = result.as_slice().unwrap()[start_idx + i];
                if val > max_val {
                    max_val = val;
                }
            }

            // Compute exp and sum
            let mut sum = T::zero();
            for i in 0..last_dim {
                let val = result.as_slice().unwrap()[start_idx + i];
                let exp_val = (val - max_val).exp();
                result.as_slice_mut().unwrap()[start_idx + i] = exp_val;
                sum += exp_val;
            }

            // Normalize
            for i in 0..last_dim {
                result.as_slice_mut().unwrap()[start_idx + i] =
                    result.as_slice().unwrap()[start_idx + i] / sum;
            }
        }

        Self::new(result)
    }
}

/// Trait for tensor-like operations
pub trait TensorLike<T = f32> {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn matmul(&self, other: &Self) -> Self;
    fn sum(&self) -> Self;
    fn mean(&self) -> Self;
}

impl<T: Float + NumAssign + ScalarOperand + 'static + Send + Sync> TensorLike<T> for Tensor<T> {
    fn add(&self, other: &Self) -> Self {
        let data = &self.data + &other.data;
        let mut result = Self::new(data);

        if self.requires_grad || other.requires_grad {
            result.requires_grad = true;
            // Note: Don't initialize grad here - let backward() do it

            // Create computation node for backprop
            let node = ComputeNode::new(Operation::Add, vec![self.clone(), other.clone()]);
            result.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        result
    }

    fn sub(&self, other: &Self) -> Self {
        let data = &self.data - &other.data;
        let mut result = Self::new(data);

        if self.requires_grad || other.requires_grad {
            result.requires_grad = true;

            let node = ComputeNode::new(Operation::Sub, vec![self.clone(), other.clone()]);
            result.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        result
    }

    fn mul(&self, other: &Self) -> Self {
        let data = &self.data * &other.data;
        let mut result = Self::new(data);

        if self.requires_grad || other.requires_grad {
            result.requires_grad = true;

            let node = ComputeNode::new(Operation::Mul, vec![self.clone(), other.clone()]);
            result.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        result
    }

    fn div(&self, other: &Self) -> Self {
        let data = &self.data / &other.data;
        let mut result = Self::new(data);

        if self.requires_grad || other.requires_grad {
            result.requires_grad = true;

            let node = ComputeNode::new(Operation::Div, vec![self.clone(), other.clone()]);
            result.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        result
    }

    fn matmul(&self, other: &Self) -> Self {
        // Simplified 2D matrix multiplication
        assert!(
            self.ndim() == 2 && other.ndim() == 2,
            "matmul requires 2D tensors"
        );

        let a = self
            .data
            .clone()
            .into_dimensionality::<ndarray::Ix2>()
            .unwrap();
        let b = other
            .data
            .clone()
            .into_dimensionality::<ndarray::Ix2>()
            .unwrap();
        let data = a.dot(&b).into_dyn();

        let mut result = Self::new(data);

        if self.requires_grad || other.requires_grad {
            result.requires_grad = true;

            let node = ComputeNode::new(Operation::MatMul, vec![self.clone(), other.clone()]);
            result.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        result
    }

    fn sum(&self) -> Self {
        let data = ArrayD::from_elem(IxDyn(&[]), self.data.sum());
        let mut result = Self::new(data);

        if self.requires_grad {
            result.requires_grad = true;

            let node = ComputeNode::new(Operation::Sum, vec![self.clone()]);
            result.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        result
    }

    fn mean(&self) -> Self {
        let size = T::from(self.size()).unwrap();
        let data = ArrayD::from_elem(IxDyn(&[]), self.data.sum() / size);
        let mut result = Self::new(data);

        if self.requires_grad {
            result.requires_grad = true;

            let node = ComputeNode::new(Operation::Mean, vec![self.clone()]);
            result.grad_fn = Some(Arc::new(Mutex::new(node)));
        }

        result
    }
}

impl<T: Float + NumAssign + ScalarOperand + Send + Sync + 'static + fmt::Display> fmt::Debug
    for Tensor<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tensor(shape={:?}, requires_grad={})",
            self.shape(),
            self.requires_grad
        )
    }
}

impl<T: Float + NumAssign + ScalarOperand + Send + Sync + 'static + fmt::Display> fmt::Display
    for Tensor<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::<f32>::zeros(&[2, 3]);
        assert_eq!(tensor.shape(), &[2, 3]);
        assert_eq!(tensor.size(), 6);
    }

    #[test]
    fn test_tensor_operations() {
        let a = Tensor::new(ArrayD::from_elem(IxDyn(&[2, 2]), 2.0_f32));
        let b = Tensor::new(ArrayD::from_elem(IxDyn(&[2, 2]), 3.0_f32));

        let c = a.add(&b);
        assert_eq!(c.data[[0, 0]], 5.0);
    }

    #[test]
    fn test_matmul() {
        let a = Tensor::new(arr2(&[[1.0_f32, 2.0], [3.0, 4.0]]).into_dyn());
        let b = Tensor::new(arr2(&[[2.0_f32, 0.0], [1.0, 2.0]]).into_dyn());

        let c = a.matmul(&b);
        assert_eq!(c.data[[0, 0]], 4.0);
        assert_eq!(c.data[[0, 1]], 4.0);
    }

    #[test]
    fn test_requires_grad() {
        let tensor = Tensor::<f32>::zeros(&[2, 2]).requires_grad_();
        assert!(tensor.requires_grad);
        assert!(tensor.grad.lock().unwrap().is_some());
    }
}
