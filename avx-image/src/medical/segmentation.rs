//! Medical image segmentation

use crate::medical::{MedicalImage, Segmentation, SegmentationTarget};
use crate::Result;

/// Segment medical image using deep learning
pub fn segment_image(img: &MedicalImage, target: SegmentationTarget) -> Result<Segmentation> {
    // TODO: Implement U-Net based segmentation
    Err(crate::AvxImageError::ProcessingError(
        "Medical segmentation not yet implemented".into(),
    ))
}
