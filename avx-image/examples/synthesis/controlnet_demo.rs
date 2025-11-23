//! ControlNet Demo
//!
//! Pose-guided and structure-guided image generation

use avx_image::synthesis::{ControlNet, ControlType, SynthesisConfig};
use avx_image::core::ImageBuffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ControlNet Demo ===\n");

    // Create ControlNet for pose control
    let config = SynthesisConfig::default();
    let mut pose_control = ControlNet::new(config.clone(), ControlType::Pose);
    pose_control.load_model()?;

    // Load or create control image
    let control_image = ImageBuffer::from_rgb(512, 512, vec![0u8; 512 * 512 * 3]);

    // Preprocess image to extract pose
    println!("Extracting pose from control image...");
    let pose_map = pose_control.preprocess(&control_image)?;
    println!("Pose map extracted\n");

    // Generate with pose control
    let prompt = "A professional dancer in elegant attire, studio lighting";
    println!("Generating with pose control: '{}'", prompt);
    let result = pose_control.generate(prompt, &pose_map, 1.0)?;
    println!("Generated pose-controlled image\n");

    // Try different control types
    println!("Trying Canny edge control...");
    let mut canny_control = ControlNet::new(config.clone(), ControlType::Canny);
    canny_control.load_model()?;

    let edge_map = canny_control.preprocess(&control_image)?;
    let edge_result = canny_control.generate(
        "A detailed architectural drawing",
        &edge_map,
        0.8,
    )?;
    println!("Generated edge-controlled image\n");

    // Depth control
    println!("Trying depth control...");
    let mut depth_control = ControlNet::new(config, ControlType::Depth);
    depth_control.load_model()?;

    let depth_map = depth_control.preprocess(&control_image)?;
    let depth_result = depth_control.generate(
        "A cinematic scene with dramatic depth of field",
        &depth_map,
        1.2,
    )?;
    println!("Generated depth-controlled image\n");

    println!("Demo complete!");

    Ok(())
}
