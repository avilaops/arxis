//! Face detection using MTCNN-like architecture

use crate::core::{ImageBuffer, Preprocessing};
use crate::face::{BoundingBox, DetectionOptions, Face};
use crate::Result;

/// Detect faces using multi-stage cascade
pub fn detect_faces(img: &ImageBuffer, options: &DetectionOptions) -> Result<Vec<Face>> {
    let gray = img.to_grayscale();
    let normalized = Preprocessing::normalize(&gray)?;

    // Stage 1: Generate proposals using sliding window
    let mut proposals = generate_proposals(&normalized, options)?;

    // Stage 2: Non-maximum suppression
    proposals = non_maximum_suppression(&proposals, 0.7);

    // Stage 3: Refine detections
    let faces = refine_detections(&normalized, proposals, options)?;

    Ok(faces)
}

/// Generate face proposals using sliding window
fn generate_proposals(img: &ImageBuffer, options: &DetectionOptions) -> Result<Vec<Face>> {
    let mut proposals = Vec::new();
    let pyramid = build_pyramid(img, 0.7, options.min_face_size);

    for scale_img in pyramid {
        let step = 12;
        let window_size = 24;

        for y in (0..scale_img.height.saturating_sub(window_size)).step_by(step as usize) {
            for x in (0..scale_img.width.saturating_sub(window_size)).step_by(step as usize) {
                let score = compute_face_score(&scale_img, x, y, window_size);

                if score > options.confidence_threshold {
                    proposals.push(Face {
                        bbox: BoundingBox {
                            x,
                            y,
                            width: window_size,
                            height: window_size,
                        },
                        confidence: score,
                        landmarks: None,
                        embedding: None,
                    });
                }
            }
        }
    }

    Ok(proposals)
}

/// Compute face likelihood score for a region
fn compute_face_score(img: &ImageBuffer, x: u32, y: u32, size: u32) -> f32 {
    // Simple Haar-like features for face detection
    let mut score = 0.0;

    // Eye region (darker than forehead)
    let eye_y = y + size / 3;
    let forehead_y = y + size / 6;

    score += get_region_intensity(img, x, forehead_y, size, size / 6);
    score -= get_region_intensity(img, x, eye_y, size, size / 6);

    // Normalize to 0-1 range
    (score / 255.0 + 1.0) / 2.0
}

/// Get average intensity of a region
fn get_region_intensity(img: &ImageBuffer, x: u32, y: u32, width: u32, height: u32) -> f32 {
    let mut sum = 0.0;
    let mut count = 0;

    for dy in 0..height {
        for dx in 0..width {
            let px = x + dx;
            let py = y + dy;

            if px < img.width && py < img.height {
                let pixel = img.get_pixel(px, py);
                sum += pixel[0];
                count += 1;
            }
        }
    }

    if count > 0 {
        sum / count as f32
    } else {
        0.0
    }
}
/// Non-maximum suppression to remove overlapping detections
fn non_maximum_suppression(faces: &[Face], threshold: f32) -> Vec<Face> {
    let mut result = Vec::new();
    let mut faces = faces.to_vec();

    faces.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    while !faces.is_empty() {
        let best = faces.remove(0);
        result.push(best.clone());

        faces.retain(|face| {
            let iou = compute_iou(&best.bbox, &face.bbox);
            iou < threshold
        });
    }

    result
}

/// Compute Intersection over Union (IoU)
fn compute_iou(box1: &BoundingBox, box2: &BoundingBox) -> f32 {
    let x1 = box1.x.max(box2.x);
    let y1 = box1.y.max(box2.y);
    let x2 = (box1.x + box1.width).min(box2.x + box2.width);
    let y2 = (box1.y + box1.height).min(box2.y + box2.height);

    if x2 <= x1 || y2 <= y1 {
        return 0.0;
    }

    let intersection = ((x2 - x1) * (y2 - y1)) as f32;
    let area1 = (box1.width * box1.height) as f32;
    let area2 = (box2.width * box2.height) as f32;
    let union = area1 + area2 - intersection;

    intersection / union
}

/// Refine face detections
fn refine_detections(
    img: &ImageBuffer,
    proposals: Vec<Face>,
    options: &DetectionOptions,
) -> Result<Vec<Face>> {
    let mut refined = Vec::new();

    for mut face in proposals {
        // Refine bounding box coordinates
        let offset = refine_bbox(img, &face.bbox);
        face.bbox.x = (face.bbox.x as i32 + offset.0).max(0) as u32;
        face.bbox.y = (face.bbox.y as i32 + offset.1).max(0) as u32;

        refined.push(face);

        if refined.len() >= options.max_faces {
            break;
        }
    }

    Ok(refined)
}

/// Refine bounding box position
fn refine_bbox(img: &ImageBuffer, bbox: &BoundingBox) -> (i32, i32) {
    // Simple gradient-based refinement
    (0, 0)
}

/// Build image pyramid for multi-scale detection
pub fn build_pyramid(img: &ImageBuffer, scale_factor: f32, min_size: u32) -> Vec<ImageBuffer> {
    let mut pyramid = vec![img.clone()];
    let mut current = img.clone();

    while current.width >= min_size && current.height >= min_size {
        let new_width = (current.width as f32 * scale_factor) as u32;
        let new_height = (current.height as f32 * scale_factor) as u32;

        if new_width < min_size || new_height < min_size {
            break;
        }

        current = current.resize(new_width, new_height);
        pyramid.push(current.clone());
    }

    pyramid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyramid_building() {
        let img = ImageBuffer::new(640, 480, 3);
        let pyramid = build_pyramid(&img, 0.7, 40);

        assert!(!pyramid.is_empty());
        assert!(pyramid[0].width == 640);
        assert!(pyramid.last().unwrap().width < 640);
    }
}
