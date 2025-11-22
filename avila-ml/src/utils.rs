//! Utility functions for machine learning

use crate::tensor::Tensor;
use ndarray::ArrayD;
use num_traits::{Float, NumAssign};

/// Initialize weights with Xavier/Glorot initialization
pub fn xavier_uniform<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static>(
    shape: &[usize],
) -> Tensor<T> {
    let fan_in = if shape.len() > 1 { shape[1] } else { shape[0] };
    let fan_out = shape[0];
    let limit = T::from((6.0 / (fan_in + fan_out) as f64).sqrt()).unwrap();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let size: usize = shape.iter().product();

    let data: Vec<T> = (0..size)
        .map(|_| {
            let val: f64 = rng.gen::<f64>() * 2.0 - 1.0;
            T::from(val).unwrap() * limit
        })
        .collect();

    Tensor::new(ArrayD::from_shape_vec(ndarray::IxDyn(shape), data).unwrap())
}

/// Initialize weights with Kaiming/He initialization
pub fn kaiming_uniform<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static>(
    shape: &[usize],
) -> Tensor<T> {
    let fan_in = if shape.len() > 1 { shape[1] } else { shape[0] };
    let limit = T::from((3.0 / fan_in as f64).sqrt()).unwrap();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let size: usize = shape.iter().product();

    let data: Vec<T> = (0..size)
        .map(|_| {
            let val: f64 = rng.gen::<f64>() * 2.0 - 1.0;
            T::from(val).unwrap() * limit
        })
        .collect();

    Tensor::new(ArrayD::from_shape_vec(ndarray::IxDyn(shape), data).unwrap())
}

/// Split data into train and test sets
pub fn train_test_split<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static>(
    data: Vec<Tensor<T>>,
    targets: Vec<Tensor<T>>,
    test_ratio: f64,
) -> (
    Vec<Tensor<T>>,
    Vec<Tensor<T>>,
    Vec<Tensor<T>>,
    Vec<Tensor<T>>,
) {
    assert_eq!(data.len(), targets.len());
    assert!(test_ratio > 0.0 && test_ratio < 1.0);

    let n = data.len();
    let test_size = (n as f64 * test_ratio) as usize;
    let train_size = n - test_size;

    // Shuffle indices
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..n).collect();
    indices.shuffle(&mut rng);

    let mut train_data = Vec::with_capacity(train_size);
    let mut train_targets = Vec::with_capacity(train_size);
    let mut test_data = Vec::with_capacity(test_size);
    let mut test_targets = Vec::with_capacity(test_size);

    for (i, &idx) in indices.iter().enumerate() {
        if i < train_size {
            train_data.push(data[idx].clone());
            train_targets.push(targets[idx].clone());
        } else {
            test_data.push(data[idx].clone());
            test_targets.push(targets[idx].clone());
        }
    }

    (train_data, train_targets, test_data, test_targets)
}

/// Compute accuracy for classification
pub fn accuracy<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static>(
    predictions: &[Tensor<T>],
    targets: &[Tensor<T>],
) -> f64 {
    assert_eq!(predictions.len(), targets.len());

    let mut correct = 0;
    let total = predictions.len();

    for (pred, target) in predictions.iter().zip(targets.iter()) {
        // For multi-class, find argmax
        let pred_class = argmax(&pred.data);
        let target_class = argmax(&target.data);

        if pred_class == target_class {
            correct += 1;
        }
    }

    correct as f64 / total as f64
}

/// Find index of maximum value
pub fn argmax<T: Float>(arr: &ArrayD<T>) -> usize {
    arr.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

/// One-hot encode a label
pub fn one_hot<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static>(
    label: usize,
    num_classes: usize,
) -> Tensor<T> {
    let mut data = ArrayD::zeros(ndarray::IxDyn(&[num_classes]));
    data[[label]] = T::one();
    Tensor::new(data)
}

/// Learning rate scheduler - Step decay
pub struct StepLR {
    initial_lr: f32,
    step_size: usize,
    gamma: f32,
    current_step: usize,
}

impl StepLR {
    pub fn new(initial_lr: f32, step_size: usize, gamma: f32) -> Self {
        Self {
            initial_lr,
            step_size,
            gamma,
            current_step: 0,
        }
    }

    pub fn step(&mut self) -> f32 {
        self.current_step += 1;
        let num_steps = self.current_step / self.step_size;
        self.initial_lr * self.gamma.powi(num_steps as i32)
    }

    pub fn get_lr(&self) -> f32 {
        let num_steps = self.current_step / self.step_size;
        self.initial_lr * self.gamma.powi(num_steps as i32)
    }
}

/// Cosine annealing learning rate scheduler
pub struct CosineAnnealingLR {
    initial_lr: f32,
    min_lr: f32,
    t_max: usize,
    current_step: usize,
}

impl CosineAnnealingLR {
    pub fn new(initial_lr: f32, t_max: usize) -> Self {
        Self {
            initial_lr,
            min_lr: 0.0,
            t_max,
            current_step: 0,
        }
    }

    pub fn with_min_lr(mut self, min_lr: f32) -> Self {
        self.min_lr = min_lr;
        self
    }

    pub fn step(&mut self) -> f32 {
        self.current_step += 1;
        self.get_lr()
    }

    pub fn get_lr(&self) -> f32 {
        let t = (self.current_step % self.t_max) as f32;
        let cos_val = (std::f32::consts::PI * t / self.t_max as f32).cos();
        self.min_lr + (self.initial_lr - self.min_lr) * (1.0 + cos_val) / 2.0
    }
}

/// Early stopping helper
pub struct EarlyStopping {
    patience: usize,
    min_delta: f32,
    counter: usize,
    best_loss: f32,
    should_stop: bool,
}

impl EarlyStopping {
    pub fn new(patience: usize, min_delta: f32) -> Self {
        Self {
            patience,
            min_delta,
            counter: 0,
            best_loss: f32::INFINITY,
            should_stop: false,
        }
    }

    pub fn step(&mut self, loss: f32) -> bool {
        if loss < self.best_loss - self.min_delta {
            self.best_loss = loss;
            self.counter = 0;
        } else {
            self.counter += 1;
            if self.counter >= self.patience {
                self.should_stop = true;
            }
        }

        self.should_stop
    }

    pub fn should_stop(&self) -> bool {
        self.should_stop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xavier_init() {
        let weights = xavier_uniform::<f32>(&[10, 20]);
        assert_eq!(weights.shape(), &[10, 20]);
    }

    #[test]
    fn test_one_hot() {
        let encoded = one_hot::<f32>(2, 5);
        assert_eq!(encoded.data[[2]], 1.0);
        assert_eq!(encoded.data[[0]], 0.0);
    }

    #[test]
    fn test_argmax() {
        let arr = ndarray::arr1(&[1.0_f32, 3.0, 2.0]).into_dyn();
        assert_eq!(argmax(&arr), 1);
    }

    #[test]
    fn test_step_lr() {
        let mut scheduler = StepLR::new(0.1, 10, 0.1);

        assert_eq!(scheduler.get_lr(), 0.1);

        for _ in 0..10 {
            scheduler.step();
        }

        let new_lr = scheduler.get_lr();
        assert!((new_lr - 0.01).abs() < 1e-5);
    }

    #[test]
    fn test_early_stopping() {
        let mut early_stop = EarlyStopping::new(3, 0.01);

        assert!(!early_stop.step(1.0));
        assert!(!early_stop.step(1.0));
        assert!(!early_stop.step(1.0));
        assert!(early_stop.step(1.0)); // Should stop after patience
    }
}
