use avila_telemetry::{AnomalyDetector, ExponentialSmoothing, Forecaster, TimeSeries};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_moving_average(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|x| x as f64).collect();
    let ts = TimeSeries::new(data);

    c.bench_function("moving_average_1000", |b| {
        b.iter(|| ts.moving_average(black_box(10)))
    });
}

fn bench_exponential_moving_average(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|x| x as f64).collect();
    let ts = TimeSeries::new(data);

    c.bench_function("exponential_moving_average_1000", |b| {
        b.iter(|| ts.exponential_moving_average(black_box(0.3)))
    });
}

fn bench_anomaly_detection(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000)
        .map(|x| x as f64 + (x as f64 / 10.0).sin() * 5.0)
        .collect();
    let ts = TimeSeries::new(data);
    let detector = AnomalyDetector::default();

    c.bench_function("zscore_detection_1000", |b| {
        b.iter(|| detector.detect_zscore(black_box(&ts)))
    });

    c.bench_function("iqr_detection_1000", |b| {
        b.iter(|| detector.detect_iqr(black_box(&ts)))
    });
}

fn bench_forecasting(c: &mut Criterion) {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let ts = TimeSeries::new(data);

    c.bench_function("exponential_smoothing_forecast", |b| {
        b.iter(|| {
            let mut model = ExponentialSmoothing::new(0.3).unwrap();
            model.fit(&ts).unwrap();
            model.forecast(black_box(10))
        })
    });
}

criterion_group!(
    benches,
    bench_moving_average,
    bench_exponential_moving_average,
    bench_anomaly_detection,
    bench_forecasting
);
criterion_main!(benches);
