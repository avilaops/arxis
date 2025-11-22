//! Face recognition module
//!
//! Detection, landmarks, recognition, liveness detection, and anti-spoofing.

pub mod detection;
pub mod landmarks;
pub mod liveness;
pub mod recognition;

use crate::core::ImageBuffer;
use crate::Result;

/// Face detection result
#[derive(Debug, Clone)]
pub struct Face {
    pub bbox: BoundingBox,
    pub confidence: f32,
    pub landmarks: Option<FaceLandmarks>,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// 68-point facial landmarks
#[derive(Debug, Clone)]
pub struct FaceLandmarks {
    pub points: Vec<(f32, f32)>,
}

impl FaceLandmarks {
    /// Get specific landmark groups
    pub fn left_eye(&self) -> &[(f32, f32)] {
        &self.points[36..42]
    }

    pub fn right_eye(&self) -> &[(f32, f32)] {
        &self.points[42..48]
    }

    pub fn nose(&self) -> &[(f32, f32)] {
        &self.points[27..36]
    }

    pub fn mouth(&self) -> &[(f32, f32)] {
        &self.points[48..68]
    }

    pub fn jaw(&self) -> &[(f32, f32)] {
        &self.points[0..17]
    }
}

/// Detect faces in an image
///
/// # Example
/// ```no_run
/// use avx_image::prelude::*;
/// use avx_image::face;
///
/// let img = ImageBuffer::load("photo.jpg")?;
/// let faces = face::detect(&img)?;
/// println!("Found {} faces", faces.len());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn detect(img: &ImageBuffer) -> Result<Vec<Face>> {
    detection::detect_faces(img, &DetectionOptions::default())
}

/// Face detection options
#[derive(Debug, Clone)]
pub struct DetectionOptions {
    pub min_face_size: u32,
    pub max_faces: usize,
    pub confidence_threshold: f32,
}

impl Default for DetectionOptions {
    fn default() -> Self {
        Self {
            min_face_size: 40,
            max_faces: 100,
            confidence_threshold: 0.7,
        }
    }
}

/// Extract face embeddings for recognition
pub fn extract_embedding(img: &ImageBuffer, face: &Face) -> Result<Vec<f32>> {
    recognition::extract_face_embedding(img, face)
}

/// Compare two face embeddings (cosine similarity)
pub fn compare_faces(embedding1: &[f32], embedding2: &[f32]) -> f32 {
    recognition::compute_similarity(embedding1, embedding2)
}

/// Detect facial landmarks
pub fn detect_landmarks(img: &ImageBuffer, face: &Face) -> Result<FaceLandmarks> {
    landmarks::detect_landmarks(img, face)
}

/// Perform liveness detection
pub fn check_liveness(img: &ImageBuffer, face: &Face) -> Result<LivenessResult> {
    liveness::detect_liveness(img, face)
}

#[derive(Debug, Clone)]
pub struct LivenessResult {
    pub is_live: bool,
    pub confidence: f32,
    pub spoofing_type: Option<SpoofingType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpoofingType {
    PrintedPhoto,
    DigitalDisplay,
    Mask3D,
    Video,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_detection_stub() {
        let img = ImageBuffer::new(640, 480, 3);
        let result = detect(&img);
        assert!(result.is_ok());
    }
}
