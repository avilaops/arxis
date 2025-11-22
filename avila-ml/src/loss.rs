//! Loss functions for training

use crate::tensor::{Tensor, TensorLike};
use num_traits::{Float, NumAssign};

/// Base trait for loss functions
pub trait Loss<T = f32>
where
    T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static,
{
    /// Compute the loss
    fn forward(&self, predictions: &Tensor<T>, targets: &Tensor<T>) -> Tensor<T>;
}

/// Mean Squared Error loss
#[derive(Default)]
pub struct MSELoss;

impl MSELoss {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Loss<T> for MSELoss {
    fn forward(&self, predictions: &Tensor<T>, targets: &Tensor<T>) -> Tensor<T> {
        // MSE = mean((predictions - targets)^2)
        let diff = predictions.sub(targets);
        let squared = diff.mul(&diff);
        squared.mean()
    }
}

/// Binary Cross Entropy loss
pub struct BCELoss {
    epsilon: f32,
}

impl Default for BCELoss {
    fn default() -> Self {
        Self::new()
    }
}

impl BCELoss {
    pub fn new() -> Self {
        Self { epsilon: 1e-7 }
    }

    pub fn with_epsilon(mut self, epsilon: f32) -> Self {
        self.epsilon = epsilon;
        self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Loss<T> for BCELoss {
    fn forward(&self, predictions: &Tensor<T>, targets: &Tensor<T>) -> Tensor<T> {
        // BCE = -mean(targets * log(predictions) + (1 - targets) * log(1 - predictions))
        let eps = T::from(self.epsilon).unwrap();
        let one = T::one();

        // Clamp predictions to avoid log(0)
        let pred_clamped = predictions.data.mapv(|x| x.max(eps).min(one - eps));

        let log_pred = pred_clamped.mapv(|x| x.ln());
        let log_one_minus_pred = pred_clamped.mapv(|x| (one - x).ln());

        let term1 = &targets.data * &log_pred;
        let term2 = targets.data.mapv(|x| one - x) * &log_one_minus_pred;

        let loss_data = -(term1 + term2);
        let loss_sum = Tensor::new(ndarray::ArrayD::from_elem(
            ndarray::IxDyn(&[]),
            loss_data.sum(),
        ));

        // Return mean
        let size = T::from(predictions.size()).unwrap();
        let mean_val = loss_sum.data[[]] / size;
        Tensor::scalar(mean_val)
    }
}

/// Cross Entropy loss (for multi-class classification)
pub struct CrossEntropyLoss {
    epsilon: f32,
}

impl Default for CrossEntropyLoss {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossEntropyLoss {
    pub fn new() -> Self {
        Self { epsilon: 1e-7 }
    }

    pub fn with_epsilon(mut self, epsilon: f32) -> Self {
        self.epsilon = epsilon;
        self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Loss<T>
    for CrossEntropyLoss
{
    fn forward(&self, predictions: &Tensor<T>, targets: &Tensor<T>) -> Tensor<T> {
        // CrossEntropy = -sum(targets * log(softmax(predictions)))
        let eps = T::from(self.epsilon).unwrap();

        // Apply softmax to predictions
        let max_val = predictions
            .data
            .iter()
            .cloned()
            .fold(T::neg_infinity(), T::max);
        let exp_data = predictions.data.mapv(|x| (x - max_val).exp());
        let sum = exp_data.sum();
        let softmax = exp_data.mapv(|x| (x / sum).max(eps));

        // Compute cross entropy
        let log_softmax = softmax.mapv(|x| x.ln());
        let loss_data = -(&targets.data * &log_softmax);

        let loss_sum = loss_data.sum();
        let size = T::from(predictions.size()).unwrap();

        Tensor::scalar(loss_sum / size)
    }
}

/// Huber loss (smooth L1 loss)
pub struct HuberLoss<T = f32> {
    delta: T,
}

impl<T: Float> HuberLoss<T> {
    pub fn new(delta: T) -> Self {
        Self { delta }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Loss<T>
    for HuberLoss<T>
{
    fn forward(&self, predictions: &Tensor<T>, targets: &Tensor<T>) -> Tensor<T> {
        // Huber(a) = 0.5 * a^2 if |a| <= delta
        //          = delta * (|a| - 0.5 * delta) otherwise
        let diff = predictions.sub(targets);
        let half = T::from(0.5).unwrap();

        let loss_data = diff.data.mapv(|x| {
            let abs_x = x.abs();
            if abs_x <= self.delta {
                half * x * x
            } else {
                self.delta * (abs_x - half * self.delta)
            }
        });

        let loss_sum = loss_data.sum();
        let size = T::from(predictions.size()).unwrap();

        Tensor::scalar(loss_sum / size)
    }
}

/// L1 loss (Mean Absolute Error)
#[derive(Default)]
pub struct L1Loss;

impl L1Loss {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Loss<T> for L1Loss {
    fn forward(&self, predictions: &Tensor<T>, targets: &Tensor<T>) -> Tensor<T> {
        let diff = predictions.sub(targets);
        let abs_diff = diff.data.mapv(|x| x.abs());
        let loss_sum = abs_diff.sum();
        let size = T::from(predictions.size()).unwrap();

        Tensor::scalar(loss_sum / size)
    }
}

/// Smooth L1 Loss
pub struct SmoothL1Loss<T = f32> {
    beta: T,
}

impl<T: Float> SmoothL1Loss<T> {
    pub fn new(beta: T) -> Self {
        Self { beta }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Loss<T>
    for SmoothL1Loss<T>
{
    fn forward(&self, predictions: &Tensor<T>, targets: &Tensor<T>) -> Tensor<T> {
        let diff = predictions.sub(targets);
        let half = T::from(0.5).unwrap();

        let loss_data = diff.data.mapv(|x| {
            let abs_x = x.abs();
            if abs_x < self.beta {
                half * x * x / self.beta
            } else {
                abs_x - half * self.beta
            }
        });

        let loss_sum = loss_data.sum();
        let size = T::from(predictions.size()).unwrap();

        Tensor::scalar(loss_sum / size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse_loss() {
        let loss_fn = MSELoss::new();

        let pred = Tensor::new(ndarray::arr1(&[1.0_f32, 2.0, 3.0]).into_dyn());
        let target = Tensor::new(ndarray::arr1(&[1.0_f32, 2.0, 3.0]).into_dyn());

        let loss = loss_fn.forward(&pred, &target);
        assert!(loss.data[[]] < 1e-6);
    }

    #[test]
    fn test_cross_entropy() {
        let loss_fn = CrossEntropyLoss::new();

        // Predictions (logits)
        let pred = Tensor::new(ndarray::arr1(&[2.0_f32, 1.0, 0.1]).into_dyn());
        // One-hot encoded targets
        let target = Tensor::new(ndarray::arr1(&[1.0_f32, 0.0, 0.0]).into_dyn());

        let loss = loss_fn.forward(&pred, &target);
        assert!(loss.data[[]] > 0.0);
    }

    #[test]
    fn test_bce_loss() {
        let loss_fn = BCELoss::new();

        let pred = Tensor::new(ndarray::arr1(&[0.9_f32, 0.1, 0.8]).into_dyn());
        let target = Tensor::new(ndarray::arr1(&[1.0_f32, 0.0, 1.0]).into_dyn());

        let loss = loss_fn.forward(&pred, &target);
        assert!(loss.data[[]] > 0.0);
    }

    #[test]
    fn test_huber_loss() {
        let loss_fn = HuberLoss::new(1.0_f32);

        let pred = Tensor::new(ndarray::arr1(&[1.0_f32, 2.0, 3.0]).into_dyn());
        let target = Tensor::new(ndarray::arr1(&[1.0_f32, 2.5, 5.0]).into_dyn());

        let loss = loss_fn.forward(&pred, &target);
        assert!(loss.data[[]] > 0.0);
    }
}
