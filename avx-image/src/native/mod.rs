//! Native implementations - 100% Rust, zero external dependencies
//!
//! This module contains all core primitives implemented from scratch:
//! - Math operations (SIMD optimized)
//! - Image codecs (PNG, JPEG, TIFF)
//! - Color space conversions
//! - Linear algebra
//! - FFT/DCT implementations
//! - Convolution kernels

pub mod math;
pub mod simd;
pub mod linalg;
pub mod color;
pub mod fft;
pub mod convolution;
pub mod codec;
pub mod buffer;
pub mod features;
pub mod optical_flow;
pub mod object_detection;

pub use math::*;
pub use simd::*;
pub use linalg::*;
pub use color::*;
pub use fft::*;
pub use convolution::*;
pub use codec::*;
pub use buffer::*;
pub use features::*;
pub use optical_flow::*;
pub use object_detection::*;
