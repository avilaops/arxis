//! Core image processing module
//!
//! Provides fundamental image operations: buffers, preprocessing, feature extraction.

pub mod features;
pub mod image_buffer;
pub mod preprocessing;

pub use features::{FeatureExtractor, Keypoint};
pub use image_buffer::ImageBuffer;
pub use preprocessing::Preprocessing;
