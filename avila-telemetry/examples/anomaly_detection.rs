use avila_telemetry::{AnomalyDetector, TimeSeries};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Anomaly Detection Example ===\n");

    // Create a time series with an obvious anomaly
    let data = vec![
        10.0, 12.0, 11.0, 13.0, 12.0, 11.0, 10.0, 100.0, // <- Anomaly
        11.0, 12.0, 10.0, 13.0, 11.0, 12.0,
    ];

    let ts = TimeSeries::new(data).with_name("sensor_readings");
    println!("Time series: {:?}\n", ts.values);

    // Create anomaly detector
    let detector = AnomalyDetector::default();

    // Detect using Z-score method
    println!("--- Z-Score Detection ---");
    let zscore_anomalies = detector.detect_zscore(&ts)?;
    for anomaly in &zscore_anomalies {
        println!(
            "Index: {}, Value: {:.2}, Score: {:.2}",
            anomaly.index, anomaly.value, anomaly.score
        );
    }

    // Detect using IQR method
    println!("\n--- IQR Detection ---");
    let iqr_anomalies = detector.detect_iqr(&ts)?;
    for anomaly in &iqr_anomalies {
        println!(
            "Index: {}, Value: {:.2}, Score: {:.2}",
            anomaly.index, anomaly.value, anomaly.score
        );
    }

    // Ensemble detection
    println!("\n--- Ensemble Detection ---");
    let ensemble_anomalies = detector.detect_ensemble(&ts)?;
    for anomaly in &ensemble_anomalies {
        println!(
            "Index: {}, Value: {:.2}, Score: {:.2}",
            anomaly.index, anomaly.value, anomaly.score
        );
    }

    Ok(())
}
