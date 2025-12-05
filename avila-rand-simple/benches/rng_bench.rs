//! Benchmarks for RNG performance
//!
//! Run with: cargo bench

#![feature(test)]
extern crate test;

use test::Bencher;
use avila_rand_simple::*;

// PCG Benchmarks
#[bench]
fn bench_pcg32_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32::new(12345);
    b.iter(|| {
        test::black_box(rng.next_u32())
    });
}

#[bench]
fn bench_pcg64_next_u64(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| {
        test::black_box(rng.next_u64())
    });
}

// Xorshift Benchmarks
#[bench]
fn bench_xorshift64_next_u64(b: &mut Bencher) {
    let mut rng = Xorshift64::new(12345);
    b.iter(|| {
        test::black_box(rng.next_u64())
    });
}

#[bench]
fn bench_xorshift128plus_next_u64(b: &mut Bencher) {
    let mut rng = Xorshift128Plus::new(12345);
    b.iter(|| {
        test::black_box(rng.next_u64())
    });
}

#[bench]
fn bench_xorshift128starstar_next_u64(b: &mut Bencher) {
    let mut rng = Xorshift128StarStar::new(12345);
    b.iter(|| {
        test::black_box(rng.next_u64())
    });
}

// Splitmix64 Benchmarks
#[bench]
fn bench_splitmix64_next_u64(b: &mut Bencher) {
    let mut rng = Splitmix64::new(12345);
    b.iter(|| {
        test::black_box(rng.next_u64())
    });
}

// Range generation benchmarks
#[bench]
fn bench_pcg64_gen_range(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| {
        test::black_box(rng.gen_range(1, 100))
    });
}

#[bench]
fn bench_range_gen_range_u64(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| {
        test::black_box(range::gen_range_u64(&mut rng, 1, 100))
    });
}

// Fill operations benchmarks
#[bench]
fn bench_pcg64_fill_bytes_1kb(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    let mut buf = [0u8; 1024];
    b.iter(|| {
        rng.fill_bytes(&mut buf);
        test::black_box(&buf)
    });
}

#[bench]
fn bench_pcg64_fill_bytes_64kb(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    let mut buf = vec![0u8; 65536];
    b.iter(|| {
        rng.fill_bytes(&mut buf);
        test::black_box(&buf)
    });
}

// Float generation benchmarks
#[bench]
fn bench_pcg64_next_f64(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| {
        test::black_box(rng.next_f64())
    });
}

#[bench]
fn bench_pcg64_next_f32(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| {
        test::black_box(rng.next_f32())
    });
}

// SIMD benchmarks
#[cfg(feature = "simd")]
#[bench]
fn bench_simd_fill_u64_10k(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    let mut buf = vec![0u64; 10000];
    b.iter(|| {
        simd::fill_u64_simd(&mut rng, &mut buf);
        test::black_box(&buf)
    });
}

#[cfg(feature = "simd")]
#[bench]
fn bench_simd_fill_u32_10k(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    let mut buf = vec![0u32; 10000];
    b.iter(|| {
        simd::fill_u32_simd(&mut rng, &mut buf);
        test::black_box(&buf)
    });
}

// Bulk generation comparison
#[bench]
fn bench_pcg64_bulk_u64_1000(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| {
        for _ in 0..1000 {
            test::black_box(rng.next_u64());
        }
    });
}

// Shuffle benchmark
#[bench]
fn bench_shuffle_100(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    let mut arr: Vec<u32> = (0..100).collect();
    b.iter(|| {
        range::shuffle(&mut arr, &mut rng);
        test::black_box(&arr)
    });
}
