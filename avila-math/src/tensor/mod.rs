pub mod conv4d;
/// Módulo de Tensores - Estruturas tensoriais generalizadas e operações
///
/// Este módulo implementa tensores de ordem arbitrária (0-4) com operações
/// para álgebra linear, machine learning e processamento de dados multidimensionais.
pub mod tensor;
pub mod tensor4d;

pub use conv4d::{avg_pool4d, conv4d, max_pool4d, Conv4DConfig, Conv4DLayer, Tensor5D, Tensor6D};
pub use tensor::{Matrix, Scalar, Tensor, Vector};
pub use tensor4d::{Tensor3D, Tensor4D};
