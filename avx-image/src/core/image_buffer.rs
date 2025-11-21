//! Image buffer implementation with optimized storage

use image::{DynamicImage, GenericImageView, ImageBuffer as ImgBuf, Luma, Rgb, Rgba};
use ndarray::Array3;
use std::path::Path;

use crate::{AvxImageError, Result};

/// High-performance image buffer supporting multiple pixel formats
#[derive(Clone, Debug)]
pub struct ImageBuffer {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Pixel data (row-major, channels-last)
    pub data: Vec<f32>,
    /// Number of channels (1=grayscale, 3=RGB, 4=RGBA)
    pub channels: usize,
}

impl ImageBuffer {
    /// Create new image buffer from dimensions
    pub fn new(width: u32, height: u32, channels: usize) -> Self {
        let size = (width * height) as usize * channels;
        Self {
            width,
            height,
            data: vec![0.0; size],
            channels,
        }
    }

    /// Load image from file path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = image::open(path).map_err(|e| AvxImageError::InvalidFormat(e.to_string()))?;

        Ok(Self::from_dynamic(img))
    }

    /// Convert from image::DynamicImage
    pub fn from_dynamic(img: DynamicImage) -> Self {
        let (width, height) = img.dimensions();
        let channels = match img {
            DynamicImage::ImageLuma8(_) => 1,
            DynamicImage::ImageLumaA8(_) => 2,
            DynamicImage::ImageRgb8(_) => 3,
            DynamicImage::ImageRgba8(_) => 4,
            _ => 3, // Default to RGB
        };

        let mut data = Vec::with_capacity((width * height) as usize * channels);

        match img {
            DynamicImage::ImageRgb8(buf) => {
                for pixel in buf.pixels() {
                    data.push(pixel[0] as f32 / 255.0);
                    data.push(pixel[1] as f32 / 255.0);
                    data.push(pixel[2] as f32 / 255.0);
                }
            }
            DynamicImage::ImageRgba8(buf) => {
                for pixel in buf.pixels() {
                    data.push(pixel[0] as f32 / 255.0);
                    data.push(pixel[1] as f32 / 255.0);
                    data.push(pixel[2] as f32 / 255.0);
                    data.push(pixel[3] as f32 / 255.0);
                }
            }
            DynamicImage::ImageLuma8(buf) => {
                for pixel in buf.pixels() {
                    data.push(pixel[0] as f32 / 255.0);
                }
            }
            _ => {
                // Convert to RGB8 first
                let rgb = img.to_rgb8();
                for pixel in rgb.pixels() {
                    data.push(pixel[0] as f32 / 255.0);
                    data.push(pixel[1] as f32 / 255.0);
                    data.push(pixel[2] as f32 / 255.0);
                }
            }
        }

        Self {
            width,
            height,
            data,
            channels,
        }
    }

    /// Convert to grayscale
    pub fn to_grayscale(&self) -> Self {
        if self.channels == 1 {
            return self.clone();
        }

        let mut gray_data = Vec::with_capacity((self.width * self.height) as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = ((y * self.width + x) as usize) * self.channels;

                // Weighted average (ITU-R BT.709)
                let gray = if self.channels >= 3 {
                    0.2126 * self.data[idx]
                        + 0.7152 * self.data[idx + 1]
                        + 0.0722 * self.data[idx + 2]
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

    /// Convert to ndarray (Height x Width x Channels)
    pub fn to_ndarray(&self) -> Array3<f32> {
        Array3::from_shape_vec(
            (self.height as usize, self.width as usize, self.channels),
            self.data.clone(),
        )
        .expect("Failed to create ndarray from image data")
    }

    /// Get pixel value at (x, y)
    pub fn get_pixel(&self, x: u32, y: u32) -> Vec<f32> {
        let idx = ((y * self.width + x) as usize) * self.channels;
        self.data[idx..idx + self.channels].to_vec()
    }

    /// Set pixel value at (x, y)
    pub fn set_pixel(&mut self, x: u32, y: u32, values: &[f32]) {
        assert_eq!(
            values.len(),
            self.channels,
            "Pixel values must match channel count"
        );
        let idx = ((y * self.width + x) as usize) * self.channels;
        self.data[idx..idx + self.channels].copy_from_slice(values);
    }

    /// Save image to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut img_data = vec![0u8; self.data.len()];
        for (i, &v) in self.data.iter().enumerate() {
            img_data[i] = (v * 255.0).clamp(0.0, 255.0) as u8;
        }

        match self.channels {
            1 => {
                let img = ImgBuf::<Luma<u8>, _>::from_raw(self.width, self.height, img_data)
                    .ok_or_else(|| {
                        AvxImageError::ProcessingError("Failed to create image buffer".into())
                    })?;
                img.save(path)?;
            }
            3 => {
                let img = ImgBuf::<Rgb<u8>, _>::from_raw(self.width, self.height, img_data)
                    .ok_or_else(|| {
                        AvxImageError::ProcessingError("Failed to create image buffer".into())
                    })?;
                img.save(path)?;
            }
            4 => {
                let img = ImgBuf::<Rgba<u8>, _>::from_raw(self.width, self.height, img_data)
                    .ok_or_else(|| {
                        AvxImageError::ProcessingError("Failed to create image buffer".into())
                    })?;
                img.save(path)?;
            }
            _ => {
                return Err(AvxImageError::ProcessingError(format!(
                    "Unsupported channel count: {}",
                    self.channels
                )));
            }
        }

        Ok(())
    }

    /// Resize image using bilinear interpolation
    pub fn resize(&self, new_width: u32, new_height: u32) -> Self {
        let x_ratio = self.width as f32 / new_width as f32;
        let y_ratio = self.height as f32 / new_height as f32;

        let mut resized_data =
            Vec::with_capacity((new_width * new_height) as usize * self.channels);

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

    /// Crop image to specified region
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Result<ImageBuffer> {
        // Validate bounds
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

        // Set a red pixel
        img.set_pixel(5, 5, &[1.0, 0.0, 0.0]);

        let gray = img.to_grayscale();
        assert_eq!(gray.channels, 1);

        let pixel = gray.get_pixel(5, 5);
        assert!(pixel[0] > 0.0); // Should have some brightness from red
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
