//! Example: Matrix multiplication on GPU

use avx_gpu_core::prelude::*;

const N: usize = 1024; // Matrix dimension

fn main() -> Result<()> {
    env_logger::init();
    println!("🚀 AVX-GPU Matrix Multiplication Example\n");

    let device = Device::auto()?;
    println!("✓ Using: {}\n", device.info().name);

    // Prepare matrices
    println!("Preparing {}x{} matrices...", N, N);
    let a: Vec<f32> = (0..N * N).map(|i| (i % 100) as f32).collect();
    let b: Vec<f32> = (0..N * N).map(|i| ((i * 2) % 100) as f32).collect();

    // GPU computation
    println!("Allocating GPU memory...");
    let gpu_a = device.buffer_from_slice(&a)?;
    let gpu_b = device.buffer_from_slice(&b)?;
    let mut gpu_c = device.buffer::<f32>(N * N)?;

    println!("Compiling kernel...");
    let kernel = device.compile_kernel(MATMUL_KERNEL, "matmul")?;

    println!("Executing kernel...");
    let start = std::time::Instant::now();
    device.execute_kernel(&kernel, &[&gpu_a, &gpu_b, &gpu_c])?;
    device.synchronize()?;
    let gpu_time = start.elapsed();

    println!("✓ GPU time: {:.2}ms", gpu_time.as_secs_f64() * 1000.0);

    // Calculate GFLOPS
    let flops = 2.0 * N.pow(3) as f64;
    let gflops = flops / gpu_time.as_secs_f64() / 1e9;
    println!("  Performance: {:.2} GFLOPS", gflops);

    Ok(())
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
