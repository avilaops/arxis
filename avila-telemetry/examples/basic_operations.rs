use avila_telemetry::TimeSeries;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Time Series Operations ===\n");

    // Create a simple time series
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 2.0];
    let ts = TimeSeries::new(data).with_name("example_series");

    println!("Series: {:?}", ts.values);
    println!("Length: {}\n", ts.len());

    // Basic statistics
    println!("--- Statistics ---");
    let stats = ts.statistics();
    println!("Mean: {:.2}", stats.mean);
    println!("Median: {:.2}", stats.median);
    println!("Std Dev: {:.2}", stats.std_dev);
    println!("Min: {:.2}", stats.min);
    println!("Max: {:.2}\n", stats.max);

    // Moving average
    println!("--- Moving Average (window=3) ---");
    let ma = ts.moving_average(3)?;
    println!("MA: {:?}\n", ma);

    // Exponential moving average
    println!("--- Exponential Moving Average (alpha=0.5) ---");
    let ema = ts.exponential_moving_average(0.5)?;
    println!("EMA: {:?}\n", ema);

    // First difference
    println!("--- First Difference ---");
    let diff = ts.diff();
    println!("Diff: {:?}\n", diff);

    // Percentage change
    println!("--- Percentage Change ---");
    let pct = ts.pct_change();
    println!("Pct Change: {:?}", pct);

    Ok(())
}
