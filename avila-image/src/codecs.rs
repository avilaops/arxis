//! Image codecs (stub implementations)

use crate::error::Result;
use crate::Image;

/// PNG codec
#[cfg(feature = "png")]
pub mod png {
    use super::*;

    /// Load PNG image
    pub fn load(_path: &str) -> Result<Image> {
        unimplemented!("PNG loading - to be implemented")
    }

    /// Save PNG image
    pub fn save(_image: &Image, _path: &str) -> Result<()> {
        unimplemented!("PNG saving - to be implemented")
    }
}

/// JPEG codec
#[cfg(feature = "jpeg")]
pub mod jpeg {
    use super::*;

    /// Load JPEG image
    pub fn load(_path: &str) -> Result<Image> {
        unimplemented!("JPEG loading - to be implemented")
    }

    /// Save JPEG image
    pub fn save(_image: &Image, _path: &str, _quality: u8) -> Result<()> {
        unimplemented!("JPEG saving - to be implemented")
    }
}

/// BMP codec
#[cfg(feature = "bmp")]
pub mod bmp {
    use super::*;

    /// Load BMP image
    pub fn load(_path: &str) -> Result<Image> {
        unimplemented!("BMP loading - to be implemented")
    }

    /// Save BMP image
    pub fn save(_image: &Image, _path: &str) -> Result<()> {
        unimplemented!("BMP saving - to be implemented")
    }
}
