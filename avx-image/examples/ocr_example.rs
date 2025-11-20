//! OCR text recognition example

use avx_image::prelude::*;
use avx_image::ocr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 AVX-Image - OCR Example\n");

    // Create test document image
    let img = ImageBuffer::new(800, 600, 3);
    println!("✅ Created document image: {}x{}", img.width, img.height);

    // Preprocess for better OCR
    println!("\n🔧 Preprocessing...");
    let gray = img.to_grayscale();
    println!("✅ Converted to grayscale");

    let normalized = Preprocessing::normalize(&gray)?;
    println!("✅ Normalized image");

    let threshold = Preprocessing::otsu_threshold(&normalized)?;
    let binary = Preprocessing::threshold(&normalized, threshold)?;
    println!("✅ Applied binary threshold: {:.3}", threshold);

    // Recognize text
    println!("\n📖 Recognizing text...");
    let options = ocr::OcrOptions {
        language: ocr::Language::Portuguese,
        min_confidence: 0.6,
        detect_orientation: true,
        preserve_layout: true,
    };

    let result = ocr::recognize_with_options(&binary, &options)?;

    println!("\n📄 OCR Results:");
    println!("   Text: {}", result.text);
    println!("   Confidence: {:.2}%", result.confidence * 100.0);
    println!("   Language: {:?}", result.language);
    println!("   Bounding boxes: {}", result.bounding_boxes.len());

    // Display detected text regions
    for (i, bbox) in result.bounding_boxes.iter().enumerate() {
        println!(
            "\n   Region {}: \"{}\"",
            i + 1,
            bbox.text
        );
        println!(
            "      Position: ({}, {}) Size: {}x{}",
            bbox.x, bbox.y, bbox.width, bbox.height
        );
        println!("      Confidence: {:.2}%", bbox.confidence * 100.0);
    }

    println!("\n✨ OCR complete!");

    Ok(())
}
