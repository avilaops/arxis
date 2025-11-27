//! Camera capture utilities

use crate::core::ImageBuffer;
use crate::Result;

/// Camera device
pub struct Camera {
    device_id: usize,
}

impl Camera {
    pub fn new(device_id: usize) -> Result<Self> {
        Ok(Self { device_id })
    }

    pub fn capture(&mut self) -> Result<ImageBuffer> {
        // TODO: Implement camera capture
        let _ = self.device_id;
        Ok(ImageBuffer::new(640, 480, 3))
    }
}
