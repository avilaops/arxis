//! DICOM file format support

use crate::medical::MedicalImage;
use crate::Result;

/// Load DICOM file
pub fn load_dicom_file(path: &str) -> Result<MedicalImage> {
    // TODO: Implement DICOM parsing
    // - Parse DICOM tags
    // - Extract pixel data
    // - Handle compression

    Err(crate::AvxImageError::ProcessingError(
        "DICOM loading not yet implemented".into(),
    ))
}
