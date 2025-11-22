//! Sorting operations

use crate::core::{DataFrame, Series};
use crate::error::{AvilaError, Result};

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

impl DataFrame {
    /// Sort by a single column
    pub fn sort(&self, by: &str, order: SortOrder) -> Result<Self> {
        self.sort_by(&[by], &[order])
    }

    /// Sort by multiple columns
    pub fn sort_by(&self, by: &[&str], order: &[SortOrder]) -> Result<Self> {
        if by.is_empty() {
            return Err(AvilaError::generic(
                "Must specify at least one column to sort by",
            ));
        }

        if by.len() != order.len() {
            return Err(AvilaError::generic(
                "Number of sort columns must match number of sort orders",
            ));
        }

        // Verify all columns exist
        for col_name in by {
            self.column(col_name)?;
        }

        // Build sort indices
        let mut indices: Vec<usize> = (0..self.height()).collect();

        // Sort indices based on column values
        indices.sort_by(|&a, &b| {
            for (col_name, &sort_order) in by.iter().zip(order.iter()) {
                let series = self.column(col_name).unwrap();
                let val_a = series.get_f64(a).unwrap_or(f64::NAN);
                let val_b = series.get_f64(b).unwrap_or(f64::NAN);

                // Handle NaN values (put them at the end)
                let cmp = match (val_a.is_nan(), val_b.is_nan()) {
                    (true, true) => std::cmp::Ordering::Equal,
                    (true, false) => std::cmp::Ordering::Greater,
                    (false, true) => std::cmp::Ordering::Less,
                    (false, false) => val_a
                        .partial_cmp(&val_b)
                        .unwrap_or(std::cmp::Ordering::Equal),
                };

                // Apply sort order
                let cmp = match sort_order {
                    SortOrder::Ascending => cmp,
                    SortOrder::Descending => cmp.reverse(),
                };

                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            std::cmp::Ordering::Equal
        });

        // Reorder all columns
        self.take_by_indices(&indices)
    }

    /// Take rows by indices
    fn take_by_indices(&self, indices: &[usize]) -> Result<Self> {
        let reordered_columns: Result<Vec<Series>> = self
            .columns
            .iter()
            .map(|series| {
                let values: Result<Vec<f64>> =
                    indices.iter().map(|&idx| series.get_f64(idx)).collect();
                Ok(Series::new(series.name(), values?))
            })
            .collect();

        DataFrame::new(reordered_columns?)
    }

    /// Get the indices that would sort the DataFrame
    pub fn argsort(&self, by: &str, order: SortOrder) -> Result<Vec<usize>> {
        self.column(by)?;

        let mut indices: Vec<usize> = (0..self.height()).collect();
        let series = self.column(by)?;

        indices.sort_by(|&a, &b| {
            let val_a = series.get_f64(a).unwrap_or(f64::NAN);
            let val_b = series.get_f64(b).unwrap_or(f64::NAN);

            let cmp = match (val_a.is_nan(), val_b.is_nan()) {
                (true, true) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                (false, false) => val_a
                    .partial_cmp(&val_b)
                    .unwrap_or(std::cmp::Ordering::Equal),
            };

            match order {
                SortOrder::Ascending => cmp,
                SortOrder::Descending => cmp.reverse(),
            }
        });

        Ok(indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_ascending() {
        let df = DataFrame::new(vec![
            Series::new("a", vec![3.0, 1.0, 2.0]),
            Series::new("b", vec![30.0, 10.0, 20.0]),
        ])
        .unwrap();

        let sorted = df.sort("a", SortOrder::Ascending).unwrap();
        let values: Vec<f64> = (0..sorted.height())
            .map(|i| sorted.column("a").unwrap().get_f64(i).unwrap())
            .collect();

        assert_eq!(values, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_sort_descending() {
        let df = DataFrame::new(vec![Series::new("x", vec![1.0, 5.0, 3.0])]).unwrap();

        let sorted = df.sort("x", SortOrder::Descending).unwrap();
        let values: Vec<f64> = (0..sorted.height())
            .map(|i| sorted.column("x").unwrap().get_f64(i).unwrap())
            .collect();

        assert_eq!(values, vec![5.0, 3.0, 1.0]);
    }

    #[test]
    fn test_sort_by_multiple() {
        let df = DataFrame::new(vec![
            Series::new("a", vec![1.0, 1.0, 2.0, 2.0]),
            Series::new("b", vec![20.0, 10.0, 40.0, 30.0]),
        ])
        .unwrap();

        let sorted = df
            .sort_by(&["a", "b"], &[SortOrder::Ascending, SortOrder::Ascending])
            .unwrap();

        let b_values: Vec<f64> = (0..sorted.height())
            .map(|i| sorted.column("b").unwrap().get_f64(i).unwrap())
            .collect();

        assert_eq!(b_values, vec![10.0, 20.0, 30.0, 40.0]);
    }
}
