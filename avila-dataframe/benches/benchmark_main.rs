use avila_dataframe::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn benchmark_groupby(c: &mut Criterion) {
    let mut group = c.benchmark_group("groupby");

    for size in [1_000, 10_000, 100_000].iter() {
        let df = create_test_dataframe(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| df.group_by(&["category"]).unwrap());
        });
    }

    group.finish();
}

fn benchmark_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter");

    for size in [1_000, 10_000, 100_000].iter() {
        let df = create_test_dataframe(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| df.filter(col("value").gt(lit(50.0))).unwrap());
        });
    }

    group.finish();
}

fn benchmark_aggregations(c: &mut Criterion) {
    let df = create_test_dataframe(100_000);
    let series = df.column("value").unwrap();

    c.bench_function("mean_100k", |b| {
        b.iter(|| black_box(series.mean().unwrap()));
    });

    c.bench_function("std_100k", |b| {
        b.iter(|| black_box(series.std().unwrap()));
    });

    c.bench_function("sum_100k", |b| {
        b.iter(|| black_box(series.sum().unwrap()));
    });
}

fn create_test_dataframe(size: usize) -> DataFrame {
    let values: Vec<f64> = (0..size).map(|i| (i as f64) * 1.5).collect();
    let categories: Vec<f64> = (0..size).map(|i| (i % 10) as f64).collect();

    DataFrame::new(vec![
        Series::new("value", values),
        Series::new("category", categories),
    ])
    .unwrap()
}

criterion_group!(
    benches,
    benchmark_groupby,
    benchmark_filter,
    benchmark_aggregations
);
criterion_main!(benches);
