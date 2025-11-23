//! # avx-gateway
//!
//! High-performance API gateway for Avila Experience Fabric.
//!
//! Production-ready API gateway built with Axum and Tower. Handles routing,
//! authentication, rate limiting, load balancing, and observability for
//! microservices architectures.
//!
//! ## Features
//!
//! - **High Performance**: Built on Tokio + Axum for async I/O
//! - **Routing**: Dynamic route configuration with path parameters
//! - **Authentication**: JWT, API keys, OAuth2 integration
//! - **Rate Limiting**: Token bucket and sliding window algorithms
//! - **Load Balancing**: Round-robin, least connections, weighted
//! - **Circuit Breaker**: Automatic failure detection and recovery
//! - **Observability**: Full integration with `avx-telemetry`
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avx_gateway::{Gateway, GatewayConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let gateway = Gateway::builder()
//!         .route("/api/users", "http://user-service:8001")
//!         .route("/api/products", "http://product-service:8002")
//!         .with_port(8080)
//!         .build()
//!         .await?;
//!
//!     gateway.serve().await?;
//!     Ok(())
//! }
//! ```

#![warn(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod auth;
pub mod cache;
pub mod circuit_breaker;
pub mod compression;
pub mod config;
pub mod error;
pub mod gateway;
pub mod health;
pub mod load_balancer;
pub mod metrics;
pub mod middleware;
pub mod retry;
pub mod routing;
pub mod transform;
pub mod webrtc;
pub mod websocket;

pub use config::GatewayConfig;
pub use error::{GatewayError, Result};
pub use gateway::{Gateway, GatewayBuilder};
pub use routing::{Route, RouteConfig};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::auth::*;
    pub use crate::cache::*;
    pub use crate::circuit_breaker::*;
    pub use crate::compression::*;
    pub use crate::config::*;
    pub use crate::error::*;
    pub use crate::gateway::*;
    pub use crate::health::*;
    pub use crate::load_balancer::*;
    pub use crate::middleware::*;
    pub use crate::retry::*;
    pub use crate::routing::*;
    pub use crate::transform::*;
    pub use crate::webrtc::*;
    pub use crate::websocket::*;
}
