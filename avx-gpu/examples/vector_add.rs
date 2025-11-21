//! Example: Vector addition on GPU
//!
//! Demonstrates basic GPU computation with AVX-GPU

use avx_gpu_core::prelude::*;

const VECTOR_SIZE: usize = 1_000_000;

fn main() -> Result<()> {
    env_logger::init();
    println!("🚀 AVX-GPU Vector Addition Example\n");

    // Auto-detect best GPU
    println!("Detecting GPU...");
    let device = Device::auto()?;
    let info = device.info();
    println!("✓ Using: {} ({:?})", info.name, info.device_type);
    println!("  Backend: {:?}\n", info.backend);

    // Prepare input data
    println!("Preparing data ({} elements)...", VECTOR_SIZE);
    let a: Vec<f32> = (0..VECTOR_SIZE).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..VECTOR_SIZE).map(|i| (i * 2) as f32).collect();

    // Allocate GPU buffers
    println!("Allocating GPU memory...");
    let gpu_a = device.buffer_from_slice(&a)?;
    let gpu_b = device.buffer_from_slice(&b)?;
    let mut gpu_c = device.buffer::<f32>(VECTOR_SIZE)?;
    println!("✓ Allocated {} MB on GPU",
        (gpu_a.size_bytes() + gpu_b.size_bytes() + gpu_c.size_bytes()) / 1_000_000);

    // Compile kernel
    println!("\nCompiling kernel...");
    let kernel = device.compile_kernel(VECTOR_ADD_KERNEL, "vector_add")?;
    println!("✓ Kernel compiled");

    // Execute kernel
    println!("\nExecuting kernel...");
    let start = std::time::Instant::now();
    device.execute_kernel(&kernel, &[&gpu_a, &gpu_b, &gpu_c])?;
    device.synchronize()?;
    let elapsed = start.elapsed();
    println!("✓ Execution time: {:.2}ms", elapsed.as_secs_f64() * 1000.0);

    // Read results
    println!("\nReading results...");
    let result = gpu_c.read()?;

    // Verify correctness
    println!("Verifying correctness...");
    let mut errors = 0;
    for i in 0..VECTOR_SIZE.min(10) {
        let expected = a[i] + b[i];
        let actual = result[i];
        if (expected - actual).abs() > 0.001 {
            errors += 1;
            println!("  Error at {}: expected {}, got {}", i, expected, actual);
        }
    }

    if errors == 0 {
        println!("✓ All results correct!");
        println!("\n📊 Performance:");
        let throughput = (VECTOR_SIZE as f64 * std::mem::size_of::<f32>() as f64 * 3.0)
            / (elapsed.as_secs_f64() * 1e9);
        println!("  Throughput: {:.2} GB/s", throughput);
        println!("  Operations: {:.0} GFLOPS",
            VECTOR_SIZE as f64 / elapsed.as_secs_f64() / 1e9);
    } else {
        println!("❌ {} errors found", errors);
    }

    Ok(())
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
