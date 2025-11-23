//! # avx-image - Advanced Computer Vision & Image Generation Library
//!
//! Complete computer vision suite with OCR, face recognition, photometry,
//! medical imaging, forensics, real-time processing, and cutting-edge
//! generative AI capabilities.
//!
//! ## Features
//!
//! ### Traditional Computer Vision
//! - **OCR**: Text detection and recognition (100% Rust, no Tesseract)
//! - **Face Recognition**: Detection, landmarks, recognition, anti-spoofing
//! - **Photometry**: Color science, illumination analysis, calibration
//! - **Medical Imaging**: DICOM support, segmentation, measurements
//! - **Forensics**: Fingerprint analysis, document verification
//! - **Real-time**: Camera streaming, object tracking, optimization
//!
//! ### Advanced Generative AI
//! - **Image Synthesis**: Stable Diffusion, ControlNet, DreamBooth
//! - **Neural Radiance Fields**: NeRF, Instant-NGP, 3D reconstruction
//! - **Video Generation**: Text-to-video, image-to-video, frame interpolation
//! - **Holographic Imaging**: Light field processing, computational photography
//! - **Quantum Processing**: QPIXL representation, quantum-inspired algorithms
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avx_image::prelude::*;
//! use avx_image::native::NativeImageBuffer;
//!
//! // Traditional image processing
//! let mut img = NativeImageBuffer::new(1920, 1080, 3);
//! let gray = img.to_grayscale();
//! let blurred = gray.gaussian_blur(2.0);
//!
//! // Generative AI
//! use avx_image::synthesis::StableDiffusion;
//! let mut sd = StableDiffusion::default();
//! sd.load_model()?;
//! let generated = sd.generate("a beautiful landscape", 512, 512)?;
//!
//! println!("Generated image: {}x{}", generated.width(), generated.height());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Native implementations (100% Rust, zero external dependencies)
pub mod native;

// High-level modules (built on top of native)
pub mod core;
pub mod face;
pub mod forensics;
pub mod medical;
pub mod ml;
pub mod ocr;
pub mod photometry;
pub mod realtime;

// Advanced generative AI modules
pub mod synthesis;
pub mod nerf;
pub mod video_gen;
pub mod holographic;
pub mod quantum;

pub mod prelude {
    // Traditional CV
    pub use crate::native::{
        buffer::NativeImageBuffer,
        color::*,
        convolution::*,
        fft::*,
        linalg::*,
        math::*,
        simd::*,
    };
    pub use crate::core::ImageBuffer;
    pub use crate::AvxImageError;

    // Generative AI
    pub use crate::synthesis::{StableDiffusion, ControlNet, DreamBooth};
    pub use crate::nerf::{NeRFScene, ViewSynthesizer, InstantNGP};
    pub use crate::video_gen::{TextToVideo, ImageToVideo, VideoInterpolator};
    pub use crate::holographic::{LightField, ComputationalPhotography};
    pub use crate::quantum::{QPIXL, QuantumAlgorithms};
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
