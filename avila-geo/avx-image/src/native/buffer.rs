//! Native optimized image buffer
//!
//! High-performance image storage with SIMD support

use crate::native::{color, convolution, math, simd};
use crate::{AvxImageError, Result};

/// Native image buffer (f32 pixels, normalized 0.0-1.0)
#[derive(Clone, Debug)]
pub struct NativeImageBuffer {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub data: Vec<f32>,
}

impl NativeImageBuffer {
    /// Create new buffer
    pub fn new(width: usize, height: usize, channels: usize) -> Self {
        Self {
            width,
            height,
            channels,
            data: vec![0.0; width * height * channels],
        }
    }

    /// Create from raw data
    pub fn from_raw(width: usize, height: usize, channels: usize, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), width * height * channels);
        Self {
            width,
            height,
            channels,
            data,
        }
    }

    /// Convert to grayscale
    pub fn to_grayscale(&self) -> Self {
        if self.channels == 1 {
            return self.clone();
        }

        let mut gray_data = Vec::with_capacity(self.width * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) * self.channels;

                let gray = if self.channels >= 3 {
                    color::rgb_to_gray(self.data[idx], self.data[idx + 1], self.data[idx + 2])
                } else {
                    self.data[idx]
                };

                gray_data.push(gray);
            }
        }

        Self::from_raw(self.width, self.height, 1, gray_data)
    }

    /// Apply Gaussian blur (separable)
    pub fn gaussian_blur(&self, sigma: f32) -> Self {
        assert_eq!(self.channels, 1, "Gaussian blur currently only supports grayscale");

        let kernel_size = ((sigma * 6.0).ceil() as usize) | 1; // Make odd
        let kernel = math::gaussian_kernel(kernel_size, sigma);

        let blurred = convolution::convolve_separable(&self.data, self.width, self.height, &kernel);

        Self::from_raw(self.width, self.height, self.channels, blurred)
    }

    /// Apply median filter
    pub fn median_filter(&self, radius: usize) -> Self {
        assert_eq!(self.channels, 1);

        let filtered = convolution::median_filter(&self.data, self.width, self.height, radius);

        Self::from_raw(self.width, self.height, self.channels, filtered)
    }

    /// Apply bilateral filter
    pub fn bilateral_filter(&self, spatial_sigma: f32, range_sigma: f32, radius: usize) -> Self {
        assert_eq!(self.channels, 1);

        let filtered = convolution::bilateral_filter(
            &self.data,
            self.width,
            self.height,
            spatial_sigma,
            range_sigma,
            radius,
        );

        Self::from_raw(self.width, self.height, self.channels, filtered)
    }

    /// Resize using bilinear interpolation
    pub fn resize(&self, new_width: usize, new_height: usize) -> Self {
        let x_ratio = self.width as f32 / new_width as f32;
        let y_ratio = self.height as f32 / new_height as f32;

        let mut resized_data = Vec::with_capacity(new_width * new_height * self.channels);

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = x as f32 * x_ratio;
                let src_y = y as f32 * y_ratio;

                let x0 = src_x.floor() as usize;
                let y0 = src_y.floor() as usize;
                let x1 = (x0 + 1).min(self.width - 1);
                let y1 = (y0 + 1).min(self.height - 1);

                let tx = src_x - x0 as f32;
                let ty = src_y - y0 as f32;

                for c in 0..self.channels {
                    let v00 = self.data[(y0 * self.width + x0) * self.channels + c];
                    let v10 = self.data[(y0 * self.width + x1) * self.channels + c];
                    let v01 = self.data[(y1 * self.width + x0) * self.channels + c];
                    let v11 = self.data[(y1 * self.width + x1) * self.channels + c];

                    let value = math::bilinear_interp(v00, v10, v01, v11, tx, ty);
                    resized_data.push(value);
                }
            }
        }

        Self::from_raw(new_width, new_height, self.channels, resized_data)
    }

    /// Get pixel value
    pub fn get_pixel(&self, x: usize, y: usize) -> Vec<f32> {
        let idx = (y * self.width + x) * self.channels;
        self.data[idx..idx + self.channels].to_vec()
    }

    /// Set pixel value
    pub fn set_pixel(&mut self, x: usize, y: usize, values: &[f32]) {
        assert_eq!(values.len(), self.channels);
        let idx = (y * self.width + x) * self.channels;
        self.data[idx..idx + self.channels].copy_from_slice(values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buf = NativeImageBuffer::new(100, 100, 3);
        assert_eq!(buf.width, 100);
        assert_eq!(buf.height, 100);
        assert_eq!(buf.channels, 3);
        assert_eq!(buf.data.len(), 30000);
    }

    #[test]
    fn test_to_grayscale() {
        let mut buf = NativeImageBuffer::new(10, 10, 3);
        buf.set_pixel(5, 5, &[1.0, 0.0, 0.0]);

        let gray = buf.to_grayscale();
        assert_eq!(gray.channels, 1);

        let pixel = gray.get_pixel(5, 5);
        assert!(pixel[0] > 0.0);
    }

    #[test]
    fn test_resize() {
        let buf = NativeImageBuffer::new(100, 100, 1);
        let resized = buf.resize(50, 50);

        assert_eq!(resized.width, 50);
        assert_eq!(resized.height, 50);
    }
}
