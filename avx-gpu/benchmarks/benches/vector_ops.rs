//! Benchmark: Vector operations (add, multiply, etc.)

use avx_gpu_core::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn vector_add_cpu(a: &[f32], b: &[f32], c: &mut [f32]) {
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
}

fn vector_add_cpu_parallel(a: &[f32], b: &[f32], c: &mut [f32]) {
    use rayon::prelude::*;
    c.par_iter_mut()
        .zip(a.par_iter().zip(b.par_iter()))
        .for_each(|(c, (a, b))| *c = a + b);
}

fn benchmark_vector_add(c: &mut Criterion) {
    let sizes = [1_000, 10_000, 100_000, 1_000_000, 10_000_000];

    let mut group = c.benchmark_group("vector_add");

    for size in sizes {
        let a: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();
        let mut c_cpu = vec![0.0f32; size];

        group.throughput(Throughput::Elements(size as u64));

        // CPU baseline
        group.bench_with_input(BenchmarkId::new("cpu", size), &size, |bench, _| {
            bench.iter(|| {
                vector_add_cpu(black_box(&a), black_box(&b), black_box(&mut c_cpu));
            });
        });

        // CPU parallel
        group.bench_with_input(BenchmarkId::new("cpu_parallel", size), &size, |bench, _| {
            bench.iter(|| {
                vector_add_cpu_parallel(black_box(&a), black_box(&b), black_box(&mut c_cpu));
            });
        });

        // GPU (if available)
        if let Ok(device) = Device::auto() {
            group.bench_with_input(BenchmarkId::new("gpu", size), &size, |bench, _| {
                let gpu_a = device.buffer_from_slice(&a).unwrap();
                let gpu_b = device.buffer_from_slice(&b).unwrap();
                let mut gpu_c = device.buffer::<f32>(size).unwrap();

                let kernel = device
                    .compile_kernel(VECTOR_ADD_KERNEL, "vector_add")
                    .unwrap();

                bench.iter(|| {
                    device
                        .execute_kernel(&kernel, &[&gpu_a, &gpu_b, &gpu_c])
                        .unwrap();
                    device.synchronize().unwrap();
                });
            });
        }
    }

    group.finish();
}

const VECTOR_ADD_KERNEL: &str = r#"
@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> c: array<f32>;

@compute @workgroup_size(256)
fn vector_add(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if (idx < arrayLength(&a)) {
        c[idx] = a[idx] + b[idx];
    }
}
"#;

criterion_group!(benches, benchmark_vector_add);
criterion_main!(benches);
