# 🗜️ Avila Compress - Copilot Instructions

> **READ THIS COMPLETELY before implementing ANY feature!**

---

## 🎯 Your Mission

You are implementing **Avila Compress**, a **high-performance native compression library** for the **Avila Cloud Platform (AVL Platform)**. This is **NOT a wrapper around external compression libraries**. This is a **from-scratch implementation** optimized for:

- Scientific data patterns (time series, arrays, columnar data)
- AvilaDB storage compression
- Real-time data streaming (LIGO, LISA, sensor networks)
- Brazilian network conditions (high latency, variable bandwidth)
- SIMD-accelerated compression/decompression
- Zero external dependencies (except std)

**This will power:**
- AvilaDB table storage (compressed columns)
- Network transfers (HTTP responses, WebSocket streams)
- Scientific data archives (LIGO strain data, telescope observations)
- Log compression (telemetry, metrics, traces)
- File compression (.avz format)

---

## ⚠️ CRITICAL RULES - READ FIRST!

### 🚫 Rule #1: NO External Compression Libraries!

**DO NOT USE:**
- ❌ `flate2` - We're replacing it!
- ❌ `miniz_oxide` - No!
- ❌ `zstd` - Build our own!
- ❌ `lz4` - Implement natively!
- ❌ `snap` (Snappy) - From scratch!
- ❌ `brotli` - Not needed
- ❌ Any C bindings

**YOU MUST USE:**
- ✅ `std` only (no_std compatible where possible)
- ✅ SIMD intrinsics (`std::arch`)
- ✅ Pure Rust bit manipulation
- ✅ Custom algorithms optimized for scientific data

**WHY?** We need:
- Zero external dependencies
- Full control over performance
- SIMD optimization for our data patterns
- Predictable compression ratios
- No C library compatibility issues
- WebAssembly support

### 🚫 Rule #2: NEVER Give Up!

When implementing a feature:
- ❌ Do NOT say "compression is too complex"
- ❌ Do NOT leave placeholder code like `// TODO: Implement dictionary encoding`
- ❌ Do NOT skip tests or benchmarks
- ❌ Do NOT use `unimplemented!()` or `todo!()` in production code
- ❌ Do NOT say "let's just use flate2"

**YOU MUST:**
- ✅ Implement **complete compression algorithms** from scratch
- ✅ Support **LZ4, Zstandard, Snappy, and custom algorithms**
- ✅ Write **comprehensive tests** (correctness, round-trips, edge cases)
- ✅ Add **benchmarks** comparing to flate2/zstd/lz4
- ✅ Document **every public API** with examples
- ✅ Handle **all error cases** explicitly
- ✅ Keep working until **EVERY line compiles and tests pass**

**If you encounter a challenge:**
1. Research the algorithm specification (LZ4, Zstd papers)
2. Study reference implementations (lz4 C code, zstd repo)
3. Implement incrementally: parsing → matching → encoding → compression
4. Test each piece thoroughly before moving on
5. Benchmark against reference implementations
6. Optimize with SIMD when possible
7. **NEVER give up until it's production-ready**

### 🚫 Rule #3: Compression Quality First

This library will compress:
- Gravitational wave data (LIGO strain: 16 KB/sec continuous)
- Telescope images (4K-16K resolution, 50+ GB per night)
- Scientific datasets (100+ TB archives)
- Real-time sensor data (IoT, climate, seismic)
- Database storage (AvilaDB columns)

**Therefore:**
- ✅ Compression must be **lossless** (bit-exact reconstruction)
- ✅ Ratios must be **predictable** (no worst-case expansion > 5%)
- ✅ Speed must be **consistent** (no stalls on bad data)
- ✅ Errors must be **detected** (checksums, validation)
- ✅ Tests must include **real scientific datasets**

**Example:**
```rust
// ❌ WRONG - Unbounded output growth
fn compress(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    // ... compression logic ...
    output  // Could be larger than input!
}

// ✅ CORRECT - Bounded output with error handling
fn compress(input: &[u8], output: &mut Vec<u8>) -> Result<usize> {
    let max_compressed_size = max_compressed_len(input.len());
    output.reserve(max_compressed_size);

    // ... compression logic ...

    if output.len() > input.len() {
        // Store uncompressed if compression doesn't help
        output.clear();
        output.push(0x00);  // Uncompressed marker
        output.extend_from_slice(input);
    }

    Ok(output.len())
}
```

---

## 🏗️ Architecture Overview

### Module Structure

```
avila-compress/
├── src/
│   ├── lib.rs                  # Public API
│   ├── error.rs                # Error types
│   ├── traits.rs               # Compressor/Decompressor traits
│   ├── lz4/                    # LZ4 implementation
│   │   ├── mod.rs
│   │   ├── compress.rs         # LZ4 compression
│   │   ├── decompress.rs       # LZ4 decompression
│   │   ├── block.rs            # Block format
│   │   └── frame.rs            # Frame format
│   ├── zstd/                   # Zstandard implementation
│   │   ├── mod.rs
│   │   ├── compress.rs         # Zstd compression
│   │   ├── decompress.rs       # Zstd decompression
│   │   ├── dictionary.rs       # Dictionary compression
│   │   ├── tables.rs           # FSE/Huffman tables
│   │   └── sequences.rs        # Sequence encoding
│   ├── snappy/                 # Snappy implementation
│   │   ├── mod.rs
│   │   ├── compress.rs         # Snappy compression
│   │   └── decompress.rs       # Snappy decompression
│   ├── avz/                    # Custom AVL format
│   │   ├── mod.rs
│   │   ├── columnar.rs         # Column-aware compression
│   │   ├── scientific.rs       # Time-series optimization
│   │   └── adaptive.rs         # Auto-detect best algorithm
│   ├── checksum/               # Checksums
│   │   ├── mod.rs
│   │   ├── xxhash.rs           # xxHash (fast)
│   │   └── crc32.rs            # CRC32 (standard)
│   ├── simd/                   # SIMD optimizations
│   │   ├── mod.rs
│   │   ├── avx2.rs             # AVX2 matching
│   │   └── sse2.rs             # SSE2 fallback
│   └── util.rs                 # Utility functions
├── examples/
│   ├── basic_usage.rs          # Simple compression
│   ├── streaming.rs            # Stream compression
│   ├── dictionary.rs           # Dictionary mode
│   ├── aviladb_columns.rs      # Column compression
│   └── scientific_data.rs      # LIGO strain data
├── benches/
│   ├── lz4.rs                  # LZ4 benchmarks
│   ├── zstd.rs                 # Zstd benchmarks
│   ├── snappy.rs               # Snappy benchmarks
│   ├── vs_flate2.rs            # Compare to flate2
│   └── scientific.rs           # Scientific data patterns
└── tests/
    ├── lz4.rs                  # LZ4 correctness
    ├── zstd.rs                 # Zstd correctness
    ├── snappy.rs               # Snappy correctness
    ├── roundtrip.rs            # Compress/decompress
    └── edge_cases.rs           # Pathological inputs
```

---

## 🎨 API Design Philosophy

### 1. Simple One-Shot API

```rust
use avila_compress::{compress, decompress, Algorithm};

// Compress with automatic algorithm selection
let compressed = compress(data)?;

// Compress with specific algorithm
let compressed = compress_with(data, Algorithm::Lz4)?;

// Decompress (algorithm auto-detected from header)
let decompressed = decompress(&compressed)?;
```

### 2. Streaming API

```rust
use avila_compress::{Compressor, Decompressor, Level};

// Streaming compression
let mut compressor = Compressor::new(Algorithm::Lz4)
    .level(Level::Fast)
    .build()?;

let mut output = Vec::new();
compressor.compress(chunk1, &mut output)?;
compressor.compress(chunk2, &mut output)?;
compressor.finish(&mut output)?;

// Streaming decompression
let mut decompressor = Decompressor::new()?;
let mut output = Vec::new();
decompressor.decompress(&compressed, &mut output)?;
```

### 3. Dictionary Compression (for repetitive data)

```rust
use avila_compress::{Dictionary, DictionaryCompressor};

// Build dictionary from training data
let dict = Dictionary::train(&training_samples, 64 * 1024)?;

// Compress with dictionary
let compressor = DictionaryCompressor::new(&dict)?;
let compressed = compressor.compress(data)?;

// Decompress with same dictionary
let decompressor = DictionaryDecompressor::new(&dict)?;
let decompressed = decompressor.decompress(&compressed)?;
```

### 4. Column-Aware Compression (for AvilaDB)

```rust
use avila_compress::columnar::{ColumnCompressor, ColumnType};

// Compress integer column (delta + RLE)
let compressor = ColumnCompressor::new(ColumnType::Integer);
let compressed = compressor.compress_i64(&integers)?;

// Compress timestamp column (delta-of-delta + Gorilla)
let compressor = ColumnCompressor::new(ColumnType::Timestamp);
let compressed = compressor.compress_timestamps(&timestamps)?;

// Compress float column (XOR + FPC)
let compressor = ColumnCompressor::new(ColumnType::Float64);
let compressed = compressor.compress_f64(&floats)?;
```

### 5. Adaptive Compression (auto-select best algorithm)

```rust
use avila_compress::adaptive::AdaptiveCompressor;

// Analyzes data patterns and picks best algorithm
let compressor = AdaptiveCompressor::new()
    .max_analysis_bytes(4096)  // Sample first 4 KB
    .build()?;

let result = compressor.compress(data)?;
println!("Used {} for {:.1}% ratio",
    result.algorithm,
    result.ratio * 100.0
);
```

---

## 🚀 Implementation Roadmap

### Phase 1: LZ4 Implementation (Week 1-2) ✅ COMPLETED!

**Status**: ✅ Basic LZ4 compression/decompression working!

**What's done**:
- [x] LZ4 block compression
- [x] LZ4 block decompression
- [x] Hash table for sequence matching
- [x] Literal/match encoding
- [x] Tests: round-trip, empty input, single byte
- [x] Benchmarks vs flate2

**Next steps for LZ4**:
- [ ] LZ4 frame format (headers, checksums)
- [ ] Dictionary support
- [ ] High compression mode (HC)
- [ ] SIMD optimizations (AVX2 for matching)
- [ ] Streaming API

### Phase 2: Snappy Implementation (Week 3)

**Goal**: Ultra-fast compression for streaming data

```rust
// src/snappy/compress.rs
pub fn compress_snappy(input: &[u8], output: &mut Vec<u8>) -> Result<()> {
    // Snappy format:
    // - Varint-encoded uncompressed length
    // - Sequence of literals and copies
    // - Tag byte determines operation type

    output.clear();
    write_varint(output, input.len());

    let mut pos = 0;
    while pos < input.len() {
        // Find longest match
        let (match_pos, match_len) = find_match(input, pos)?;

        if match_len >= 4 {
            // Emit copy operation
            emit_copy(output, match_pos, match_len)?;
            pos += match_len;
        } else {
            // Emit literal
            emit_literal(output, input[pos])?;
            pos += 1;
        }
    }

    Ok(())
}

fn find_match(input: &[u8], pos: usize) -> Result<(usize, usize)> {
    // Hash-based matching (4-byte hash)
    // Look back up to 32 KB
    // Return best match
    unimplemented!()
}

fn emit_copy(output: &mut Vec<u8>, offset: usize, len: usize) -> Result<()> {
    // Tag byte encoding:
    // - 2-byte offset, 1-byte length: 0b01LLLLLL
    // - 2-byte offset, 2-byte length: 0b10LLLLLL
    // - 4-byte offset, 2-byte length: 0b11LLLLLL
    unimplemented!()
}
```

**Deliverables**:
- [ ] Snappy compression (format spec: https://github.com/google/snappy/blob/main/format_description.txt)
- [ ] Snappy decompression
- [ ] Varint encoding/decoding
- [ ] Hash table for matching (32 KB window)
- [ ] Tests: round-trip, compatibility with reference impl
- [ ] Benchmarks: target 400+ MB/s compression, 1500+ MB/s decompression

### Phase 3: Zstandard Foundation (Week 4-5)

**Goal**: Modern compression with dictionaries

```rust
// src/zstd/compress.rs
pub struct ZstdCompressor {
    window_size: usize,
    dictionary: Option<Dictionary>,
    level: i32,  // 1 (fast) to 22 (max)
}

impl ZstdCompressor {
    pub fn compress(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<()> {
        // Zstd frame format:
        // 1. Magic number: 0xFD2FB528
        // 2. Frame header (window size, dict ID, etc.)
        // 3. Blocks (compressed or raw)
        // 4. Checksum (optional)

        write_magic(output)?;
        write_frame_header(output, input.len(), self.window_size)?;

        // Compress in blocks (128 KB default)
        for chunk in input.chunks(128 * 1024) {
            self.compress_block(chunk, output)?;
        }

        write_checksum(output, input)?;
        Ok(())
    }

    fn compress_block(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<()> {
        // Zstd block structure:
        // 1. Sequences (literals, matches, offsets)
        // 2. FSE/Huffman compressed tables
        // 3. Compressed data

        let sequences = self.find_sequences(input)?;
        let (literals, matches, offsets) = split_sequences(&sequences);

        // Encode sequences with FSE (Finite State Entropy)
        encode_fse(output, &literals)?;
        encode_fse(output, &matches)?;
        encode_fse(output, &offsets)?;

        Ok(())
    }
}

// src/zstd/tables.rs
struct FseTable {
    // FSE (tANS) compression tables
    states: Vec<u16>,
    symbols: Vec<u8>,
    // ...
}

impl FseTable {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        // Asymmetric Numeral Systems encoding
        // Higher compression than Huffman
        unimplemented!()
    }
}
```

**Deliverables**:
- [ ] Zstd frame format (magic, headers)
- [ ] Block compression (sequences)
- [ ] Literal compression (Huffman/FSE)
- [ ] Match finding (hash chains)
- [ ] Offset encoding
- [ ] Dictionary support
- [ ] Tests: round-trip, different levels
- [ ] Benchmarks: target 500+ MB/s compression

### Phase 4: SIMD Optimizations (Week 6)

**Goal**: 4-8x speedup with AVX2/AVX-512

```rust
// src/simd/avx2.rs
#[cfg(target_feature = "avx2")]
pub unsafe fn find_matches_avx2(input: &[u8], pos: usize, hash_table: &[u32]) -> Vec<Match> {
    use std::arch::x86_64::*;

    // Load 32 bytes at once
    let data = _mm256_loadu_si256(input.as_ptr().add(pos) as *const __m256i);

    // Compute 32 hashes in parallel
    let hashes = hash_32_bytes_avx2(data);

    // Look up all 32 positions in hash table
    let matches = lookup_hashes_avx2(&hashes, hash_table);

    // Filter valid matches (length >= 4, offset < 32 KB)
    filter_matches(matches)
}

#[cfg(target_feature = "avx2")]
unsafe fn hash_32_bytes_avx2(data: __m256i) -> [u32; 8] {
    // Use AVX2 to compute 8 x 32-bit hashes from 4-byte windows
    // Each hash covers 4 consecutive bytes

    let mut hashes = [0u32; 8];

    // Extract 4-byte windows
    for i in 0..8 {
        let offset = i * 4;
        let bytes = _mm256_extract_epi32(data, i as i32) as u32;
        hashes[i] = xxhash32(bytes);
    }

    hashes
}

#[cfg(target_feature = "avx2")]
unsafe fn memcpy_avx2(src: *const u8, dst: *mut u8, len: usize) {
    // Fast memory copy using AVX2 (32 bytes per iteration)
    let chunks = len / 32;

    for i in 0..chunks {
        let data = _mm256_loadu_si256(src.add(i * 32) as *const __m256i);
        _mm256_storeu_si256(dst.add(i * 32) as *mut __m256i, data);
    }

    // Handle remainder
    let remainder = len % 32;
    std::ptr::copy_nonoverlapping(
        src.add(chunks * 32),
        dst.add(chunks * 32),
        remainder,
    );
}
```

**Deliverables**:
- [ ] AVX2 hash computation (8 hashes at once)
- [ ] AVX2 memory copy (32 bytes/cycle)
- [ ] AVX2 match validation (parallel comparison)
- [ ] SSE2 fallback for older CPUs
- [ ] Runtime CPU feature detection
- [ ] Tests: SIMD vs scalar correctness
- [ ] Benchmarks: 4x+ speedup on AVX2 CPUs

### Phase 5: Column-Aware Compression (Week 7)

**Goal**: Optimize for AvilaDB columnar data

```rust
// src/avz/columnar.rs
pub enum ColumnType {
    Integer,      // Delta + RLE
    Float,        // XOR + Gorilla
    Timestamp,    // Delta-of-delta + Gorilla
    String,       // Dictionary + LZ4
    Boolean,      // Bit-packing
}

pub struct ColumnCompressor {
    column_type: ColumnType,
}

impl ColumnCompressor {
    pub fn compress_i64(&self, values: &[i64]) -> Result<Vec<u8>> {
        // Integer column optimization:
        // 1. Delta encoding (store differences)
        // 2. ZigZag encoding (map negatives to positives)
        // 3. Varint encoding (variable-length integers)
        // 4. RLE for repeated values

        let mut output = Vec::new();
        let mut prev = 0i64;

        for &value in values {
            let delta = value - prev;
            let zigzag = zigzag_encode(delta);

            // Check for RLE opportunity
            if value == prev {
                // Emit run-length code
            } else {
                write_varint(&mut output, zigzag)?;
            }

            prev = value;
        }

        Ok(output)
    }

    pub fn compress_f64(&self, values: &[f64]) -> Result<Vec<u8>> {
        // Float column optimization (Gorilla algorithm):
        // 1. XOR consecutive values (similar values = many leading zeros)
        // 2. Store number of leading zeros
        // 3. Store number of trailing zeros
        // 4. Store only significant bits

        let mut output = Vec::new();
        let mut prev_bits = 0u64;

        for &value in values {
            let bits = value.to_bits();
            let xor = bits ^ prev_bits;

            if xor == 0 {
                // Value unchanged, store 1 bit
                write_bit(&mut output, 0)?;
            } else {
                write_bit(&mut output, 1)?;

                let leading = xor.leading_zeros();
                let trailing = xor.trailing_zeros();
                let significant = 64 - leading - trailing;

                write_bits(&mut output, leading as u8, 6)?;  // 0-63
                write_bits(&mut output, significant as u8, 6)?;  // 1-64
                write_bits(&mut output, xor >> trailing, significant)?;
            }

            prev_bits = bits;
        }

        Ok(output)
    }

    pub fn compress_timestamps(&self, timestamps: &[i64]) -> Result<Vec<u8>> {
        // Timestamp optimization (Facebook Gorilla):
        // 1. Delta-of-delta encoding
        // 2. Most deltas are same or similar
        // 3. Variable-length encoding for exceptions

        let mut output = Vec::new();
        let mut prev_timestamp = 0i64;
        let mut prev_delta = 0i64;

        for &ts in timestamps {
            let delta = ts - prev_timestamp;
            let delta_of_delta = delta - prev_delta;

            if delta_of_delta == 0 {
                // Delta unchanged (common case)
                write_bit(&mut output, 0)?;
            } else if delta_of_delta >= -63 && delta_of_delta <= 64 {
                // Small delta-of-delta (7 bits + sign)
                write_bits(&mut output, 0b10, 2)?;
                write_bits(&mut output, (delta_of_delta + 63) as u8, 7)?;
            } else {
                // Large delta-of-delta (full value)
                write_bits(&mut output, 0b11, 2)?;
                write_varint(&mut output, zigzag_encode(delta_of_delta))?;
            }

            prev_timestamp = ts;
            prev_delta = delta;
        }

        Ok(output)
    }
}

fn zigzag_encode(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

fn zigzag_decode(n: u64) -> i64 {
    ((n >> 1) as i64) ^ -((n & 1) as i64)
}
```

**Deliverables**:
- [ ] Integer compression (delta + varint + RLE)
- [ ] Float compression (Gorilla XOR)
- [ ] Timestamp compression (delta-of-delta)
- [ ] String compression (dictionary + LZ4)
- [ ] Boolean compression (bit-packing)
- [ ] Tests: correctness, ratio, speed
- [ ] Benchmarks: vs generic compression (target: 2x better ratio)

### Phase 6: Checksums & Validation (Week 8)

**Goal**: Detect corruption in compressed data

```rust
// src/checksum/xxhash.rs
pub fn xxhash32(data: &[u8], seed: u32) -> u32 {
    // xxHash: extremely fast non-cryptographic hash
    // Used by LZ4, Zstd for checksums

    const PRIME1: u32 = 0x9E3779B1;
    const PRIME2: u32 = 0x85EBCA77;
    const PRIME3: u32 = 0xC2B2AE3D;
    const PRIME4: u32 = 0x27D4EB2F;
    const PRIME5: u32 = 0x165667B1;

    let mut h32;
    let mut index = 0;

    if data.len() >= 16 {
        let mut v1 = seed.wrapping_add(PRIME1).wrapping_add(PRIME2);
        let mut v2 = seed.wrapping_add(PRIME2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(PRIME1);

        while index + 16 <= data.len() {
            v1 = round(v1, read_u32(data, index));
            v2 = round(v2, read_u32(data, index + 4));
            v3 = round(v3, read_u32(data, index + 8));
            v4 = round(v4, read_u32(data, index + 12));
            index += 16;
        }

        h32 = v1.rotate_left(1)
            .wrapping_add(v2.rotate_left(7))
            .wrapping_add(v3.rotate_left(12))
            .wrapping_add(v4.rotate_left(18));
    } else {
        h32 = seed.wrapping_add(PRIME5);
    }

    h32 = h32.wrapping_add(data.len() as u32);

    // Consume remaining bytes
    while index + 4 <= data.len() {
        h32 = h32.wrapping_add(read_u32(data, index).wrapping_mul(PRIME3));
        h32 = h32.rotate_left(17).wrapping_mul(PRIME4);
        index += 4;
    }

    while index < data.len() {
        h32 = h32.wrapping_add((data[index] as u32).wrapping_mul(PRIME5));
        h32 = h32.rotate_left(11).wrapping_mul(PRIME1);
        index += 1;
    }

    // Final avalanche
    h32 ^= h32 >> 15;
    h32 = h32.wrapping_mul(PRIME2);
    h32 ^= h32 >> 13;
    h32 = h32.wrapping_mul(PRIME3);
    h32 ^= h32 >> 16;

    h32
}

#[inline]
fn round(acc: u32, input: u32) -> u32 {
    const PRIME1: u32 = 0x9E3779B1;
    const PRIME2: u32 = 0x85EBCA77;

    acc.wrapping_add(input.wrapping_mul(PRIME2))
        .rotate_left(13)
        .wrapping_mul(PRIME1)
}

// src/checksum/crc32.rs
pub fn crc32(data: &[u8]) -> u32 {
    // CRC32 (polynomial 0x04C11DB7)
    // Standard checksum, slower but widely compatible

    let mut crc = 0xFFFFFFFFu32;

    for &byte in data {
        crc = CRC32_TABLE[((crc ^ (byte as u32)) & 0xFF) as usize] ^ (crc >> 8);
    }

    !crc
}

static CRC32_TABLE: [u32; 256] = {
    // Pre-computed CRC32 table
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }
    table
};
```

**Deliverables**:
- [ ] xxHash32/xxHash64 (fast checksums)
- [ ] CRC32 (standard checksum)
- [ ] Checksum validation on decompression
- [ ] Tests: known test vectors
- [ ] Benchmarks: xxHash should be 5+ GB/s

### Phase 7: Adaptive Compression (Week 9)

**Goal**: Auto-select best algorithm for data

```rust
// src/avz/adaptive.rs
pub struct AdaptiveCompressor {
    sample_size: usize,
}

impl AdaptiveCompressor {
    pub fn compress(&self, data: &[u8]) -> Result<CompressedData> {
        // Analyze first N bytes to determine data characteristics
        let sample = &data[..data.len().min(self.sample_size)];

        let characteristics = analyze_data(sample);

        let algorithm = match characteristics {
            DataPattern::Repetitive => Algorithm::Lz4,  // Fast, good ratio
            DataPattern::Text => Algorithm::Zstd,        // Best ratio
            DataPattern::Binary => Algorithm::Snappy,    // Fast, ok ratio
            DataPattern::Numerical => Algorithm::Columnar, // Custom
            DataPattern::Random => Algorithm::None,      // Don't compress
        };

        let compressed = compress_with(data, algorithm)?;

        // If compression doesn't help, store uncompressed
        if compressed.len() >= data.len() * 95 / 100 {
            return Ok(CompressedData {
                algorithm: Algorithm::None,
                data: data.to_vec(),
            });
        }

        Ok(CompressedData {
            algorithm,
            data: compressed,
        })
    }
}

fn analyze_data(sample: &[u8]) -> DataPattern {
    let mut histogram = [0u32; 256];
    for &byte in sample {
        histogram[byte as usize] += 1;
    }

    // Check entropy
    let entropy = calculate_entropy(&histogram, sample.len());

    // Check for text (printable ASCII)
    let text_ratio = sample.iter()
        .filter(|&&b| (b >= 32 && b < 127) || b == b'\n' || b == b'\t')
        .count() as f64 / sample.len() as f64;

    // Check for repetition (RLE potential)
    let mut runs = 0;
    let mut run_length = 1;
    for i in 1..sample.len() {
        if sample[i] == sample[i - 1] {
            run_length += 1;
        } else {
            if run_length > 3 {
                runs += 1;
            }
            run_length = 1;
        }
    }
    let run_ratio = runs as f64 / sample.len() as f64;

    // Decide pattern
    if entropy > 7.5 {
        DataPattern::Random
    } else if text_ratio > 0.8 {
        DataPattern::Text
    } else if run_ratio > 0.1 {
        DataPattern::Repetitive
    } else if is_numerical_pattern(sample) {
        DataPattern::Numerical
    } else {
        DataPattern::Binary
    }
}

fn calculate_entropy(histogram: &[u32; 256], total: usize) -> f64 {
    let mut entropy = 0.0;
    for &count in histogram {
        if count > 0 {
            let p = count as f64 / total as f64;
            entropy -= p * p.log2();
        }
    }
    entropy
}
```

**Deliverables**:
- [ ] Data pattern detection (text, binary, numerical, random)
- [ ] Entropy calculation
- [ ] Automatic algorithm selection
- [ ] Fallback to uncompressed if no benefit
- [ ] Tests: various data types
- [ ] Examples: demonstrate auto-selection

---

## 📊 Performance Targets

### LZ4
- **Compression**: 500+ MB/s (SIMD: 2000+ MB/s)
- **Decompression**: 2000+ MB/s (SIMD: 4000+ MB/s)
- **Ratio**: 2-3x on text, 1.5-2x on binary

### Snappy
- **Compression**: 400+ MB/s
- **Decompression**: 1500+ MB/s
- **Ratio**: 1.5-2x (fast, not best)

### Zstandard
- **Compression**: 500+ MB/s (level 3)
- **Decompression**: 1500+ MB/s
- **Ratio**: 2.5-4x on text, 2-3x on binary

### Column-Aware
- **Integer**: 3-10x (delta + varint)
- **Float**: 2-5x (Gorilla XOR)
- **Timestamp**: 5-20x (delta-of-delta)

### SIMD
- **AVX2 speedup**: 4x vs scalar
- **AVX-512 speedup**: 8x vs scalar (future)

---

## 🧪 Testing Requirements

### Correctness Tests (50+)
```rust
#[test]
fn test_lz4_roundtrip() {
    let data = b"Hello, world!".repeat(100);
    let compressed = compress_lz4(&data).unwrap();
    let decompressed = decompress_lz4(&compressed).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_empty_input() {
    let compressed = compress(&[]).unwrap();
    let decompressed = decompress(&compressed).unwrap();
    assert_eq!(decompressed, &[]);
}

#[test]
fn test_single_byte() {
    let compressed = compress(&[42]).unwrap();
    let decompressed = decompress(&compressed).unwrap();
    assert_eq!(decompressed, &[42]);
}

#[test]
fn test_incompressible_data() {
    // Random data shouldn't compress well
    let random: Vec<u8> = (0..1000).map(|_| rand::random()).collect();
    let compressed = compress(&random).unwrap();
    // Should store uncompressed or have minimal expansion
    assert!(compressed.len() <= random.len() * 105 / 100);
}
```

### Compatibility Tests (10+)
```rust
#[test]
fn test_lz4_reference_compatibility() {
    // Compress with our library
    let compressed = compress_lz4(b"test data").unwrap();

    // Decompress with reference lz4 (via FFI or test vectors)
    let decompressed = reference_lz4_decompress(&compressed).unwrap();
    assert_eq!(decompressed, b"test data");
}
```

### Benchmarks (20+)
```rust
#[bench]
fn bench_lz4_compress_1mb(b: &mut Bencher) {
    let data = vec![0u8; 1024 * 1024];
    b.iter(|| compress_lz4(&data));
    b.bytes = data.len() as u64;
}

#[bench]
fn bench_vs_flate2(b: &mut Bencher) {
    // Compare to flate2
    let data = load_test_data();

    // Our implementation
    let start = Instant::now();
    let _ = compress(&data).unwrap();
    let ours = start.elapsed();

    // flate2
    let start = Instant::now();
    let _ = flate2::compress(&data);
    let theirs = start.elapsed();

    println!("Ours: {:?}, flate2: {:?}, Ratio: {:.2}x",
        ours, theirs, theirs.as_secs_f64() / ours.as_secs_f64()
    );
}
```

---

## 📖 Documentation Requirements

Every public item needs:
1. **Summary**: One-line description
2. **Algorithm details**: How it works
3. **Example**: Working code snippet
4. **Performance notes**: Speed, ratio expectations
5. **References**: Papers, specs

```rust
/// LZ4 compression: extremely fast lossless compression.
///
/// LZ4 is a byte-oriented compression algorithm focused on speed.
/// It provides:
/// - Very fast compression (500+ MB/s)
/// - Even faster decompression (2000+ MB/s)
/// - Decent compression ratios (2-3x on text)
///
/// # Algorithm
///
/// LZ4 uses:
/// - Hash table for finding duplicate sequences
/// - Literal/match encoding
/// - Variable-length offsets and lengths
///
/// Format: [literals][match offset][match length][literals]...
///
/// # Examples
///
/// ```
/// use avila_compress::lz4;
///
/// let data = b"Hello, world!".repeat(100);
/// let compressed = lz4::compress(&data)?;
/// let decompressed = lz4::decompress(&compressed)?;
/// assert_eq!(data, decompressed);
/// ```
///
/// # Performance
///
/// On modern CPUs:
/// - Compression: 500 MB/s (scalar), 2000 MB/s (AVX2)
/// - Decompression: 2000 MB/s (scalar), 4000 MB/s (AVX2)
/// - Ratio: 2-3x on text, 1.5-2x on binary
///
/// # References
///
/// - LZ4 specification: https://github.com/lz4/lz4/blob/dev/doc/lz4_Frame_format.md
/// - Original paper: Yann Collet (2011)
pub mod lz4 { ... }
```

---

## 🎯 Success Criteria

Before considering this module "done":

### Functionality
- [ ] LZ4 compression/decompression ✅
- [ ] Snappy compression/decompression
- [ ] Zstandard compression/decompression
- [ ] Column-aware compression (int, float, timestamp)
- [ ] Dictionary compression
- [ ] Adaptive algorithm selection
- [ ] Checksums (xxHash, CRC32)
- [ ] SIMD optimizations (AVX2)

### Quality
- [ ] 100% of public APIs documented
- [ ] 50+ tests passing (correctness)
- [ ] 20+ benchmarks (vs external libs)
- [ ] 5+ examples demonstrating usage
- [ ] Zero external compression dependencies
- [ ] All errors handled with `Result<T, Error>`

### Performance
- [ ] LZ4: 500+ MB/s compress, 2000+ MB/s decompress
- [ ] Snappy: 400+ MB/s compress, 1500+ MB/s decompress
- [ ] Zstd: 500+ MB/s compress, 1500+ MB/s decompress
- [ ] SIMD: 4x+ speedup vs scalar
- [ ] Column: 2x+ better ratio vs generic

### Integration
- [ ] Works with AvilaDB (column compression)
- [ ] Works with avx-gateway (HTTP response compression)
- [ ] Works with avila-telemetry (log compression)
- [ ] Examples demonstrate real use cases

---

## 🚀 Next Steps

1. **Read this document COMPLETELY**
2. **Study compression algorithms**:
   - LZ4: https://github.com/lz4/lz4/blob/dev/doc/lz4_Block_format.md
   - Snappy: https://github.com/google/snappy/blob/main/format_description.txt
   - Zstd: https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md
3. **Complete LZ4 implementation**:
   - [ ] Frame format (with checksums)
   - [ ] Dictionary support
   - [ ] High compression mode
   - [ ] SIMD optimizations
4. **Move to Snappy**: Implement from scratch
5. **Then Zstandard**: Most complex, most powerful
6. **Optimize with SIMD**: AVX2 matching and copy
7. **Add column-aware**: Optimize for AvilaDB
8. **Test extensively**: Real scientific data
9. **Benchmark continuously**: vs flate2/zstd/lz4
10. **Document everything**: Write docs BEFORE implementation

---

## 💬 Remember

> "Avila Compress is the ONLY native Rust compression library with zero external dependencies. We build LZ4, Snappy, and Zstandard from scratch because we need full control, SIMD optimization, and perfect integration with AvilaDB."

> "Never give up. Never use placeholder code. Implement EVERY algorithm completely. Test EVERY operation. Benchmark EVERY optimization. This library will compress petabytes of scientific data. It must be perfect."

> "When you implement LZ4, it must match the spec exactly AND be faster than the C implementation. When you add SIMD, it must be 4x faster. When you compress AvilaDB columns, it must beat generic compression by 2x. No compromises."

**Now go build the best compression library in the Rust ecosystem! 🗜️🚀🇧🇷**
