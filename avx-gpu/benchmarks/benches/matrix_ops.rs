//! Benchmark: Matrix operations

use avx_gpu_core::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn matmul_cpu(a: &[f32], b: &[f32], c: &mut [f32], n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

fn benchmark_matmul(c: &mut Criterion) {
    let sizes = [128, 256, 512, 1024];

    let mut group = c.benchmark_group("matrix_multiply");
    group.sample_size(10); // Fewer samples for expensive ops

    for n in sizes {
        let size = n * n;
        let a: Vec<f32> = (0..size).map(|i| (i % 100) as f32).collect();
        let b: Vec<f32> = (0..size).map(|i| ((i * 2) % 100) as f32).collect();
        let mut c_cpu = vec![0.0f32; size];

        // 2 * N^3 FLOPS for matrix multiply
        let flops = 2 * n * n * n;
        group.throughput(Throughput::Elements(flops as u64));

        // CPU baseline (only for smaller sizes)
        if n <= 512 {
            group.bench_with_input(BenchmarkId::new("cpu", n), &n, |bench, &n| {
                bench.iter(|| {
                    matmul_cpu(black_box(&a), black_box(&b), black_box(&mut c_cpu), n);
                });
            });
        }

        // GPU
        if let Ok(device) = Device::auto() {
            group.bench_with_input(BenchmarkId::new("gpu", n), &n, |bench, _| {
                let gpu_a = device.buffer_from_slice(&a).unwrap();
                let gpu_b = device.buffer_from_slice(&b).unwrap();
                let mut gpu_c = device.buffer::<f32>(size).unwrap();

                let kernel_src = MATMUL_KERNEL.replace("1024u", &format!("{}u", n));
                let kernel = device.compile_kernel(&kernel_src, "matmul").unwrap();

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

const MATMUL_KERNEL: &str = r#"
@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> c: array<f32>;

const N: u32 = 1024u;

@compute @workgroup_size(16, 16)
fn matmul(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let row = global_id.y;
    let col = global_id.x;

    if (row >= N || col >= N) {
        return;
    }

    var sum = 0.0;
    for (var k = 0u; k < N; k = k + 1u) {
        sum += a[row * N + k] * b[k * N + col];
    }

    c[row * N + col] = sum;
}
"#;

criterion_group!(benches, benchmark_matmul);
criterion_main!(benches);
