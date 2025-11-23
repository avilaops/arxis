# Avila Platform

[![Crates.io](https://img.shields.io/crates/v/avila.svg)](https://crates.io/crates/avila)
[![Documentation](https://docs.rs/avila/badge.svg)](https://docs.rs/avila)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

High-performance cloud platform optimized for Brazil and LATAM 🇧🇷

## Overview

**Avila** is a comprehensive cloud platform built in Rust, designed to provide:

- 🚀 **High Performance**: SIMD-optimized operations, zero-copy processing
- 🌎 **LATAM-First**: Optimized for Brazilian and Latin American infrastructure
- 🔒 **Enterprise Grade**: Production-ready components with security audits
- 📊 **Data Processing**: Arrow-based columnar data processing
- 🤖 **Machine Learning**: Clustering, classification, and ML primitives
- 🌐 **Networking**: High-performance HTTP client/server

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
avila = "0.1"
```

Or install with specific features:

```toml
[dependencies]
avila = { version = "0.1", features = ["full"] }
```

## Features

- **`compression`** - High-performance compression (SIMD, columnar)
- **`clustering`** - Advanced clustering algorithms (K-means, DBSCAN, Hierarchical)
- **`math`** - Mathematical operations and linear algebra
- **`data`** - Arrow-based data processing
- **`http`** - High-performance HTTP client/server
- **`cli`** - Command-line interface tools
- **`telemetry`** - Observability and monitoring
- **`full`** - Enable all features

## Examples

### Compression

```rust
use avila::compress::{compress_zstd, decompress_zstd};

let data = b"Hello, Avila Platform!";
let compressed = compress_zstd(data, 3)?;
let decompressed = decompress_zstd(&compressed)?;
```

### Clustering

```rust
use avila::clustering::kmeans::KMeans;

let data = vec![
    vec![1.0, 2.0],
    vec![2.0, 3.0],
    vec![10.0, 11.0],
];

let kmeans = KMeans::new(2, 100);
let labels = kmeans.fit(&data)?;
```

### HTTP

```rust
use avila::http::Client;

let client = Client::new();
let response = client.get("https://avila.cloud").await?;
```

## Components

| Crate              | Description                  | Version |
| ------------------ | ---------------------------- | ------- |
| `avila-compress`   | High-performance compression | 0.8.0   |
| `avila-clustering` | Clustering algorithms        | 0.1.0   |
| `avila-math`       | Mathematical operations      | 0.1.0   |
| `avila-linalg`     | Linear algebra               | 0.1.1   |
| `avila-arrow`      | Arrow data processing        | 0.1.0   |
| `avx-http`         | HTTP client/server           | 0.2.0   |
| `avx-cli`          | CLI tools                    | 0.1.0   |

## Platform Components

### AvilaDB
Multi-model distributed database with:
- 🌍 Global distribution with multi-region writes
- ⚡ Sub-10ms latency in Brazil
- 📄 4 MB document size (vs 400 KB DynamoDB, 2 MB Cosmos DB)
- 💰 40-60% cheaper than AWS/Azure for Brazilian workloads

### AVL Services
- **Auth**: Authentication and authorization
- **Storage**: Object storage
- **Queue**: Message queuing
- **Observability**: Monitoring and logging
- **Load Balancer**: Traffic distribution

## Documentation

- [Platform Docs](https://docs.avila.cloud)
- [API Reference](https://docs.rs/avila)
- [GitHub](https://github.com/avilaops/arxis)

## Performance

Optimized for Brazilian infrastructure:
- **5-10ms latency** in São Paulo region
- **SIMD acceleration** on x86_64 (AVX2, AVX-512)
- **Zero-copy** operations where possible
- **Elastic scaling** with automatic resource management

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! See our [Contributing Guide](CONTRIBUTING.md).

## About

Built with ❤️ in Brazil by [Nícolas Ávila](https://github.com/avilaops) and the AVL community.

🇧🇷 Desenvolvido no Brasil para o mundo.
