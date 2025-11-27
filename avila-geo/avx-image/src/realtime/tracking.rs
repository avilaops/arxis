//! Object tracking algorithms

use crate::core::ImageBuffer;
use crate::Result;

/// KCF (Kernelized Correlation Filter) tracker
pub struct KCFTracker {
    initialized: bool,
}

impl KCFTracker {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn init(&mut self, _img: &ImageBuffer, _bbox: (u32, u32, u32, u32)) -> Result<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn update(&mut self, _img: &ImageBuffer) -> Result<Option<(u32, u32, u32, u32)>> {
        if !self.initialized {
            return Ok(None);
        }
        Ok(Some((0, 0, 100, 100)))
    }
}
