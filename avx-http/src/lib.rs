//! # avx-http - Pure Rust HTTP/1.1 + HTTP/2 Library
//!
//! **ZERO external dependencies** - No tokio, no serde, no hyper, 100% proprietary!
//!
//! Everything implemented from scratch using only `std::*`:
//! - HTTP/1.1 parser (finite state machine)
//! - HTTP/2 frame parser, HPACK compression, multiplexing
//! - Custom async runtime (thread pool + I/O reactor)
//! - Zero-copy bytes buffer
//! - Pure Rust JSON parser
//! - Connection pooling
//!
//! ## Philosophy
//!
//! - **Zero Dependencies**: Full control, no supply chain attacks
//! - **Predictable Performance**: No hidden allocations or async overhead
//! - **Readable Code**: Algorithms you can understand and audit
//! - **Brazilian Latency**: Optimized for São Paulo DC (5-10ms)
//!
//! ## Quick Start
//!
//! ### HTTP/1.1 Client
//!
//! ```rust,no_run
//! use avx_http::Client;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!     let response = client.get("http://api.avila.cloud/data")?;
//!     println!("Status: {}", response.status());
//!     println!("Body: {}", response.text()?);
//!     Ok(())
//! }
//! ```
//!
//! ### HTTP/2 Client
//!
//! ```rust,no_run
//! use avx_http::http2::Http2Connection;
//! use avx_http::net::TcpStream;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let stream = TcpStream::connect("api.avila.cloud:443")?;
//!     let mut conn = Http2Connection::new_client(stream)?;
//!
//!     let stream_id = conn.request(
//!         "GET",
//!         "/data",
//!         "api.avila.cloud",
//!         vec![],
//!         None,
//!     )?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Core modules - pure std implementation
pub mod error;
pub mod http;
pub mod bytes;
pub mod json;
pub mod runtime;
pub mod net;
pub mod reactor;
pub mod timer;
pub mod async_net;

// TLS support (optional)
#[cfg(feature = "tls")]
pub mod tls;

// HTTP/2 implementation
pub mod http2;

// Re-exports
pub use error::{Error, Result};
pub use http::{Method, StatusCode, Headers, Request, Response};
pub use bytes::Bytes;
pub use json::JsonValue;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent string
pub const USER_AGENT: &str = concat!("avx-http/", env!("CARGO_PKG_VERSION"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.starts_with("0."));
    }

    #[test]
    fn test_user_agent() {
        assert!(USER_AGENT.contains("avx-http"));
        assert!(USER_AGENT.contains(VERSION));
    }
}
