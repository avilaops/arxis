# AvilaDB DataFrame - Development Guide

## 🎯 Project Goals

Create the **fastest and most feature-rich DataFrame library** for scientific computing, with unique capabilities nobody else has:

1. **Beat Polars** in performance benchmarks
2. **Native scientific types** (Quaternions, Tensor4D, Spinors, Geodesics)
3. **Built-in signal processing** (FFT, wavelets)
4. **GPU-first architecture**
5. **Brazilian cloud optimization** (AvilaDB integration)

## 📋 Implementation Roadmap

### Phase 1: Core Foundation ✅ (Current)

- [x] Project structure
- [x] Error handling
- [x] Basic DataFrame/Series
- [x] Scientific data types (Quaternion, SpinorWeyl, GeodesicCoord)
- [x] Expression system
- [ ] Arrow integration (in progress)

### Phase 2: Essential Operations

- [ ] Filter implementation
- [ ] Group by & aggregations
- [ ] Joins (inner, left, right, outer, cross)
- [ ] Sorting
- [ ] Window functions
- [ ] Pivot/unpivot

### Phase 3: I/O Layer

- [ ] Parquet read/write (Zstd, LZ4, Snappy compression)
- [ ] CSV read/write
- [ ] Arrow IPC
- [ ] HDF5 support
- [ ] FITS format (astronomy)
- [ ] NetCDF (climate science)
- [ ] AvilaDB native connector

### Phase 4: Scientific Operations (DIFFERENTIAL!)

- [ ] FFT (rustfft)
- [ ] Wavelets (CWT, DWT)
- [ ] Signal processing (filters, resampling)
- [ ] Statistical tests (KS, chi-square, Anderson-Darling)
- [ ] Time series (autocorr, cross-corr, seasonal decompose)
- [ ] Astronomy functions (redshift, luminosity, angular sep)

### Phase 5: Query Engines

- [ ] Lazy evaluation framework
- [ ] Query optimizer (predicate pushdown, projection pushdown)
- [ ] Physical plan executor
- [ ] SQL parser integration (DataFusion)
- [ ] Scientific SQL extensions

### Phase 6: Advanced Features

- [ ] GPU acceleration (CUDA/ROCm via cudarc)
- [ ] Distributed computing (cluster scheduler)
- [ ] Streaming data support
- [ ] Memory-mapped I/O
- [ ] SIMD vectorization

### Phase 7: Integrations

- [ ] AvilaDB native connector
- [ ] Cloud storage (S3, Azure, GCS)
- [ ] avila-ml integration
- [ ] Python bindings (PyO3)
- [ ] Visualization (plots, heatmaps)

## 🔬 Testing Strategy

### Unit Tests

Each module must have comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

Located in `tests/`:

- `test_dataframe.rs` - DataFrame operations
- `test_scientific.rs` - Scientific type operations
- `test_io.rs` - I/O operations
- `test_performance.rs` - Performance regression tests

### Benchmarks

Use Criterion for benchmarking:

```bash
cargo bench
```

Compare against Polars:

```bash
./scripts/compare_polars.sh
```

## 📊 Performance Guidelines

### Must Be Faster Than Polars

Every operation must meet or exceed Polars performance:

1. **Use Arrow efficiently** - Zero-copy operations
2. **SIMD vectorization** - Leverage portable_simd
3. **Parallel execution** - Use Rayon for parallelism
4. **Memory efficiency** - Minimize allocations
5. **Smart caching** - Cache intermediate results

### Profiling

```bash
# Profile with perf
cargo build --release
perf record --call-graph=dwarf ./target/release/examples/basic_usage
perf report

# Profile with flamegraph
cargo flamegraph --example basic_usage
```

## 🎨 Code Style

### Rust Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Document all public APIs

### Documentation

Every public item needs documentation:

```rust
/// Brief description
///
/// Longer explanation if needed
///
/// # Examples
///
/// ```
/// use avila_dataframe::prelude::*;
///
/// let df = DataFrame::new(...)?;
/// ```
///
/// # Errors
///
/// Returns error if...
pub fn function() -> Result<()> {
    // Implementation
}
```

## 🚀 Release Process

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Run full test suite**: `cargo test --all-features`
4. **Run benchmarks**: `cargo bench`
5. **Build docs**: `cargo doc --all-features`
6. **Tag release**: `git tag v0.1.0`
7. **Publish**: `cargo publish`

## 📚 Resources

### Arrow Ecosystem

- [Arrow Rust](https://github.com/apache/arrow-rs)
- [Parquet](https://github.com/apache/arrow-rs/tree/master/parquet)
- [DataFusion](https://github.com/apache/arrow-datafusion)

### Competitors (Learn from them!)

- [Polars](https://github.com/pola-rs/polars) - Main competitor
- [Pandas](https://pandas.pydata.org/) - Python reference
- [cuDF](https://github.com/rapidsai/cudf) - GPU DataFrames

### Scientific Computing

- [rustfft](https://github.com/ejmahler/RustFFT)
- [ndarray](https://github.com/rust-ndarray/ndarray)
- [nalgebra](https://github.com/dimforge/nalgebra)

## 🐛 Known Issues

Track issues in GitHub Issues with labels:

- `performance` - Performance improvements needed
- `bug` - Something isn't working
- `enhancement` - New feature request
- `scientific` - Scientific computing features
- `documentation` - Documentation improvements

## 💡 Design Decisions

### Why Arrow?

- Industry standard for columnar data
- Zero-copy interoperability
- Excellent compression
- Rich ecosystem

### Why Not X?

- **Not using ndarray for core** - Arrow is better for DataFrames
- **Not using DataFusion directly** - Need custom optimizations
- **Not using cuDF** - Want pure Rust, portable solution

### Scientific Types Decision

Added Quaternions, Spinors, Geodesics because:

1. **Real use cases** - LISA mission, particle physics, robotics
2. **Performance** - Native types avoid conversions
3. **Competitive advantage** - Nobody else has this
4. **Brazilian science** - Support local research (LIGO-BR, INPE)

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📧 Contact

- Engineering: engineering@avila.cloud
- Documentation: docs@avila.cloud
- Security: security@avila.cloud

---

**Let's build the best DataFrame library in the world!** 🚀🇧🇷
