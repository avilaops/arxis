//! # Avila Convexa2D
//!
//! Processamento de dados bidimensionais (2D) para imagens e matrizes.
//! Otimizado para álgebra linear, visão computacional e processamento de imagens.

pub mod image;
pub mod matrix;
pub mod filters;
pub mod transform;
pub mod common;

pub use image::{Image, ImageProcessor, ColorSpace, Pixel};
pub use matrix::{Matrix2D, MatrixOps};
pub use filters::{ConvolutionKernel2D, Filter, EdgeDetection};
pub use transform::{Transform2D, Interpolation};
pub use common::{Point2D, Size2D, Rect};

/// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
