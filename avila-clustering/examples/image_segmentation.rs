//! Image Segmentation Example
//!
//! This example demonstrates color-based image segmentation using K-Means clustering.
//! Each pixel is treated as a point in RGB color space.

use avila_clustering::algorithms::kmeans::KMeansBuilder;
use ndarray::{Array2, Array3};

fn main() -> avila_clustering::Result<()> {
    println!("=== Image Segmentation Example ===\n");

    // Simulate a small 10x10 RGB image (in reality, you'd load from file)
    let (width, height) = (10, 10);
    let n_pixels = width * height;

    // Create synthetic image with 3 color regions
    let mut image_rgb = Array3::<f64>::zeros((height, width, 3));

    // Region 1: Red (top-left)
    for i in 0..5 {
        for j in 0..5 {
            image_rgb[[i, j, 0]] = 200.0 + (rand::random::<f64>() * 20.0);
            image_rgb[[i, j, 1]] = 50.0 + (rand::random::<f64>() * 20.0);
            image_rgb[[i, j, 2]] = 50.0 + (rand::random::<f64>() * 20.0);
        }
    }

    // Region 2: Green (top-right)
    for i in 0..5 {
        for j in 5..10 {
            image_rgb[[i, j, 0]] = 50.0 + (rand::random::<f64>() * 20.0);
            image_rgb[[i, j, 1]] = 200.0 + (rand::random::<f64>() * 20.0);
            image_rgb[[i, j, 2]] = 50.0 + (rand::random::<f64>() * 20.0);
        }
    }

    // Region 3: Blue (bottom)
    for i in 5..10 {
        for j in 0..10 {
            image_rgb[[i, j, 0]] = 50.0 + (rand::random::<f64>() * 20.0);
            image_rgb[[i, j, 1]] = 50.0 + (rand::random::<f64>() * 20.0);
            image_rgb[[i, j, 2]] = 200.0 + (rand::random::<f64>() * 20.0);
        }
    }

    // Reshape image to (n_pixels, 3) for clustering
    let mut pixels = Array2::<f64>::zeros((n_pixels, 3));
    let mut idx = 0;
    for i in 0..height {
        for j in 0..width {
            pixels[[idx, 0]] = image_rgb[[i, j, 0]];
            pixels[[idx, 1]] = image_rgb[[i, j, 1]];
            pixels[[idx, 2]] = image_rgb[[i, j, 2]];
            idx += 1;
        }
    }

    println!("Image size: {}x{} ({} pixels)", width, height, n_pixels);
    println!("Color space: RGB\n");

    // Try different numbers of clusters
    for k in [2, 3, 4, 5] {
        println!("Clustering with k={}", k);
        println!("{}", "=".repeat(40));

        let kmeans = KMeansBuilder::new(k)
            .max_iter(100)
            .n_init(5)
            .fit(pixels.view())?;

        println!("Converged in {} iterations", kmeans.n_iter());
        println!("Final inertia: {:.2}", kmeans.inertia());

        // Show cluster centers (dominant colors)
        println!("Dominant colors (cluster centers):");
        for (i, center) in kmeans.cluster_centers().outer_iter().enumerate() {
            println!(
                "  Color {}: RGB({:.0}, {:.0}, {:.0})",
                i + 1,
                center[0],
                center[1],
                center[2]
            );
        }

        // Count pixels in each cluster
        let mut counts = vec![0; k];
        for &label in kmeans.labels() {
            counts[label as usize] += 1;
        }

        println!("Pixels per cluster:");
        for (i, &count) in counts.iter().enumerate() {
            let percentage = (count as f64 / n_pixels as f64) * 100.0;
            println!("  Cluster {}: {} pixels ({:.1}%)", i + 1, count, percentage);
        }
        println!();
    }

    println!("=== Use Cases ===");
    println!("• Medical imaging: Segment organs, tumors, tissue types");
    println!("• Satellite imagery: Land cover classification, crop monitoring");
    println!("• Document processing: Background removal, text extraction");
    println!("• Computer vision: Object detection preprocessing");
    println!("• Art/Design: Color palette extraction, style transfer");

    Ok(())
}
