//! Benchmarks for RNG performance
//!
//! Run with: cargo +nightly bench

#![feature(test)]
extern crate test;

use test::Bencher;
use avila_rand_simple::{Pcg32, Pcg64, Xorshift64, Xorshift128Plus, Xorshift128StarStar, Splitmix64, FastRng};

// PCG Benchmarks
#[bench]
fn bench_pcg32_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32::new(12345);
    b.iter(|| rng.next_u32());
}

#[bench]
fn bench_pcg64_next_u64(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| rng.next_u64());
}

// Xorshift Benchmarks
#[bench]
fn bench_xorshift64_next_u64(b: &mut Bencher) {
    let mut rng = Xorshift64::new(12345);
    b.iter(|| rng.next_u64());
}

#[bench]
fn bench_xorshift128plus_next_u64(b: &mut Bencher) {
    let mut rng = Xorshift128Plus::new(12345);
    b.iter(|| rng.next_u64());
}

#[bench]
fn bench_xorshift128starstar_next_u64(b: &mut Bencher) {
    let mut rng = Xorshift128StarStar::new(12345);
    b.iter(|| rng.next_u64());
}

// Splitmix64 Benchmarks
#[bench]
fn bench_splitmix64_next_u64(b: &mut Bencher) {
    let mut rng = Splitmix64::new(12345);
    b.iter(|| rng.next_u64());
}

// Range generation benchmarks
#[bench]
fn bench_pcg64_gen_range(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| rng.gen_range(1, 100));
}

// Fill operations benchmarks
#[bench]
fn bench_pcg64_fill_bytes_1kb(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    let mut buf = [0u8; 1024];
    b.iter(|| rng.fill_bytes(&mut buf));
}

// Float generation benchmarks
#[bench]
fn bench_pcg64_next_f64(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| rng.next_f64());
}

#[bench]
fn bench_pcg64_next_f32(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| rng.next_f32());
}

// Bulk generation
#[bench]
fn bench_pcg64_bulk_u64_1000(b: &mut Bencher) {
    let mut rng = Pcg64::new(12345);
    b.iter(|| {
        for _ in 0..1000 {
            test::black_box(rng.next_u64());
        }
    });
}
