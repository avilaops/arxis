//! Text-to-Video Generation Demo
//!
//! Generate video sequences from text descriptions

use avx_image::video_gen::{TextToVideo, VideoConfig, CameraMotion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Text-to-Video Generation Demo ===\n");

    // Configure video generation
    let config = VideoConfig {
        resolution: (512, 512),
        fps: 24.0,
        duration: 3.0,
        guidance_scale: 7.5,
        num_inference_steps: 50,
        motion_strength: 0.5,
    };

    // Create text-to-video generator
    let mut ttv = TextToVideo::new(config);
    ttv.load_model("modelscope/text-to-video")?;

    // Generate simple video
    let prompt = "A golden retriever puppy playing in a garden, sunny day";
    println!("Generating video: '{}'", prompt);

    let video = ttv.generate(prompt)?;
    println!("Generated video:");
    println!("  Frames: {}", video.num_frames());
    println!("  Duration: {:.2}s", video.duration());
    println!("  Resolution: {}x{}\n", video.width, video.height);

    // Generate with camera motion
    println!("Generating with camera motion...");
    let video_with_motion = ttv.generate_with_camera(
        "A futuristic cityscape at night",
        CameraMotion::ZoomIn,
    )?;
    println!("Generated video with zoom effect\n");

    // Long video with multiple segments
    println!("Creating long-form video...");
    let segments = vec![
        "A sunrise over mountains".to_string(),
        "Birds flying in formation".to_string(),
        "A peaceful lake with reflections".to_string(),
    ];

    let long_video = ttv.generate_long_video(&segments, 10)?;
    println!("Generated long video:");
    println!("  Total frames: {}", long_video.num_frames());
    println!("  Duration: {:.2}s\n", long_video.duration());

    // Save video
    println!("Saving video...");
    video.save("output_video.mp4")?;
    println!("Video saved!\n");

    println!("Demo complete!");

    Ok(())
}
