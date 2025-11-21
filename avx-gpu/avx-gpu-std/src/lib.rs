//! Standard library for AVX-GPU
//!
//! Common GPU operations optimized for various backends.

pub mod linalg;
pub mod signal;
pub mod image;

pub mod prelude {
    pub use crate::linalg::*;
    pub use crate::signal::*;
    pub use crate::image::*;
}
