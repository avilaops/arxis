//! # AvilaDB DataFrame
//!
//! Revolutionary DataFrame library for scientific computing with native astrophysics support.
//!
//! ## Features
//!
//! - **Columnar Storage**: Apache Arrow-based zero-copy data structures
//! - **Scientific Types**: Quaternions, Tensors, Spinors, Geodesic coordinates
//! - **Query Engine**: Lazy evaluation with SQL support
//! - **Built-in Science**: FFT, wavelets, signal processing, astronomy functions
//! - **High Performance**: SIMD, GPU acceleration, distributed computing
//! - **Native Integration**: AvilaDB, cloud storage (S3, Azure, GCS)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avila_dataframe::prelude::*;
//!
//! # fn main() -> Result<()> {
//! // Create DataFrame from scratch
//! let df = DataFrame::new(vec![
//!     Series::new("mass1", vec![30.0, 35.0, 25.0]),
//!     Series::new("mass2", vec![25.0, 30.0, 20.0]),
//!     Series::new("snr", vec![12.5, 15.3, 10.8]),
//! ])?;
//!
//! // Query with fluent API
//! let result = df
//!     .filter(col("snr") > 10.0)?
//!     .with_column((col("mass1") + col("mass2")).alias("total_mass"))?
//!     .select(&["total_mass", "snr"])?;
//!
//! println!("{}", result);
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![allow(dead_code)] // During development

pub mod core;
pub mod error;
pub mod io;
pub mod lazy;
pub mod ops;

#[cfg(feature = "scientific")]
pub mod scientific;

#[cfg(feature = "sql")]
pub mod sql;

#[cfg(feature = "distributed")]
pub mod distributed;

#[cfg(feature = "gpu")]
pub mod gpu;

pub mod ai;
pub mod edge;
pub mod observability;
pub mod security;

pub mod prelude;

pub use core::{DataFrame, Series};
pub use error::{AvilaError, Result};
