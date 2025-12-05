//! Benchmarks for avila-rand
//!
//! Run with: cargo bench

use avila_rand::*;
use std::time::{Duration, Instant};

fn benchmark<F: FnMut()>(name: &str, mut f: F, iterations: usize) {
    // Warmup
    for _ in 0..100 {
        f();
    }
    
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed();
    
    let ns_per_iter = duration.as_nanos() / iterations as u128;
    let throughput = if ns_per_iter > 0 {
        1_000_000_000.0 / ns_per_iter as f64
    } else {
        0.0
    };
    
    println!("{:30} {:10.2} ns/iter  {:10.2} Mops/s", 
             name, ns_per_iter, throughput / 1_000_000.0);
}

fn benchmark_throughput<F: FnMut()>(name: &str, mut f: F, bytes_per_iter: usize) {
    let iterations = 10000;
    
    // Warmup
    for _ in 0..100 {
        f();
    }
    
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed();
    
    let total_bytes = iterations * bytes_per_iter;
    let mb_per_sec = (total_bytes as f64 / 1_048_576.0) / duration.as_secs_f64();
    
    println!("{:30} {:10.2} MB/s", name, mb_per_sec);
}

fn main() {
    println!("\n=== Avila Rand Benchmarks ===\n");
    
    println!("Individual Operations:");
    println!("{:30} {:>10}  {:>10}", "Test", "Time", "Throughput");
    println!("{}", "=".repeat(60));
    
    // ChaCha20 benchmarks
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("ChaCha20::next_u32", || { rng.next_u32(); }, 100000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("ChaCha20::next_u64", || { rng.next_u64(); }, 100000);
    
    // Xoshiro benchmarks
    let mut rng = Xoshiro256StarStar::seed_from_u64(42);
    benchmark("Xoshiro256**::next_u32", || { rng.next_u32(); }, 100000);
    
    let mut rng = Xoshiro256StarStar::seed_from_u64(42);
    benchmark("Xoshiro256**::next_u64", || { rng.next_u64(); }, 100000);
    
    // OsRng benchmarks
    let mut rng = OsRng::new();
    benchmark("OsRng::next_u64", || { rng.next_u64(); }, 1000);
    
    println!("\n{}", "=".repeat(60));
    println!("\nThroughput Tests:");
    println!("{:30} {:>10}", "Test", "Speed");
    println!("{}", "=".repeat(60));
    
    // Bulk operations
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let mut buf = [0u8; 1024];
    benchmark_throughput("ChaCha20 fill 1KB", || { rng.fill_bytes(&mut buf); }, 1024);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let mut buf = [0u8; 4096];
    benchmark_throughput("ChaCha20 fill 4KB", || { rng.fill_bytes(&mut buf); }, 4096);
    
    let mut rng = Xoshiro256StarStar::seed_from_u64(42);
    let mut buf = [0u8; 1024];
    benchmark_throughput("Xoshiro256** fill 1KB", || { rng.fill_bytes(&mut buf); }, 1024);
    
    let mut rng = Xoshiro256StarStar::seed_from_u64(42);
    let mut buf = [0u8; 4096];
    benchmark_throughput("Xoshiro256** fill 4KB", || { rng.fill_bytes(&mut buf); }, 4096);
    
    println!("\n{}", "=".repeat(60));
    println!("\nDistribution Benchmarks:");
    println!("{:30} {:>10}  {:>10}", "Test", "Time", "Throughput");
    println!("{}", "=".repeat(60));
    
    // Distribution benchmarks
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let uniform = Uniform::new(0.0, 100.0);
    benchmark("Uniform<f64>", || { uniform.sample(&mut rng); }, 10000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let normal = Normal::new(0.0, 1.0);
    benchmark("Normal<f64>", || { normal.sample(&mut rng); }, 10000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let exp = Exponential::new(1.0);
    benchmark("Exponential<f64>", || { exp.sample(&mut rng); }, 10000);
    
    println!("\n{}", "=".repeat(60));
    println!("\nType Generation Benchmarks:");
    println!("{:30} {:>10}  {:>10}", "Test", "Time", "Throughput");
    println!("{}", "=".repeat(60));
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("gen::<u8>", || { let _: u8 = rng.gen(); }, 100000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("gen::<u32>", || { let _: u32 = rng.gen(); }, 100000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("gen::<u64>", || { let _: u64 = rng.gen(); }, 100000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("gen::<u128>", || { let _: u128 = rng.gen(); }, 100000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("gen::<f64>", || { let _: f64 = rng.gen(); }, 100000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    benchmark("gen_range::<u32>", || { let _: u32 = rng.gen_range(0..100); }, 100000);
    
    println!("\n{}", "=".repeat(60));
    println!("\nUtility Benchmarks:");
    println!("{:30} {:>10}  {:>10}", "Test", "Time", "Throughput");
    println!("{}", "=".repeat(60));
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let mut arr = [0u32; 100];
    for i in 0..100 { arr[i] = i as u32; }
    benchmark("shuffle 100 elements", || { shuffle(&mut rng, &mut arr); }, 1000);
    
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let mut arr = [0u32; 1000];
    for i in 0..1000 { arr[i] = i as u32; }
    benchmark("shuffle 1000 elements", || { shuffle(&mut rng, &mut arr); }, 100);
    
    println!("\n");
}
