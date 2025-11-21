//! DataFrame: A table of data

use crate::core::series::Series;
use crate::error::{AvilaError, Result};
use arrow::array::RecordBatch;
use arrow::datatypes::{Field, Schema as ArrowSchema};
use std::sync::Arc;

/// A two-dimensional table of data with labeled columns
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub(crate) columns: Vec<Series>,
}

impl DataFrame {
    /// Create a new DataFrame from a vector of Series
    pub fn new(columns: Vec<Series>) -> Result<Self> {
        if columns.is_empty() {
            return Ok(Self { columns });
        }

        // Check all series have the same length
        let len = columns[0].len();
        for series in &columns {
            if series.len() != len {
                return Err(AvilaError::ShapeMismatch(format!(
                    "All series must have the same length. Expected {}, got {}",
                    len,
                    series.len()
                )));
            }
        }

        Ok(Self { columns })
    }

    /// Create an empty DataFrame
    pub fn empty() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    /// Get the number of rows
    pub fn height(&self) -> usize {
        self.columns.first().map(|s| s.len()).unwrap_or(0)
    }

    /// Get the number of rows (alias for height)
    pub fn len(&self) -> usize {
        self.height()
    }

    /// Get the number of columns
    pub fn width(&self) -> usize {
        self.columns.len()
    }

    /// Get the shape as (height, width)
    pub fn shape(&self) -> (usize, usize) {
        (self.height(), self.width())
    }

    /// Check if DataFrame is empty
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty() || self.height() == 0
    }

    /// Get column names
    pub fn column_names(&self) -> Vec<&str> {
        self.columns.iter().map(|s| s.name()).collect()
    }

    /// Get a column by name
    pub fn column(&self, name: &str) -> Result<&Series> {
        self.columns
            .iter()
            .find(|s| s.name() == name)
            .ok_or_else(|| AvilaError::column_not_found(name))
    }

    /// Get a column by index
    pub fn column_at(&self, index: usize) -> Result<&Series> {
        self.columns
            .get(index)
            .ok_or_else(|| AvilaError::generic(format!("Column index {} out of bounds", index)))
    }

    /// Select specific columns
    pub fn select(&self, names: &[&str]) -> Result<Self> {
        let mut selected = Vec::new();
        for name in names {
            selected.push(self.column(name)?.clone());
        }
        Ok(Self { columns: selected })
    }

    /// Drop columns
    pub fn drop(&self, names: &[&str]) -> Result<Self> {
        let drop_set: std::collections::HashSet<_> = names.iter().cloned().collect();
        let columns = self
            .columns
            .iter()
            .filter(|s| !drop_set.contains(s.name()))
            .cloned()
            .collect();
        Ok(Self { columns })
    }

    /// Add a new column
    pub fn with_column(&self, series: Series) -> Result<Self> {
        if !self.is_empty() && series.len() != self.height() {
            return Err(AvilaError::ShapeMismatch(format!(
                "Series length {} does not match DataFrame height {}",
                series.len(),
                self.height()
            )));
        }

        let mut columns = self.columns.clone();

        // Replace if column with same name exists
        if let Some(pos) = columns.iter().position(|s| s.name() == series.name()) {
            columns[pos] = series;
        } else {
            columns.push(series);
        }

        Ok(Self { columns })
    }

    /// Describe the DataFrame (summary statistics)
    pub fn describe(&self) -> Result<Self> {
        let mut stats = Vec::new();

        // Stat names
        let stat_names =
            Series::from_strings("statistic", vec!["count", "mean", "std", "min", "max"]);
        stats.push(stat_names);

        // Calculate stats for each numeric column
        for series in &self.columns {
            let count = series.len() as f64;
            let mean = series.mean().unwrap_or(f64::NAN);
            let std = series.std().unwrap_or(f64::NAN);

            // Simplified min/max (would need proper implementation)
            let min = f64::NAN;
            let max = f64::NAN;

            let col_stats = Series::new(series.name(), vec![count, mean, std, min, max]);
            stats.push(col_stats);
        }

        Self::new(stats)
    }

    /// Get the first n rows
    pub fn head(&self, n: usize) -> Result<Self> {
        let _n = std::cmp::min(n, self.height());
        // TODO: Implement slicing
        Err(AvilaError::not_implemented("head"))
    }

    /// Convert to Arrow RecordBatch
    pub fn to_record_batch(&self) -> Result<RecordBatch> {
        let fields: Vec<Field> = self
            .columns
            .iter()
            .map(|s| Field::new(s.name(), s.dtype().clone(), true))
            .collect();

        let schema = Arc::new(ArrowSchema::new(fields));
        let arrays: Vec<_> = self.columns.iter().map(|s| s.array().clone()).collect();

        RecordBatch::try_new(schema, arrays).map_err(|e| e.into())
    }
}

impl std::fmt::Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DataFrame: shape = {:?}", self.shape())?;
        writeln!(f, "Columns: {}", self.column_names().join(", "))?;
        writeln!(f, "┌{}┐", "─".repeat(80))?;

        // Header
        write!(f, "│ ")?;
        for (i, name) in self.column_names().iter().enumerate() {
            if i > 0 {
                write!(f, " │ ")?;
            }
            write!(f, "{:12}", name)?;
        }
        writeln!(f, " │")?;
        writeln!(f, "├{}┤", "─".repeat(80))?;

        // Rows (first 10)
        let display_rows = std::cmp::min(10, self.height());
        for row_idx in 0..display_rows {
            write!(f, "│ ")?;
            for (col_idx, series) in self.columns.iter().enumerate() {
                if col_idx > 0 {
                    write!(f, " │ ")?;
                }

                if let Ok(val) = series.get_f64(row_idx) {
                    write!(f, "{:12.4}", val)?;
                } else {
                    write!(f, "{:12}", "...")?;
                }
            }
            writeln!(f, " │")?;
        }

        if self.height() > display_rows {
            writeln!(f, "│ ... ({} more rows) ...", self.height() - display_rows)?;
        }

        writeln!(f, "└{}┘", "─".repeat(80))?;
        Ok(())
    }
}

// Internal helper for I/O modules
impl DataFrame {
    /// Convert Arrow RecordBatch to DataFrame
    pub(crate) fn from_record_batch(batch: RecordBatch) -> Result<Self> {
        let mut columns = Vec::new();

        for (i, field) in batch.schema().fields().iter().enumerate() {
            let array = batch.column(i).clone();
            let series = Series::from_arrow(field.name(), array);
            columns.push(series);
        }

        DataFrame::new(columns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe_creation() {
        let df = DataFrame::new(vec![
            Series::new("a", vec![1.0, 2.0, 3.0]),
            Series::new("b", vec![4.0, 5.0, 6.0]),
        ])
        .unwrap();

        assert_eq!(df.shape(), (3, 2));
        assert_eq!(df.column_names(), vec!["a", "b"]);
    }

    #[test]
    fn test_select_columns() {
        let df = DataFrame::new(vec![
            Series::new("a", vec![1.0, 2.0]),
            Series::new("b", vec![3.0, 4.0]),
            Series::new("c", vec![5.0, 6.0]),
        ])
        .unwrap();

        let selected = df.select(&["a", "c"]).unwrap();
        assert_eq!(selected.column_names(), vec!["a", "c"]);
        assert_eq!(selected.width(), 2);
    }

    #[test]
    fn test_with_column() {
        let df = DataFrame::new(vec![Series::new("a", vec![1.0, 2.0])]).unwrap();

        let df2 = df.with_column(Series::new("b", vec![3.0, 4.0])).unwrap();
        assert_eq!(df2.width(), 2);
        assert_eq!(df2.column_names(), vec!["a", "b"]);
    }
}
