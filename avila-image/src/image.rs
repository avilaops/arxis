//! Image buffer and core image type

use crate::color::{Rgb, Pixel};
use crate::error::{ImageError, Result};

/// Image buffer storing pixel data
#[derive(Clone)]
pub struct ImageBuffer<P: Pixel> {
    width: u32,
    height: u32,
    data: Vec<u8>,
    _phantom: std::marker::PhantomData<P>,
}

impl<P: Pixel> ImageBuffer<P> {
    /// Create a new image buffer with default color
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = (width * height) as usize;
        let data = vec![0u8; pixel_count * P::CHANNEL_COUNT];

        Self {
            width,
            height,
            data,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get image width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get image height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get raw pixel data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable raw pixel data
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Get pixel at coordinates
    pub fn get_pixel(&self, x: u32, y: u32) -> P {
        let index = ((y * self.width + x) as usize) * P::CHANNEL_COUNT;
        P::from_channels(&self.data[index..index + P::CHANNEL_COUNT])
    }

    /// Set pixel at coordinates
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: P) {
        let index = ((y * self.width + x) as usize) * P::CHANNEL_COUNT;
        let channels = pixel.channels();
        self.data[index..index + P::CHANNEL_COUNT].copy_from_slice(channels);
    }

    /// Iterate over pixels
    pub fn pixels(&self) -> PixelIterator<'_, P> {
        PixelIterator {
            buffer: self,
            x: 0,
            y: 0,
        }
    }
}

/// Iterator over pixels in an image
pub struct PixelIterator<'a, P: Pixel> {
    buffer: &'a ImageBuffer<P>,
    x: u32,
    y: u32,
}

impl<'a, P: Pixel> Iterator for PixelIterator<'a, P> {
    type Item = (u32, u32, P);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.buffer.height {
            return None;
        }

        let pixel = self.buffer.get_pixel(self.x, self.y);
        let result = (self.x, self.y, pixel);

        self.x += 1;
        if self.x >= self.buffer.width {
            self.x = 0;
            self.y += 1;
        }

        Some(result)
    }
}

/// Main image type (RGB by default)
pub type Image = ImageBuffer<Rgb>;

impl Image {
    /// Load image from file
    pub fn load(_path: &str) -> Result<Self> {
        // Stub - actual codec implementation needed
        Err(ImageError::UnsupportedFormat)
    }

    /// Save image to file
    pub fn save(&self, _path: &str, _format: crate::ImageFormat) -> Result<()> {
        // Stub - actual codec implementation needed
        Err(ImageError::UnsupportedFormat)
    }

    /// Create image from raw RGB data
    pub fn from_raw(width: u32, height: u32, data: Vec<u8>) -> Result<Self> {
        let expected_size = (width * height * 3) as usize;
        if data.len() != expected_size {
            return Err(ImageError::InvalidDimensions);
        }

        Ok(Self {
            width,
            height,
            data,
            _phantom: std::marker::PhantomData,
        })
    }
}
