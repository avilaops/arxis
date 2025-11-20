//! Text detection using EAST-like architecture
//!
//! Detects text regions in images before recognition.

use crate::core::ImageBuffer;
use crate::ocr::BoundingBox;
use crate::Result;

/// Detect text regions in an image
pub fn detect_text_regions(img: &ImageBuffer) -> Result<Vec<BoundingBox>> {
    // TODO: Implement EAST (Efficient and Accurate Scene Text detector)
    // - Feature extraction with ResNet-like backbone
    // - Multi-scale feature fusion
    // - Text region proposal
    // - Non-maximum suppression

    Ok(vec![])
}

/// Apply non-maximum suppression to bounding boxes
pub fn non_max_suppression(boxes: &[BoundingBox], iou_threshold: f32) -> Vec<BoundingBox> {
    let mut result = Vec::new();
    let mut sorted_boxes = boxes.to_vec();

    // Sort by confidence (descending)
    sorted_boxes.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    while !sorted_boxes.is_empty() {
        let best = sorted_boxes.remove(0);
        result.push(best.clone());

        // Remove overlapping boxes
        sorted_boxes.retain(|box_| {
            let iou = compute_iou(&best, box_);
            iou < iou_threshold
        });
    }

    result
}

/// Compute Intersection over Union (IoU) between two boxes
fn compute_iou(a: &BoundingBox, b: &BoundingBox) -> f32 {
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);
    let x2 = (a.x + a.width).min(b.x + b.width);
    let y2 = (a.y + a.height).min(b.y + b.height);

    if x2 <= x1 || y2 <= y1 {
        return 0.0;
    }

    let intersection = (x2 - x1) * (y2 - y1);
    let area_a = a.width * a.height;
    let area_b = b.width * b.height;
    let union = area_a + area_b - intersection;

    intersection as f32 / union as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iou() {
        let box1 = BoundingBox {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            text: String::new(),
            confidence: 0.9,
        };

        let box2 = BoundingBox {
            x: 5,
            y: 5,
            width: 10,
            height: 10,
            text: String::new(),
            confidence: 0.8,
        };

        let iou = compute_iou(&box1, &box2);
        assert!(iou > 0.0 && iou < 1.0);
    }
}
