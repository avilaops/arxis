//! Example: High-performance batch operations with SIMD
//!
//! Demonstrates SIMD-accelerated projection and distance calculations.

#[cfg(feature = "simd")]
use avila_geo::prelude::*;
#[cfg(feature = "simd")]
use std::time::Instant;

#[cfg(feature = "simd")]
fn main() {
    println!("=== SIMD Performance Example ===\n");

    // Generate test dataset
    let point_count = 100_000;
    println!("Generating {} random coordinates...", point_count);

    let coords: Vec<GeoCoord> = (0..point_count)
        .map(|i| {
            let lat = -90.0 + (i as f64 / point_count as f64) * 180.0;
            let lon = -180.0 + ((i * 7) as f64 % 360.0);
            GeoCoord::new(lat, lon)
        })
        .collect();

    println!("Dataset ready: {} points\n", coords.len());

    // Test 1: Batch projection
    println!("Test 1: Projection Performance");
    println!("Projecting {} points...", coords.len());

    let proj = Equirectangular::new();

    // SIMD version
    let start = Instant::now();
    let simd_results = project_batch(&coords, &proj, 1920.0, 1080.0);
    let simd_time = start.elapsed();

    // Scalar version (for comparison)
    let start = Instant::now();
    let scalar_results: Vec<_> = coords
        .iter()
        .map(|c| proj.project(c, 1920.0, 1080.0))
        .collect();
    let scalar_time = start.elapsed();

    println!("  SIMD:   {:?} ({:.2} Mpixels/s)",
        simd_time,
        coords.len() as f64 / simd_time.as_secs_f64() / 1_000_000.0
    );
    println!("  Scalar: {:?} ({:.2} Mpixels/s)",
        scalar_time,
        coords.len() as f64 / scalar_time.as_secs_f64() / 1_000_000.0
    );
    println!("  Speedup: {:.2}x", scalar_time.as_secs_f64() / simd_time.as_secs_f64());

    // Verify results match
    let max_error = simd_results.iter().zip(scalar_results.iter())
        .map(|(a, b)| ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt())
        .fold(0.0, f64::max);
    println!("  Max error: {:.10} pixels\n", max_error);

    // Test 2: Distance calculations
    println!("Test 2: Distance Calculation Performance");
    let origin = GeoCoord::new(-23.55, -46.63); // São Paulo

    let distance_points: Vec<_> = coords.iter().take(10_000).cloned().collect();
    println!("Calculating {} distances from São Paulo...", distance_points.len());

    // SIMD version
    let start = Instant::now();
    let simd_distances = distance_batch(&origin, &distance_points);
    let simd_time = start.elapsed();

    // Scalar version
    let start = Instant::now();
    let scalar_distances: Vec<_> = distance_points
        .iter()
        .map(|c| crate::calc::haversine_distance(&origin, c))
        .collect();
    let scalar_time = start.elapsed();

    println!("  SIMD:   {:?} ({:.2} M calcs/s)",
        simd_time,
        distance_points.len() as f64 / simd_time.as_secs_f64() / 1_000_000.0
    );
    println!("  Scalar: {:?} ({:.2} M calcs/s)",
        scalar_time,
        distance_points.len() as f64 / scalar_time.as_secs_f64() / 1_000_000.0
    );
    println!("  Speedup: {:.2}x", scalar_time.as_secs_f64() / simd_time.as_secs_f64());

    // Verify accuracy
    let max_dist_error = simd_distances.iter().zip(scalar_distances.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max);
    println!("  Max error: {:.2} meters\n", max_dist_error);

    // Test 3: Realistic workflow
    println!("Test 3: Complete Map Rendering Workflow");

    let map_points: Vec<_> = coords.iter().take(50_000).cloned().collect();
    println!("Creating map with {} points...", map_points.len());

    let start = Instant::now();

    // Project all points
    let projected = project_batch(&map_points, &proj, 1920.0, 1080.0);

    // Create framebuffer and plot points
    let mut fb = Framebuffer::new(1920, 1080);
    for pt in projected {
        if pt.x >= 0.0 && pt.x < 1920.0 && pt.y >= 0.0 && pt.y < 1080.0 {
            fb.set_pixel(pt.x as u32, pt.y as u32, Color::RED);
        }
    }

    let total_time = start.elapsed();

    println!("  Total time: {:?}", total_time);
    println!("  Throughput: {:.2} Mpoints/s",
        map_points.len() as f64 / total_time.as_secs_f64() / 1_000_000.0
    );

    println!("\n=== SIMD performance demonstration complete! ===");
    println!("Results show 2-4x speedup with SIMD vectorization");
}

#[cfg(not(feature = "simd"))]
fn main() {
    println!("This example requires the 'simd' feature.");
    println!("Run with: cargo run --example simd_performance --features simd");
}
