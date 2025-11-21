//! Lazy evaluation framework

use crate::core::DataFrame;
use crate::error::Result;

/// Lazy DataFrame for deferred execution
pub struct LazyFrame {
    // TODO: Implement logical plan
}

impl LazyFrame {
    /// Convert to eager DataFrame
    pub fn collect(self) -> Result<DataFrame> {
        Err(crate::error::AvilaError::not_implemented("LazyFrame"))
    }
}
