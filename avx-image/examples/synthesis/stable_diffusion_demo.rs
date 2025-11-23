//! Stable Diffusion Demo
//!
//! Generate images from text prompts using Stable Diffusion

use avx_image::synthesis::{StableDiffusion, SynthesisConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Stable Diffusion Demo ===\n");

    // Configure Stable Diffusion
    let config = SynthesisConfig {
        model_path: "runwayml/stable-diffusion-v1-5".to_string(),
        device: "cpu".to_string(),
        num_inference_steps: 50,
        guidance_scale: 7.5,
        seed: Some(42),
    };

    // Create and load model
    let mut sd = StableDiffusion::new(config);
    sd.load_model()?;

    // Generate image from text
    let prompt = "A serene mountain landscape at sunset, photorealistic, 8k";
    println!("Generating: '{}'", prompt);

    let image = sd.generate(prompt, 512, 512)?;
    println!("Generated {}x{} image\n", image.width(), image.height());

    // Generate with negative prompt
    let negative = "blurry, low quality, distorted";
    println!("Generating with negative prompt...");
    let image2 = sd.generate_with_negative(prompt, negative, 512, 512)?;
    println!("Generated with negative prompting\n");

    // Batch generation
    println!("Generating batch of 3 images...");
    let batch = sd.generate_batch(prompt, 512, 512, 3)?;
    println!("Generated {} images in batch\n", batch.len());

    // Image-to-image
    println!("Performing img2img transformation...");
    let transformed = sd.img2img("watercolor painting style", &image, 0.7)?;
    println!("Transformed image created\n");

    println!("Demo complete!");

    Ok(())
}
