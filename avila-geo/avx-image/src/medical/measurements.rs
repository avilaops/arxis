//! Anatomical measurements

use crate::medical::{Measurements, MedicalImage, Segmentation};
use crate::Result;

/// Compute measurements from segmentation
pub fn compute_measurements(img: &MedicalImage, seg: &Segmentation) -> Result<Measurements> {
    // TODO: Calculate volume, area, diameter
    Ok(Measurements {
        volume_ml: 0.0,
        area_mm2: 0.0,
        diameter_mm: 0.0,
    })
}
