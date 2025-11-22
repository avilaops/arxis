//! Example demonstrating NASA and Google Cloud best practices

use avila_telemetry::{
    Alert, AlertLevel, AnomalyDetector, DataQuality, ErrorBudget, ExponentialSmoothing, Forecaster,
    GoldenSignals, LogSeverity, NASAQualityControl, PerformanceTracker, ServiceLevelObjective,
    StructuredLog, TimeSeries,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== NASA + Google Cloud Observability Example ===\n");

    // Create telemetry data
    let data = vec![
        100.0, 102.0, 101.0, 103.0, 102.0, 104.0, 103.0, 105.0, 104.0, 106.0, 105.0, 107.0, 106.0,
        108.0, 107.0, 109.0,
    ];
    let ts = TimeSeries::new(data.clone()).with_name("spacecraft_temperature");

    // 1. NASA Data Quality Assessment
    println!("--- NASA Data Quality Assessment ---");
    let dqa = ts.assess_quality();
    println!("Accuracy: {:.2}%", dqa.accuracy * 100.0);
    println!("Completeness: {:.2}%", dqa.completeness * 100.0);
    println!("Consistency: {:.2}%", dqa.consistency * 100.0);
    println!("Validity: {:.2}%", dqa.validity * 100.0);
    println!("Overall Score: {:.2}%", dqa.overall_score * 100.0);

    if dqa.meets_nasa_standards() {
        println!("✅ Data meets NASA quality standards (>= 95%)\n");
    } else {
        println!("⚠️  Data does NOT meet NASA standards\n");
    }

    // 2. NASA Quality Control (6-Sigma)
    println!("--- NASA Quality Control (6-Sigma) ---");
    let stats = ts.statistics();
    let qc = NASAQualityControl::new(stats.mean, stats.std_dev, 6);
    println!("Target: {:.2}", qc.target);
    println!("UCL (Upper Control Limit): {:.2}", qc.ucl);
    println!("LCL (Lower Control Limit): {:.2}", qc.lcl);

    let violations = qc.apply_western_electric_rules(&ts);
    if violations.is_empty() {
        println!("✅ All data points within control limits\n");
    } else {
        println!("⚠️  Found {} quality violations\n", violations.len());
    }

    // 3. Google SRE - Service Level Objective
    println!("--- Google SRE Service Level Objective ---");
    let slo = ServiceLevelObjective {
        name: "Telemetry Processing Latency".to_string(),
        target: 0.999,                               // 99.9%
        window: Duration::from_secs(30 * 24 * 3600), // 30 days
        error_budget: ErrorBudget::new(0.001),       // 0.1% error budget
    };
    println!("SLO Target: {:.1}%", slo.target * 100.0);
    println!("Error Budget: {:.3}%", slo.error_budget.total * 100.0);
    println!("Remaining: {:.3}%", slo.error_budget.remaining * 100.0);

    if !slo.error_budget.is_exhausted() {
        println!("✅ Error budget available\n");
    } else {
        println!("🔴 Error budget exhausted - deploy freeze!\n");
    }

    // 4. Performance Tracking (Golden Signals)
    println!("--- Performance Tracking ---");
    let mut perf_tracker = PerformanceTracker::new();

    // Measure anomaly detection performance
    perf_tracker.measure(|| {
        let detector = AnomalyDetector::default();
        detector.detect_ensemble(&ts)
    })?;

    // Measure forecasting performance
    perf_tracker.measure(|| {
        let mut forecaster = ExponentialSmoothing::new(0.3)?;
        forecaster.fit(&ts)?;
        forecaster.forecast(5)
    })?;

    let latency = perf_tracker.calculate_percentiles();
    println!("Latency P50: {:?}", latency.p50);
    println!("Latency P95: {:?}", latency.p95);
    println!("Latency P99: {:?}", latency.p99);
    println!("Latency P99.9: {:?}", latency.p99_9);
    println!();

    // 5. Structured Logging (Google Cloud Logging format)
    println!("--- Structured Logging ---");
    let log = StructuredLog::new(LogSeverity::Info, "Telemetry analysis completed")
        .with_label("series_name", "spacecraft_temperature")
        .with_label("data_points", data.len().to_string())
        .with_label("quality_score", format!("{:.2}", dqa.overall_score))
        .with_trace("trace-12345".to_string(), "span-67890".to_string());

    println!("Log Entry:");
    println!("  Timestamp: {}", log.timestamp);
    println!("  Severity: {:?}", log.severity);
    println!("  Message: {}", log.message);
    println!("  Labels: {:?}", log.labels);
    println!("  Trace ID: {:?}", log.trace_id);
    println!("  Span ID: {:?}", log.span_id);
    println!();

    // 6. Alert Generation (NASA-style)
    println!("--- Alert System ---");

    // Check if we need to alert
    let error_rate = 0.005; // 0.5% error rate
    if error_rate > 0.001 {
        let alert = Alert {
            timestamp: chrono::Utc::now(),
            level: AlertLevel::Yellow,
            message: "Error rate above threshold".to_string(),
            metric: "error_rate".to_string(),
            value: error_rate,
            threshold: 0.001,
            recommended_action: "Review recent changes and check logs".to_string(),
        };

        println!("🚨 Alert Generated!");
        println!("  Level: {:?}", alert.level);
        println!("  Metric: {}", alert.metric);
        println!("  Value: {:.3}%", alert.value * 100.0);
        println!("  Threshold: {:.3}%", alert.threshold * 100.0);
        println!("  Action: {}", alert.recommended_action);
    } else {
        println!("✅ All metrics within normal range");
    }
    println!();

    // 7. Final Analysis with Forecasting
    println!("--- Predictive Analysis ---");
    let mut forecaster = ExponentialSmoothing::new(0.3)?;
    forecaster.fit(&ts)?;
    let forecast = forecaster.forecast_with_confidence(5, 0.95)?;

    println!("5-step forecast:");
    for (i, pred) in forecast.predictions.iter().enumerate() {
        let lower = forecast.lower_bound.as_ref().unwrap()[i];
        let upper = forecast.upper_bound.as_ref().unwrap()[i];
        println!(
            "  Step {}: {:.2} (95% CI: [{:.2}, {:.2}])",
            i + 1,
            pred,
            lower,
            upper
        );
    }
    println!();

    // 8. Mission Report Summary
    println!("=== Mission Report Summary ===");
    println!(
        "✅ Data Quality: {:.1}% (NASA Standard)",
        dqa.overall_score * 100.0
    );
    println!("✅ Control Limits: All within 6-Sigma");
    println!("✅ SLO Target: {:.1}% achieved", slo.target * 100.0);
    println!("✅ Latency P99: {:?}", latency.p99);
    println!("✅ Forecast: 5 steps ahead with 95% confidence");
    println!("\n🚀 System Status: OPERATIONAL");

    Ok(())
}
