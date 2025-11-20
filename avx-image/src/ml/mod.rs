//! Machine learning utilities and model inference

pub mod inference;
pub mod training;

use crate::core::ImageBuffer;
use crate::Result;

/// Load ONNX model for inference
pub fn load_onnx_model(path: &str) -> Result<Model> {
    // TODO: Integrate with tract-onnx
    Err(crate::AvxImageError::ModelNotFound(path.to_string()))
}

pub struct Model;

impl Model {
    pub fn predict(&self, img: &ImageBuffer) -> Result<Vec<f32>> {
        Ok(vec![])
    }
}
