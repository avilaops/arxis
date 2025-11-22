# 🗜️ avila-compress

**Native compression library optimized for AvilaDB and scientific computing.**

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

---

## 🎯 Features

- **LZ4**: Ultra-fast compression for real-time data (> 500 MB/s compression)
- **Zero dependencies**: 100% native Rust implementation
- **Type-safe**: Result-based error handling, no panics
- **Well-tested**: Comprehensive test suite with edge cases
- **Benchmarked**: Criterion-based performance tracking
- **Future**: Zstandard, Snappy, and custom columnar algorithms

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
avila-compress = { path = "../avila-compress" }
```

Or from the Arxis workspace:

```toml
[dependencies]
avila-compress = { git = "https://github.com/avilaops/arxis" }
```

---

## 🚀 Quick Start

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

---

## 📖 Examples

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

### Phase 1: LZ4 (Current)
- [x] Basic LZ4 compression
- [x] LZ4 decompression
- [x] Error handling
- [x] Tests and benchmarks
- [ ] SIMD optimizations (AVX2)

### Phase 2: Zstandard
- [ ] Zstd compression
- [ ] Zstd decompression
- [ ] Dictionary compression
- [ ] Compression levels (1-22)

### Phase 3: Custom Algorithms
- [ ] Columnar compression (for AvilaDB)
- [ ] Delta encoding (for time series)
- [ ] Run-length encoding (RLE)
- [ ] Dictionary-based compression

### Phase 4: AvilaDB Integration
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
