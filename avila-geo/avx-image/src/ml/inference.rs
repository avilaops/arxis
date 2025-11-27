//! Model inference utilities

use crate::core::ImageBuffer;
use crate::Result;

/// Run inference on an image
pub fn infer(img: &ImageBuffer, model_path: &str) -> Result<Vec<f32>> {
    // TODO: Implement ONNX/TensorFlow/PyTorch inference
    let _ = (img, model_path);
    Ok(vec![])
}

/// Batch inference
pub fn batch_infer(images: &[ImageBuffer], model_path: &str) -> Result<Vec<Vec<f32>>> {
    let _ = (images, model_path);
    Ok(vec![])
}
