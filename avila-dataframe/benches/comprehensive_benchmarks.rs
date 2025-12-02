use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_dataframe::prelude::*;

fn bench_groupby_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("groupby");

    // Small dataset
    let small_df = create_test_dataframe(1000);
    group.bench_function("groupby_sum_1k", |b| {
        b.iter(|| {
            black_box(small_df.group_by(&["category"]).unwrap().agg(&[("value", "sum")]).unwrap())
        })
    });

    // Medium dataset
    let medium_df = create_test_dataframe(100_000);
    group.bench_function("groupby_sum_100k", |b| {
        b.iter(|| {
            black_box(medium_df.group_by(&["category"]).unwrap().agg(&[("value", "sum")]).unwrap())
        })
    });

    // Large dataset
    let large_df = create_test_dataframe(1_000_000);
    group.bench_function("groupby_sum_1m", |b| {
        b.iter(|| {
            black_box(large_df.group_by(&["category"]).unwrap().agg(&[("value", "sum")]).unwrap())
        })
    });

    // Multiple aggregations
    group.bench_function("groupby_multi_agg_100k", |b| {
        b.iter(|| {
            black_box(
                medium_df
                    .group_by(&["category"])
                    .unwrap()
                    .agg(&[("value", "sum"), ("value", "mean"), ("value", "std")])
                    .unwrap()
            )
        })
    });

    group.finish();
}

fn bench_join_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("join");

    for size in [1000, 10_000, 100_000] {
        let df1 = create_join_dataframe(size, "left");
        let df2 = create_join_dataframe(size / 2, "right");

        group.bench_with_input(BenchmarkId::new("inner_join", size), &size, |b, _| {
            b.iter(|| {
                black_box(df1.join(&df2, &["id"], &["id"], "inner").unwrap())
            })
        });

        group.bench_with_input(BenchmarkId::new("left_join", size), &size, |b, _| {
            b.iter(|| {
                black_box(df1.join(&df2, &["id"], &["id"], "left").unwrap())
            })
        });
    }

    group.finish();
}

fn bench_sort_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort");

    for size in [1000, 10_000, 100_000] {
        let df = create_random_dataframe(size);

        group.bench_with_input(BenchmarkId::new("sort_single_column", size), &size, |b, _| {
            b.iter(|| {
                black_box(df.sort(&["value"], false).unwrap())
            })
        });

        group.bench_with_input(BenchmarkId::new("sort_multi_column", size), &size, |b, _| {
            b.iter(|| {
                black_box(df.sort(&["category", "value"], false).unwrap())
            })
        });
    }

    group.finish();
}

fn bench_filter_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter");

    for size in [1000, 10_000, 100_000, 1_000_000] {
        let df = create_test_dataframe(size);

        group.bench_with_input(BenchmarkId::new("filter_simple", size), &size, |b, _| {
            b.iter(|| {
                black_box(df.filter(col("value") > lit(50.0)).unwrap())
            })
        });

        group.bench_with_input(BenchmarkId::new("filter_complex", size), &size, |b, _| {
            b.iter(|| {
                black_box(
                    df.filter((col("value") > lit(30.0)) & (col("value") < lit(70.0))).unwrap()
                )
            })
        });
    }

    group.finish();
}

fn bench_aggregation_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregation");

    for size in [1000, 10_000, 100_000] {
        let series = Series::new("data", (0..size).map(|i| i as f64).collect::<Vec<_>>());

        group.bench_with_input(BenchmarkId::new("sum", size), &size, |b, _| {
            b.iter(|| black_box(series.sum()))
        });

        group.bench_with_input(BenchmarkId::new("mean", size), &size, |b, _| {
            b.iter(|| black_box(series.mean()))
        });

        group.bench_with_input(BenchmarkId::new("std", size), &size, |b, _| {
            b.iter(|| black_box(series.std()))
        });

        group.bench_with_input(BenchmarkId::new("min_max", size), &size, |b, _| {
            b.iter(|| {
                black_box((series.min(), series.max()))
            })
        });
    }

    group.finish();
}

fn bench_io_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("io");

    for size in [1000, 10_000, 100_000] {
        let df = create_test_dataframe(size);
        let csv_path = format!("bench_data_{}.csv", size);

        group.bench_with_input(BenchmarkId::new("csv_write", size), &size, |b, _| {
            b.iter(|| {
                black_box(df.write_csv(&csv_path).unwrap())
            })
        });

        // Write once for read benchmark
        df.write_csv(&csv_path).unwrap();

        group.bench_with_input(BenchmarkId::new("csv_read", size), &size, |b, _| {
            b.iter(|| {
                black_box(DataFrame::read_csv(&csv_path).unwrap())
            })
        });

        // Cleanup
        std::fs::remove_file(&csv_path).ok();
    }

    group.finish();
}

fn bench_series_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("series");

    for size in [1000, 10_000, 100_000] {
        let s1 = Series::new("a", (0..size).map(|i| i as f64).collect::<Vec<_>>());
        let s2 = Series::new("b", (0..size).map(|i| (i * 2) as f64).collect::<Vec<_>>());

        group.bench_with_input(BenchmarkId::new("add", size), &size, |b, _| {
            b.iter(|| black_box(&s1 + &s2))
        });

        group.bench_with_input(BenchmarkId::new("multiply", size), &size, |b, _| {
            b.iter(|| black_box(&s1 * &s2))
        });

        group.bench_with_input(BenchmarkId::new("comparison", size), &size, |b, _| {
            b.iter(|| black_box(&s1 > &s2))
        });
    }

    group.finish();
}

#[cfg(feature = "scientific")]
fn bench_scientific_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("scientific");

    for size in [128, 512, 2048, 8192] {
        let series = Series::new("signal", (0..size).map(|i| (i as f64 * 0.1).sin()).collect::<Vec<_>>());

        group.bench_with_input(BenchmarkId::new("fft", size), &size, |b, _| {
            b.iter(|| black_box(series.fft().unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("rolling_mean", size), &size, |b, _| {
            b.iter(|| black_box(series.rolling_mean(10).unwrap()))
        });

        group.bench_with_input(BenchmarkId::new("standardize", size), &size, |b, _| {
            b.iter(|| black_box(series.standardize().unwrap()))
        });
    }

    group.finish();
}

// Helper functions
fn create_test_dataframe(size: usize) -> DataFrame {
    let categories: Vec<&str> = (0..size)
        .map(|i| match i % 5 {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            _ => "E",
        })
        .collect();

    let values: Vec<f64> = (0..size).map(|i| (i as f64) * 1.5).collect();

    DataFrame::new(vec![
        Series::new("category", categories),
        Series::new("value", values),
    ])
    .unwrap()
}

fn create_join_dataframe(size: usize, prefix: &str) -> DataFrame {
    let ids: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let values: Vec<f64> = (0..size).map(|i| (i as f64) * 2.0).collect();

    DataFrame::new(vec![
        Series::new("id", ids),
        Series::new(&format!("{}_value", prefix), values),
    ])
    .unwrap()
}

fn create_random_dataframe(size: usize) -> DataFrame {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let categories: Vec<&str> = (0..size)
        .map(|_| match rng.gen_range(0..3) {
            0 => "X",
            1 => "Y",
            _ => "Z",
        })
        .collect();

    let values: Vec<f64> = (0..size).map(|_| rng.gen_range(0.0..100.0)).collect();

    DataFrame::new(vec![
        Series::new("category", categories),
        Series::new("value", values),
    ])
    .unwrap()
}

criterion_group!(
    benches,
    bench_groupby_operations,
    bench_join_operations,
    bench_sort_operations,
    bench_filter_operations,
    bench_aggregation_functions,
    bench_io_operations,
    bench_series_operations,
);

#[cfg(feature = "scientific")]
criterion_group!(scientific_benches, bench_scientific_operations);

#[cfg(feature = "scientific")]
criterion_main!(benches, scientific_benches);

#[cfg(not(feature = "scientific"))]
criterion_main!(benches);
