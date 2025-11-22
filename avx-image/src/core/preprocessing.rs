//! Image preprocessing operations: filtering, normalization, enhancement

use crate::core::ImageBuffer;
use crate::{AvxImageError, Result};

/// Preprocessing operations for images
pub struct Preprocessing;

impl Preprocessing {
    /// Apply Gaussian blur to reduce noise
    pub fn gaussian_blur(img: &ImageBuffer, sigma: f32) -> Result<ImageBuffer> {
        let kernel_size = (sigma * 6.0).ceil() as usize | 1; // Ensure odd
        let kernel = Self::gaussian_kernel(kernel_size, sigma);
        Self::convolve(img, &kernel)
    }

    /// Apply box blur (fast approximation)
    pub fn box_blur(img: &ImageBuffer, kernel_size: usize) -> Result<ImageBuffer> {
        let kernel = vec![vec![1.0 / (kernel_size * kernel_size) as f32; kernel_size]; kernel_size];
        Self::convolve(img, &kernel)
    }

    /// Sharpen image using unsharp mask
    pub fn sharpen(img: &ImageBuffer, amount: f32) -> Result<ImageBuffer> {
        let kernel = vec![
            vec![0.0, -amount, 0.0],
            vec![-amount, 1.0 + 4.0 * amount, -amount],
            vec![0.0, -amount, 0.0],
        ];
        Self::convolve(img, &kernel)
    }

    /// Edge detection using Sobel operator
    pub fn sobel_edge_detection(img: &ImageBuffer) -> Result<ImageBuffer> {
        let gray = img.to_grayscale();

        // Sobel kernels
        let sobel_x = vec![
            vec![-1.0, 0.0, 1.0],
            vec![-2.0, 0.0, 2.0],
            vec![-1.0, 0.0, 1.0],
        ];

        let sobel_y = vec![
            vec![-1.0, -2.0, -1.0],
            vec![0.0, 0.0, 0.0],
            vec![1.0, 2.0, 1.0],
        ];

        let gx = Self::convolve(&gray, &sobel_x)?;
        let gy = Self::convolve(&gray, &sobel_y)?;

        // Gradient magnitude
        let mut magnitude = ImageBuffer::new(img.width, img.height, 1);
        for i in 0..magnitude.data.len() {
            magnitude.data[i] = (gx.data[i].powi(2) + gy.data[i].powi(2)).sqrt();
        }

        Ok(magnitude)
    }

    /// Normalize image to [0, 1] range
    pub fn normalize(img: &ImageBuffer) -> Result<ImageBuffer> {
        let mut normalized = img.clone();

        let min_val = img.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_val = img.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        if (max_val - min_val).abs() < 1e-6 {
            return Ok(normalized); // Already uniform
        }

        for pixel in &mut normalized.data {
            *pixel = (*pixel - min_val) / (max_val - min_val);
        }

        Ok(normalized)
    }

    /// Histogram equalization for contrast enhancement
    pub fn histogram_equalization(img: &ImageBuffer) -> Result<ImageBuffer> {
        if img.channels != 1 {
            return Err(AvxImageError::ProcessingError(
                "Histogram equalization requires grayscale image".into(),
            ));
        }

        let mut hist = vec![0usize; 256];

        // Build histogram
        for &pixel in &img.data {
            let bin = (pixel * 255.0).clamp(0.0, 255.0) as usize;
            hist[bin] += 1;
        }

        // Cumulative distribution function
        let mut cdf = vec![0usize; 256];
        cdf[0] = hist[0];
        for i in 1..256 {
            cdf[i] = cdf[i - 1] + hist[i];
        }

        let total_pixels = (img.width * img.height) as usize;

        // Normalize CDF
        let mut equalized = img.clone();
        for pixel in &mut equalized.data {
            let bin = (*pixel * 255.0).clamp(0.0, 255.0) as usize;
            *pixel = (cdf[bin] as f32 / total_pixels as f32).clamp(0.0, 1.0);
        }

        Ok(equalized)
    }

    /// Binary thresholding
    pub fn threshold(img: &ImageBuffer, threshold: f32) -> Result<ImageBuffer> {
        let mut binary = img.clone();

        for pixel in &mut binary.data {
            *pixel = if *pixel > threshold { 1.0 } else { 0.0 };
        }

        Ok(binary)
    }

    /// Otsu's automatic threshold selection
    pub fn otsu_threshold(img: &ImageBuffer) -> Result<f32> {
        if img.channels != 1 {
            return Err(AvxImageError::ProcessingError(
                "Otsu threshold requires grayscale image".into(),
            ));
        }

        let mut hist = vec![0usize; 256];

        for &pixel in &img.data {
            let bin = (pixel * 255.0).clamp(0.0, 255.0) as usize;
            hist[bin] += 1;
        }

        let total = (img.width * img.height) as f32;
        let mut sum = 0.0;
        for i in 0..256 {
            sum += i as f32 * hist[i] as f32;
        }

        let mut sum_bg = 0.0;
        let mut weight_bg = 0;
        let mut max_variance = 0.0;
        let mut best_threshold = 0.0;

        for t in 0..256 {
            weight_bg += hist[t];
            if weight_bg == 0 {
                continue;
            }

            let weight_fg = total as usize - weight_bg;
            if weight_fg == 0 {
                break;
            }

            sum_bg += t as f32 * hist[t] as f32;

            let mean_bg = sum_bg / weight_bg as f32;
            let mean_fg = (sum - sum_bg) / weight_fg as f32;

            let variance = weight_bg as f32 * weight_fg as f32 * (mean_bg - mean_fg).powi(2);

            if variance > max_variance {
                max_variance = variance;
                best_threshold = t as f32 / 255.0;
            }
        }

        Ok(best_threshold)
    }

    // Helper: Generate Gaussian kernel
    fn gaussian_kernel(size: usize, sigma: f32) -> Vec<Vec<f32>> {
        let mut kernel = vec![vec![0.0; size]; size];
        let center = size / 2;
        let mut sum = 0.0;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center as f32;
                let dy = y as f32 - center as f32;
                let value = (-(dx * dx + dy * dy) / (2.0 * sigma * sigma)).exp();
                kernel[y][x] = value;
                sum += value;
            }
        }

        // Normalize
        for row in &mut kernel {
            for val in row {
                *val /= sum;
            }
        }

        kernel
    }

    // Helper: 2D convolution
    fn convolve(img: &ImageBuffer, kernel: &[Vec<f32>]) -> Result<ImageBuffer> {
        let kernel_size = kernel.len();
        let offset = (kernel_size / 2) as i32;

        let mut result = ImageBuffer::new(img.width, img.height, img.channels);

        for y in 0..img.height as i32 {
            for x in 0..img.width as i32 {
                for c in 0..img.channels {
                    let mut sum = 0.0;

                    for ky in 0..kernel_size as i32 {
                        for kx in 0..kernel_size as i32 {
                            let px = (x + kx - offset).clamp(0, img.width as i32 - 1) as u32;
                            let py = (y + ky - offset).clamp(0, img.height as i32 - 1) as u32;

                            let pixel = img.get_pixel(px, py);
                            sum += pixel[c] * kernel[ky as usize][kx as usize];
                        }
                    }

                    let idx = ((y as u32 * img.width + x as u32) as usize) * img.channels + c;
                    result.data[idx] = sum.clamp(0.0, 1.0);
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let mut img = ImageBuffer::new(10, 10, 1);
        img.data[0] = 0.2;
        img.data[50] = 0.8;

        let normalized = Preprocessing::normalize(&img).unwrap();

        let min = normalized
            .data
            .iter()
            .cloned()
            .fold(f32::INFINITY, f32::min);
        let max = normalized
            .data
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);

        assert!((min - 0.0).abs() < 1e-5);
        assert!((max - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_threshold() {
        let mut img = ImageBuffer::new(10, 10, 1);
        img.data[0] = 0.3;
        img.data[50] = 0.7;

        let binary = Preprocessing::threshold(&img, 0.5).unwrap();

        assert_eq!(binary.data[0], 0.0);
        assert_eq!(binary.data[50], 1.0);
    }

    #[test]
    fn test_blur() {
        let img = ImageBuffer::new(10, 10, 1);
        let blurred = Preprocessing::gaussian_blur(&img, 1.0).unwrap();

        assert_eq!(blurred.width, img.width);
        assert_eq!(blurred.height, img.height);
    }
}
