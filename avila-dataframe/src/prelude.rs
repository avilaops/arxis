//! Convenience re-exports for common types and traits

pub use crate::core::{DataFrame, Series};
pub use crate::error::{AvilaError, Result};
pub use crate::ops::expressions::{col, lit, Expr};

#[cfg(feature = "lazy")]
pub use crate::lazy::LazyFrame;

#[cfg(feature = "sql")]
pub use crate::sql::SqlContext;

pub use arrow::datatypes::DataType;
