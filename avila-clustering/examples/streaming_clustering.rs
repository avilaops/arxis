//! Real-Time Streaming Clustering Example
//!
//! Demonstrates online clustering for streaming data with concept drift detection.

use avila_clustering::algorithms::online::{OnlineKMeans, OnlineBIRCH, SlidingWindowClustering};
use ndarray::{array, Array2};
use std::thread;
use std::time::Duration;

fn main() -> avila_clustering::Result<()> {
    println!("=== Real-Time Streaming Clustering Example ===\n");

    // Scenario: IoT sensor monitoring with concept drift
    // Sensors produce readings continuously, patterns may shift over time

    // 1. Online K-Means
    println!("1. Online K-Means for Streaming Data");
    println!("{}", "=".repeat(50));

    let mut online_kmeans = OnlineKMeans::new(3)
        .batch_size(50)
        .learning_rate(0.1);

    // Initial batch
    let initial_data = array![
        [1.0, 2.0],
        [1.5, 2.5],
        [8.0, 9.0],
        [8.5, 9.5],
        [15.0, 16.0],
    ];

    online_kmeans.fit_batch(&initial_data)?;
    println!("✓ Initialized with {} samples", initial_data.nrows());

    // Simulate streaming data
    println!("\nProcessing streaming data:");
    let stream_batches = [
        array![[1.2, 2.2], [1.3, 2.3]],    // Batch 1
        array![[8.2, 9.2], [8.3, 9.3]],    // Batch 2
        array![[15.2, 16.2], [14.8, 15.8]], // Batch 3
    ];

    for (i, batch) in stream_batches.iter().enumerate() {
        println!("  Batch {}: {} points", i + 1, batch.nrows());

        for row_idx in 0..batch.nrows() {
            let point = batch.row(row_idx);
            let cluster = online_kmeans.partial_fit(point)?;
            println!("    Point [{:.1}, {:.1}] -> Cluster {}",
                     point[0], point[1], cluster);
        }

        // Simulate time delay
        thread::sleep(Duration::from_millis(100));
    }

    online_kmeans.flush()?;

    if let Some(centroids) = online_kmeans.centroids() {
        println!("\nFinal Centroids:");
        for (i, centroid) in centroids.outer_iter().enumerate() {
            println!("  Cluster {}: [{:.2}, {:.2}]", i, centroid[0], centroid[1]);
        }
    }

    // 2. Online BIRCH (memory-efficient)
    println!("\n2. Online BIRCH for High-Volume Streams");
    println!("{}", "=".repeat(50));

    let mut birch = OnlineBIRCH::new(1.5, 10);

    // Simulate high-frequency sensor data
    println!("Processing 20 data points...");
    for i in 0..20 {
        let x = (i as f64 % 10.0) + (rand::random::<f64>() * 0.5);
        let y = (i as f64 % 10.0) + (rand::random::<f64>() * 0.5);

        let cluster = birch.partial_fit(array![x, y].view())?;

        if i % 5 == 0 {
            println!("  Step {}: {} clusters active", i, birch.n_clusters());
        }
    }

    println!("\nFinal state:");
    println!("  Total clusters: {}", birch.n_clusters());
    println!("  Centroids shape: {:?}", birch.centroids().dim());

    // 3. Sliding Window with Concept Drift Detection
    println!("\n3. Concept Drift Detection");
    println!("{}", "=".repeat(50));

    let mut sliding_window = SlidingWindowClustering::new(2, 100, 20);

    // Phase 1: Initial pattern
    println!("\nPhase 1: Normal operating conditions");
    let phase1 = Array2::from_shape_fn((50, 2), |(i, j)| {
        if j == 0 { i as f64 % 10.0 } else { i as f64 % 10.0 + 1.0 }
    });

    let info1 = sliding_window.update(&phase1)?;
    println!("  Processed {} points", info1.labels.len());
    println!("  Drift detected: {}", info1.drift_detected);

    // Phase 2: Concept drift (pattern shifts)
    println!("\nPhase 2: Operating conditions shift");
    let phase2 = Array2::from_shape_fn((50, 2), |(i, j)| {
        if j == 0 { i as f64 % 10.0 + 5.0 } else { i as f64 % 10.0 + 6.0 }
    });

    let info2 = sliding_window.update(&phase2)?;
    println!("  Processed {} points", info2.labels.len());
    println!("  Drift detected: {} {}",
             info2.drift_detected,
             if info2.drift_detected { "⚠️ ALERT" } else { "" });

    // Phase 3: Return to normal
    println!("\nPhase 3: Back to normal");
    let phase3 = Array2::from_shape_fn((50, 2), |(i, j)| {
        if j == 0 { i as f64 % 10.0 } else { i as f64 % 10.0 + 1.0 }
    });

    let info3 = sliding_window.update(&phase3)?;
    println!("  Processed {} points", info3.labels.len());
    println!("  Drift detected: {} {}",
             info3.drift_detected,
             if info3.drift_detected { "⚠️ ALERT" } else { "" });

    // 4. Performance metrics
    println!("\n=== Performance Characteristics ===");
    println!("Online K-Means:");
    println!("  • Memory: O(k × d) - stores only centroids");
    println!("  • Update time: O(d) per point");
    println!("  • Best for: Balanced clusters, known k");

    println!("\nOnline BIRCH:");
    println!("  • Memory: O(B × d) - B = max clusters");
    println!("  • Update time: O(B × d) per point");
    println!("  • Best for: Unknown cluster count, hierarchical structure");

    println!("\nSliding Window:");
    println!("  • Memory: O(W × d) - W = window size");
    println!("  • Drift detection: Pattern change monitoring");
    println!("  • Best for: Non-stationary data, concept drift");

    println!("\n=== Use Cases ===");
    println!("• IoT Monitoring: Sensor streams, equipment health");
    println!("• Network Security: Intrusion detection, traffic analysis");
    println!("• Financial: Real-time fraud detection, trading signals");
    println!("• Manufacturing: Quality control, predictive maintenance");
    println!("• Web Analytics: User behavior, A/B testing");
    println!("• Healthcare: Patient monitoring, early warning systems");

    println!("\n=== When to Use Online Clustering ===");
    println!("✓ Data arrives continuously (streams)");
    println!("✓ Dataset too large for memory");
    println!("✓ Patterns evolve over time");
    println!("✓ Real-time decisions required");
    println!("✓ Concept drift is expected");

    Ok(())
}
