//! Image buffer implementation
//!
//! LEGACY MODULE: Use `native::buffer::NativeImageBuffer` instead
//! This module is kept for backward compatibility only

use crate::{AvxImageError, Result};

/// Legacy image buffer (deprecated - use NativeImageBuffer)
#[deprecated(since = "0.1.0", note = "Use native::buffer::NativeImageBuffer instead")]
#[derive(Clone, Debug)]
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
    pub channels: usize,
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32, channels: usize) -> Self {
        let size = (width * height) as usize * channels;
        Self {
            width,
            height,
            data: vec![0.0; size],
            channels,
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(_path: P) -> Result<Self> {
        Err(AvxImageError::ProcessingError(
            "Image loading requires codec implementation. Use native::buffer::NativeImageBuffer".into(),
        ))
    }

    pub fn to_grayscale(&self) -> Self {
        if self.channels == 1 {
            return self.clone();
        }

        let mut gray_data = Vec::with_capacity((self.width * self.height) as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = ((y * self.width + x) as usize) * self.channels;

                let gray = if self.channels >= 3 {
                    0.2126 * self.data[idx] + 0.7152 * self.data[idx + 1] + 0.0722 * self.data[idx + 2]
                } else {
                    self.data[idx]
                };

                gray_data.push(gray);
            }
        }

        Self {
            width: self.width,
            height: self.height,
            data: gray_data,
            channels: 1,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Vec<f32> {
        let idx = ((y * self.width + x) as usize) * self.channels;
        self.data[idx..idx + self.channels].to_vec()
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, values: &[f32]) {
        assert_eq!(values.len(), self.channels);
        let idx = ((y * self.width + x) as usize) * self.channels;
        self.data[idx..idx + self.channels].copy_from_slice(values);
    }

    pub fn resize(&self, new_width: u32, new_height: u32) -> Self {
        let x_ratio = self.width as f32 / new_width as f32;
        let y_ratio = self.height as f32 / new_height as f32;

        let mut resized_data = Vec::with_capacity((new_width * new_height) as usize * self.channels);

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 * x_ratio).floor() as u32;
                let src_y = (y as f32 * y_ratio).floor() as u32;

                let src_x = src_x.min(self.width - 1);
                let src_y = src_y.min(self.height - 1);

                let pixel = self.get_pixel(src_x, src_y);
                resized_data.extend_from_slice(&pixel);
            }
        }

        Self {
            width: new_width,
            height: new_height,
            data: resized_data,
            channels: self.channels,
        }
    }

    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Result<ImageBuffer> {
        if x + width > self.width || y + height > self.height {
            return Err(AvxImageError::ProcessingError(format!(
                "Crop region ({}x{}+{}+{}) exceeds image bounds ({}x{})",
                width, height, x, y, self.width, self.height
            )));
        }

        let mut cropped_data = Vec::with_capacity((width * height) as usize * self.channels);

        for row in y..(y + height) {
            for col in x..(x + width) {
                let pixel = self.get_pixel(col, row);
                cropped_data.extend_from_slice(&pixel);
            }
        }

        Ok(ImageBuffer {
            width,
            height,
            channels: self.channels,
            data: cropped_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_buffer_creation() {
        let img = ImageBuffer::new(100, 100, 3);
        assert_eq!(img.width, 100);
        assert_eq!(img.height, 100);
        assert_eq!(img.channels, 3);
        assert_eq!(img.data.len(), 100 * 100 * 3);
    }

    #[test]
    fn test_grayscale_conversion() {
        let mut img = ImageBuffer::new(10, 10, 3);
        img.set_pixel(5, 5, &[1.0, 0.0, 0.0]);

        let gray = img.to_grayscale();
        assert_eq!(gray.channels, 1);

        let pixel = gray.get_pixel(5, 5);
        assert!(pixel[0] > 0.0);
    }

    #[test]
    fn test_pixel_operations() {
        let mut img = ImageBuffer::new(10, 10, 3);

        let test_pixel = vec![0.5, 0.7, 0.9];
        img.set_pixel(3, 4, &test_pixel);

        let retrieved = img.get_pixel(3, 4);
        assert_eq!(retrieved, test_pixel);
    }

    #[test]
    fn test_resize() {
        let img = ImageBuffer::new(100, 100, 3);
        let resized = img.resize(50, 50);

        assert_eq!(resized.width, 50);
        assert_eq!(resized.height, 50);
        assert_eq!(resized.channels, 3);
    }
}
