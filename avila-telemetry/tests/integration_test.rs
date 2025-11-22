//! Integration tests for avila-telemetry

use avila_telemetry::{
    AnomalyDetector, Decomposer, DecompositionType, ExponentialSmoothing, FeatureExtractor,
    Forecaster, TimeSeries,
};

#[test]
fn test_end_to_end_analysis() {
    // Create synthetic data
    let data: Vec<f64> = (0..100)
        .map(|x| x as f64 + (x as f64 / 5.0).sin() * 10.0)
        .collect();

    let ts = TimeSeries::new(data);

    // Test statistics
    let stats = ts.statistics();
    assert!(stats.mean > 0.0);
    assert!(stats.std_dev > 0.0);

    // Test moving average
    let ma = ts.moving_average(5).unwrap();
    assert_eq!(ma.len(), 96);

    // Test anomaly detection
    let detector = AnomalyDetector::default();
    let anomalies = detector.detect_zscore(&ts);
    assert!(anomalies.is_ok());

    // Test forecasting
    let mut forecaster = ExponentialSmoothing::new(0.3).unwrap();
    forecaster.fit(&ts).unwrap();
    let forecast = forecaster.forecast(10).unwrap();
    assert_eq!(forecast.predictions.len(), 10);
}

#[test]
fn test_feature_engineering_pipeline() {
    let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
    let ts = TimeSeries::new(data);

    // Lag features
    let lags = FeatureExtractor::create_lag_features(&ts, &[1, 2, 3]).unwrap();
    assert_eq!(lags.len(), 3);
    assert_eq!(lags[0].len(), 49);

    // Rolling statistics
    let stats = FeatureExtractor::rolling_statistics(&ts, 5).unwrap();
    assert_eq!(stats.means.len(), 46);

    // Trend features
    let trends = FeatureExtractor::trend_features(&ts, 5).unwrap();
    assert_eq!(trends.len(), 46);

    // Rate of change
    let roc = FeatureExtractor::rate_of_change(&ts, 3).unwrap();
    assert_eq!(roc.len(), 47);
}

#[test]
fn test_decomposition_workflow() {
    // Create seasonal data
    let mut data = Vec::new();
    for i in 0..40 {
        let trend = i as f64 * 0.5;
        let seasonal = 10.0 * (i as f64 * std::f64::consts::PI / 4.0).sin();
        data.push(trend + seasonal + 50.0);
    }

    let ts = TimeSeries::new(data);
    let decomposer = Decomposer::new(DecompositionType::Additive, 8).unwrap();

    let result = decomposer.decompose(&ts).unwrap();

    assert_eq!(result.trend.len(), ts.len());
    assert_eq!(result.seasonal.len(), ts.len());
    assert_eq!(result.residual.len(), ts.len());
}

#[test]
fn test_anomaly_detection_accuracy() {
    // Create data with known anomaly
    let mut data: Vec<f64> = (0..50).map(|x| x as f64).collect();
    data[25] = 1000.0; // Clear anomaly

    let ts = TimeSeries::new(data);
    let detector = AnomalyDetector::default();

    let anomalies = detector.detect_zscore(&ts).unwrap();

    // Should detect the anomaly at index 25
    assert!(!anomalies.is_empty());
    assert!(anomalies.iter().any(|a| a.index == 25));
}

#[test]
fn test_forecast_with_confidence_intervals() {
    let data: Vec<f64> = (1..=30).map(|x| x as f64).collect();
    let ts = TimeSeries::new(data);

    let mut forecaster = ExponentialSmoothing::new(0.5).unwrap();
    forecaster.fit(&ts).unwrap();

    let forecast = forecaster.forecast_with_confidence(5, 0.95).unwrap();

    assert_eq!(forecast.predictions.len(), 5);
    assert!(forecast.lower_bound.is_some());
    assert!(forecast.upper_bound.is_some());

    let lower = forecast.lower_bound.unwrap();
    let upper = forecast.upper_bound.unwrap();

    // Confidence intervals should be wider than point estimates
    for i in 0..5 {
        assert!(lower[i] < forecast.predictions[i]);
        assert!(upper[i] > forecast.predictions[i]);
    }
}
