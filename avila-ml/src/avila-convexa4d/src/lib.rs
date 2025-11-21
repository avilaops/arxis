//! # avila-convexa4d
//!
//! Biblioteca para processamento de dados 4D em Rust puro.
//!
//! ## Características
//!
//! - **Tensor4D**: Tensores 4D (tempo × profundidade × altura × largura)
//! - **VolumeSequence**: Sequências de volumes 3D ao longo do tempo
//! - **Convolução 4D**: Filtros espaço-temporais
//! - **Processamento**: Operações em dados 4D
//!
//! ## Exemplos
//!
//! ```rust
//! use avila_convexa4d::tensor::Tensor4D;
//! use avila_convexa4d::sequence::{VolumeSequence, VolumeSequenceProcessor};
//!
//! // Criar tensor 4D
//! let tensor = Tensor4D::zeros(10, 20, 30, 40, 1);
//!
//! // Criar sequência de volumes
//! let sequence = VolumeSequenceProcessor::create_test_sequence(5, 10, 10, 10);
//! ```

pub mod common;
pub mod tensor;
pub mod sequence;
pub mod filters;
pub mod processor;

// Re-exports principais
pub use common::{Point4D, Size4D, BoundingBox4D, Axis4D};
pub use tensor::{Tensor4D, TensorOps};
pub use sequence::{VolumeSequence, VolumeSequenceProcessor};
pub use filters::{ConvolutionKernel4D, Filter4D};
pub use processor::{SpatioTemporalProcessor, MotionAnalyzer};
