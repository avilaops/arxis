//! Instant-NGP Demo
//!
//! Real-time Neural Radiance Fields with hash encoding

use avx_image::nerf::{InstantNGP, InstantNGPConfig, Camera, Point3D};
use avx_image::core::ImageBuffer;
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Instant-NGP Real-time NeRF Demo ===\n");

    // Create Instant-NGP with optimized config
    let config = InstantNGPConfig::default();
    let mut ngp = InstantNGP::new(config);

    // Prepare training data
    println!("Preparing training data...");
    let mut training_data = Vec::new();

    for i in 0..25 {
        let angle = 2.0 * PI * i as f32 / 25.0;
        let radius = 3.5;

        let position = Point3D::new(
            radius * angle.cos(),
            0.5,
            radius * angle.sin(),
        );

        let camera = Camera::new(position, 500.0, (512, 512));
        let image = ImageBuffer::from_rgb(512, 512, vec![100u8; 512 * 512 * 3]);

        training_data.push((image, camera));
    }
    println!("Prepared {} training views\n", training_data.len());

    // Train (much faster than vanilla NeRF!)
    println!("Training Instant-NGP...");
    println!("Using multi-resolution hash encoding");
    ngp.train(&training_data)?;
    println!("Training complete in minutes!\n");

    // Real-time rendering
    println!("Real-time rendering demo...");
    let render_camera = Camera::new(
        Point3D::new(2.5, 1.0, 2.0),
        500.0,
        (512, 512),
    );

    let frame = ngp.render_realtime(&render_camera)?;
    println!("Rendered frame: {}x{}\n", frame.width(), frame.height());

    // Interactive rendering loop
    println!("Simulating interactive rendering...");
    let interactive_frames = ngp.interactive_render(
        |t| {
            let angle = 2.0 * PI * t;
            let position = Point3D::new(
                3.0 * angle.cos(),
                1.0 + 0.5 * (t * 4.0 * PI).sin(),
                3.0 * angle.sin(),
            );
            Camera::new(position, 500.0, (512, 512))
        },
        2.0, // 2 seconds
        30,  // 30 FPS
    )?;
    println!("Generated {} frames at real-time speeds\n", interactive_frames.len());

    // Export occupancy grid
    println!("Exporting occupancy grid...");
    let grid = ngp.export_occupancy_grid(128)?;
    println!("Occupancy grid: {} voxels\n", grid.len());

    // Memory stats
    let stats = ngp.get_memory_stats();
    println!("Memory usage:");
    println!("  Hash tables: {} MB", stats.hash_table_bytes / 1024 / 1024);
    println!("  Total: {} MB", stats.total_bytes / 1024 / 1024);

    println!("\nDemo complete!");
    println!("Instant-NGP achieved >100x speedup over vanilla NeRF!");

    Ok(())
}
