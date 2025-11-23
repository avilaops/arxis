//! Optimizers for training neural networks

use crate::tensor::Tensor;
use num_traits::{Float, NumAssign};
use std::collections::HashMap;

/// Base trait for all optimizers
pub trait Optimizer<T = f32>
where
    T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static,
{
    /// Perform a single optimization step
    fn step(&mut self);

    /// Zero out all gradients
    fn zero_grad(&mut self);

    /// Get the learning rate
    fn lr(&self) -> T;

    /// Set the learning rate
    fn set_lr(&mut self, lr: T);
}

/// Stochastic Gradient Descent optimizer
pub struct SGD<T = f32> {
    parameters: Vec<*mut Tensor<T>>,
    lr: T,
    momentum: T,
    weight_decay: T,
    velocities: HashMap<usize, ndarray::ArrayD<T>>,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> SGD<T> {
    /// Create a new SGD optimizer
    pub fn new(parameters: Vec<&mut Tensor<T>>, lr: T) -> Self {
        let param_ptrs = parameters.into_iter().map(|p| p as *mut _).collect();
        Self {
            parameters: param_ptrs,
            lr,
            momentum: T::zero(),
            weight_decay: T::zero(),
            velocities: HashMap::new(),
        }
    }

    /// Set momentum factor
    pub fn with_momentum(mut self, momentum: T) -> Self {
        self.momentum = momentum;
        self
    }

    /// Set weight decay (L2 regularization)
    pub fn with_weight_decay(mut self, weight_decay: T) -> Self {
        self.weight_decay = weight_decay;
        self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Optimizer<T>
    for SGD<T>
{
    fn step(&mut self) {
        for (idx, param_ptr) in self.parameters.iter().enumerate() {
            unsafe {
                let param = &mut **param_ptr;

                if let Some(ref grad) = *param.grad.lock().unwrap() {
                    let mut update = grad.clone();

                    // Add weight decay
                    if self.weight_decay != T::zero() {
                        update = &update + &(&param.data * self.weight_decay);
                    }

                    // Apply momentum
                    if self.momentum != T::zero() {
                        let velocity = self
                            .velocities
                            .entry(idx)
                            .or_insert_with(|| ndarray::ArrayD::zeros(param.data.raw_dim()));

                        *velocity = &*velocity * self.momentum + &update;
                        update = velocity.clone();
                    }

                    // Update parameters: θ = θ - lr * gradient
                    param.data = &param.data - &(update * self.lr);
                }
            }
        }
    }

    fn zero_grad(&mut self) {
        for param_ptr in &self.parameters {
            unsafe {
                (**param_ptr).zero_grad();
            }
        }
    }

    fn lr(&self) -> T {
        self.lr
    }

    fn set_lr(&mut self, lr: T) {
        self.lr = lr;
    }
}

/// Adam optimizer (Adaptive Moment Estimation)
pub struct Adam<T = f32> {
    parameters: Vec<*mut Tensor<T>>,
    lr: T,
    beta1: T,
    beta2: T,
    epsilon: T,
    weight_decay: T,
    step_count: usize,
    m: HashMap<usize, ndarray::ArrayD<T>>, // First moment
    v: HashMap<usize, ndarray::ArrayD<T>>, // Second moment
}

impl<T: Float + NumAssign + Send + Sync + 'static> Adam<T> {
    /// Create a new Adam optimizer
    pub fn new(parameters: Vec<&mut Tensor<T>>, lr: T) -> Self {
        let param_ptrs = parameters.into_iter().map(|p| p as *mut _).collect();
        Self {
            parameters: param_ptrs,
            lr,
            beta1: T::from(0.9).unwrap(),
            beta2: T::from(0.999).unwrap(),
            epsilon: T::from(1e-8).unwrap(),
            weight_decay: T::zero(),
            step_count: 0,
            m: HashMap::new(),
            v: HashMap::new(),
        }
    }
    /// Set beta parameters
    pub fn with_betas(mut self, beta1: T, beta2: T) -> Self {
        self.beta1 = beta1;
        self.beta2 = beta2;
        self
    }

    /// Set weight decay
    pub fn with_weight_decay(mut self, weight_decay: T) -> Self {
        self.weight_decay = weight_decay;
        self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Optimizer<T>
    for Adam<T>
{
    fn step(&mut self) {
        self.step_count += 1;
        let t = T::from(self.step_count).unwrap();

        // Bias correction
        let bias_correction1 = T::one() - self.beta1.powf(t);
        let bias_correction2 = T::one() - self.beta2.powf(t);

        for (idx, param_ptr) in self.parameters.iter().enumerate() {
            unsafe {
                let param = &mut **param_ptr;

                if let Some(ref grad) = *param.grad.lock().unwrap() {
                    let mut g = grad.clone();

                    // Add weight decay
                    if self.weight_decay != T::zero() {
                        g = &g + &(&param.data * self.weight_decay);
                    }

                    // Update first moment (mean)
                    let m = self
                        .m
                        .entry(idx)
                        .or_insert_with(|| ndarray::ArrayD::zeros(param.data.raw_dim()));
                    *m = &*m * self.beta1 + &(&g * (T::one() - self.beta1));

                    // Update second moment (variance)
                    let v = self
                        .v
                        .entry(idx)
                        .or_insert_with(|| ndarray::ArrayD::zeros(param.data.raw_dim()));
                    *v = &*v * self.beta2 + &(g.mapv(|x| x * x) * (T::one() - self.beta2));

                    // Bias-corrected moments
                    let m_hat = &*m / bias_correction1;
                    let v_hat = &*v / bias_correction2;

                    // Update parameters
                    let update = m_hat / (v_hat.mapv(|x| x.sqrt()) + self.epsilon);
                    param.data = &param.data - &(update * self.lr);
                }
            }
        }
    }

    fn zero_grad(&mut self) {
        for param_ptr in &self.parameters {
            unsafe {
                (**param_ptr).zero_grad();
            }
        }
    }

    fn lr(&self) -> T {
        self.lr
    }

    fn set_lr(&mut self, lr: T) {
        self.lr = lr;
    }
}

/// AdamW optimizer (Adam with decoupled weight decay)
pub struct AdamW<T = f32> {
    adam: Adam<T>,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> AdamW<T> {
    pub fn new(parameters: Vec<&mut Tensor<T>>, lr: T) -> Self {
        Self {
            adam: Adam::new(parameters, lr),
        }
    }

    pub fn with_betas(mut self, beta1: T, beta2: T) -> Self {
        self.adam = self.adam.with_betas(beta1, beta2);
        self
    }

    pub fn with_weight_decay(mut self, weight_decay: T) -> Self {
        self.adam = self.adam.with_weight_decay(weight_decay);
        self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Optimizer<T>
    for AdamW<T>
{
    fn step(&mut self) {
        self.adam.step();
    }

    fn zero_grad(&mut self) {
        self.adam.zero_grad();
    }

    fn lr(&self) -> T {
        self.adam.lr()
    }

    fn set_lr(&mut self, lr: T) {
        self.adam.set_lr(lr);
    }
}

/// RMSprop optimizer
pub struct RMSprop<T = f32> {
    parameters: Vec<*mut Tensor<T>>,
    lr: T,
    alpha: T,
    epsilon: T,
    weight_decay: T,
    v: HashMap<usize, ndarray::ArrayD<T>>,
}

impl<T: Float + NumAssign + Send + Sync + 'static> RMSprop<T> {
    pub fn new(parameters: Vec<&mut Tensor<T>>, lr: T) -> Self {
        let param_ptrs = parameters.into_iter().map(|p| p as *mut _).collect();
        Self {
            parameters: param_ptrs,
            lr,
            alpha: T::from(0.99).unwrap(),
            epsilon: T::from(1e-8).unwrap(),
            weight_decay: T::zero(),
            v: HashMap::new(),
        }
    }
    pub fn with_alpha(mut self, alpha: T) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn with_weight_decay(mut self, weight_decay: T) -> Self {
        self.weight_decay = weight_decay;
        self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Optimizer<T>
    for RMSprop<T>
{
    fn step(&mut self) {
        for (idx, param_ptr) in self.parameters.iter().enumerate() {
            unsafe {
                let param = &mut **param_ptr;

                if let Some(ref grad) = *param.grad.lock().unwrap() {
                    let mut g = grad.clone();

                    if self.weight_decay != T::zero() {
                        g = &g + &(&param.data * self.weight_decay);
                    }

                    let v = self
                        .v
                        .entry(idx)
                        .or_insert_with(|| ndarray::ArrayD::zeros(param.data.raw_dim()));
                    *v = &*v * self.alpha + &(g.mapv(|x| x * x) * (T::one() - self.alpha));

                    let update = g / (v.mapv(|x| x.sqrt()) + self.epsilon);
                    param.data = &param.data - &(update * self.lr);
                }
            }
        }
    }

    fn zero_grad(&mut self) {
        for param_ptr in &self.parameters {
            unsafe {
                (**param_ptr).zero_grad();
            }
        }
    }

    fn lr(&self) -> T {
        self.lr
    }

    fn set_lr(&mut self, lr: T) {
        self.lr = lr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sgd_creation() {
        let mut param = Tensor::<f32>::randn(&[10, 10]).requires_grad_();
        let _optimizer = SGD::new(vec![&mut param], 0.01);
    }

    #[test]
    fn test_adam_creation() {
        let mut param = Tensor::<f32>::randn(&[10, 10]).requires_grad_();
        let _optimizer = Adam::new(vec![&mut param], 0.001);
    }
}
