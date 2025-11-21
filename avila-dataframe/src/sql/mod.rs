//! SQL query engine with scientific extensions

use crate::core::DataFrame;
use crate::error::Result;

/// SQL context for executing queries
pub struct SqlContext {
    tables: std::collections::HashMap<String, DataFrame>,
}

impl SqlContext {
    /// Create a new SQL context
    pub fn new() -> Self {
        Self {
            tables: std::collections::HashMap::new(),
        }
    }

    /// Register a DataFrame as a table
    pub fn register_table(&mut self, name: impl Into<String>, df: DataFrame) {
        self.tables.insert(name.into(), df);
    }

    /// Execute SQL query
    ///
    /// # Examples
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// # use avila_dataframe::sql::SqlContext;
    /// # fn main() -> Result<()> {
    /// let mut ctx = SqlContext::new();
    /// ctx.register_table("lisa_data", df);
    ///
    /// let result = ctx.sql(r#"
    ///     SELECT timestamp, strain_h
    ///     FROM lisa_data
    ///     WHERE abs(strain_h) > 1e-21
    ///     LIMIT 100
    /// "#)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sql(&self, query: &str) -> Result<DataFrame> {
        // TODO: Integrate with DataFusion for SQL parsing and execution
        Err(crate::error::AvilaError::not_implemented("SQL execution"))
    }
}

impl DataFrame {
    /// Execute SQL query on this DataFrame
    pub fn sql(&self, query: &str) -> Result<Self> {
        let mut ctx = SqlContext::new();
        ctx.register_table("this", self.clone());
        ctx.sql(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_context_creation() {
        let ctx = SqlContext::new();
        assert_eq!(ctx.tables.len(), 0);
    }
}
