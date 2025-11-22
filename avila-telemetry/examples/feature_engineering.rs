use avila_telemetry::{FeatureExtractor, TimeSeries};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Feature Engineering Example ===\n");

    // Create a time series
    let data: Vec<f64> = (1..=20)
        .map(|x| x as f64 + (x as f64 / 5.0).sin() * 3.0)
        .collect();
    let ts = TimeSeries::new(data.clone());

    println!("Original data (first 10): {:?}\n", &ts.values[..10]);

    // Create lag features
    println!("--- Lag Features ---");
    let lags = FeatureExtractor::create_lag_features(&ts, &[1, 2, 3])?;
    println!("Lag 1 (first 10): {:?}", &lags[0][..10]);
    println!("Lag 2 (first 10): {:?}", &lags[1][..10]);
    println!("Lag 3 (first 10): {:?}\n", &lags[2][..10]);

    // Rolling statistics
    println!("--- Rolling Statistics (window=5) ---");
    let stats = FeatureExtractor::rolling_statistics(&ts, 5)?;
    println!("Rolling means (first 10): {:?}", &stats.means[..10]);
    println!("Rolling stds (first 10): {:?}", &stats.stds[..10]);
    println!("Rolling mins (first 10): {:?}", &stats.mins[..10]);
    println!("Rolling maxs (first 10): {:?}\n", &stats.maxs[..10]);

    // Trend features
    println!("--- Trend Features (window=5) ---");
    let trends = FeatureExtractor::trend_features(&ts, 5)?;
    println!("Trends (first 10): {:?}\n", &trends[..10]);

    // Rate of change
    println!("--- Rate of Change (periods=3) ---");
    let roc = FeatureExtractor::rate_of_change(&ts, 3)?;
    println!("ROC (first 10): {:?}", &roc[..10]);

    Ok(())
}
