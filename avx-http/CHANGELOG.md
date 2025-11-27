# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-11-27

### Added ✨
- **Custom Async Runtime** - Complete async runtime with zero external dependencies
  - ThreadPool-based task execution
  - I/O Reactor with epoll (Linux), kqueue (macOS), IOCP (Windows)
  - Hierarchical timer wheel (3-level: 1ms, 256ms, 65s)
  - Non-blocking TCP with `AsyncTcpStream` and `AsyncTcpListener`
  - Future-based API with async/await support

- **TLS 1.3 Support** (optional feature)
  - rustls integration for HTTPS
  - Client TLS connector
  - Native root certificate loading
  - Enable with `features = ["tls"]`

- **Windows IOCP** - Complete I/O Completion Ports implementation
  - `CreateIoCompletionPort` integration
  - `GetQueuedCompletionStatusEx` for event processing
  - Full parity with Linux/macOS reactor

- **Performance Benchmarks**
  - Timer wheel benchmarks
  - Runtime spawn/block_on benchmarks
  - Tokio comparison suite (optional)
  - Latency percentile measurements

### Changed 🔄
- Runtime now uses platform-specific I/O reactor instead of busy polling
- Timer wheel integrated into reactor event loop
- Network I/O changed to non-blocking by default

### Performance 📊
- Timer insert: ~20ns (O(1))
- Task spawn: ~500ns
- Block_on immediate: ~100ns
- Zero-copy bytes slice: ~5ns
- JSON parse: ~2μs

## [0.3.0] - 2025-11-20

### Added
- HTTP/2 HPACK compression/decompression
- Static table (61 predefined headers)
- Dynamic table with LRU eviction
- 50-70% header size reduction

### Changed
- HTTP/2 connection now uses HPACK for all headers
- Improved stream multiplexing

## [0.2.0] - 2025-11-15

### Added
- HTTP/2 frame parsing (DATA, HEADERS, SETTINGS, etc.)
- Stream multiplexing
- Flow control (per-stream and connection-level)
- Priority support
- PING and GOAWAY frames

### Changed
- Split HTTP/1.1 and HTTP/2 into separate modules

## [0.1.0] - 2025-11-10

### Added
- Initial HTTP/1.1 implementation
- Zero-copy parser with FSM
- Request/Response types
- Headers management
- Basic client functionality
- Connection pooling
- Zero-copy bytes buffer
- JSON parser (no serde)

### Philosophy
- **ZERO dependencies** - Everything from scratch using std::*
- **100% auditable** - Complete control over code
- **Pure Rust** - Memory safe, no C dependencies

## Dependencies by Version

### Core Library (Zero Dependencies!)
- v0.1.0 - v0.3.0: **0 dependencies**
- v0.4.0+: **0 dependencies** for core features

### Optional Features
- `tls` feature (v0.4.0+):
  - rustls 0.23
  - rustls-pemfile 2.0
  - rustls-native-certs 0.7

### Dev Dependencies (Benchmarks Only)
- criterion 0.5
- tokio 1.35 (optional, for comparisons)

## Migration Guide

### From 0.3.x to 0.4.0

#### Async Runtime Required
```rust
// Old (0.3.x) - Synchronous
use avx_http::net::TcpStream;
let mut stream = TcpStream::connect("example.com:80")?;

// New (0.4.0) - Async
use avx_http::async_net::AsyncTcpStream;
use avx_http::runtime;

runtime::block_on(async {
    let mut stream = AsyncTcpStream::connect("example.com:80").await?;
});
```

#### TLS Support
```rust
// New in 0.4.0
#[cfg(feature = "tls")]
use avx_http::tls::TlsConnector;

let connector = TlsConnector::new()?;
let tls_stream = connector.connect("example.com", tcp_stream).await?;
```

## Breaking Changes

### 0.4.0
- Network I/O now requires async runtime
- `TcpStream::connect()` is now async
- Timer API changed to use timer wheel instead of simple sleep

### 0.3.0
- HTTP/2 headers now use HPACK encoding
- Header representation changed internally

### 0.2.0
- Split HTTP/1.1 and HTTP/2 modules
- Some APIs reorganized

## Known Issues

### 0.4.0
- Windows IOCP needs more testing
- Server TLS not yet implemented
- HTTP/2 server push not implemented

## Upcoming Features

### 0.5.0 (Planned)
- [ ] HTTP/2 Server Push
- [ ] WebSocket support
- [ ] Connection pooling for async
- [ ] Zero-copy sendfile()

### 1.0.0 (Planned)
- [ ] Full production testing
- [ ] Complete documentation
- [ ] Performance optimization
- [ ] Stability guarantees

## Benchmarks

Run with:
```bash
cargo bench --bench tokio_comparison
cargo bench --bench async_bench
```

Compare with Tokio (optional):
```bash
cargo bench --bench tokio_comparison --features tokio-comparison
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT OR Apache-2.0

---

**AVX-HTTP** - Pure Rust HTTP. Zero Dependencies. Maximum Control. 🦀
