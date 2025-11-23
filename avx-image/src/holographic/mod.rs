//! Holographic Imaging Module
//!
//! Light field capture, processing, and computational photography

pub mod light_field;
pub mod computational;

pub use light_field::LightField;
pub use computational::ComputationalPhotography;

use crate::core::ImageBuffer;

/// Result type for holographic operations
pub type HolographicResult<T> = Result<T, HolographicError>;

/// Errors specific to holographic imaging
#[derive(Debug, Clone)]
pub enum HolographicError {
    /// Invalid light field data
    InvalidLightField(String),
    /// Insufficient views
    InsufficientViews(String),
    /// Processing failed
    ProcessingFailed(String),
    /// Unsupported format
    UnsupportedFormat(String),
}

impl std::fmt::Display for HolographicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLightField(msg) => write!(f, "Invalid light field: {}", msg),
            Self::InsufficientViews(msg) => write!(f, "Insufficient views: {}", msg),
            Self::ProcessingFailed(msg) => write!(f, "Processing failed: {}", msg),
            Self::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
        }
    }
}

impl std::error::Error for HolographicError {}

/// 4D light field coordinates (spatial + angular)
#[derive(Debug, Clone, Copy)]
pub struct LightRay {
    /// Spatial coordinates (u, v)
    pub spatial: (f32, f32),
    /// Angular coordinates (s, t)
    pub angular: (f32, f32),
}

impl LightRay {
    pub fn new(u: f32, v: f32, s: f32, t: f32) -> Self {
        Self {
            spatial: (u, v),
            angular: (s, t),
        }
    }
}

/// RGB color with intensity
#[derive(Debug, Clone, Copy)]
pub struct Radiance {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Radiance {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
}
