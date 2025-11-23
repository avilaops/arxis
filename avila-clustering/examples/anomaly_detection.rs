//! Anomaly Detection Example
//!
//! This example shows how to use DBSCAN for anomaly/outlier detection
//! in network traffic or sensor data.

use avila_clustering::algorithms::dbscan::DBSCANBuilder;
use ndarray::{array, Array2};

fn main() -> avila_clustering::Result<()> {
    println!("=== Anomaly Detection Example ===\n");

    // Simulated network traffic data: [packets/sec, bytes/sec, connection_count]
    // Normal traffic pattern around (100, 5000, 10)
    let normal_traffic = array![
        [98.0, 4900.0, 9.0],
        [102.0, 5100.0, 11.0],
        [100.0, 5000.0, 10.0],
        [99.0, 4950.0, 10.0],
        [101.0, 5050.0, 11.0],
        [97.0, 4850.0, 9.0],
        [103.0, 5150.0, 12.0],
        [100.0, 5000.0, 10.0],
        [98.0, 4900.0, 9.0],
        [102.0, 5100.0, 11.0],
    ];

    // Anomalous traffic patterns
    let anomalies = array![
        [500.0, 25000.0, 50.0],  // DDoS attack
        [10.0, 500.0, 1.0],       // System failure
        [200.0, 50000.0, 100.0],  // Port scan
    ];

    // Combine normal and anomalous data
    let n_normal = normal_traffic.nrows();
    let n_anomaly = anomalies.nrows();
    let n_total = n_normal + n_anomaly;

    let mut traffic_data = Array2::<f64>::zeros((n_total, 3));
    for i in 0..n_normal {
        for j in 0..3 {
            traffic_data[[i, j]] = normal_traffic[[i, j]];
        }
    }
    for i in 0..n_anomaly {
        for j in 0..3 {
            traffic_data[[n_normal + i, j]] = anomalies[[i, j]];
        }
    }

    println!("Dataset: {} traffic samples", n_total);
    println!("  - Normal traffic: {}", n_normal);
    println!("  - Known anomalies: {}\n", n_anomaly);

    // Use DBSCAN for anomaly detection
    // Points marked as noise (label -1) are potential anomalies
    println!("Running DBSCAN for anomaly detection...");
    println!("{}", "=".repeat(50));

    let dbscan = DBSCANBuilder::new(50.0, 3)
        .fit(traffic_data.view())?;

    println!("Clusters found: {}", dbscan.n_clusters());

    // Identify anomalies (noise points)
    let anomaly_indices: Vec<usize> = dbscan
        .labels()
        .iter()
        .enumerate()
        .filter_map(|(idx, &label)| if label == -1 { Some(idx) } else { None })
        .collect();

    println!("Anomalies detected: {}", anomaly_indices.len());
    println!("\nDetailed results:");
    println!("{}", "-".repeat(50));

    for (idx, &label) in dbscan.labels().iter().enumerate() {
        let sample = traffic_data.row(idx);
        let status = if label == -1 { "ANOMALY" } else { "NORMAL" };
        let true_label = if idx >= n_normal { "(actual anomaly)" } else { "(actual normal)" };

        println!(
            "Sample {:2}: [{:6.0}, {:7.0}, {:4.0}] -> {} {}",
            idx,
            sample[0],
            sample[1],
            sample[2],
            status,
            true_label
        );
    }

    // Calculate detection accuracy
    let true_positives = anomaly_indices.iter().filter(|&&idx| idx >= n_normal).count();
    let false_positives = anomaly_indices.iter().filter(|&&idx| idx < n_normal).count();
    let false_negatives = (0..n_anomaly).filter(|&i| dbscan.labels()[n_normal + i] != -1).count();

    println!("\n=== Detection Performance ===");
    println!("True Positives: {}/{}", true_positives, n_anomaly);
    println!("False Positives: {}", false_positives);
    println!("False Negatives: {}", false_negatives);

    if n_anomaly > 0 {
        let recall = true_positives as f64 / n_anomaly as f64;
        println!("Recall: {:.2}%", recall * 100.0);
    }

    println!("\n=== Use Cases ===");
    println!("• Network Security: Detect DDoS, intrusions, port scans");
    println!("• IoT Monitoring: Identify sensor failures, unusual readings");
    println!("• Fraud Detection: Credit card fraud, insurance claims");
    println!("• Manufacturing: Equipment failure prediction, quality control");
    println!("• Healthcare: Patient monitoring, disease outbreak detection");

    println!("\n=== DBSCAN Parameters ===");
    println!("• eps (50.0): Maximum distance between points in same cluster");
    println!("• min_samples (3): Minimum points needed to form dense region");
    println!("• Points with < min_samples neighbors are marked as noise");

    Ok(())
}
