use avila_telemetry::{ExponentialSmoothing, Forecaster, MovingAverageForecaster, TimeSeries};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Forecasting Example ===\n");

    // Create historical data (simulating monthly sales)
    let historical_data = vec![
        100.0, 120.0, 115.0, 130.0, 125.0, 140.0, 135.0, 150.0, 145.0, 160.0, 155.0, 170.0,
    ];

    let ts = TimeSeries::new(historical_data).with_name("monthly_sales");
    println!("Historical data: {:?}\n", ts.values);

    // Exponential Smoothing
    println!("--- Exponential Smoothing ---");
    let mut es_model = ExponentialSmoothing::new(0.3)?;
    es_model.fit(&ts)?;

    let es_forecast = es_model.forecast(6)?;
    println!("6-month forecast: {:?}", es_forecast.predictions);
    println!("Confidence: {:.2}\n", es_forecast.confidence);

    // With confidence intervals
    let es_forecast_ci = es_model.forecast_with_confidence(6, 0.95)?;
    println!("With 95% confidence intervals:");
    for (i, ((pred, lower), upper)) in es_forecast_ci
        .predictions
        .iter()
        .zip(es_forecast_ci.lower_bound.as_ref().unwrap())
        .zip(es_forecast_ci.upper_bound.as_ref().unwrap())
        .enumerate()
    {
        println!(
            "  Month {}: {:.2} (CI: [{:.2}, {:.2}])",
            i + 1,
            pred,
            lower,
            upper
        );
    }

    // Moving Average Forecaster
    println!("\n--- Moving Average Forecaster ---");
    let mut ma_model = MovingAverageForecaster::new(3)?;
    ma_model.fit(&ts)?;

    let ma_forecast = ma_model.forecast(6)?;
    println!("6-month forecast: {:?}", ma_forecast.predictions);
    println!("Confidence: {:.2}", ma_forecast.confidence);

    Ok(())
}
