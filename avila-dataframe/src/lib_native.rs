//! AvilaDB DataFrame - 100% Rust Nativo
//!
//! Zero overhead, máxima simplicidade e performance.

pub mod core;
pub mod error;
pub mod io;
pub mod ops;

pub mod prelude {
    //! Tudo que você precisa para começar
    pub use crate::core::{Column, DataFrame, Series};
    pub use crate::error::{AvilaError, Result};
    pub use crate::ops::*;
}

// Re-exports
pub use crate::core::{DataFrame, Series};
pub use crate::error::{AvilaError, Result};
