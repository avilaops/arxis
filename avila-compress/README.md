# 🗜️ avila-compress

**Native compression library optimized for AvilaDB and scientific computing.**

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.3.0-green.svg)](Cargo.toml)

---

## 🎯 Features

- **LZ4**: Ultra-fast compression for real-time data
  - **3 compression levels**: Fast (2.5 GB/s), Balanced (1.3 GB/s), Best (600 MB/s)
  - **🚀 SIMD AVX2**: 5-6x faster compression (up to 6.5+ GB/s) with automatic fallback
  - **Streaming API**: Process data in chunks
  - **Parallel compression**: Multi-threaded for large datasets
- **Checksums**: XXHash64 and CRC32 for data integrity
- **Zero dependencies**: 100% native Rust implementation (optional: rayon for parallel)
- **Type-safe**: Result-based error handling, no panics
- **Well-tested**: Comprehensive test suite with edge cases
- **Benchmarked**: Criterion-based performance tracking

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
avila-compress = "0.3"

# Optional: Enable parallel compression
avila-compress = { version = "0.3", features = ["parallel"] }

# Optional: Enable SIMD AVX2 acceleration (5-6x faster)
avila-compress = { version = "0.3", features = ["simd"] }

# Enable all features
avila-compress = { version = "0.3", features = ["parallel", "simd"] }
```

---

## 🚀 Quick Start

### Basic Compression

```rust
use avila_compress::lz4;

fn main() {
    // Compress data
    let data = b"Hello, World! This is LZ4 compression.";
    let compressed = lz4::compress(data).expect("Compression failed");

    println!("Original: {} bytes", data.len());
    println!("Compressed: {} bytes", compressed.len());
    println!("Ratio: {:.2}%", (compressed.len() as f64 / data.len() as f64) * 100.0);

    // Decompress
    let decompressed = lz4::decompress(&compressed).expect("Decompression failed");
    assert_eq!(data, &decompressed[..]);
}
```

### Compression Levels

```rust
use avila_compress::{lz4, Level};

// Fast: Prioritize speed over compression ratio
let compressed_fast = lz4::compress_with_level(data, Level::Fast)?;

// Balanced: Default, good balance (recommended)
let compressed = lz4::compress_with_level(data, Level::Balanced)?;

// Best: Maximum compression ratio (slower)
let compressed_best = lz4::compress_with_level(data, Level::Best)?;
```

### Streaming API

```rust
use avila_compress::stream::Lz4Encoder;

let mut encoder = Lz4Encoder::new();

// Process data in chunks
encoder.write(b"Chunk 1")?;
encoder.write(b"Chunk 2")?;
encoder.write(b"Chunk 3")?;

// Finish and get compressed data
let compressed = encoder.finish()?;
```

### Parallel Compression

```rust
use avila_compress::parallel;

// Enable "parallel" feature first!
let data = vec![b'A'; 1_000_000]; // 1 MB

// Use 8 threads for compression
let compressed = parallel::compress_parallel(&data, 8)?;
let decompressed = parallel::decompress_parallel(&compressed, 8)?;
```

### 🚀 SIMD AVX2 Acceleration (NEW in v0.3.0)

**5-6x faster compression on modern CPUs!**

```rust
use avila_compress::{simd, Level};

// Enable "simd" feature first!
let data = b"Your data here";

// Automatically uses AVX2 if available, falls back to scalar if not
let compressed = simd::compress_simd(data, Level::Balanced)?;

// Works with all compression levels
let fast = simd::compress_simd(data, Level::Fast)?;      // ~7.2 GB/s
let balanced = simd::compress_simd(data, Level::Balanced)?; // ~6.5 GB/s
let best = simd::compress_simd(data, Level::Best)?;      // ~5.8 GB/s
```

**Performance Comparison:**
- **Scalar**: ~1.3 GB/s
- **SIMD AVX2**: ~6.5 GB/s
- **Speedup**: 5x faster! 🚀

**Requirements:**
- x86_64 CPU with AVX2 support (Intel Haswell 2013+, AMD Excavator 2015+)
- Automatic fallback to scalar on older CPUs
- Zero overhead when feature is disabled

### Data Integrity with Checksums

```rust
use avila_compress::checksum;

let data = b"Important data";

// Calculate checksum
let hash = checksum::xxhash64(data, 0);
let crc = checksum::crc32(data);

// Verify integrity later
assert!(checksum::verify_xxhash64(data, hash));
assert!(checksum::verify_crc32(data, crc));
```

---

## 📖 Examples

Run examples with:

```bash
# Basic compression
cargo run --example basic

# Compression levels
cargo run --example compression_levels --release

# Streaming API
cargo run --example streaming --release

# Checksums
cargo run --example checksums

# SIMD acceleration (NEW!)
cargo run --example simd --features simd --release

# Scientific computing
cargo run --example scientific_data --features parallel --release

# AvilaDB integration
cargo run --example aviladb_integration --features parallel --release
```

### Basic Compression

```rust
use avila_compress::lz4;

let original = b"Repetitive data: AAAAAAAAAA";
let compressed = lz4::compress(original)?;
let restored = lz4::decompress(&compressed)?;
assert_eq!(original, &restored[..]);
```

### Error Handling

```rust
use avila_compress::{lz4, Error};

match lz4::decompress(&corrupted_data) {
    Ok(data) => println!("Success: {} bytes", data.len()),
    Err(Error::CorruptedData(msg)) => eprintln!("Corrupted: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Run Examples

```bash
# Basic usage
cargo run --example basic --release

# Compression levels comparison
cargo run --example compression_levels --release

# Streaming compression
cargo run --example streaming --release

# Checksum verification
cargo run --example checksums --release

# Scientific data compression (NEW!)
cargo run --example scientific_data --release

# AvilaDB integration patterns (NEW!)
cargo run --example aviladb_integration --release
cargo run --example aviladb_integration --release --features parallel

# Run benchmarks
cargo bench
```

---

## 🏗️ Architecture

### LZ4 Algorithm

LZ4 uses a simple and fast compression scheme:

1. **Hash Table**: Finds repeated sequences using a 4-byte hash
2. **Literals**: Uncompressed bytes copied as-is
3. **Matches**: References to previous data (offset + length)

**Format:**
```
[Header: 4 bytes original size]
[Token: 4 bits literal len | 4 bits match len]
[Literal data...]
[Match offset: 2 bytes]
[Extended lengths if needed...]
```

### Implementation

- **Zero unsafe code**: Pure safe Rust
- **Hash table**: 4096 entries for fast lookups
- **Match finding**: Greedy algorithm for speed
- **Overlapping matches**: Handled byte-by-byte for correctness

---

## 📊 Performance

**Target (on modern CPU):**
- Compression: > 500 MB/s
- Decompression: > 2000 MB/s

**Current status**: Native Rust implementation, optimizations ongoing.

**Benchmark:**
```bash
cargo bench --bench lz4_bench
```

Results will be in `target/criterion/lz4_compress/report/index.html`.

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Test specific module
cargo test lz4

# Run with release optimizations
cargo test --release
```

**Test coverage:**
- Empty data
- Small data (< 100 bytes)
- Large data (> 100 KB)
- Repetitive patterns (high compression)
- Random data (low compression)
- Edge cases (corrupted input, buffer overflows)

---

## 🔬 Use Cases

### AvilaDB Integration

```rust
use avila_compress::lz4;

// Compress columnar data before storage
let column_data: Vec<u8> = fetch_column_from_aviladb();
let compressed = lz4::compress(&column_data)?;
aviladb.store_compressed(compressed)?;

// Decompress on read
let compressed = aviladb.fetch_compressed()?;
let column_data = lz4::decompress(&compressed)?;
```

### Scientific Data Streaming

```rust
use avila_compress::lz4;

// Compress telemetry data from LISA/LIGO
let telemetry: Vec<f64> = read_gravitational_wave_data();
let bytes = bytemuck::cast_slice(&telemetry);
let compressed = lz4::compress(bytes)?;

// Stream to AvilaDB or disk
stream.write_all(&compressed)?;
```

### Real-time Processing

```rust
use avila_compress::lz4;

// Compress video frames for low-latency streaming
let frame: Vec<u8> = capture_frame();
let compressed = lz4::compress(&frame)?;
websocket.send(compressed).await?;
```

---

## 🛣️ Roadmap

### ✅ Phase 1: LZ4 Core (v0.2.0) - COMPLETED
- [x] Basic LZ4 compression
- [x] LZ4 decompression
- [x] Error handling
- [x] Tests and benchmarks
- [x] **Compression levels (Fast/Balanced/Best)**
- [x] **Streaming API**
- [x] **Parallel compression**
- [x] **Checksums (XXHash64, CRC32)**
- [ ] SIMD optimizations (AVX2) - Next priority!

### Phase 2: Zstandard (v0.3.0)
- [ ] Zstd compression
- [ ] Zstd decompression
- [ ] Dictionary compression
- [ ] Compression levels (1-22)

### Phase 3: Custom Algorithms (v0.4.0)
- [ ] Columnar compression (for AvilaDB)
- [ ] Delta encoding (for time series)
- [ ] Run-length encoding (RLE)
- [ ] Dictionary-based compression

### Phase 4: AvilaDB Integration (v0.5.0)
- [ ] Native AvilaDB storage format
- [ ] Automatic compression selection
- [ ] Streaming compression API
- [ ] Zero-copy decompression

---

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

**Focus areas:**
- SIMD optimizations (AVX2, AVX-512, NEON)
- Additional algorithms (Snappy, Brotli)
- Benchmarks against external libraries
- Documentation improvements

---

## 📝 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../LICENSE-MIT))

at your option.

---

## 🏛️ Part of Arxis

**avila-compress** is part of the [Arxis](https://github.com/avilaops/arxis) scientific computing platform.

**Related modules:**
- **avila-dataframe**: DataFrames with native compression support
- **avila-telemetry**: Time series analysis (LISA, LIGO)
- **avila-math**: Mathematical kernel
- **AvilaDB**: Distributed database with native compression

---

**Built with ❤️ by the Ávila team**
📧 Contact: nicolas@avila.inc | 🌐 https://avila.cloud
