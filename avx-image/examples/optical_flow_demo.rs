/// Optical Flow Demo
///
/// Demonstrates Lucas-Kanade sparse and Farnebäck dense optical flow algorithms.

use avx_image::native::{NativeImageBuffer, lucas_kanade_sparse, farneback_dense, fast_detector};
use avx_image::Result;
use std::time::Instant;

fn main() -> Result<()> {
    println!("=== AVX-Image Optical Flow Demo ===\n");

    // Test 1: Lucas-Kanade Sparse Optical Flow
    println!("--- Test 1: Lucas-Kanade Sparse Optical Flow ---");
    test_lucas_kanade()?;

    // Test 2: Farnebäck Dense Optical Flow
    println!("\n--- Test 2: Farnebäck Dense Optical Flow ---");
    test_farneback()?;

    // Test 3: Feature Tracking
    println!("\n--- Test 3: Feature Tracking (FAST + Lucas-Kanade) ---");
    test_feature_tracking()?;

    println!("\n=== All optical flow tests completed ===");
    Ok(())
}

fn test_lucas_kanade() -> Result<()> {
    // Create two frames with a moving square
    let mut frame1 = NativeImageBuffer::new(200, 200, 1);
    let mut frame2 = NativeImageBuffer::new(200, 200, 1);

    // Background gradient
    for y in 0..200 {
        for x in 0..200 {
            let val = ((x + y) as f32 / 400.0) * 0.3;
            frame1.data[y * 200 + x] = val;
            frame2.data[y * 200 + x] = val;
        }
    }

    // Square at (50, 50) in frame1
    for y in 50..70 {
        for x in 50..70 {
            frame1.data[y * 200 + x] = 0.9;
        }
    }

    // Same square moved to (60, 55) in frame2 (motion: dx=10, dy=5)
    for y in 55..75 {
        for x in 60..80 {
            frame2.data[y * 200 + x] = 0.9;
        }
    }

    // Track points from the square
    let points = vec![
        (55.0, 55.0),
        (60.0, 60.0),
        (65.0, 65.0),
    ];

    let start = Instant::now();
    let results = lucas_kanade_sparse(&frame1, &frame2, &points, 15, 10, 0.01)?;
    let elapsed = start.elapsed();

    println!("✓ Lucas-Kanade tracking completed");
    println!("  Points tracked: {}", points.len());
    println!("  Window size: 15x15");
    println!("  Time: {:.4}ms", elapsed.as_secs_f64() * 1000.0);

    println!("\n  Tracking results:");
    for (i, (&(px, py), result)) in points.iter().zip(results.iter()).enumerate() {
        match result {
            Some((nx, ny)) => {
                let dx = nx - px;
                let dy = ny - py;
                println!("    Point {}: ({:.1}, {:.1}) → ({:.1}, {:.1}) | motion: ({:.1}, {:.1})",
                    i + 1, px, py, nx, ny, dx, dy);
            }
            None => {
                println!("    Point {}: ({:.1}, {:.1}) → LOST", i + 1, px, py);
            }
        }
    }

    // Validate motion (should be approximately dx=10, dy=5)
    let mut avg_dx = 0.0;
    let mut avg_dy = 0.0;
    let mut count = 0;

    for (i, result) in results.iter().enumerate() {
        if let Some((nx, ny)) = result {
            avg_dx += nx - points[i].0;
            avg_dy += ny - points[i].1;
            count += 1;
        }
    }

    if count > 0 {
        avg_dx /= count as f32;
        avg_dy /= count as f32;
        println!("\n  Average motion: ({:.2}, {:.2}) pixels", avg_dx, avg_dy);
        println!("  Expected motion: (10.0, 5.0) pixels");

        let error = ((avg_dx - 10.0).powi(2) + (avg_dy - 5.0).powi(2)).sqrt();
        println!("  Error: {:.2} pixels", error);
    }

    Ok(())
}

fn test_farneback() -> Result<()> {
    // Create two frames with horizontal motion
    let mut frame1 = NativeImageBuffer::new(128, 128, 1);
    let mut frame2 = NativeImageBuffer::new(128, 128, 1);

    // Create a pattern with texture
    for y in 0..128 {
        for x in 0..128 {
            let val = ((x / 8 + y / 8) % 2) as f32 * 0.5 + 0.25;
            frame1.data[y * 128 + x] = val;
        }
    }

    // Shift pattern 5 pixels to the right
    for y in 0..128 {
        for x in 0..128 {
            let src_x = if x >= 5 { x - 5 } else { 0 };
            frame2.data[y * 128 + x] = frame1.data[y * 128 + src_x];
        }
    }

    println!("Computing dense optical flow...");
    let start = Instant::now();
    let flow = farneback_dense(
        &frame1,
        &frame2,
        0.5,    // pyramid scale
        3,      // levels
        13,     // window size
        5,      // iterations
        5,      // poly_n
        1.2,    // poly_sigma
    )?;
    let elapsed = start.elapsed();

    println!("✓ Farnebäck dense flow computed");
    println!("  Image size: 128x128");
    println!("  Pyramid levels: 3");
    println!("  Window size: 13");
    println!("  Time: {:.4}ms", elapsed.as_secs_f64() * 1000.0);

    // Analyze flow statistics
    let mut avg_vx = 0.0;
    let mut avg_vy = 0.0;
    let mut max_magnitude: f32 = 0.0;
    let mut count = 0;

    for y in 20..108 {  // Avoid borders
        for x in 20..108 {
            let v = flow.get_vector(x, y);
            avg_vx += v.x;
            avg_vy += v.y;
            max_magnitude = max_magnitude.max(v.magnitude);
            count += 1;
        }
    }

    avg_vx /= count as f32;
    avg_vy /= count as f32;

    println!("\n  Flow statistics (excluding borders):");
    println!("    Average flow: ({:.2}, {:.2}) pixels/frame", avg_vx, avg_vy);
    println!("    Expected: (5.0, 0.0) pixels/frame");
    println!("    Max magnitude: {:.2} pixels", max_magnitude);

    let error = ((avg_vx - 5.0).powi(2) + avg_vy.powi(2)).sqrt();
    println!("    Error: {:.2} pixels", error);

    Ok(())
}

fn test_feature_tracking() -> Result<()> {
    // Simulate a rotating pattern
    let mut frame1 = NativeImageBuffer::new(150, 150, 1);
    let mut frame2 = NativeImageBuffer::new(150, 150, 1);

    // Create star-like pattern with features
    for &(cx, cy) in &[(50, 50), (100, 50), (75, 100), (50, 100), (100, 100)] {
        for dy in -5..=5 {
            for dx in -5..=5 {
                if dx * dx + dy * dy <= 25 {
                    let x = (cx as i32 + dx) as usize;
                    let y = (cy as i32 + dy) as usize;
                    if x < 150 && y < 150 {
                        frame1.data[y * 150 + x] = 0.9;
                    }
                }
            }
        }
    }

    // Move pattern slightly (simulate camera motion)
    for y in 0..150 {
        for x in 0..150 {
            let src_x = if x >= 3 { x - 3 } else { 0 };
            let src_y = if y >= 2 { y - 2 } else { 0 };
            if src_x < 150 && src_y < 150 {
                frame2.data[y * 150 + x] = frame1.data[src_y * 150 + src_x];
            }
        }
    }

    // Detect features in frame1 using FAST
    println!("Detecting features with FAST...");
    let start = Instant::now();
    let keypoints = fast_detector(&frame1, 20, 9)?;
    let detect_time = start.elapsed();

    println!("✓ Features detected: {}", keypoints.len());
    println!("  Detection time: {:.4}ms", detect_time.as_secs_f64() * 1000.0);

    // Track features with Lucas-Kanade
    let points: Vec<(f32, f32)> = keypoints.iter()
        .take(20)  // Track top 20 features
        .map(|kp| (kp.x, kp.y))
        .collect();

    println!("\nTracking {} features with Lucas-Kanade...", points.len());
    let start = Instant::now();
    let results = lucas_kanade_sparse(&frame1, &frame2, &points, 15, 10, 0.01)?;
    let track_time = start.elapsed();

    println!("✓ Tracking completed");
    println!("  Tracking time: {:.4}ms", track_time.as_secs_f64() * 1000.0);
    println!("  Time per feature: {:.4}ms", track_time.as_secs_f64() * 1000.0 / points.len() as f64);

    // Analyze tracking success
    let tracked = results.iter().filter(|r| r.is_some()).count();
    let lost = results.len() - tracked;

    println!("\n  Tracking statistics:");
    println!("    Successfully tracked: {}/{} ({:.1}%)",
        tracked, results.len(), 100.0 * tracked as f32 / results.len() as f32);
    println!("    Lost: {}", lost);

    if tracked > 0 {
        let mut avg_dx = 0.0;
        let mut avg_dy = 0.0;
        let mut count = 0;

        for (i, result) in results.iter().enumerate() {
            if let Some((nx, ny)) = result {
                avg_dx += nx - points[i].0;
                avg_dy += ny - points[i].1;
                count += 1;
            }
        }

        avg_dx /= count as f32;
        avg_dy /= count as f32;

        println!("    Average displacement: ({:.2}, {:.2}) pixels", avg_dx, avg_dy);
        println!("    Expected: (3.0, 2.0) pixels");
    }

    // Show sample tracked features
    println!("\n  Sample tracked features (first 5):");
    for (i, (&(px, py), result)) in points.iter().zip(results.iter()).take(5).enumerate() {
        match result {
            Some((nx, ny)) => {
                let dx = nx - px;
                let dy = ny - py;
                let dist = (dx * dx + dy * dy).sqrt();
                println!("    Feature {}: ({:.1}, {:.1}) → ({:.1}, {:.1}) | distance: {:.2}px",
                    i + 1, px, py, nx, ny, dist);
            }
            None => {
                println!("    Feature {}: ({:.1}, {:.1}) → LOST", i + 1, px, py);
            }
        }
    }

    Ok(())
}
