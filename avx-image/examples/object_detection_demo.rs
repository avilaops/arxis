/// Object Detection Demo
///
/// Demonstrates Haar Cascade object detection with integral images.

use avx_image::native::{
    NativeImageBuffer, HaarCascade, IntegralImage, Detection, non_maximum_suppression,
};
use avx_image::Result;
use std::time::Instant;

fn main() -> Result<()> {
    println!("=== AVX-Image Object Detection Demo ===\n");

    // Test 1: Integral Image Performance
    test_integral_image()?;

    // Test 2: Face Detection Simulation
    test_face_detection()?;

    // Test 3: Non-Maximum Suppression
    test_nms()?;

    println!("\n=== All object detection tests completed ===");
    Ok(())
}

fn test_integral_image() -> Result<()> {
    println!("--- Test 1: Integral Image Computation ---");

    // Create test image
    let size = 512;
    let mut img = NativeImageBuffer::new(size, size, 1);

    // Fill with gradient pattern
    for y in 0..size {
        for x in 0..size {
            let val = ((x + y) as f32 / (2 * size) as f32).min(1.0);
            img.data[y * size + x] = val;
        }
    }

    let start = Instant::now();
    let integral = IntegralImage::from_image(&img)?;
    let duration = start.elapsed();

    println!("✅ Integral image computed");
    println!("  Image size: {}x{}", size, size);
    println!("  Time: {:.2}ms", duration.as_secs_f64() * 1000.0);

    // Test rectangle sum
    let rect_sum = integral.rect_sum(100, 100, 50, 50);
    println!("  Sample rect sum (100,100,50x50): {:.2}", rect_sum);

    // Verify correctness with known rectangle
    let img_data = &img.data;
    let manual_sum: f32 = (100..150)
        .flat_map(|y| (100..150).map(move |x| img_data[y * size + x]))
        .sum();
    let error = (rect_sum - manual_sum).abs();
    println!("  Manual verification: {:.2} (error: {:.6})", manual_sum, error);

    println!();
    Ok(())
}

fn test_face_detection() -> Result<()> {
    println!("--- Test 2: Face Detection (Simulated) ---");

    // Create synthetic "face-like" image
    let width = 200;
    let height = 200;
    let mut img = NativeImageBuffer::new(width, height, 1);

    // Background
    for i in 0..img.data.len() {
        img.data[i] = 0.7;
    }

    // Draw "face" structure (bright oval with darker eyes/mouth)
    let center_x = width / 2;
    let center_y = height / 2;

    // Face oval (brighter)
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x as f32;
            let dy = y as f32 - center_y as f32;
            let dist = ((dx * dx / 40.0) + (dy * dy / 50.0)).sqrt();

            if dist < 30.0 {
                img.data[y * width + x] = 0.9;
            }
        }
    }

    // Eyes (darker regions)
    for (eye_x, eye_y) in [(center_x - 15, center_y - 10), (center_x + 15, center_y - 10)] {
        for y in (eye_y - 5)..(eye_y + 5) {
            for x in (eye_x - 5)..(eye_x + 5) {
                if y < height && x < width {
                    img.data[y * width + x] = 0.3;
                }
            }
        }
    }

    // Mouth (darker horizontal line)
    let mouth_y = center_y + 15;
    for x in (center_x - 12)..(center_x + 12) {
        for y in (mouth_y - 2)..(mouth_y + 2) {
            if y < height && x < width {
                img.data[y * width + x] = 0.4;
            }
        }
    }

    println!("Created synthetic face image: {}x{}", width, height);

    // Create detector
    let detector = HaarCascade::simple_face_detector();
    println!("✅ Face detector initialized");
    println!("  Base window: {}x{}", detector.base_width, detector.base_height);
    println!("  Cascade stages: {}", detector.stages.len());

    // Multi-scale detection
    let start = Instant::now();
    let detections = detector.detect_multi_scale(
        &img,
        1.0,  // min_scale
        3.0,  // max_scale
        1.2,  // scale_factor
        4,    // step_size
    )?;
    let duration = start.elapsed();

    println!("✅ Detection completed");
    println!("  Time: {:.2}ms", duration.as_secs_f64() * 1000.0);
    println!("  Raw detections: {}", detections.len());

    // Apply NMS
    let filtered = non_maximum_suppression(detections, 0.3);
    println!("  After NMS: {}", filtered.len());

    if !filtered.is_empty() {
        println!("\n  Top detections:");
        for (i, det) in filtered.iter().take(3).enumerate() {
            println!(
                "    Detection {}: ({}, {}) {}x{} | confidence: {:.3}",
                i + 1,
                det.x,
                det.y,
                det.width,
                det.height,
                det.confidence
            );
        }
    }

    println!();
    Ok(())
}

fn test_nms() -> Result<()> {
    println!("--- Test 3: Non-Maximum Suppression ---");

    // Create overlapping detections
    let detections = vec![
        Detection::new(50, 50, 40, 40, 0.95),  // Best
        Detection::new(52, 52, 40, 40, 0.90),  // Overlaps with #1
        Detection::new(54, 48, 40, 40, 0.85),  // Overlaps with #1
        Detection::new(150, 50, 40, 40, 0.88), // Different location
        Detection::new(152, 52, 40, 40, 0.82), // Overlaps with #4
        Detection::new(50, 150, 40, 40, 0.75), // Different location
    ];

    println!("Input detections: {}", detections.len());
    for (i, det) in detections.iter().enumerate() {
        println!(
            "  Det {}: ({},{}) conf={:.2}",
            i + 1,
            det.x,
            det.y,
            det.confidence
        );
    }

    // Apply NMS with different thresholds
    for iou_threshold in [0.3, 0.5, 0.7] {
        let filtered = non_maximum_suppression(detections.clone(), iou_threshold);
        println!(
            "\n  IoU threshold {:.1}: {} detections kept",
            iou_threshold,
            filtered.len()
        );

        for (i, det) in filtered.iter().enumerate() {
            println!(
                "    #{}: ({},{}) conf={:.2}",
                i + 1,
                det.x,
                det.y,
                det.confidence
            );
        }
    }

    // Test IoU calculation
    println!("\n  IoU Examples:");
    let det1 = Detection::new(10, 10, 20, 20, 1.0);
    let det2 = Detection::new(15, 15, 20, 20, 1.0);
    let det3 = Detection::new(50, 50, 20, 20, 1.0);

    println!("    det1 vs det2 (overlapping): IoU = {:.3}", det1.iou(&det2));
    println!("    det1 vs det3 (separate): IoU = {:.3}", det1.iou(&det3));
    println!("    det1 vs det1 (identical): IoU = {:.3}", det1.iou(&det1));

    println!();
    Ok(())
}
