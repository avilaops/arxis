//! # avx-http - AVL Platform HTTP Client/Server
//!
//! Native HTTP library optimized for Brazilian infrastructure and AVL Platform services.
//!
//! ## Features
//!
//! - **High Performance**: < 500µs request overhead, 100k+ req/s
//! - **Brazilian Optimized**: Regional routing, smart retries
//! - **AVL Platform Native**: Built-in auth, telemetry, AvilaDB integration
//! - **Developer Friendly**: Simple async/await API
//!
//! ## Quick Start
//!
//! ### Client
//!
//! ```rust,no_run
//! use avx_http::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::builder()
//!         .build()?;
//!
//!     let response = client
//!         .get("https://api.avila.cloud/data")
//!         .send()
//!         .await?;
//!
//!     println!("Status: {}", response.status());
//!     Ok(())
//! }
//! ```
//!
//! ### Server
//!
//! ```rust,no_run
//! use avx_http::{Server, Router, Response};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let router = Router::new()
//!         .get("/", || async { Response::text("Hello!") });
//!
//!     Server::bind("0.0.0.0:3000")
//!         .router(router)
//!         .run()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "client")]
pub mod pool;

#[cfg(feature = "client")]
pub mod streaming;

#[cfg(feature = "client")]
pub mod interceptors;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub mod middleware;

#[cfg(any(feature = "events", feature = "telemetry"))]
pub mod events;

mod common;

// Re-exports
pub use error::{Error, Result};

#[cfg(feature = "client")]
pub use client::{Client, ClientBuilder, Request, Response as ClientResponse};

#[cfg(feature = "client")]
pub use pool::{ConnectionPool, PoolConfig, PoolStats};

#[cfg(feature = "client")]
pub use streaming::{StreamingBody, ChunkedEncoder, SseStream};

#[cfg(feature = "client")]
pub use interceptors::{Interceptors, RequestData, ResponseData, RequestInterceptor, ResponseInterceptor};

#[cfg(feature = "server")]
pub use server::{Server, Router, Response as ServerResponse};

#[cfg(feature = "server")]
pub use middleware::{Middleware, Next, Handler, Logger, Cors, RateLimit, Auth};

pub use http::{Method, StatusCode, HeaderMap, HeaderValue, Uri};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.starts_with("0."));
    }
}
