//! Feature detection demonstration
//! Shows Harris, FAST, and ORB detectors

use avx_image::native::buffer::NativeImageBuffer;
use avx_image::native::features::*;

fn main() -> std::io::Result<()> {
    println!("=== AVX-Image Feature Detection Demo ===\n");

    // Create a test image with synthetic features
    let width = 300;
    let height = 300;
    let mut img = NativeImageBuffer::new(width, height, 1);

    // Generate a pattern with corners and edges
    println!("Creating synthetic test image...");
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;

            // Checkerboard pattern
            let checker = if (x / 30 + y / 30) % 2 == 0 {
                0.8
            } else {
                0.2
            };

            // Add some circles
            let cx1 = 100.0;
            let cy1 = 100.0;
            let r1 = ((x as f32 - cx1).powi(2) + (y as f32 - cy1).powi(2)).sqrt();

            let cx2 = 200.0;
            let cy2 = 200.0;
            let r2 = ((x as f32 - cx2).powi(2) + (y as f32 - cy2).powi(2)).sqrt();

            let circles = if r1 < 40.0 || r2 < 30.0 { 0.9 } else { checker };

            // Add horizontal lines
            let lines = if y % 50 == 0 { 1.0 } else { circles };

            img.data[idx] = lines;
        }
    }

    println!("✓ Test image created: {}x{}\n", width, height);

    // Test Harris corner detector
    test_harris(&img)?;

    // Test FAST detector
    test_fast(&img)?;

    // Test ORB
    test_orb(&img)?;

    // Test feature matching
    test_matching(&img)?;

    println!("\n=== All feature detection tests completed ===");
    Ok(())
}

fn test_harris(img: &NativeImageBuffer) -> std::io::Result<()> {
    println!("--- Harris Corner Detector ---");

    let threshold = 0.01;
    let k = 0.04;

    let start = std::time::Instant::now();
    let corners = harris_corners(img, threshold, k)?;
    let elapsed = start.elapsed();

    println!("✓ Harris corners detected: {}", corners.len());
    println!("  Threshold: {}", threshold);
    println!("  k parameter: {}", k);
    println!("  Time: {:?}", elapsed);

    if !corners.is_empty() {
        // Show top 5 corners by response
        let mut sorted = corners.clone();
        sorted.sort_by(|a, b| b.response.partial_cmp(&a.response).unwrap());

        println!("\n  Top 5 corners:");
        for (i, corner) in sorted.iter().take(5).enumerate() {
            println!(
                "    {}. ({:.1}, {:.1}) response: {:.4}",
                i + 1,
                corner.x,
                corner.y,
                corner.response
            );
        }
    }

    println!();
    Ok(())
}

fn test_fast(img: &NativeImageBuffer) -> std::io::Result<()> {
    println!("--- FAST Detector ---");

    let threshold = 30;
    let n_consecutive = 9;

    let start = std::time::Instant::now();
    let keypoints = fast_detector(img, threshold, n_consecutive)?;
    let elapsed = start.elapsed();

    println!("✓ FAST keypoints detected: {}", keypoints.len());
    println!("  Threshold: {}", threshold);
    println!("  Consecutive pixels: {}", n_consecutive);
    println!("  Time: {:?}", elapsed);

    if !keypoints.is_empty() {
        // Show statistics
        let total_response: f32 = keypoints.iter().map(|kp| kp.response).sum();
        let avg_response = total_response / keypoints.len() as f32;

        println!("\n  Statistics:");
        println!("    Average response: {:.2}", avg_response);

        // Distribution by quadrant
        let q1 = keypoints
            .iter()
            .filter(|kp| kp.x < 150.0 && kp.y < 150.0)
            .count();
        let q2 = keypoints
            .iter()
            .filter(|kp| kp.x >= 150.0 && kp.y < 150.0)
            .count();
        let q3 = keypoints
            .iter()
            .filter(|kp| kp.x < 150.0 && kp.y >= 150.0)
            .count();
        let q4 = keypoints
            .iter()
            .filter(|kp| kp.x >= 150.0 && kp.y >= 150.0)
            .count();

        println!("    Distribution by quadrant:");
        println!("      Q1 (top-left):     {}", q1);
        println!("      Q2 (top-right):    {}", q2);
        println!("      Q3 (bottom-left):  {}", q3);
        println!("      Q4 (bottom-right): {}", q4);
    }

    println!();
    Ok(())
}

fn test_orb(img: &NativeImageBuffer) -> std::io::Result<()> {
    println!("--- ORB (Oriented FAST + BRIEF) ---");

    let max_features = 100;

    let start = std::time::Instant::now();
    let (keypoints, descriptors) = orb_detect_and_compute(img, max_features)?;
    let elapsed = start.elapsed();

    println!("✓ ORB features computed: {}", keypoints.len());
    println!("  Max features: {}", max_features);
    println!("  Time: {:?}", elapsed);

    if !keypoints.is_empty() {
        // Analyze orientations
        let oriented = keypoints.iter().filter(|kp| kp.angle >= 0.0).count();
        println!("\n  Orientation analysis:");
        println!("    Keypoints with orientation: {}", oriented);

        if oriented > 0 {
            let avg_angle: f32 = keypoints
                .iter()
                .filter(|kp| kp.angle >= 0.0)
                .map(|kp| kp.angle)
                .sum::<f32>()
                / oriented as f32;
            println!("    Average angle: {:.2} rad", avg_angle);
        }

        // Descriptor analysis
        println!("\n  Descriptor statistics:");
        println!("    Descriptor size: {} bytes", descriptors[0].data.len());
        println!("    Total descriptors: {}", descriptors.len());

        // Sample descriptor bits
        if let Some(first_desc) = descriptors.first() {
            let ones: u32 = first_desc.data.iter().map(|b| b.count_ones()).sum();
            let total_bits = first_desc.data.len() * 8;
            println!(
                "    First descriptor density: {}/{} bits set ({:.1}%)",
                ones,
                total_bits,
                (ones as f32 / total_bits as f32) * 100.0
            );
        }
    }

    println!();
    Ok(())
}

fn test_matching(img: &NativeImageBuffer) -> std::io::Result<()> {
    println!("--- Feature Matching ---");

    // Compute features for original image
    let (kp1, desc1) = orb_detect_and_compute(img, 50)?;

    // Create a slightly translated version
    let mut img2 = NativeImageBuffer::new(img.width, img.height, 1);
    let offset = 10;

    for y in 0..img.height {
        for x in 0..img.width {
            let idx = (y * img.width + x) as usize;
            let src_x = if x >= offset { x - offset } else { 0 };
            let src_y = if y >= offset { y - offset } else { 0 };
            let src_idx = (src_y * img.width + src_x) as usize;
            img2.data[idx] = img.data[src_idx];
        }
    }

    let (kp2, desc2) = orb_detect_and_compute(&img2, 50)?;

    println!("  Image 1: {} features", kp1.len());
    println!("  Image 2: {} features", kp2.len());

    // Match features
    let max_distance = 50;
    let start = std::time::Instant::now();
    let matches = match_features(&desc1, &desc2, max_distance);
    let elapsed = start.elapsed();

    println!("\n✓ Feature matching completed:");
    println!("  Matches found: {}", matches.len());
    println!("  Max distance threshold: {}", max_distance);
    println!("  Time: {:?}", elapsed);

    if !matches.is_empty() {
        // Analyze match quality
        let avg_distance: f32 = matches.iter().map(|m| m.distance).sum::<f32>() / matches.len() as f32;
        let min_distance = matches
            .iter()
            .map(|m| m.distance)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_distance_found = matches
            .iter()
            .map(|m| m.distance)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        println!("\n  Match quality:");
        println!("    Average distance: {:.2}", avg_distance);
        println!("    Min distance: {:.2}", min_distance);
        println!("    Max distance: {:.2}", max_distance_found);

        // Show best matches
        let mut sorted_matches = matches.clone();
        sorted_matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        println!("\n  Top 5 matches:");
        for (i, m) in sorted_matches.iter().take(5).enumerate() {
            let kp_a = &kp1[m.query_idx];
            let kp_b = &kp2[m.train_idx];
            let dx = kp_b.x - kp_a.x;
            let dy = kp_b.y - kp_a.y;

            println!(
                "    {}. ({:.1}, {:.1}) → ({:.1}, {:.1}) | distance: {:.1} | offset: ({:.1}, {:.1})",
                i + 1,
                kp_a.x,
                kp_a.y,
                kp_b.x,
                kp_b.y,
                m.distance,
                dx,
                dy
            );
        }
    }

    Ok(())
}
