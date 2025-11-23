/// Object Detection - Haar Cascades & Viola-Jones
///
/// Classical object detection using integral images and AdaBoost cascades.
/// Based on Viola-Jones (2001) "Rapid Object Detection using a Boosted Cascade".

use crate::native::buffer::NativeImageBuffer;
use crate::{AvxImageError, Result};

/// Bounding box for detected object
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Detection {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub confidence: f32,
}

impl Detection {
    pub fn new(x: usize, y: usize, width: usize, height: usize, confidence: f32) -> Self {
        Self { x, y, width, height, confidence }
    }

    /// Calculate Intersection over Union (IoU) with another detection
    pub fn iou(&self, other: &Detection) -> f32 {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        if x2 <= x1 || y2 <= y1 {
            return 0.0; // No overlap
        }

        let intersection = ((x2 - x1) * (y2 - y1)) as f32;
        let area1 = (self.width * self.height) as f32;
        let area2 = (other.width * other.height) as f32;
        let union = area1 + area2 - intersection;

        intersection / union
    }
}

/// Integral image for fast rectangle sum computation
#[derive(Debug, Clone)]
pub struct IntegralImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
    pub squared_data: Vec<f32>, // For variance computation
}

impl IntegralImage {
    /// Compute integral image from grayscale buffer
    pub fn from_image(img: &NativeImageBuffer) -> Result<Self> {
        if img.channels != 1 {
            return Err(AvxImageError::ProcessingError(
                "Integral image requires grayscale input".into(),
            ));
        }

        let width = img.width;
        let height = img.height;
        let mut data = vec![0.0; width * height];
        let mut squared_data = vec![0.0; width * height];

        // First row
        for x in 0..width {
            let val = img.data[x];
            data[x] = if x > 0 { data[x - 1] + val } else { val };
            squared_data[x] = if x > 0 {
                squared_data[x - 1] + val * val
            } else {
                val * val
            };
        }

        // Remaining rows
        for y in 1..height {
            let mut row_sum = 0.0;
            let mut row_sq_sum = 0.0;

            for x in 0..width {
                let idx = y * width + x;
                let val = img.data[idx];

                row_sum += val;
                row_sq_sum += val * val;

                data[idx] = data[idx - width] + row_sum;
                squared_data[idx] = squared_data[idx - width] + row_sq_sum;
            }
        }

        Ok(Self {
            width,
            height,
            data,
            squared_data,
        })
    }

    /// Get sum of rectangle using 4 lookups (O(1))
    /// rect: (x, y, width, height)
    pub fn rect_sum(&self, x: usize, y: usize, w: usize, h: usize) -> f32 {
        if x + w > self.width || y + h > self.height {
            return 0.0;
        }

        let x2 = x + w;
        let y2 = y + h;

        let a = if x > 0 && y > 0 {
            self.data[(y - 1) * self.width + (x - 1)]
        } else {
            0.0
        };
        let b = if y > 0 {
            self.data[(y - 1) * self.width + x2 - 1]
        } else {
            0.0
        };
        let c = if x > 0 {
            self.data[(y2 - 1) * self.width + (x - 1)]
        } else {
            0.0
        };
        let d = self.data[(y2 - 1) * self.width + x2 - 1];

        d - b - c + a
    }

    /// Get variance of rectangle (for normalization)
    pub fn rect_variance(&self, x: usize, y: usize, w: usize, h: usize) -> f32 {
        if x + w > self.width || y + h > self.height {
            return 0.0;
        }

        let sum = self.rect_sum(x, y, w, h);
        let sq_sum = self.rect_squared_sum(x, y, w, h);
        let area = (w * h) as f32;

        let mean = sum / area;
        let mean_sq = sq_sum / area;

        (mean_sq - mean * mean).max(0.0)
    }

    fn rect_squared_sum(&self, x: usize, y: usize, w: usize, h: usize) -> f32 {
        let x2 = x + w;
        let y2 = y + h;

        let a = if x > 0 && y > 0 {
            self.squared_data[(y - 1) * self.width + (x - 1)]
        } else {
            0.0
        };
        let b = if y > 0 {
            self.squared_data[(y - 1) * self.width + x2 - 1]
        } else {
            0.0
        };
        let c = if x > 0 {
            self.squared_data[(y2 - 1) * self.width + (x - 1)]
        } else {
            0.0
        };
        let d = self.squared_data[(y2 - 1) * self.width + x2 - 1];

        d - b - c + a
    }
}

/// Haar-like feature types
#[derive(Debug, Clone, Copy)]
pub enum HaarFeatureType {
    TwoRectHorizontal, // ▄▀ (white above black)
    TwoRectVertical,   // ▌▐ (white left, black right)
    ThreeRectHorizontal, // ▄▀▄ (white-black-white)
    ThreeRectVertical,   // ▌▐▌ (white-black-white)
    FourRect,          // ▚▞ (diagonal pattern)
}

/// Haar-like feature
#[derive(Debug, Clone)]
pub struct HaarFeature {
    pub feature_type: HaarFeatureType,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub threshold: f32,
    pub polarity: i8, // 1 or -1
    pub weight: f32,
}

impl HaarFeature {
    /// Compute feature response at given position
    pub fn compute(&self, integral: &IntegralImage, x: usize, y: usize, scale: f32) -> f32 {
        let w = (self.width as f32 * scale) as usize;
        let h = (self.height as f32 * scale) as usize;

        if x + w > integral.width || y + h > integral.height {
            return 0.0;
        }

        match self.feature_type {
            HaarFeatureType::TwoRectHorizontal => {
                let h_half = h / 2;
                let top = integral.rect_sum(x, y, w, h_half);
                let bottom = integral.rect_sum(x, y + h_half, w, h - h_half);
                top - bottom
            }
            HaarFeatureType::TwoRectVertical => {
                let w_half = w / 2;
                let left = integral.rect_sum(x, y, w_half, h);
                let right = integral.rect_sum(x + w_half, y, w - w_half, h);
                left - right
            }
            HaarFeatureType::ThreeRectHorizontal => {
                let h_third = h / 3;
                let top = integral.rect_sum(x, y, w, h_third);
                let middle = integral.rect_sum(x, y + h_third, w, h_third);
                let bottom = integral.rect_sum(x, y + 2 * h_third, w, h - 2 * h_third);
                top - middle + bottom
            }
            HaarFeatureType::ThreeRectVertical => {
                let w_third = w / 3;
                let left = integral.rect_sum(x, y, w_third, h);
                let middle = integral.rect_sum(x + w_third, y, w_third, h);
                let right = integral.rect_sum(x + 2 * w_third, y, w - 2 * w_third, h);
                left - middle + right
            }
            HaarFeatureType::FourRect => {
                let w_half = w / 2;
                let h_half = h / 2;
                let tl = integral.rect_sum(x, y, w_half, h_half);
                let tr = integral.rect_sum(x + w_half, y, w - w_half, h_half);
                let bl = integral.rect_sum(x, y + h_half, w_half, h - h_half);
                let br = integral.rect_sum(x + w_half, y + h_half, w - w_half, h - h_half);
                (tl + br) - (tr + bl)
            }
        }
    }

    /// Classify feature (above/below threshold)
    pub fn classify(&self, value: f32) -> bool {
        if self.polarity == 1 {
            value > self.threshold
        } else {
            value < self.threshold
        }
    }
}

/// Cascade stage (weak classifiers)
#[derive(Debug, Clone)]
pub struct CascadeStage {
    pub features: Vec<HaarFeature>,
    pub stage_threshold: f32,
}

impl CascadeStage {
    /// Evaluate stage on window
    pub fn evaluate(&self, integral: &IntegralImage, x: usize, y: usize, scale: f32) -> (bool, f32) {
        let mut score = 0.0;

        for feature in &self.features {
            let value = feature.compute(integral, x, y, scale);
            if feature.classify(value) {
                score += feature.weight;
            }
        }

        (score >= self.stage_threshold, score)
    }
}

/// Haar Cascade classifier
#[derive(Debug, Clone)]
pub struct HaarCascade {
    pub stages: Vec<CascadeStage>,
    pub base_width: usize,
    pub base_height: usize,
}

impl HaarCascade {
    /// Detect objects at single scale
    pub fn detect_single_scale(
        &self,
        integral: &IntegralImage,
        scale: f32,
        step: usize,
    ) -> Vec<Detection> {
        let mut detections = Vec::new();
        let window_w = (self.base_width as f32 * scale) as usize;
        let window_h = (self.base_height as f32 * scale) as usize;

        for y in (0..integral.height.saturating_sub(window_h)).step_by(step) {
            for x in (0..integral.width.saturating_sub(window_w)).step_by(step) {
                let mut passed = true;
                let mut total_score = 0.0;

                // Evaluate cascade stages
                for stage in &self.stages {
                    let (stage_passed, stage_score) = stage.evaluate(integral, x, y, scale);
                    total_score += stage_score;

                    if !stage_passed {
                        passed = false;
                        break; // Early rejection
                    }
                }

                if passed {
                    let confidence = total_score / self.stages.len() as f32;
                    detections.push(Detection::new(x, y, window_w, window_h, confidence));
                }
            }
        }

        detections
    }

    /// Multi-scale detection
    pub fn detect_multi_scale(
        &self,
        img: &NativeImageBuffer,
        min_scale: f32,
        max_scale: f32,
        scale_factor: f32,
        step_size: usize,
    ) -> Result<Vec<Detection>> {
        let integral = IntegralImage::from_image(img)?;
        let mut all_detections = Vec::new();

        let mut scale = min_scale;
        while scale <= max_scale {
            let detections = self.detect_single_scale(&integral, scale, step_size);
            all_detections.extend(detections);
            scale *= scale_factor;
        }

        Ok(all_detections)
    }

    /// Create simple face detector (demonstration)
    pub fn simple_face_detector() -> Self {
        // Simplified face detector with 3 stages
        // Real cascades have 20+ stages and thousands of features

        let mut stages = Vec::new();

        // Stage 1: Basic face structure (2 features)
        let stage1 = CascadeStage {
            features: vec![
                // Eyes region darker than forehead
                HaarFeature {
                    feature_type: HaarFeatureType::TwoRectHorizontal,
                    x: 0,
                    y: 0,
                    width: 24,
                    height: 24,
                    threshold: -0.5,
                    polarity: -1,
                    weight: 0.5,
                },
                // Nose bridge vertical feature
                HaarFeature {
                    feature_type: HaarFeatureType::TwoRectVertical,
                    x: 10,
                    y: 8,
                    width: 4,
                    height: 10,
                    threshold: 0.2,
                    polarity: 1,
                    weight: 0.3,
                },
            ],
            stage_threshold: 0.4,
        };

        // Stage 2: Eye and mouth features
        let stage2 = CascadeStage {
            features: vec![
                // Left eye
                HaarFeature {
                    feature_type: HaarFeatureType::ThreeRectHorizontal,
                    x: 4,
                    y: 8,
                    width: 6,
                    height: 6,
                    threshold: -0.3,
                    polarity: -1,
                    weight: 0.4,
                },
                // Right eye
                HaarFeature {
                    feature_type: HaarFeatureType::ThreeRectHorizontal,
                    x: 14,
                    y: 8,
                    width: 6,
                    height: 6,
                    threshold: -0.3,
                    polarity: -1,
                    weight: 0.4,
                },
            ],
            stage_threshold: 0.5,
        };

        // Stage 3: Detailed features
        let stage3 = CascadeStage {
            features: vec![
                // Mouth region
                HaarFeature {
                    feature_type: HaarFeatureType::TwoRectHorizontal,
                    x: 6,
                    y: 16,
                    width: 12,
                    height: 6,
                    threshold: 0.1,
                    polarity: 1,
                    weight: 0.3,
                },
            ],
            stage_threshold: 0.2,
        };

        stages.push(stage1);
        stages.push(stage2);
        stages.push(stage3);

        Self {
            stages,
            base_width: 24,
            base_height: 24,
        }
    }
}

/// Non-Maximum Suppression (NMS) to remove overlapping detections
pub fn non_maximum_suppression(detections: Vec<Detection>, iou_threshold: f32) -> Vec<Detection> {
    if detections.is_empty() {
        return Vec::new();
    }

    // Sort by confidence (descending)
    let mut sorted = detections;
    sorted.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    let mut keep = Vec::new();

    while !sorted.is_empty() {
        let best = sorted.remove(0);
        keep.push(best);

        sorted.retain(|det| det.iou(&best) < iou_threshold);
    }

    keep
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_iou() {
        let det1 = Detection::new(10, 10, 20, 20, 1.0);
        let det2 = Detection::new(15, 15, 20, 20, 0.9);

        let iou = det1.iou(&det2);
        assert!(iou > 0.0 && iou < 1.0);

        // No overlap
        let det3 = Detection::new(100, 100, 20, 20, 0.8);
        assert_eq!(det1.iou(&det3), 0.0);

        // Complete overlap
        let det4 = Detection::new(10, 10, 20, 20, 0.95);
        assert!((det1.iou(&det4) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_integral_image() {
        let mut img = NativeImageBuffer::new(4, 4, 1);
        // Simple 4x4 image with ones
        for i in 0..16 {
            img.data[i] = 1.0;
        }

        let integral = IntegralImage::from_image(&img).unwrap();

        // Sum of 2x2 region should be 4
        assert_eq!(integral.rect_sum(0, 0, 2, 2), 4.0);

        // Sum of full image should be 16
        assert_eq!(integral.rect_sum(0, 0, 4, 4), 16.0);
    }

    #[test]
    fn test_integral_image_variable() {
        let mut img = NativeImageBuffer::new(3, 3, 1);
        // Pattern: [1, 2, 3]
        //          [4, 5, 6]
        //          [7, 8, 9]
        for i in 0..9 {
            img.data[i] = (i + 1) as f32;
        }

        let integral = IntegralImage::from_image(&img).unwrap();

        // Top-left 2x2 sum = 1+2+4+5 = 12
        assert_eq!(integral.rect_sum(0, 0, 2, 2), 12.0);

        // Full image sum = 1+2+3+4+5+6+7+8+9 = 45
        assert_eq!(integral.rect_sum(0, 0, 3, 3), 45.0);
    }

    #[test]
    fn test_haar_feature_two_rect_horizontal() {
        let mut img = NativeImageBuffer::new(4, 4, 1);
        // Top half white (1.0), bottom half black (0.0)
        for y in 0..2 {
            for x in 0..4 {
                img.data[y * 4 + x] = 1.0;
            }
        }

        let integral = IntegralImage::from_image(&img).unwrap();
        let feature = HaarFeature {
            feature_type: HaarFeatureType::TwoRectHorizontal,
            x: 0,
            y: 0,
            width: 4,
            height: 4,
            threshold: 0.0,
            polarity: 1,
            weight: 1.0,
        };

        let response = feature.compute(&integral, 0, 0, 1.0);
        assert!(response > 0.0); // Top brighter than bottom
    }

    #[test]
    fn test_nms() {
        let detections = vec![
            Detection::new(10, 10, 20, 20, 0.9),
            Detection::new(12, 12, 20, 20, 0.85), // Overlaps with first
            Detection::new(100, 100, 20, 20, 0.8), // Far away
        ];

        let result = non_maximum_suppression(detections, 0.3);
        assert_eq!(result.len(), 2); // Should keep best + non-overlapping
        assert_eq!(result[0].confidence, 0.9); // Best confidence first
    }

    #[test]
    fn test_simple_face_detector_creation() {
        let detector = HaarCascade::simple_face_detector();
        assert_eq!(detector.base_width, 24);
        assert_eq!(detector.base_height, 24);
        assert_eq!(detector.stages.len(), 3);
    }

    #[test]
    fn test_cascade_stage_evaluation() {
        let mut img = NativeImageBuffer::new(30, 30, 1);
        // Create simple pattern
        for i in 0..img.data.len() {
            img.data[i] = 0.5;
        }

        let integral = IntegralImage::from_image(&img).unwrap();
        let detector = HaarCascade::simple_face_detector();

        let (passed, score) = detector.stages[0].evaluate(&integral, 0, 0, 1.0);
        assert!(score >= 0.0); // Score should be computed
    }
}
