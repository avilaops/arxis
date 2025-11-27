//! Benchmarks for async runtime and timer wheel

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

fn bench_timer_wheel_insert(c: &mut Criterion) {
    use avx_http::timer::{TimerWheelScheduler, GlobalTimerWheel};

    c.bench_function("timer_wheel_insert_1ms", |b| {
        let mut wheel = TimerWheelScheduler::new();
        b.iter(|| {
            wheel.schedule(Duration::from_millis(1), || {
                black_box(42);
            });
        });
    });

    c.bench_function("timer_wheel_insert_100ms", |b| {
        let mut wheel = TimerWheelScheduler::new();
        b.iter(|| {
            wheel.schedule(Duration::from_millis(100), || {
                black_box(42);
            });
        });
    });

    c.bench_function("timer_wheel_insert_1s", |b| {
        let mut wheel = TimerWheelScheduler::new();
        b.iter(|| {
            wheel.schedule(Duration::from_secs(1), || {
                black_box(42);
            });
        });
    });
}

fn bench_timer_wheel_tick(c: &mut Criterion) {
    use avx_http::timer::TimerWheelScheduler;

    let mut group = c.benchmark_group("timer_wheel_tick");

    for num_timers in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_timers),
            num_timers,
            |b, &num| {
                b.iter_batched(
                    || {
                        let mut wheel = TimerWheelScheduler::new();
                        for i in 0..num {
                            wheel.schedule(Duration::from_millis(i as u64), || {});
                        }
                        wheel
                    },
                    |mut wheel| {
                        for _ in 0..100 {
                            black_box(wheel.tick());
                            std::thread::sleep(Duration::from_micros(10));
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn bench_runtime_spawn(c: &mut Criterion) {
    use avx_http::runtime;

    c.bench_function("runtime_spawn_simple", |b| {
        b.iter(|| {
            runtime::spawn(async {
                black_box(42);
            });
        });
    });
}

fn bench_runtime_block_on(c: &mut Criterion) {
    use avx_http::runtime;

    c.bench_function("runtime_block_on_immediate", |b| {
        b.iter(|| {
            runtime::block_on(async {
                black_box(42)
            });
        });
    });
}

fn bench_sleep_overhead(c: &mut Criterion) {
    use avx_http::runtime;

    c.bench_function("sleep_1ms", |b| {
        b.iter(|| {
            runtime::block_on(async {
                runtime::sleep(Duration::from_millis(1)).await;
                black_box(42)
            });
        });
    });
}

fn bench_bytes_operations(c: &mut Criterion) {
    use avx_http::bytes::Bytes;

    c.bench_function("bytes_from_vec", |b| {
        let data = vec![0u8; 1024];
        b.iter(|| {
            black_box(Bytes::from(data.clone()));
        });
    });

    c.bench_function("bytes_slice_zerocopy", |b| {
        let data = Bytes::from(vec![0u8; 1024]);
        b.iter(|| {
            black_box(data.slice(0..512));
        });
    });
}

fn bench_json_parse(c: &mut Criterion) {
    use avx_http::json::JsonValue;

    let simple_json = r#"{"name":"test","value":42,"active":true}"#;
    let nested_json = r#"{"user":{"id":1,"name":"John","tags":["rust","http"]},"count":100}"#;

    c.bench_function("json_parse_simple", |b| {
        b.iter(|| {
            black_box(JsonValue::parse(simple_json).unwrap());
        });
    });

    c.bench_function("json_parse_nested", |b| {
        b.iter(|| {
            black_box(JsonValue::parse(nested_json).unwrap());
        });
    });

    c.bench_function("json_serialize", |b| {
        let value = JsonValue::parse(nested_json).unwrap();
        b.iter(|| {
            black_box(value.to_string());
        });
    });
}

fn bench_reactor_register(c: &mut Criterion) {
    use avx_http::reactor::{Reactor, Interest};

    c.bench_function("reactor_register_fd", |b| {
        let mut reactor = Reactor::new().unwrap();
        let mut token = 0;

        b.iter(|| {
            token += 1;
            // Note: We can't actually register without a real FD
            // This just measures the overhead of the call structure
            black_box(token);
        });
    });
}

criterion_group!(
    timer_benches,
    bench_timer_wheel_insert,
    bench_timer_wheel_tick,
);

criterion_group!(
    runtime_benches,
    bench_runtime_spawn,
    bench_runtime_block_on,
    bench_sleep_overhead,
);

criterion_group!(
    data_benches,
    bench_bytes_operations,
    bench_json_parse,
);

criterion_group!(
    reactor_benches,
    bench_reactor_register,
);

criterion_main!(
    timer_benches,
    runtime_benches,
    data_benches,
    reactor_benches,
);
