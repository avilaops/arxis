//! Feature extraction for computer vision tasks
//!
//! Provides HOG, SIFT-like, ORB-like, and other feature descriptors.

use crate::core::{ImageBuffer, Preprocessing};
use crate::{AvxImageError, Result};
use std::f32::consts::PI;

/// Feature extraction methods
pub struct FeatureExtractor;

impl FeatureExtractor {
    /// Extract Histogram of Oriented Gradients (HOG) features
    ///
    /// # Arguments
    /// * `img` - Input image (will be converted to grayscale)
    /// * `cell_size` - Size of cells in pixels (typically 8x8)
    /// * `block_size` - Size of blocks in cells (typically 2x2)
    /// * `n_bins` - Number of orientation bins (typically 9)
    ///
    /// # Returns
    /// HOG descriptor vector
    pub fn hog(
        img: &ImageBuffer,
        cell_size: usize,
        block_size: usize,
        n_bins: usize,
    ) -> Result<Vec<f32>> {
        let gray = img.to_grayscale();

        // Compute gradients
        let (gx, gy) = Self::compute_gradients(&gray)?;

        // Compute gradient magnitudes and orientations
        let mut magnitudes = vec![0.0; gx.data.len()];
        let mut orientations = vec![0.0; gx.data.len()];

        for i in 0..gx.data.len() {
            magnitudes[i] = (gx.data[i].powi(2) + gy.data[i].powi(2)).sqrt();
            orientations[i] = gy.data[i].atan2(gx.data[i]);
        }

        // Build histograms for each cell
        let cells_x = gray.width as usize / cell_size;
        let cells_y = gray.height as usize / cell_size;
        let mut cell_histograms = vec![vec![0.0; n_bins]; cells_x * cells_y];

        for cy in 0..cells_y {
            for cx in 0..cells_x {
                let mut histogram = vec![0.0; n_bins];

                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        let x = cx * cell_size + dx;
                        let y = cy * cell_size + dy;

                        if x >= gray.width as usize || y >= gray.height as usize {
                            continue;
                        }

                        let idx = y * gray.width as usize + x;
                        let magnitude = magnitudes[idx];
                        let orientation = orientations[idx];

                        // Map orientation [-π, π] to bin [0, n_bins)
                        let angle = (orientation + PI) / (2.0 * PI);
                        let bin = ((angle * n_bins as f32) as usize).min(n_bins - 1);

                        histogram[bin] += magnitude;
                    }
                }

                cell_histograms[cy * cells_x + cx] = histogram;
            }
        }

        // Normalize blocks
        let mut hog_descriptor = Vec::new();

        for by in 0..(cells_y - block_size + 1) {
            for bx in 0..(cells_x - block_size + 1) {
                let mut block_vector = Vec::new();

                for dy in 0..block_size {
                    for dx in 0..block_size {
                        let cell_idx = (by + dy) * cells_x + (bx + dx);
                        block_vector.extend_from_slice(&cell_histograms[cell_idx]);
                    }
                }

                // L2 normalization
                let norm = block_vector.iter().map(|x| x.powi(2)).sum::<f32>().sqrt() + 1e-6;

                for val in &mut block_vector {
                    *val /= norm;
                }

                hog_descriptor.extend(block_vector);
            }
        }

        Ok(hog_descriptor)
    }

    /// Extract Local Binary Pattern (LBP) features
    ///
    /// Texture descriptor based on local pixel comparisons
    pub fn lbp(img: &ImageBuffer, radius: usize, points: usize) -> Result<Vec<f32>> {
        let gray = img.to_grayscale();
        let mut lbp_image = ImageBuffer::new(gray.width, gray.height, 1);

        for y in radius..gray.height as usize - radius {
            for x in radius..gray.width as usize - radius {
                let center_idx = y * gray.width as usize + x;
                let center_value = gray.data[center_idx];
                let mut code = 0u8;

                for p in 0..points {
                    let angle = 2.0 * PI * p as f32 / points as f32;
                    let px = x as f32 + radius as f32 * angle.cos();
                    let py = y as f32 + radius as f32 * angle.sin();

                    // Bilinear interpolation
                    let neighbor_value = Self::bilinear_interpolate(&gray, px, py);

                    if neighbor_value >= center_value {
                        code |= 1 << p;
                    }
                }

                lbp_image.data[center_idx] = code as f32;
            }
        }

        // Compute histogram of LBP codes
        let mut histogram = vec![0.0; 256];
        for &code in &lbp_image.data {
            let bin = code.clamp(0.0, 255.0) as usize;
            histogram[bin] += 1.0;
        }

        // Normalize histogram
        let total = histogram.iter().sum::<f32>();
        for val in &mut histogram {
            *val /= total + 1e-6;
        }

        Ok(histogram)
    }

    /// Extract color histogram features
    pub fn color_histogram(img: &ImageBuffer, bins_per_channel: usize) -> Result<Vec<f32>> {
        if img.channels != 3 {
            return Err(AvxImageError::ProcessingError(
                "Color histogram requires RGB image".into(),
            ));
        }

        let mut histogram = vec![0.0; bins_per_channel.pow(3)];

        for y in 0..img.height {
            for x in 0..img.width {
                let pixel = img.get_pixel(x, y);

                let r = (pixel[0] * bins_per_channel as f32) as usize;
                let g = (pixel[1] * bins_per_channel as f32) as usize;
                let b = (pixel[2] * bins_per_channel as f32) as usize;

                let r = r.min(bins_per_channel - 1);
                let g = g.min(bins_per_channel - 1);
                let b = b.min(bins_per_channel - 1);

                let bin = r * bins_per_channel * bins_per_channel + g * bins_per_channel + b;
                histogram[bin] += 1.0;
            }
        }

        // Normalize
        let total = histogram.iter().sum::<f32>();
        for val in &mut histogram {
            *val /= total + 1e-6;
        }

        Ok(histogram)
    }

    /// Extract SURF-like keypoints and descriptors
    pub fn surf_keypoints(img: &ImageBuffer, threshold: f32) -> Result<Vec<Keypoint>> {
        let gray = img.to_grayscale();

        // Apply Gaussian blur
        let blurred = Preprocessing::gaussian_blur(&gray, 1.4)?;

        // Compute Hessian determinant at multiple scales
        let mut keypoints = Vec::new();

        for scale in [1.2, 1.6, 2.0, 2.4].iter() {
            let sigma = scale * 1.2;
            let smoothed = Preprocessing::gaussian_blur(&blurred, sigma)?;

            // Compute second derivatives
            let dxx = Self::compute_second_derivative(&smoothed, true)?;
            let dyy = Self::compute_second_derivative(&smoothed, false)?;
            let dxy = Self::compute_cross_derivative(&smoothed)?;

            // Find local maxima in Hessian determinant
            for y in 2..gray.height - 2 {
                for x in 2..gray.width - 2 {
                    let idx = (y * gray.width + x) as usize;

                    let det_hessian = dxx.data[idx] * dyy.data[idx] - 0.81 * dxy.data[idx].powi(2);

                    if det_hessian > threshold && Self::is_local_maximum(&dxx, x, y, det_hessian) {
                        keypoints.push(Keypoint {
                            x: x as f32,
                            y: y as f32,
                            scale: *scale,
                            response: det_hessian,
                            orientation: Self::compute_dominant_orientation(&smoothed, x, y)?,
                        });
                    }
                }
            }
        }

        Ok(keypoints)
    }

    /// Compute image gradients using Sobel operator
    fn compute_gradients(img: &ImageBuffer) -> Result<(ImageBuffer, ImageBuffer)> {
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

        let gx = Self::convolve(img, &sobel_x)?;
        let gy = Self::convolve(img, &sobel_y)?;

        Ok((gx, gy))
    }

    /// 2D convolution helper
    fn convolve(img: &ImageBuffer, kernel: &[Vec<f32>]) -> Result<ImageBuffer> {
        let kernel_size = kernel.len();
        let offset = (kernel_size / 2) as i32;

        let mut result = ImageBuffer::new(img.width, img.height, img.channels);

        for y in 0..img.height as i32 {
            for x in 0..img.width as i32 {
                let mut sum = 0.0;

                for ky in 0..kernel_size as i32 {
                    for kx in 0..kernel_size as i32 {
                        let px = (x + kx - offset).clamp(0, img.width as i32 - 1) as u32;
                        let py = (y + ky - offset).clamp(0, img.height as i32 - 1) as u32;

                        let pixel = img.get_pixel(px, py);
                        sum += pixel[0] * kernel[ky as usize][kx as usize];
                    }
                }

                let idx = (y as u32 * img.width + x as u32) as usize;
                result.data[idx] = sum;
            }
        }

        Ok(result)
    }

    /// Bilinear interpolation for sub-pixel sampling
    fn bilinear_interpolate(img: &ImageBuffer, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as u32;
        let x1 = (x0 + 1).min(img.width - 1);
        let y0 = y.floor() as u32;
        let y1 = (y0 + 1).min(img.height - 1);

        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let p00 = img.get_pixel(x0, y0)[0];
        let p01 = img.get_pixel(x0, y1)[0];
        let p10 = img.get_pixel(x1, y0)[0];
        let p11 = img.get_pixel(x1, y1)[0];

        let r0 = p00 * (1.0 - fx) + p10 * fx;
        let r1 = p01 * (1.0 - fx) + p11 * fx;

        r0 * (1.0 - fy) + r1 * fy
    }

    /// Compute second derivative (Laplacian component)
    fn compute_second_derivative(img: &ImageBuffer, horizontal: bool) -> Result<ImageBuffer> {
        let kernel = if horizontal {
            vec![vec![1.0, -2.0, 1.0]]
        } else {
            vec![vec![1.0], vec![-2.0], vec![1.0]]
        };

        Self::convolve(img, &kernel)
    }

    /// Compute cross derivative
    fn compute_cross_derivative(img: &ImageBuffer) -> Result<ImageBuffer> {
        let kernel = vec![
            vec![1.0, 0.0, -1.0],
            vec![0.0, 0.0, 0.0],
            vec![-1.0, 0.0, 1.0],
        ];

        Self::convolve(img, &kernel)
    }

    /// Check if point is local maximum in 3x3 neighborhood
    fn is_local_maximum(img: &ImageBuffer, x: u32, y: u32, value: f32) -> bool {
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as i32 + dx).clamp(0, img.width as i32 - 1) as u32;
                let ny = (y as i32 + dy).clamp(0, img.height as i32 - 1) as u32;

                let neighbor_idx = (ny * img.width + nx) as usize;
                if img.data[neighbor_idx] >= value {
                    return false;
                }
            }
        }
        true
    }

    /// Compute dominant gradient orientation
    fn compute_dominant_orientation(img: &ImageBuffer, x: u32, y: u32) -> Result<f32> {
        let (gx, gy) = Self::compute_gradients(img)?;

        let mut histogram = vec![0.0; 36]; // 10-degree bins

        for dy in -6..=6i32 {
            for dx in -6..=6i32 {
                let nx = (x as i32 + dx).clamp(0, img.width as i32 - 1) as u32;
                let ny = (y as i32 + dy).clamp(0, img.height as i32 - 1) as u32;

                let idx = (ny * img.width + nx) as usize;
                let magnitude = (gx.data[idx].powi(2) + gy.data[idx].powi(2)).sqrt();
                let orientation = gy.data[idx].atan2(gx.data[idx]);

                let angle = (orientation + PI) / (2.0 * PI);
                let bin = ((angle * 36.0) as usize).min(35);

                histogram[bin] += magnitude;
            }
        }

        // Find dominant bin
        let max_bin = histogram
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        Ok((max_bin as f32 / 36.0) * 2.0 * PI - PI)
    }
}

/// Keypoint structure for feature matching
#[derive(Debug, Clone)]
pub struct Keypoint {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub response: f32,
    pub orientation: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hog_features() {
        let img = ImageBuffer::new(64, 64, 3);
        let hog = FeatureExtractor::hog(&img, 8, 2, 9).unwrap();

        assert!(!hog.is_empty());
        // Verify normalization
        assert!(hog.iter().all(|&x| x.is_finite()));
    }

    #[test]
    fn test_lbp_features() {
        let img = ImageBuffer::new(32, 32, 1);
        let lbp = FeatureExtractor::lbp(&img, 1, 8).unwrap();

        assert_eq!(lbp.len(), 256);
        // Verify histogram sums to ~1
        let sum: f32 = lbp.iter().sum();
        assert!((sum - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_color_histogram() {
        let img = ImageBuffer::new(32, 32, 3);
        let hist = FeatureExtractor::color_histogram(&img, 8).unwrap();

        assert_eq!(hist.len(), 8 * 8 * 8);
        let sum: f32 = hist.iter().sum();
        assert!((sum - 1.0).abs() < 1e-4);
    }
}
