//! Face detection and recognition example

use avx_image::prelude::*;
use avx_image::face;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("👤 AVX-Image - Face Recognition Example\n");

    // Create test image
    let img = ImageBuffer::new(640, 480, 3);
    println!("✅ Loaded image: {}x{}", img.width, img.height);

    // Detect faces
    println!("\n🔍 Detecting faces...");
    let faces = face::detect(&img)?;
    println!("✅ Found {} faces", faces.len());

    // Process each face
    for (i, detected_face) in faces.iter().enumerate() {
        println!("\n👤 Face {}:", i + 1);
        println!("   Position: ({}, {})", detected_face.bbox.x, detected_face.bbox.y);
        println!("   Size: {}x{}", detected_face.bbox.width, detected_face.bbox.height);
        println!("   Confidence: {:.2}%", detected_face.confidence * 100.0);

        // Detect landmarks
        if let Ok(landmarks) = face::detect_landmarks(&img, detected_face) {
            println!("   Landmarks: {} points", landmarks.points.len());
        }

        // Extract embedding
        if let Ok(embedding) = face::extract_embedding(&img, detected_face) {
            println!("   Embedding: {} dimensions", embedding.len());
        }

        // Check liveness
        if let Ok(liveness) = face::check_liveness(&img, detected_face) {
            println!(
                "   Liveness: {} (confidence: {:.2}%)",
                if liveness.is_live { "LIVE" } else { "SPOOFED" },
                liveness.confidence * 100.0
            );
        }
    }

    // Face matching example
    println!("\n🔐 Face matching example:");
    let emb1 = vec![0.1; 128];
    let emb2 = vec![0.1; 128];
    let emb3 = vec![0.9; 128];

    let similarity_same = face::compare_faces(&emb1, &emb2);
    let similarity_diff = face::compare_faces(&emb1, &emb3);

    println!("   Same person similarity: {:.3}", similarity_same);
    println!("   Different person similarity: {:.3}", similarity_diff);

    println!("\n✨ Face recognition complete!");

    Ok(())
}
