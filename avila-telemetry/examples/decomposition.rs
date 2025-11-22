use avila_telemetry::{Decomposer, DecompositionType, TimeSeries};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Time Series Decomposition Example ===\n");

    // Create a seasonal time series: trend + seasonal + noise
    let mut data = Vec::new();
    for i in 0..40 {
        let trend = i as f64 * 0.5;
        let seasonal = 10.0 * (i as f64 * std::f64::consts::PI / 4.0).sin();
        let noise = (i as f64 * 0.7).sin() * 2.0;
        data.push(trend + seasonal + noise + 50.0);
    }

    let ts = TimeSeries::new(data).with_name("seasonal_sales");
    println!("Original series (first 12): {:?}\n", &ts.values[..12]);

    // Additive decomposition
    println!("--- Additive Decomposition (period=8) ---");
    let decomposer = Decomposer::new(DecompositionType::Additive, 8)?;
    let result = decomposer.decompose(&ts)?;

    println!("Trend (first 12):");
    for (i, &val) in result.trend[..12].iter().enumerate() {
        println!("  {}: {:.2}", i, val);
    }

    println!("\nSeasonal pattern (one cycle):");
    for (i, &val) in result.seasonal[..8].iter().enumerate() {
        println!("  {}: {:.2}", i, val);
    }

    println!("\nResiduals (first 12):");
    for (i, &val) in result.residual[..12].iter().enumerate() {
        println!("  {}: {:.2}", i, val);
    }

    // Calculate reconstruction error
    println!("\n--- Reconstruction ---");
    let mut reconstruction_error = 0.0;
    for i in 0..ts.len() {
        if !result.trend[i].is_nan() {
            let reconstructed = result.trend[i] + result.seasonal[i] + result.residual[i];
            let error = (ts.values[i] - reconstructed).abs();
            reconstruction_error += error;
        }
    }
    println!(
        "Average reconstruction error: {:.6}",
        reconstruction_error / ts.len() as f64
    );

    Ok(())
}
