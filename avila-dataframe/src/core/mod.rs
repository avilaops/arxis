//! Core module - 100% Rust nativo

pub mod dataframe_native;
pub mod series_native;

// Re-exports
pub use dataframe_native::{Column, DataFrame};
pub use series_native::{DataType, Series, Value};
