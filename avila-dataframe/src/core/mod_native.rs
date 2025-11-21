//! Core module - 100% Rust nativo

pub mod series_native;
pub mod dataframe_native;

// Re-exports
pub use series_native::{Series, Value, DataType};
pub use dataframe_native::{DataFrame, Column};
