# Changelog

All notable changes to AvilaDB will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-11-23

### Added
- 🎉 Initial release of AvilaDB
- 📦 4 MB document size (2x larger than competitors)
- 🔍 Native vector search with HNSW index
- 🌍 Multi-region writes at no extra cost
- ⚡ 5-10ms latency in Brazil
- 🗜️ Automatic compression via `avila-compress`
- 📊 Built-in telemetry and monitoring
- 🏛️ ACID-compliant transactions
- 🔐 Encryption at rest and in transit
- 💰 40-60% cheaper than AWS DynamoDB for Brazilian workloads

### Features
- `AvilaClient` for connecting to databases
- `Database` handle for managing collections
- `Collection` for CRUD operations
- `Document` with builder pattern
- `Query` engine with filtering and pagination
- `Vector` search with semantic similarity
- Storage classes: Standard, InfrequentAccess, Archive
- Hierarchical Partition Keys (HPK) support
- Automatic connection pooling
- Batch operations for bulk inserts

### Documentation
- Complete API documentation
- Quick start guide
- Architecture overview
- Best practices guide
- Integration examples
- Performance benchmarks

### Optimizations
- Native compression (LZ4/Zstd)
- Connection pooling
- Query result caching
- Adaptive batching
- Smart indexing

[Unreleased]: https://github.com/avilaops/arxis/compare/v0.1.0-aviladb...HEAD
[0.1.0]: https://github.com/avilaops/arxis/releases/tag/v0.1.0-aviladb
