# avila-errors

**Rich error handling for AVL Platform**

[![Crates.io](https://img.shields.io/crates/v/avila-errors.svg)](https://crates.io/crates/avila-errors)
[![Documentation](https://docs.rs/avila-errors/badge.svg)](https://docs.rs/avila-errors)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 🚀 Overview

`avila-errors` provides comprehensive error handling utilities for the AVL Platform ecosystem with rich context, composability, and excellent debugging experience.

### Key Features

- ✅ **Derive macro** for automatic Error trait implementation
- ✅ **Rich context** with error chains and backtraces
- ✅ **Zero overhead** when backtraces disabled
- ✅ **Type-safe** error composition
- ✅ **std::error::Error compatible**
- ✅ **Optimized for AVL Platform** workflows

## 📦 Installation

```toml
[dependencies]
avila-errors = "0.1"
```

### Feature Flags

- `derive` - Derive macros (default)
- `backtrace` - Capture backtraces
- `full` - All features enabled

## 🎯 Quick Start

### Using Derive Macro

```rust
use avila_errors::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("connection failed: {reason}")]
    ConnectionFailed { reason: String },

    #[error("query failed: {0}")]
    QueryFailed(String),

    #[error("record not found")]
    NotFound,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

### Manual Implementation

```rust
use avila_errors::{Error as AvilaError, Context, Result};
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    InvalidInput(String),
    NetworkError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MyError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}
```

### Using Context

```rust
use avila_errors::{Context, Result};

fn read_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("Failed to read configuration file")?;
    Ok(config)
}

fn parse_config(content: &str) -> Result<Config> {
    toml::from_str(content)
        .context("Failed to parse TOML configuration")?;
    Ok(config)
}
```

## 🔥 Features

### Error Chains

```rust
use avila_errors::{Error, Result};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error")]
    Database(#[source] DatabaseError),

    #[error("API error")]
    Api(#[source] ApiError),
}

// Errors are automatically chained
fn complex_operation() -> Result<()> {
    database_call()?; // DatabaseError -> AppError
    api_call()?;      // ApiError -> AppError
    Ok(())
}
```

### Backtrace Support

```rust
use avila_errors::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("something went wrong")]
    Failed {
        #[backtrace]
        backtrace: std::backtrace::Backtrace,
    },
}
```

### Result Type Alias

```rust
use avila_errors::Result;

// Convenient Result type
fn my_function() -> Result<String> {
    Ok("success".to_string())
}

// With custom error type
type MyResult<T> = std::result::Result<T, MyError>;
```

## 🎮 Use Cases

### AvilaDB Operations

```rust
use avila_errors::{Error, Result};

#[derive(Debug, Error)]
pub enum AvilaDBError {
    #[error("document not found: {id}")]
    DocumentNotFound { id: String },

    #[error("partition key missing")]
    PartitionKeyMissing,

    #[error("throughput exceeded")]
    ThroughputExceeded,

    #[error("serialization error: {0}")]
    Serialization(String),
}

async fn get_document(id: &str) -> Result<Document> {
    client.get(id)
        .await
        .context(format!("Failed to fetch document {}", id))?
}
```

### API Error Responses

```rust
use avila_errors::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("bad request: {message}")]
    BadRequest { message: String },

    #[error("unauthorized")]
    Unauthorized,

    #[error("not found")]
    NotFound,

    #[error("internal server error")]
    InternalError(#[from] Box<dyn std::error::Error>),
}

impl ApiError {
    pub fn status_code(&self) -> u16 {
        match self {
            ApiError::BadRequest { .. } => 400,
            ApiError::Unauthorized => 401,
            ApiError::NotFound => 404,
            ApiError::InternalError(_) => 500,
        }
    }
}
```

## 🧪 Testing

```bash
cargo test
```

## 📖 Documentation

Full documentation at [docs.rs/avila-errors](https://docs.rs/avila-errors)

## 🤝 Contributing

Part of the [AVL Platform](https://avila.inc) - contributions welcome!

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

---

**Built with 🇧🇷 by Avila Development Team**
