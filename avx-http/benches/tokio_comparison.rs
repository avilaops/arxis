//! Benchmark: AVX-HTTP vs Tokio
//!
//! Compara performance entre runtime proprietário e Tokio

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

// ============================================================================
// Timer Benchmarks
// ============================================================================

fn bench_timer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("timers");

    // AVX-HTTP timer wheel
    group.bench_function("avx_timer_insert", |b| {
        use avx_http::timer::TimerWheelScheduler;
        let mut wheel = TimerWheelScheduler::new();
        b.iter(|| {
            wheel.schedule(Duration::from_millis(100), || {
                black_box(42);
            });
        });
    });

    // Tokio timer (for comparison - requires tokio dependency in dev-deps)
    #[cfg(feature = "tokio-comparison")]
    group.bench_function("tokio_timer_insert", |b| {
        use tokio::time::{sleep, timeout};
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.spawn(async {
                sleep(Duration::from_millis(100)).await;
                black_box(42)
            });
        });
    });

    group.finish();
}

// ============================================================================
// Runtime Spawn Benchmarks
// ============================================================================

fn bench_task_spawn(c: &mut Criterion) {
    let mut group = c.benchmark_group("spawn");

    // AVX-HTTP spawn
    group.bench_function("avx_spawn", |b| {
        use avx_http::runtime;
        b.iter(|| {
            runtime::spawn(async {
                black_box(42);
            });
        });
    });

    // Tokio spawn (for comparison)
    #[cfg(feature = "tokio-comparison")]
    group.bench_function("tokio_spawn", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.spawn(async {
                black_box(42);
            });
        });
    });

    group.finish();
}

// ============================================================================
// Block_on Benchmarks
// ============================================================================

fn bench_block_on(c: &mut Criterion) {
    let mut group = c.benchmark_group("block_on");

    // AVX-HTTP block_on
    group.bench_function("avx_block_on_immediate", |b| {
        use avx_http::runtime;
        b.iter(|| {
            runtime::block_on(async {
                black_box(42)
            })
        });
    });

    // Tokio block_on (for comparison)
    #[cfg(feature = "tokio-comparison")]
    group.bench_function("tokio_block_on_immediate", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                black_box(42)
            })
        });
    });

    group.finish();
}

// ============================================================================
// Async I/O Benchmarks
// ============================================================================

fn bench_async_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_io");
    group.sample_size(50); // Reduced for I/O operations

    // AVX-HTTP async read/write simulation
    group.bench_function("avx_async_echo", |b| {
        use avx_http::runtime;
        b.iter(|| {
            runtime::block_on(async {
                // Simulate async I/O
                runtime::sleep(Duration::from_micros(100)).await;
                black_box(42)
            })
        });
    });

    // Tokio async read/write simulation
    #[cfg(feature = "tokio-comparison")]
    group.bench_function("tokio_async_echo", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                tokio::time::sleep(Duration::from_micros(100)).await;
                black_box(42)
            })
        });
    });

    group.finish();
}

// ============================================================================
// Parallel Tasks Benchmark
// ============================================================================

fn bench_parallel_tasks(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel");
    group.sample_size(20);

    for num_tasks in [10, 100, 1000].iter() {
        // AVX-HTTP parallel
        group.bench_with_input(
            BenchmarkId::new("avx", num_tasks),
            num_tasks,
            |b, &n| {
                use avx_http::runtime;
                b.iter(|| {
                    runtime::block_on(async move {
                        for _ in 0..n {
                            runtime::spawn(async {
                                black_box(42);
                            });
                        }
                        runtime::sleep(Duration::from_millis(10)).await;
                    })
                });
            },
        );

        // Tokio parallel
        #[cfg(feature = "tokio-comparison")]
        group.bench_with_input(
            BenchmarkId::new("tokio", num_tasks),
            num_tasks,
            |b, &n| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                b.iter(|| {
                    rt.block_on(async move {
                        for _ in 0..n {
                            tokio::spawn(async {
                                black_box(42);
                            });
                        }
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    })
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Memory & Data Structure Benchmarks
// ============================================================================

fn bench_data_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("data");

    // Bytes operations
    group.bench_function("avx_bytes_clone", |b| {
        use avx_http::bytes::Bytes;
        let data = Bytes::from(vec![0u8; 1024]);
        b.iter(|| {
            black_box(data.clone());
        });
    });

    group.bench_function("avx_bytes_slice", |b| {
        use avx_http::bytes::Bytes;
        let data = Bytes::from(vec![0u8; 1024]);
        b.iter(|| {
            black_box(data.slice(0..512));
        });
    });

    // JSON parsing
    let json_data = r#"{"user":{"id":123,"name":"John Doe","email":"john@example.com"},"items":[1,2,3,4,5],"active":true}"#;

    group.bench_function("avx_json_parse", |b| {
        use avx_http::json::JsonValue;
        b.iter(|| {
            black_box(JsonValue::parse(json_data).unwrap());
        });
    });

    group.finish();
}

// ============================================================================
// Latency Percentiles
// ============================================================================

fn bench_latency_percentiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);

    // AVX-HTTP latency
    group.bench_function("avx_task_latency", |b| {
        use avx_http::runtime;
        b.iter(|| {
            runtime::block_on(async {
                runtime::sleep(Duration::from_micros(10)).await;
                black_box(42)
            })
        });
    });

    // Tokio latency
    #[cfg(feature = "tokio-comparison")]
    group.bench_function("tokio_task_latency", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                tokio::time::sleep(Duration::from_micros(10)).await;
                black_box(42)
            })
        });
    });

    group.finish();
}

criterion_group!(
    timer_benches,
    bench_timer_operations,
);

criterion_group!(
    runtime_benches,
    bench_task_spawn,
    bench_block_on,
    bench_async_io,
    bench_parallel_tasks,
);

criterion_group!(
    data_benches,
    bench_data_structures,
);

criterion_group!(
    latency_benches,
    bench_latency_percentiles,
);

criterion_main!(
    timer_benches,
    runtime_benches,
    data_benches,
    latency_benches,
);
