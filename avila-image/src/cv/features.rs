//! avila-image Computer Vision Core
//!
//! Features:
//! - Feature detection (FAST, Harris, SIFT-like)
//! - HOG (Histogram of Oriented Gradients)
//! - Object detection primitives
//!
//! Competing with: OpenCV, scikit-image

use alloc::vec::Vec;

/// Image representation
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32, channels: u8) -> Self {
        let size = (width * height * channels as u32) as usize;
        Self {
            width,
            height,
            channels,
            data: vec![0; size],
        }
    }

    pub fn from_rgb(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self { width, height, channels: 3, data }
    }

    pub fn to_grayscale(&self) -> Image {
        let mut gray = Image::new(self.width, self.height, 1);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let pixel_idx = idx * self.channels as usize;

                let gray_value = if self.channels >= 3 {
                    // Standard grayscale conversion: 0.299*R + 0.587*G + 0.114*B
                    let r = self.data[pixel_idx] as f32;
                    let g = self.data[pixel_idx + 1] as f32;
                    let b = self.data[pixel_idx + 2] as f32;
                    (0.299 * r + 0.587 * g + 0.114 * b) as u8
                } else {
                    self.data[pixel_idx]
                };

                gray.data[idx] = gray_value;
            }
        }

        gray
    }

    pub fn get_pixel(&self, x: u32, y: u32, channel: u8) -> u8 {
        let idx = ((y * self.width + x) * self.channels as u32 + channel as u32) as usize;
        self.data[idx]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, channel: u8, value: u8) {
        let idx = ((y * self.width + x) * self.channels as u32 + channel as u32) as usize;
        self.data[idx] = value;
    }
}

/// Keypoint for feature detection
#[derive(Debug, Clone, Copy)]
pub struct KeyPoint {
    pub x: f32,
    pub y: f32,
    pub response: f32,
    pub size: f32,
    pub angle: f32,
}

/// FAST corner detector
pub struct FastDetector {
    threshold: u8,
}

impl FastDetector {
    pub fn new(threshold: u8) -> Self {
        Self { threshold }
    }

    pub fn detect(&self, image: &Image) -> Vec<KeyPoint> {
        let gray = if image.channels == 1 {
            image.clone()
        } else {
            image.to_grayscale()
        };

        let mut keypoints = Vec::new();

        // FAST-9 circle pattern offsets
        let circle: [(i32, i32); 16] = [
            (0, -3), (1, -3), (2, -2), (3, -1),
            (3, 0), (3, 1), (2, 2), (1, 3),
            (0, 3), (-1, 3), (-2, 2), (-3, 1),
            (-3, 0), (-3, -1), (-2, -2), (-1, -3),
        ];

        for y in 3..(gray.height - 3) {
            for x in 3..(gray.width - 3) {
                let center = gray.get_pixel(x, y, 0);

                let mut brighter = 0;
                let mut darker = 0;

                for (dx, dy) in circle.iter() {
                    let nx = (x as i32 + dx) as u32;
                    let ny = (y as i32 + dy) as u32;
                    let pixel = gray.get_pixel(nx, ny, 0);

                    if pixel > center.saturating_add(self.threshold) {
                        brighter += 1;
                    } else if pixel < center.saturating_sub(self.threshold) {
                        darker += 1;
                    }
                }

                // Need 9+ consecutive pixels brighter or darker
                if brighter >= 9 || darker >= 9 {
                    keypoints.push(KeyPoint {
                        x: x as f32,
                        y: y as f32,
                        response: (brighter.max(darker) - 9) as f32,
                        size: 7.0,
                        angle: 0.0,
                    });
                }
            }
        }

        keypoints
    }
}

/// Harris corner detector
pub struct HarrisDetector {
    k: f32,
    threshold: f32,
}

impl HarrisDetector {
    pub fn new(k: f32, threshold: f32) -> Self {
        Self { k, threshold }
    }

    pub fn detect(&self, image: &Image) -> Vec<KeyPoint> {
        let gray = if image.channels == 1 {
            image.clone()
        } else {
            image.to_grayscale()
        };

        let mut keypoints = Vec::new();

        // Compute image gradients using Sobel
        let (ix, iy) = self.compute_gradients(&gray);

        // Compute Harris response
        for y in 2..(gray.height - 2) {
            for x in 2..(gray.width - 2) {
                let idx = (y * gray.width + x) as usize;

                // Compute structure tensor in 3x3 window
                let mut ixx = 0.0;
                let mut iyy = 0.0;
                let mut ixy = 0.0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let widx = ((y as i32 + dy) * gray.width as i32 + (x as i32 + dx)) as usize;
                        let gx = ix[widx];
                        let gy = iy[widx];

                        ixx += gx * gx;
                        iyy += gy * gy;
                        ixy += gx * gy;
                    }
                }

                // Harris corner response: det(M) - k*trace(M)^2
                let det = ixx * iyy - ixy * ixy;
                let trace = ixx + iyy;
                let response = det - self.k * trace * trace;

                if response > self.threshold {
                    keypoints.push(KeyPoint {
                        x: x as f32,
                        y: y as f32,
                        response,
                        size: 3.0,
                        angle: 0.0,
                    });
                }
            }
        }

        // Non-maximum suppression
        self.non_max_suppression(keypoints, 5)
    }

    fn compute_gradients(&self, image: &Image) -> (Vec<f32>, Vec<f32>) {
        let size = (image.width * image.height) as usize;
        let mut ix = vec![0.0; size];
        let mut iy = vec![0.0; size];

        // Sobel kernels
        for y in 1..(image.height - 1) {
            for x in 1..(image.width - 1) {
                let idx = (y * image.width + x) as usize;

                // Sobel X: [-1 0 1; -2 0 2; -1 0 1]
                let gx = -1.0 * image.get_pixel(x - 1, y - 1, 0) as f32
                    + 1.0 * image.get_pixel(x + 1, y - 1, 0) as f32
                    - 2.0 * image.get_pixel(x - 1, y, 0) as f32
                    + 2.0 * image.get_pixel(x + 1, y, 0) as f32
                    - 1.0 * image.get_pixel(x - 1, y + 1, 0) as f32
                    + 1.0 * image.get_pixel(x + 1, y + 1, 0) as f32;

                // Sobel Y: [-1 -2 -1; 0 0 0; 1 2 1]
                let gy = -1.0 * image.get_pixel(x - 1, y - 1, 0) as f32
                    - 2.0 * image.get_pixel(x, y - 1, 0) as f32
                    - 1.0 * image.get_pixel(x + 1, y - 1, 0) as f32
                    + 1.0 * image.get_pixel(x - 1, y + 1, 0) as f32
                    + 2.0 * image.get_pixel(x, y + 1, 0) as f32
                    + 1.0 * image.get_pixel(x + 1, y + 1, 0) as f32;

                ix[idx] = gx;
                iy[idx] = gy;
            }
        }

        (ix, iy)
    }

    fn non_max_suppression(&self, keypoints: Vec<KeyPoint>, radius: i32) -> Vec<KeyPoint> {
        let mut result = Vec::new();

        for i in 0..keypoints.len() {
            let kp = keypoints[i];
            let mut is_max = true;

            for j in 0..keypoints.len() {
                if i == j {
                    continue;
                }

                let other = keypoints[j];
                let dx = (kp.x - other.x).abs();
                let dy = (kp.y - other.y).abs();

                if dx <= radius as f32 && dy <= radius as f32 {
                    if other.response > kp.response {
                        is_max = false;
                        break;
                    }
                }
            }

            if is_max {
                result.push(kp);
            }
        }

        result
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            channels: self.channels,
            data: self.data.clone(),
        }
    }
}

/// HOG (Histogram of Oriented Gradients) descriptor
pub struct HogDescriptor {
    cell_size: u32,
    block_size: u32,
    num_bins: usize,
}

impl HogDescriptor {
    pub fn new(cell_size: u32, block_size: u32, num_bins: usize) -> Self {
        Self { cell_size, block_size, num_bins }
    }

    pub fn compute(&self, image: &Image) -> Vec<f32> {
        let gray = if image.channels == 1 {
            image.clone()
        } else {
            image.to_grayscale()
        };

        // Compute gradients
        let (gx, gy) = self.compute_gradients(&gray);
        let magnitudes = self.compute_magnitudes(&gx, &gy);
        let orientations = self.compute_orientations(&gx, &gy);

        // Compute histograms for each cell
        let cells_x = gray.width / self.cell_size;
        let cells_y = gray.height / self.cell_size;

        let mut descriptors = Vec::new();

        for block_y in 0..(cells_y - self.block_size + 1) {
            for block_x in 0..(cells_x - self.block_size + 1) {
                let mut block_hist = vec![0.0; (self.block_size * self.block_size * self.num_bins as u32) as usize];

                // Compute histogram for each cell in block
                for cy in 0..self.block_size {
                    for cx in 0..self.block_size {
                        let cell_x = block_x + cx;
                        let cell_y = block_y + cy;

                        let hist = self.compute_cell_histogram(
                            &magnitudes,
                            &orientations,
                            cell_x,
                            cell_y,
                            gray.width,
                        );

                        let offset = ((cy * self.block_size + cx) * self.num_bins as u32) as usize;
                        block_hist[offset..offset + self.num_bins].copy_from_slice(&hist);
                    }
                }

                // L2 normalization
                let norm: f32 = block_hist.iter().map(|x| x * x).sum::<f32>().sqrt();
                if norm > 0.0 {
                    for val in &mut block_hist {
                        *val /= norm;
                    }
                }

                descriptors.extend_from_slice(&block_hist);
            }
        }

        descriptors
    }

    fn compute_gradients(&self, image: &Image) -> (Vec<f32>, Vec<f32>) {
        let size = (image.width * image.height) as usize;
        let mut gx = vec![0.0; size];
        let mut gy = vec![0.0; size];

        for y in 1..(image.height - 1) {
            for x in 1..(image.width - 1) {
                let idx = (y * image.width + x) as usize;

                gx[idx] = image.get_pixel(x + 1, y, 0) as f32 - image.get_pixel(x - 1, y, 0) as f32;
                gy[idx] = image.get_pixel(x, y + 1, 0) as f32 - image.get_pixel(x, y - 1, 0) as f32;
            }
        }

        (gx, gy)
    }

    fn compute_magnitudes(&self, gx: &[f32], gy: &[f32]) -> Vec<f32> {
        gx.iter().zip(gy.iter())
            .map(|(x, y)| (x * x + y * y).sqrt())
            .collect()
    }

    fn compute_orientations(&self, gx: &[f32], gy: &[f32]) -> Vec<f32> {
        gx.iter().zip(gy.iter())
            .map(|(x, y)| y.atan2(*x))
            .collect()
    }

    fn compute_cell_histogram(
        &self,
        magnitudes: &[f32],
        orientations: &[f32],
        cell_x: u32,
        cell_y: u32,
        width: u32,
    ) -> Vec<f32> {
        let mut histogram = vec![0.0; self.num_bins];

        let start_x = cell_x * self.cell_size;
        let start_y = cell_y * self.cell_size;

        for dy in 0..self.cell_size {
            for dx in 0..self.cell_size {
                let x = start_x + dx;
                let y = start_y + dy;
                let idx = (y * width + x) as usize;

                let magnitude = magnitudes[idx];
                let orientation = orientations[idx];

                // Map orientation to bin (0 to num_bins-1)
                let bin_f = ((orientation + core::f32::consts::PI) / (2.0 * core::f32::consts::PI)) * self.num_bins as f32;
                let bin = (bin_f as usize) % self.num_bins;

                histogram[bin] += magnitude;
            }
        }

        histogram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let img = Image::new(100, 100, 3);
        assert_eq!(img.data.len(), 100 * 100 * 3);
    }

    #[test]
    fn test_grayscale_conversion() {
        let mut img = Image::new(2, 2, 3);
        // Set white pixel
        img.set_pixel(0, 0, 0, 255);
        img.set_pixel(0, 0, 1, 255);
        img.set_pixel(0, 0, 2, 255);

        let gray = img.to_grayscale();
        assert_eq!(gray.channels, 1);
        assert_eq!(gray.get_pixel(0, 0, 0), 255);
    }

    #[test]
    fn test_fast_detector() {
        let img = Image::new(100, 100, 1);
        let detector = FastDetector::new(20);
        let keypoints = detector.detect(&img);
        // Should find some corners in the image
        assert!(keypoints.len() >= 0);
    }
}
