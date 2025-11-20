//! Basic image processing example

use avx_image::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖼️  AVX-Image - Basic Image Processing Example\n");

    // Create a test image
    let mut img = ImageBuffer::new(640, 480, 3);

    println!("✅ Created {}x{} image with {} channels", img.width, img.height, img.channels);

    // Add some pattern
    for y in 0..img.height {
        for x in 0..img.width {
            let r = (x as f32 / img.width as f32);
            let g = (y as f32 / img.height as f32);
            let b = 0.5;
            img.set_pixel(x, y, &[r, g, b]);
        }
    }

    println!("✅ Added gradient pattern");

    // Convert to grayscale
    let gray = img.to_grayscale();
    println!("✅ Converted to grayscale");

    // Apply preprocessing
    let blurred = Preprocessing::gaussian_blur(&gray, 2.0)?;
    println!("✅ Applied Gaussian blur (sigma=2.0)");

    let edges = Preprocessing::sobel_edge_detection(&gray)?;
    println!("✅ Detected edges with Sobel operator");

    let normalized = Preprocessing::normalize(&edges)?;
    println!("✅ Normalized image to [0, 1]");

    // Binary threshold
    let threshold = Preprocessing::otsu_threshold(&gray)?;
    println!("✅ Computed Otsu threshold: {:.3}", threshold);

    let binary = Preprocessing::threshold(&gray, threshold)?;
    println!("✅ Applied binary thresholding");

    // Resize
    let resized = img.resize(320, 240);
    println!("✅ Resized to {}x{}", resized.width, resized.height);

    // Extract features
    println!("\n🔍 Extracting features...");

    let hog = FeatureExtractor::hog(&img, 8, 2, 9)?;
    println!("✅ HOG features: {} dimensions", hog.len());

    let lbp = FeatureExtractor::lbp(&gray, 1, 8)?;
    println!("✅ LBP features: {} dimensions", lbp.len());

    let color_hist = FeatureExtractor::color_histogram(&img, 8)?;
    println!("✅ Color histogram: {} bins", color_hist.len());

    println!("\n✨ Processing complete!");

    Ok(())
}
