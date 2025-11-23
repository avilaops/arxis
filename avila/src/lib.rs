//! # Avila Platform
//!
//! High-performance cloud platform optimized for Brazil and LATAM.
//!
//! ## Features
//!
//! - **Compression**: High-performance compression with SIMD optimization
//! - **Clustering**: Advanced clustering algorithms (K-means, DBSCAN, Hierarchical)
//! - **Math**: Mathematical operations and linear algebra
//! - **Data**: Arrow-based data processing
//! - **HTTP**: High-performance HTTP client/server
//! - **CLI**: Command-line interface tools
//!
//! ## Quick Start
//!
//! ```rust
//! // Compression example
//! #[cfg(feature = "compression")]
//! use avila::compress;
//!
//! // Math example
//! #[cfg(feature = "math")]
//! use avila::math;
//! ```

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export core libraries
#[cfg(feature = "compression")]
#[cfg_attr(docsrs, doc(cfg(feature = "compression")))]
pub use avila_compress as compress;

#[cfg(feature = "clustering")]
#[cfg_attr(docsrs, doc(cfg(feature = "clustering")))]
pub use avila_clustering as clustering;

#[cfg(feature = "math")]
#[cfg_attr(docsrs, doc(cfg(feature = "math")))]
pub use avila_math as math;

#[cfg(feature = "math")]
#[cfg_attr(docsrs, doc(cfg(feature = "math")))]
pub use avila_linalg as linalg;

#[cfg(feature = "data")]
#[cfg_attr(docsrs, doc(cfg(feature = "data")))]
pub use avila_arrow as arrow;

#[cfg(feature = "telemetry")]
#[cfg_attr(docsrs, doc(cfg(feature = "telemetry")))]
pub use avila_telemetry as telemetry;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub use avx_http as http;

#[cfg(feature = "cli")]
#[cfg_attr(docsrs, doc(cfg(feature = "cli")))]
pub use avx_cli as cli;

#[cfg(feature = "cli")]
#[cfg_attr(docsrs, doc(cfg(feature = "cli")))]
pub use avx_config as config;

/// Version of the Avila platform
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Platform name
pub const PLATFORM_NAME: &str = "AVL Platform";

/// Platform homepage
pub const HOMEPAGE: &str = "https://avila.cloud";
