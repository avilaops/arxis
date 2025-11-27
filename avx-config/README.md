# avx-config

**Configuration management for Avila Experience Fabric**

[![Crates.io](https://img.shields.io/crates/v/avx-config.svg)](https://crates.io/crates/avx-config)
[![Documentation](https://docs.rs/avx-config/badge.svg)](https://docs.rs/avx-config)
[![License](https://img.shields.io/crates/l/avx-config.svg)](https://github.com/avilaops/arxis#license)

Centralized configuration management for the AVX (Avila Experience) platform. Provides type-safe configuration loading from multiple sources with validation and environment-based overrides.

## Features

- **Type-safe configuration**: Strong typing with serde
- **Multiple sources**: Load from files, environment variables, command-line args
- **Environment-based**: Development, staging, production configs
- **Validation**: Built-in validation with helpful error messages
- **Hot reload**: Watch for configuration changes (optional)
- **Hierarchical**: Merge configurations from multiple sources

## Installation

```toml
[dependencies]
avx-config = "0.1"
```

## Usage

```rust
use avx_config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load from config file
    let config: AppConfig = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?;

    println!("Server: {}:{}", config.server.host, config.server.port);
    Ok(())
}
```

### Configuration File Example

```toml
# config/default.toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgres://localhost/mydb"
pool_size = 10
```

### Environment Variables

Override any config value using environment variables:

```bash
export APP_SERVER__PORT=3000
export APP_DATABASE__URL="postgres://prod-server/db"
```

## Configuration Sources Priority

1. Command-line arguments (highest priority)
2. Environment variables
3. Environment-specific config files (`config/production.toml`)
4. Default config file (`config/default.toml`)

## Part of AVX Ecosystem

`avx-config` is part of the Avila Experience (AVX) platform, providing configuration management for:

- **avx-gateway**: API gateway configuration
- **avx-telemetry**: Observability settings
- **avx-quantum-render**: Renderer parameters
- Custom AVX applications

## License

MIT OR Apache-2.0

See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

## Links

- **Repository**: https://github.com/avilaops/arxis
- **Documentation**: https://docs.rs/avx-config
- **Crates.io**: https://crates.io/crates/avx-config
- **AVX Platform**: https://avila.inc
- **Documentation**: https://docs.avila.inc
