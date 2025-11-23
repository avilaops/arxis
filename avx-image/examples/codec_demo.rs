//! Codec demonstration
//! Tests native PNG and JPEG codecs

use avx_image::native::buffer::NativeImageBuffer;
use avx_image::native::codec::{JpegDecoder, JpegEncoder, PngDecoder, PngEncoder};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("=== AVX-Image Codec Demo ===\n");

    // Create a test image
    let width = 256;
    let height = 256;
    let mut img = NativeImageBuffer::new(width, height, 4);

    // Generate gradient pattern
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            img.data[idx] = (x * 255 / width) as f32; // R
            img.data[idx + 1] = (y * 255 / height) as f32; // G
            img.data[idx + 2] = ((x + y) * 255 / (width + height)) as f32; // B
            img.data[idx + 3] = 255.0; // A
        }
    }

    println!("✓ Created {}x{} test image", width, height);

    // Test PNG encoding
    test_png_encode(&img)?;

    // Test PNG decoding
    test_png_decode()?;

    // Test JPEG structures
    test_jpeg_structures()?;

    println!("\n=== All codec tests completed ===");
    Ok(())
}

fn test_png_encode(img: &NativeImageBuffer) -> std::io::Result<()> {
    println!("\n--- PNG Encoding Test ---");

    let encoder = PngEncoder::new(img.width as u32, img.height as u32);

    // Convert f32 to u8
    let mut rgba_data = vec![0u8; img.data.len()];
    for (i, &val) in img.data.iter().enumerate() {
        rgba_data[i] = val.min(255.0).max(0.0) as u8;
    }

    // Encode to memory
    let mut output = Vec::new();
    match encoder.encode(&mut output, &rgba_data) {
        Ok(_) => {
            println!("✓ PNG encoded: {} bytes", output.len());

            // Save to file
            if let Ok(mut file) = File::create("test_output.png") {
                file.write_all(&output)?;
                println!("✓ Saved to test_output.png");
            }
        }
        Err(e) => {
            println!("⚠ PNG encoding: {} (expected for compressed)", e);
        }
    }

    Ok(())
}

fn test_png_decode() -> std::io::Result<()> {
    println!("\n--- PNG Decoding Test ---");

    // Try to decode if file exists
    if let Ok(mut file) = File::open("test_output.png") {
        let mut decoder = PngDecoder::new();
        match decoder.decode(&mut file) {
            Ok(rgba) => {
                println!(
                    "✓ PNG decoded: {}x{} = {} pixels",
                    decoder.width,
                    decoder.height,
                    rgba.len() / 4
                );
                println!("  Color type: {:?}", decoder.color_type);
                println!("  Bit depth: {}", decoder.bit_depth);
            }
            Err(e) => {
                println!("⚠ PNG decoding: {} (need full DEFLATE)", e);
            }
        }
    } else {
        println!("⚠ No test PNG file found (expected)");
    }

    Ok(())
}

fn test_jpeg_structures() -> std::io::Result<()> {
    println!("\n--- JPEG Structures Test ---");

    // Test JPEG decoder creation
    let decoder = JpegDecoder::new();
    println!("✓ JPEG decoder created");
    println!("  Initial size: {}x{}", decoder.width, decoder.height);
    println!("  Quantization tables: {}", decoder.quant_tables.len());

    // Test JPEG encoder creation
    let encoder = JpegEncoder::new(640, 480, 90);
    println!("✓ JPEG encoder created");
    println!("  Size: {}x{}", encoder.width, encoder.height);
    println!("  Quality: {}", encoder.quality);

    // Display standard quantization tables
    let lum_table = encoder.luminance_quant_table();
    println!("\n  Luminance quant table (first 8 values):");
    print!("  ");
    for i in 0..8 {
        print!("{:3} ", lum_table[i]);
    }
    println!();

    let chrom_table = encoder.chrominance_quant_table();
    println!("  Chrominance quant table (first 8 values):");
    print!("  ");
    for i in 0..8 {
        print!("{:3} ", chrom_table[i]);
    }
    println!();

    println!("\n⚠ Note: Full JPEG encoding/decoding requires:");
    println!("  • Huffman coding tables");
    println!("  • DCT/IDCT (already have in fft.rs)");
    println!("  • YCbCr color space (already have in color.rs)");
    println!("  • Zigzag scanning");
    println!("  • Entropy encoding");

    Ok(())
}
