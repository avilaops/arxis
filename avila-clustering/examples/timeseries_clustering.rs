//! Advanced Clustering Example - Time Series Analysis
//!
//! This example demonstrates clustering time series data like stock prices,
//! sensor readings, or heartbeat signals using DTW distance.

use avila_clustering::algorithms::timeseries::{TimeSeriesKMeans, TimeSeriesMetric, MotifDiscovery};
use ndarray::array;

fn main() -> avila_clustering::Result<()> {
    println!("=== Time Series Clustering Example ===\n");

    // Simulated heartbeat signals (ECG-like patterns)
    // Each row is a time series
    let heartbeats = array![
        // Normal heartbeat pattern
        [0.0, 0.1, 0.3, 0.8, 1.0, 0.7, 0.3, 0.1, 0.0, 0.0],
        [0.0, 0.1, 0.3, 0.9, 1.0, 0.6, 0.2, 0.1, 0.0, 0.0],
        [0.0, 0.1, 0.4, 0.8, 1.0, 0.7, 0.3, 0.1, 0.0, 0.0],

        // Abnormal pattern 1 (arrhythmia)
        [0.0, 0.2, 0.5, 0.4, 0.6, 0.9, 1.0, 0.5, 0.1, 0.0],
        [0.0, 0.2, 0.4, 0.5, 0.6, 0.8, 1.0, 0.4, 0.1, 0.0],

        // Abnormal pattern 2 (tachycardia)
        [0.0, 0.5, 1.0, 0.8, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.6, 1.0, 0.7, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0],
    ];

    println!("Dataset: {} heartbeat signals, {} time points each\n",
             heartbeats.nrows(), heartbeats.ncols());

    // 1. Cluster using Euclidean distance
    println!("1. Clustering with Euclidean Distance");
    println!("{}", "=".repeat(50));

    let euclidean_result = TimeSeriesKMeans::new(3)
        .metric(TimeSeriesMetric::Euclidean)
        .max_iter(50)
        .fit(heartbeats.view())?;

    println!("Labels: {:?}", euclidean_result.labels);
    println!("Centroids shape: {:?}\n", euclidean_result.centroids.dim());

    // 2. Cluster using DTW (better for temporal patterns)
    println!("2. Clustering with Dynamic Time Warping (DTW)");
    println!("{}", "=".repeat(50));

    let dtw_result = TimeSeriesKMeans::new(3)
        .metric(TimeSeriesMetric::DTW)
        .max_iter(30)
        .fit(heartbeats.view())?;

    println!("Labels: {:?}", dtw_result.labels);
    println!("Number of clusters: {}\n", dtw_result.n_clusters);

    // Display clusters
    for cluster_id in 0..3 {
        let members: Vec<usize> = dtw_result.labels
            .iter()
            .enumerate()
            .filter_map(|(i, &label)| if label == cluster_id { Some(i) } else { None })
            .collect();

        if !members.is_empty() {
            println!("Cluster {}: signals {:?}", cluster_id, members);
        }
    }

    // 3. Motif Discovery - Find recurring patterns
    println!("\n3. Motif Discovery");
    println!("{}", "=".repeat(50));

    // Create a longer time series with repeating patterns
    let long_series = array![
        0.0, 0.1, 0.3, 0.8, 1.0, 0.7, 0.3, 0.1, // Pattern 1
        0.5, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0, // Noise
        0.0, 0.1, 0.3, 0.8, 1.0, 0.7, 0.3, 0.1, // Pattern 1 repeats
        0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.5, 0.4, // More noise
        0.0, 0.1, 0.3, 0.9, 1.0, 0.7, 0.3, 0.1, // Pattern 1 again
    ];

    let motif_finder = MotifDiscovery::new(8, 2) // Find 2 motifs of length 8
        .threshold(0.5);

    let motifs = motif_finder.find_motifs(long_series.view())?;

    println!("Found {} motifs:", motifs.len());
    for motif in &motifs {
        println!("  Motif with {} occurrences at positions: {:?}",
                 motif.frequency, motif.occurrences);
    }

    // 4. Shape-Based Distance (SBD)
    println!("\n4. Clustering with Shape-Based Distance");
    println!("{}", "=".repeat(50));

    let sbd_result = TimeSeriesKMeans::new(3)
        .metric(TimeSeriesMetric::SBD)
        .max_iter(30)
        .fit(heartbeats.view())?;

    println!("Labels: {:?}", sbd_result.labels);

    // Compare methods
    println!("\n=== Method Comparison ===");
    println!("{:<20} {}", "Method", "Cluster Assignments");
    println!("{}", "-".repeat(50));
    println!("{:<20} {:?}", "Euclidean", euclidean_result.labels);
    println!("{:<20} {:?}", "DTW", dtw_result.labels);
    println!("{:<20} {:?}", "Shape-Based", sbd_result.labels);

    println!("\n=== Use Cases ===");
    println!("• Medical: ECG classification, patient monitoring");
    println!("• Finance: Stock pattern recognition, trading signals");
    println!("• IoT: Sensor anomaly detection, predictive maintenance");
    println!("• Audio: Voice pattern matching, music genre classification");
    println!("• Weather: Climate pattern analysis, forecast improvement");

    println!("\n=== DTW Advantages ===");
    println!("✓ Handles temporal misalignment");
    println!("✓ Works with different sequence lengths");
    println!("✓ More robust to noise and variations");
    println!("✓ Better captures shape similarity");

    Ok(())
}
