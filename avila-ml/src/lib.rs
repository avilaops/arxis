//! # Avila ML
//!
//! High-performance Machine Learning library designed for scientific computing.
//!
//! ## Features
//!
//! - **Autograd**: Automatic differentiation with computational graph
//! - **Neural Networks**: Linear, Conv2d, Conv4d, Attention layers
//! - **Optimizers**: SGD, Adam, AdamW, RMSprop with schedulers
//! - **Scientific Computing**: 4D convolutions for astrophysical data
//! - **Performance**: Built on ndarray with rayon parallelism
//!
//! ## Quick Start
//!
//! ```rust
//! use avila_ml::prelude::*;
//!
//! // Create a simple neural network
//! let mut model = Sequential::new(vec![
//!     Box::new(Linear::new(784, 128)),
//!     Box::new(ReLU::new()),
//!     Box::new(Linear::new(128, 10)),
//! ]);
//!
//! // Train with optimizer
//! let mut optimizer = Adam::new(model.parameters(), 0.001);
//! ```

pub mod autograd;
pub mod data;
pub mod loss;
pub mod nn;
pub mod optim;
pub mod tensor;
pub mod utils;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::data::{DataLoader, Dataset};
    pub use crate::loss::{BCELoss, CrossEntropyLoss, Loss, MSELoss};
    pub use crate::nn::activation::{ReLU, Sigmoid, Softmax, Tanh};
    pub use crate::nn::attention::{Attention, MultiHeadAttention};
    pub use crate::nn::normalization::BatchNorm;
    pub use crate::nn::{Conv2d, Conv4d, Linear, Module, Sequential};
    pub use crate::optim::{Adam, AdamW, Optimizer, RMSprop, SGD};
    pub use crate::tensor::{Tensor, TensorLike};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_import() {
        // Smoke test to ensure library compiles
    }
}
