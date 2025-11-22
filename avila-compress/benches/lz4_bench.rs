//! Benchmark LZ4 compression performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use avila_compress::lz4;

fn compress_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lz4_compress");

    // Different data sizes
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        // Repetitive data (compresses well)
        let repetitive = vec![b'A'; *size];
        group.bench_with_input(
            BenchmarkId::new("repetitive", size),
            &repetitive,
            |b, data| {
                b.iter(|| lz4::compress(black_box(data)).unwrap());
            },
        );

        // Random data (compresses poorly)
        let random: Vec<u8> = (0..*size).map(|i| (i * 17) as u8).collect();
        group.bench_with_input(
            BenchmarkId::new("random", size),
            &random,
            |b, data| {
                b.iter(|| lz4::compress(black_box(data)).unwrap());
            },
        );

        // Text-like data (realistic)
        let text = "The quick brown fox jumps over the lazy dog. ".repeat(*size / 46);
        group.bench_with_input(
            BenchmarkId::new("text", size),
            &text.as_bytes(),
            |b, data| {
                b.iter(|| lz4::compress(black_box(data)).unwrap());
            },
        );
    }

    group.finish();
}

fn decompress_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lz4_decompress");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        // Prepare compressed data
        let data = vec![b'A'; *size];
        let compressed = lz4::compress(&data).unwrap();

        group.bench_with_input(
            BenchmarkId::new("repetitive", size),
            &compressed,
            |b, data| {
                b.iter(|| lz4::decompress(black_box(data)).unwrap());
            },
        );

        // Random data
        let random: Vec<u8> = (0..*size).map(|i| (i * 17) as u8).collect();
        let compressed = lz4::compress(&random).unwrap();
        group.bench_with_input(
            BenchmarkId::new("random", size),
            &compressed,
            |b, data| {
                b.iter(|| lz4::decompress(black_box(data)).unwrap());
            },
        );
    }

    group.finish();
}

fn round_trip_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lz4_round_trip");

    for size in [100, 1_000, 10_000].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        let data = vec![b'A'; *size];
        group.bench_with_input(
            BenchmarkId::new("compress_decompress", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let compressed = lz4::compress(black_box(data)).unwrap();
                    lz4::decompress(black_box(&compressed)).unwrap()
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, compress_benchmark, decompress_benchmark, round_trip_benchmark);
criterion_main!(benches);
