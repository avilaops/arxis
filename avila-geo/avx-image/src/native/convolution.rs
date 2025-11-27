//! Native convolution operations
//!
//! Pure Rust implementation of 2D convolution with SIMD optimization

use crate::native::simd;

/// 2D Convolution (single channel)
pub fn convolve_2d(
    image: &[f32],
    width: usize,
    height: usize,
    kernel: &[f32],
    kernel_size: usize,
) -> Vec<f32> {
    assert_eq!(image.len(), width * height);
    assert_eq!(kernel.len(), kernel_size * kernel_size);
    assert!(kernel_size % 2 == 1, "Kernel size must be odd");

    let mut output = vec![0.0; width * height];
    let half_kernel = (kernel_size / 2) as i32;

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let iy = y as i32 + ky as i32 - half_kernel;
                    let ix = x as i32 + kx as i32 - half_kernel;

                    // Handle boundaries (replicate edge pixels)
                    let iy = iy.clamp(0, height as i32 - 1) as usize;
                    let ix = ix.clamp(0, width as i32 - 1) as usize;

                    let pixel = image[iy * width + ix];
                    let k = kernel[ky * kernel_size + kx];

                    sum += pixel * k;
                }
            }

            output[y * width + x] = sum;
        }
    }

    output
}

/// Separable convolution (faster for separable kernels like Gaussian)
/// Applies 1D kernel horizontally then vertically
pub fn convolve_separable(
    image: &[f32],
    width: usize,
    height: usize,
    kernel: &[f32],
) -> Vec<f32> {
    let kernel_size = kernel.len();
    assert!(kernel_size % 2 == 1, "Kernel size must be odd");

    // Horizontal pass
    let mut temp = vec![0.0; width * height];
    let half_kernel = (kernel_size / 2) as i32;

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;

            for k in 0..kernel_size {
                let ix = x as i32 + k as i32 - half_kernel;
                let ix = ix.clamp(0, width as i32 - 1) as usize;

                sum += image[y * width + ix] * kernel[k];
            }

            temp[y * width + x] = sum;
        }
    }

    // Vertical pass
    let mut output = vec![0.0; width * height];

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;

            for k in 0..kernel_size {
                let iy = y as i32 + k as i32 - half_kernel;
                let iy = iy.clamp(0, height as i32 - 1) as usize;

                sum += temp[iy * width + x] * kernel[k];
            }

            output[y * width + x] = sum;
        }
    }

    output
}

/// Box filter (fast average filter)
pub fn box_filter(image: &[f32], width: usize, height: usize, radius: usize) -> Vec<f32> {
    let mut output = vec![0.0; width * height];
    let window_size = (2 * radius + 1) * (2 * radius + 1);
    let scale = 1.0 / window_size as f32;

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;

            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let iy = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                    let ix = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;

                    sum += image[iy * width + ix];
                }
            }

            output[y * width + x] = sum * scale;
        }
    }

    output
}

/// Median filter (noise reduction, non-linear)
pub fn median_filter(image: &[f32], width: usize, height: usize, radius: usize) -> Vec<f32> {
    let mut output = vec![0.0; width * height];
    let window_size = (2 * radius + 1) * (2 * radius + 1);
    let mut window = Vec::with_capacity(window_size);

    for y in 0..height {
        for x in 0..width {
            window.clear();

            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let iy = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                    let ix = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;

                    window.push(image[iy * width + ix]);
                }
            }

            // Find median
            window.sort_by(|a, b| a.partial_cmp(b).unwrap());
            output[y * width + x] = window[window.len() / 2];
        }
    }

    output
}

/// Bilateral filter (edge-preserving blur)
pub fn bilateral_filter(
    image: &[f32],
    width: usize,
    height: usize,
    spatial_sigma: f32,
    range_sigma: f32,
    radius: usize,
) -> Vec<f32> {
    let mut output = vec![0.0; width * height];

    for y in 0..height {
        for x in 0..width {
            let center_value = image[y * width + x];
            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let iy = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                    let ix = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;

                    let neighbor_value = image[iy * width + ix];

                    // Spatial weight
                    let spatial_dist_sq = (dx * dx + dy * dy) as f32;
                    let spatial_weight =
                        (-spatial_dist_sq / (2.0 * spatial_sigma * spatial_sigma)).exp();

                    // Range weight
                    let value_diff = center_value - neighbor_value;
                    let range_weight =
                        (-value_diff * value_diff / (2.0 * range_sigma * range_sigma)).exp();

                    let weight = spatial_weight * range_weight;

                    sum += neighbor_value * weight;
                    weight_sum += weight;
                }
            }

            output[y * width + x] = sum / weight_sum;
        }
    }

    output
}

/// Morphological erosion
pub fn erode(image: &[f32], width: usize, height: usize, radius: usize) -> Vec<f32> {
    let mut output = vec![0.0; width * height];

    for y in 0..height {
        for x in 0..width {
            let mut min_val = f32::MAX;

            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let iy = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                    let ix = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;

                    min_val = min_val.min(image[iy * width + ix]);
                }
            }

            output[y * width + x] = min_val;
        }
    }

    output
}

/// Morphological dilation
pub fn dilate(image: &[f32], width: usize, height: usize, radius: usize) -> Vec<f32> {
    let mut output = vec![0.0; width * height];

    for y in 0..height {
        for x in 0..width {
            let mut max_val = f32::MIN;

            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let iy = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                    let ix = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;

                    max_val = max_val.max(image[iy * width + ix]);
                }
            }

            output[y * width + x] = max_val;
        }
    }

    output
}

/// Morphological opening (erosion followed by dilation)
pub fn opening(image: &[f32], width: usize, height: usize, radius: usize) -> Vec<f32> {
    let eroded = erode(image, width, height, radius);
    dilate(&eroded, width, height, radius)
}

/// Morphological closing (dilation followed by erosion)
pub fn closing(image: &[f32], width: usize, height: usize, radius: usize) -> Vec<f32> {
    let dilated = dilate(image, width, height, radius);
    erode(&dilated, width, height, radius)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolve_2d() {
        let image = vec![
            1.0, 2.0, 3.0, //
            4.0, 5.0, 6.0, //
            7.0, 8.0, 9.0, //
        ];

        // Identity kernel
        let kernel = vec![
            0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, //
        ];

        let result = convolve_2d(&image, 3, 3, &kernel, 3);

        // Should be approximately the same (edge effects)
        assert!((result[4] - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_box_filter() {
        let image = vec![1.0; 9];
        let result = box_filter(&image, 3, 3, 1);

        // All pixels should remain 1.0
        for &val in &result {
            assert!((val - 1.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_median_filter() {
        let mut image = vec![1.0; 9];
        image[4] = 100.0; // Outlier

        let result = median_filter(&image, 3, 3, 1);

        // Outlier should be removed
        assert!(result[4] < 10.0);
    }
}
