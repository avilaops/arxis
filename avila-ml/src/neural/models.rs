//! Neural Network Models

use super::tensor::{Tensor, Shape, Layer, Linear};
use alloc::vec::Vec;

/// Sequential model - chain of layers
pub struct Sequential {
    layers: Vec<Box<dyn Layer>>,
}

impl Sequential {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add<L: Layer + 'static>(&mut self, layer: L) {
        self.layers.push(Box::new(layer));
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        let mut output = input.clone();
        for layer in &self.layers {
            output = layer.forward(&output);
        }
        output
    }

    pub fn parameters_mut(&mut self) -> Vec<&mut Tensor> {
        self.layers.iter_mut()
            .flat_map(|layer| layer.parameters_mut())
            .collect()
    }
}

/// Multi-layer Perceptron
pub struct MLP {
    layers: Vec<Linear>,
}

impl MLP {
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();

        for i in 0..layer_sizes.len() - 1 {
            layers.push(Linear::new(layer_sizes[i], layer_sizes[i + 1]));
        }

        Self { layers }
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        let mut output = input.clone();

        for (i, layer) in self.layers.iter().enumerate() {
            output = layer.forward(&output);

            // Apply ReLU to all layers except the last
            if i < self.layers.len() - 1 {
                output = output.relu();
            }
        }

        output
    }

    pub fn parameters_mut(&mut self) -> Vec<&mut Tensor> {
        self.layers.iter_mut()
            .flat_map(|layer| layer.parameters_mut())
            .collect()
    }
}

/// Loss functions
pub struct MSELoss;

impl MSELoss {
    pub fn forward(predictions: &Tensor, targets: &Tensor) -> Tensor {
        let diff = predictions.add(&targets.clone()); // Would need subtraction
        diff.sum()
    }
}

pub struct CrossEntropyLoss;

impl CrossEntropyLoss {
    pub fn forward(logits: &Tensor, targets: &Tensor) -> Tensor {
        // Simplified - would need proper softmax + log
        logits.sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mlp_creation() {
        let mlp = MLP::new(&[784, 128, 10]);
        assert_eq!(mlp.layers.len(), 2);
    }

    #[test]
    fn test_mlp_forward() {
        let mlp = MLP::new(&[2, 4, 1]);
        let input = Tensor::ones(Shape::new(&[1, 2]), false);
        let output = mlp.forward(&input);

        assert_eq!(output.shape.dims(), &[1, 1]);
    }
}
