//! Medical imaging module (DICOM, segmentation, measurements)

pub mod dicom;
pub mod measurements;
pub mod segmentation;

use crate::core::ImageBuffer;
use crate::Result;

/// Load DICOM image
pub fn load_dicom(path: &str) -> Result<MedicalImage> {
    dicom::load_dicom_file(path)
}

/// Segment organs/tissues
pub fn segment(img: &MedicalImage, target: SegmentationTarget) -> Result<Segmentation> {
    segmentation::segment_image(img, target)
}

/// Measure anatomical structures
pub fn measure(img: &MedicalImage, seg: &Segmentation) -> Result<Measurements> {
    measurements::compute_measurements(img, seg)
}

#[derive(Debug, Clone)]
pub struct MedicalImage {
    pub pixel_data: ImageBuffer,
    pub metadata: DicomMetadata,
}

#[derive(Debug, Clone)]
pub struct DicomMetadata {
    pub patient_id: String,
    pub modality: Modality,
    pub pixel_spacing: (f32, f32),
    pub slice_thickness: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modality {
    CT,
    MRI,
    XRay,
    Ultrasound,
    PET,
}

#[derive(Debug, Clone)]
pub enum SegmentationTarget {
    Lungs,
    Heart,
    Liver,
    Brain,
    Tumor,
}

#[derive(Debug, Clone)]
pub struct Segmentation {
    pub mask: ImageBuffer,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct Measurements {
    pub volume_ml: f32,
    pub area_mm2: f32,
    pub diameter_mm: f32,
}
