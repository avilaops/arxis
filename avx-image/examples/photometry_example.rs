//! Color analysis and photometry example

use avx_image::photometry;
use avx_image::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 AVX-Image - Photometry Example\n");

    // Create test image with color gradient
    let mut img = ImageBuffer::new(400, 300, 3);

    for y in 0..img.height {
        for x in 0..img.width {
            let r = (x as f32 / img.width as f32);
            let g = (y as f32 / img.height as f32);
            let b = 0.5;
            img.set_pixel(x, y, &[r, g, b]);
        }
    }

    println!("✅ Created test image with gradient");

    // Analyze colors
    println!("\n🔍 Analyzing colors...");
    let analysis = photometry::analyze_colors(&img)?;

    println!(
        "   Average color: RGB({:.3}, {:.3}, {:.3})",
        analysis.average_color.0, analysis.average_color.1, analysis.average_color.2
    );
    println!("   Color temperature: {:.0}K", analysis.color_temperature);
    println!("   Dominant colors: {}", analysis.dominant_colors.len());

    // Estimate illumination
    println!("\n💡 Estimating illumination...");
    let illuminant = photometry::estimate_illumination(&img)?;

    println!("   Color temperature: {:.0}K", illuminant.color_temperature);
    println!(
        "   RGB multipliers: ({:.3}, {:.3}, {:.3})",
        illuminant.rgb_multipliers.0, illuminant.rgb_multipliers.1, illuminant.rgb_multipliers.2
    );

    // Apply white balance
    println!("\n⚖️  Applying white balance...");
    let corrected = photometry::white_balance(&img, &illuminant)?;
    println!("✅ White balance applied");

    // Color space conversions
    println!("\n🌈 Converting color spaces...");

    let hsv = photometry::color_spaces::rgb_to_hsv(&img)?;
    println!("✅ Converted to HSV");

    let ycbcr = photometry::color_spaces::rgb_to_ycbcr(&img)?;
    println!("✅ Converted to YCbCr");

    // Extract color histogram
    println!("\n📊 Extracting color histogram...");
    let histogram = FeatureExtractor::color_histogram(&img, 8)?;
    println!("✅ Computed histogram with {} bins", histogram.len());

    let max_bin = histogram
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, v)| (i, v))
        .unwrap();

    println!(
        "   Most frequent bin: {} (value: {:.4})",
        max_bin.0, max_bin.1
    );

    println!("\n✨ Color analysis complete!");

    Ok(())
}
