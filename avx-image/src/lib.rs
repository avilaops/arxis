//! # avx-image - Computer Vision Library for Avila Platform
//!
//! Complete computer vision suite with OCR, face recognition, photometry,
//! medical imaging, forensics, and real-time processing.
//!
//! ## Features
//!
//! - **OCR**: Text detection and recognition (100% Rust, no Tesseract)
//! - **Face Recognition**: Detection, landmarks, recognition, anti-spoofing
//! - **Photometry**: Color science, illumination analysis, calibration
//! - **Medical Imaging**: DICOM support, segmentation, measurements
//! - **Forensics**: Fingerprint analysis, document verification
//! - **Real-time**: Camera streaming, object tracking, optimization
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avx_image::prelude::*;
//!
//! // Load image
//! let img = ImageBuffer::load("document.jpg")?;
//!
//! // OCR
//! let text = ocr::recognize(&img)?;
//! println!("Detected text: {}", text);
//!
//! // Face detection
//! let faces = face::detect(&img)?;
//! println!("Found {} faces", faces.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod core;
pub mod face;
pub mod forensics;
pub mod medical;
pub mod ml;
pub mod ocr;
pub mod photometry;
pub mod realtime;

pub mod prelude {
    pub use crate::core::{FeatureExtractor, ImageBuffer, Preprocessing};
    pub use crate::face;
    pub use crate::ocr;
    pub use crate::photometry;
    pub use crate::AvxImageError;
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AvxImageError {
    #[error("Image processing error: {0}")]
    ProcessingError(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Invalid image format: {0}")]
    InvalidFormat(String),

    #[error("OCR error: {0}")]
    OcrError(String),

    #[error("Face detection error: {0}")]
    FaceError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}

pub type Result<T> = std::result::Result<T, AvxImageError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_compiles() {
        // Smoke test
        assert!(true);
    }
}
