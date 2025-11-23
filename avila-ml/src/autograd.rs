//! Automatic differentiation engine
//!
//! This module implements the backward propagation engine for computing gradients.

use crate::tensor::Tensor;
use ndarray::ArrayD;
use num_traits::{Float, NumAssign};

/// Operations that can be performed on tensors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    MatMul,
    Sum,
    Mean,
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
    Log,
    Exp,
    Pow,
}

/// A node in the computational graph
pub struct ComputeNode<T = f32> {
    /// The operation performed at this node
    pub operation: Operation,

    /// Input tensors to this operation
    pub inputs: Vec<Tensor<T>>,

    /// Cached values for backward pass
    pub cache: Option<Vec<ArrayD<T>>>,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> ComputeNode<T> {
    /// Create a new compute node
    pub fn new(operation: Operation, inputs: Vec<Tensor<T>>) -> Self {
        Self {
            operation,
            inputs,
            cache: None,
        }
    }

    /// Perform backward pass through this node
    pub fn backward(&mut self, grad_output: ArrayD<T>) {
        use Operation::*;

        match self.operation {
            Add => self.backward_add(grad_output),
            Sub => self.backward_sub(grad_output),
            Mul => self.backward_mul(grad_output),
            Div => self.backward_div(grad_output),
            MatMul => self.backward_matmul(grad_output),
            Sum => self.backward_sum(grad_output),
            Mean => self.backward_mean(grad_output),
            ReLU => self.backward_relu(grad_output),
            Sigmoid => self.backward_sigmoid(grad_output),
            Tanh => self.backward_tanh(grad_output),
            Softmax => self.backward_softmax(grad_output),
            Log => self.backward_log(grad_output),
            Exp => self.backward_exp(grad_output),
            Pow => self.backward_pow(grad_output),
        }
    }

    fn backward_add(&mut self, grad_output: ArrayD<T>) {
        // d(a + b)/da = 1, d(a + b)/db = 1
        if self.inputs.len() >= 2 {
            if self.inputs[0].requires_grad {
                self.accumulate_grad(0, grad_output.clone());
            }
            if self.inputs[1].requires_grad {
                self.accumulate_grad(1, grad_output);
            }
        }
    }

    fn backward_sub(&mut self, grad_output: ArrayD<T>) {
        // d(a - b)/da = 1, d(a - b)/db = -1
        if self.inputs.len() >= 2 {
            if self.inputs[0].requires_grad {
                self.accumulate_grad(0, grad_output.clone());
            }
            if self.inputs[1].requires_grad {
                self.accumulate_grad(1, -grad_output);
            }
        }
    }

    fn backward_mul(&mut self, grad_output: ArrayD<T>) {
        // d(a * b)/da = b, d(a * b)/db = a
        if self.inputs.len() >= 2 {
            if self.inputs[0].requires_grad {
                let grad = &grad_output * &self.inputs[1].data;
                self.accumulate_grad(0, grad);
            }
            if self.inputs[1].requires_grad {
                let grad = &grad_output * &self.inputs[0].data;
                self.accumulate_grad(1, grad);
            }
        }
    }

    fn backward_div(&mut self, grad_output: ArrayD<T>) {
        // d(a / b)/da = 1/b, d(a / b)/db = -a/b²
        if self.inputs.len() >= 2 {
            if self.inputs[0].requires_grad {
                let grad = &grad_output / &self.inputs[1].data;
                self.accumulate_grad(0, grad);
            }
            if self.inputs[1].requires_grad {
                let grad = -(&grad_output * &self.inputs[0].data)
                    / (&self.inputs[1].data * &self.inputs[1].data);
                self.accumulate_grad(1, grad);
            }
        }
    }

    fn backward_matmul(&mut self, grad_output: ArrayD<T>) {
        // d(A @ B)/dA = grad @ B^T, d(A @ B)/dB = A^T @ grad
        if self.inputs.len() >= 2 {
            let grad_2d = grad_output.into_dimensionality::<ndarray::Ix2>().unwrap();

            if self.inputs[0].requires_grad {
                let b = self.inputs[1]
                    .data
                    .clone()
                    .into_dimensionality::<ndarray::Ix2>()
                    .unwrap();
                let grad_a = grad_2d.dot(&b.t()).into_dyn();
                self.accumulate_grad(0, grad_a);
            }

            if self.inputs[1].requires_grad {
                let a = self.inputs[0]
                    .data
                    .clone()
                    .into_dimensionality::<ndarray::Ix2>()
                    .unwrap();
                let grad_b = a.t().dot(&grad_2d).into_dyn();
                self.accumulate_grad(1, grad_b);
            }
        }
    }

    fn backward_sum(&mut self, grad_output: ArrayD<T>) {
        // d(sum(a))/da = ones_like(a) * grad_output
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            let grad = ArrayD::from_elem(self.inputs[0].data.raw_dim(), grad_output[[]]);
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_mean(&mut self, grad_output: ArrayD<T>) {
        // d(mean(a))/da = ones_like(a) / size * grad_output
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            let size = T::from(self.inputs[0].size()).unwrap();
            let grad_val = grad_output[[]] / size;
            let grad = ArrayD::from_elem(self.inputs[0].data.raw_dim(), grad_val);
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_relu(&mut self, grad_output: ArrayD<T>) {
        // d(relu(a))/da = grad_output if a > 0 else 0
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            let mask = self.inputs[0]
                .data
                .mapv(|x| if x > T::zero() { T::one() } else { T::zero() });
            let grad = grad_output * mask;
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_sigmoid(&mut self, grad_output: ArrayD<T>) {
        // d(sigmoid(a))/da = sigmoid(a) * (1 - sigmoid(a)) * grad_output
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            let sigmoid = self.inputs[0]
                .data
                .mapv(|x| T::one() / (T::one() + (-x).exp()));
            let grad = grad_output * &sigmoid * sigmoid.mapv(|x| T::one() - x);
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_tanh(&mut self, grad_output: ArrayD<T>) {
        // d(tanh(a))/da = (1 - tanh²(a)) * grad_output
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            let tanh = self.inputs[0].data.mapv(|x| x.tanh());
            let grad = grad_output * tanh.mapv(|x| T::one() - x * x);
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_softmax(&mut self, grad_output: ArrayD<T>) {
        // Softmax backward: grad_input[i] = softmax[i] * (grad_output[i] - sum(grad_output * softmax))
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            // Compute softmax of input
            let input = &self.inputs[0].data;
            let max_val = input.fold(T::neg_infinity(), |a, &b| if b > a { b } else { a });
            let exp_vals = input.mapv(|x| (x - max_val).exp());
            let sum_exp = exp_vals.sum();
            let softmax = exp_vals.mapv(|x| x / sum_exp);

            // Compute gradient
            let dot_product = (&grad_output * &softmax).sum();
            let grad = &softmax * (grad_output - dot_product);
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_log(&mut self, grad_output: ArrayD<T>) {
        // d(log(a))/da = 1/a * grad_output
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            let grad = grad_output / &self.inputs[0].data;
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_exp(&mut self, grad_output: ArrayD<T>) {
        // d(exp(a))/da = exp(a) * grad_output
        if !self.inputs.is_empty() && self.inputs[0].requires_grad {
            let grad = grad_output * self.inputs[0].data.mapv(|x| x.exp());
            self.accumulate_grad(0, grad);
        }
    }

    fn backward_pow(&mut self, grad_output: ArrayD<T>) {
        // d(a^n)/da = n * a^(n-1) * grad_output
        // Simplified for scalar exponent
        if self.inputs.len() >= 2 && self.inputs[0].requires_grad {
            let n = self.inputs[1].data[[]];
            let grad = grad_output * n * self.inputs[0].data.mapv(|x| x.powf(n - T::one()));
            self.accumulate_grad(0, grad);
        }
    }

    fn accumulate_grad(&mut self, input_idx: usize, grad: ArrayD<T>) {
        // Initialize gradient if it doesn't exist
        {
            let mut grad_borrow = self.inputs[input_idx].grad.lock().unwrap();
            if grad_borrow.is_none() {
                *grad_borrow = Some(ArrayD::zeros(self.inputs[input_idx].data.raw_dim()));
            }
        }

        // Accumulate gradient
        {
            let mut grad_borrow = self.inputs[input_idx].grad.lock().unwrap();
            if let Some(ref mut input_grad) = *grad_borrow {
                *input_grad = &*input_grad + &grad;
            }
        }

        // Continue backward pass through the computational graph
        if let Some(ref grad_fn) = self.inputs[input_idx].grad_fn {
            grad_fn.lock().unwrap().backward(grad);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensor::TensorLike;

    #[test]
    fn test_backward_add() {
        let a = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 2.0_f32)).requires_grad_();
        let b = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f32)).requires_grad_();

        let mut c = a.add(&b);
        c.backward();

        // Gradient should be 1 for both inputs
        assert!(a.grad.lock().unwrap().is_some());
        assert!(b.grad.lock().unwrap().is_some());
    }

    #[test]
    fn test_backward_mul() {
        let a = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 2.0_f32)).requires_grad_();
        let b = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f32)).requires_grad_();

        let mut c = a.mul(&b);
        c.backward();

        assert!(a.grad.lock().unwrap().is_some());
        assert!(b.grad.lock().unwrap().is_some());
    }
}
