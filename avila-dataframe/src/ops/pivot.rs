//! Pivot and unpivot operations

use crate::core::{DataFrame, Series};
use crate::error::Result;
use std::collections::{HashMap, HashSet};

impl DataFrame {
    /// Pivot table: reshape data from long to wide format
    ///
    /// # Arguments
    /// * `index` - Column(s) to use as index
    /// * `columns` - Column to pivot into new columns
    /// * `values` - Column to aggregate
    /// * `agg_func` - Aggregation function (sum, mean, count, etc.)
    pub fn pivot(
        &self,
        index: &[&str],
        columns: &str,
        values: &str,
        agg_func: PivotAggFunc,
    ) -> Result<Self> {
        // Verify columns exist
        for idx_col in index {
            self.column(idx_col)?;
        }
        self.column(columns)?;
        self.column(values)?;

        // Get unique values in the columns field (these become new columns)
        let pivot_values = self.get_unique_values(columns)?;

        // Build index groups
        let groups = self.build_pivot_groups(index)?;

        // Build result columns
        let mut result_columns = Vec::new();

        // Add index columns
        for idx_col in index {
            let series = self.column(idx_col)?;
            let values: Result<Vec<f64>> = groups
                .iter()
                .map(|(_, first_idx)| series.get_f64(*first_idx))
                .collect();
            result_columns.push(Series::new(*idx_col, values?));
        }

        // Add pivoted columns
        let columns_series = self.column(columns)?;
        let values_series = self.column(values)?;

        for pivot_val in pivot_values {
            let col_values: Result<Vec<f64>> = groups
                .iter()
                .map(|(group_indices, _)| {
                    // Find values where column matches pivot_val
                    let matching_values: Result<Vec<f64>> = group_indices
                        .iter()
                        .filter_map(|&idx| {
                            let col_val = columns_series.get_f64(idx).ok()?;
                            if (col_val - pivot_val).abs() < f64::EPSILON {
                                Some(values_series.get_f64(idx))
                            } else {
                                None
                            }
                        })
                        .collect();

                    let matching = matching_values?;
                    if matching.is_empty() {
                        return Ok(f64::NAN);
                    }

                    // Apply aggregation
                    Ok(match agg_func {
                        PivotAggFunc::Sum => matching.iter().sum(),
                        PivotAggFunc::Mean => matching.iter().sum::<f64>() / matching.len() as f64,
                        PivotAggFunc::Count => matching.len() as f64,
                        PivotAggFunc::Min => matching.iter().cloned()
                            .min_by(|a, b| a.partial_cmp(b).unwrap())
                            .unwrap(),
                        PivotAggFunc::Max => matching.iter().cloned()
                            .max_by(|a, b| a.partial_cmp(b).unwrap())
                            .unwrap(),
                    })
                })
                .collect();

            let col_name = format!("{}_{}", values, pivot_val);
            result_columns.push(Series::new(col_name, col_values?));
        }

        DataFrame::new(result_columns)
    }

    /// Unpivot (melt): reshape data from wide to long format
    ///
    /// # Arguments
    /// * `id_vars` - Columns to keep as identifiers
    /// * `value_vars` - Columns to unpivot
    /// * `var_name` - Name for the variable column
    /// * `value_name` - Name for the value column
    pub fn unpivot(
        &self,
        id_vars: &[&str],
        value_vars: &[&str],
        var_name: &str,
        value_name: &str,
    ) -> Result<Self> {
        // Verify columns exist
        for col in id_vars {
            self.column(col)?;
        }
        for col in value_vars {
            self.column(col)?;
        }

        let num_rows = self.height() * value_vars.len();
        let mut result_columns = Vec::new();

        // Repeat id columns
        for id_var in id_vars {
            let series = self.column(id_var)?;
            let mut values = Vec::with_capacity(num_rows);
            for _ in value_vars {
                for i in 0..self.height() {
                    values.push(series.get_f64(i)?);
                }
            }
            result_columns.push(Series::new(*id_var, values));
        }

        // Create variable column
        let mut var_values = Vec::with_capacity(num_rows);
        for var_name_val in value_vars {
            // Convert string to numeric (hash or index)
            let numeric_val = value_vars.iter().position(|&v| v == *var_name_val).unwrap() as f64;
            for _ in 0..self.height() {
                var_values.push(numeric_val);
            }
        }
        result_columns.push(Series::new(var_name, var_values));

        // Create value column
        let mut values = Vec::with_capacity(num_rows);
        for var in value_vars {
            let series = self.column(var)?;
            for i in 0..self.height() {
                values.push(series.get_f64(i)?);
            }
        }
        result_columns.push(Series::new(value_name, values));

        DataFrame::new(result_columns)
    }

    /// Get unique values from a column
    fn get_unique_values(&self, col_name: &str) -> Result<Vec<f64>> {
        let series = self.column(col_name)?;
        let mut unique_set = HashSet::new();

        for i in 0..series.len() {
            let val = series.get_f64(i)?;
            unique_set.insert(OrderedFloat(val));
        }

        let mut unique_vec: Vec<f64> = unique_set
            .into_iter()
            .map(|of| of.0)
            .collect();
        unique_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Ok(unique_vec)
    }

    /// Build groups for pivot based on index columns
    fn build_pivot_groups(&self, index: &[&str]) -> Result<Vec<(Vec<usize>, usize)>> {
        let mut groups_map: HashMap<Vec<OrderedFloat>, Vec<usize>> = HashMap::new();

        for row_idx in 0..self.height() {
            let mut key = Vec::new();
            for idx_col in index {
                let series = self.column(idx_col)?;
                key.push(OrderedFloat(series.get_f64(row_idx)?));
            }
            groups_map.entry(key).or_insert_with(Vec::new).push(row_idx);
        }

        // Return groups with first index for each group
        Ok(groups_map
            .into_iter()
            .map(|(_, indices)| {
                let first_idx = indices[0];
                (indices, first_idx)
            })
            .collect())
    }
}

/// Pivot aggregation function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PivotAggFunc {
    /// Sum
    Sum,
    /// Mean
    Mean,
    /// Count
    Count,
    /// Min
    Min,
    /// Max
    Max,
}

/// Wrapper for f64 to make it hashable
#[derive(Debug, Clone, Copy)]
struct OrderedFloat(f64);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for OrderedFloat {}

impl std::hash::Hash for OrderedFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot() {
        let df = DataFrame::new(vec![
            Series::new("date", vec![1.0, 1.0, 2.0, 2.0]),
            Series::new("category", vec![1.0, 2.0, 1.0, 2.0]),
            Series::new("value", vec![10.0, 20.0, 30.0, 40.0]),
        ])
        .unwrap();

        let pivoted = df
            .pivot(&["date"], "category", "value", PivotAggFunc::Sum)
            .unwrap();

        assert_eq!(pivoted.height(), 2); // Two unique dates
        assert_eq!(pivoted.width(), 3); // date + 2 categories
    }

    #[test]
    fn test_unpivot() {
        let df = DataFrame::new(vec![
            Series::new("id", vec![1.0, 2.0]),
            Series::new("a", vec![10.0, 20.0]),
            Series::new("b", vec![30.0, 40.0]),
        ])
        .unwrap();

        let melted = df
            .unpivot(&["id"], &["a", "b"], "variable", "value")
            .unwrap();

        assert_eq!(melted.height(), 4); // 2 rows * 2 value_vars
        assert_eq!(melted.width(), 3); // id + variable + value
    }
}
