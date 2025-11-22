//! Group by operations

use crate::core::{DataFrame, Series};
use crate::error::{AvilaError, Result};
use crate::ops::expressions::{AggFunc, Expr};
use std::collections::HashMap;

impl DataFrame {
    /// Group by one or more columns
    pub fn group_by(&self, by: &[&str]) -> Result<GroupBy> {
        if by.is_empty() {
            return Err(AvilaError::generic(
                "Must specify at least one column to group by",
            ));
        }

        // Verify all grouping columns exist
        for col_name in by {
            self.column(col_name)?;
        }

        Ok(GroupBy {
            df: self.clone(),
            by: by.iter().map(|s| s.to_string()).collect(),
        })
    }
}

/// Group by builder
pub struct GroupBy {
    df: DataFrame,
    by: Vec<String>,
}

impl GroupBy {
    /// Apply aggregations
    pub fn agg(&self, aggs: &[Expr]) -> Result<DataFrame> {
        if aggs.is_empty() {
            return Err(AvilaError::generic("Must specify at least one aggregation"));
        }

        // Build group keys and indices
        let groups = self.build_groups()?;

        // Apply each aggregation
        let mut result_columns = Vec::new();

        // Add grouping columns
        for col_name in &self.by {
            let series = self.df.column(col_name)?;
            let group_values = self.extract_group_values(series, &groups)?;
            result_columns.push(group_values);
        }

        // Add aggregated columns
        for agg_expr in aggs {
            let agg_series = self.apply_aggregation(agg_expr, &groups)?;
            result_columns.push(agg_series);
        }

        DataFrame::new(result_columns)
    }

    /// Build groups: map from group key to row indices
    fn build_groups(&self) -> Result<Vec<(GroupKey, Vec<usize>)>> {
        let mut groups_map: HashMap<GroupKey, Vec<usize>> = HashMap::new();

        for row_idx in 0..self.df.height() {
            let key = self.build_group_key(row_idx)?;
            groups_map.entry(key).or_insert_with(Vec::new).push(row_idx);
        }

        Ok(groups_map.into_iter().collect())
    }

    /// Build a group key for a row
    fn build_group_key(&self, row_idx: usize) -> Result<GroupKey> {
        let mut key = Vec::new();
        for col_name in &self.by {
            let series = self.df.column(&col_name)?;
            let value = series.get_f64(row_idx)?;
            key.push(OrderedFloat(value));
        }
        Ok(GroupKey(key))
    }

    /// Extract representative values for each group
    fn extract_group_values(
        &self,
        series: &Series,
        groups: &[(GroupKey, Vec<usize>)],
    ) -> Result<Series> {
        let values: Result<Vec<f64>> = groups
            .iter()
            .map(|(_, indices)| {
                // Take first value from each group
                series.get_f64(indices[0])
            })
            .collect();

        Ok(Series::new(series.name(), values?))
    }

    /// Apply an aggregation expression
    fn apply_aggregation(&self, expr: &Expr, groups: &[(GroupKey, Vec<usize>)]) -> Result<Series> {
        match expr {
            Expr::Agg { input, func } => {
                // Get column name from input expression
                let col_name = match input.as_ref() {
                    Expr::Column(name) => name,
                    _ => {
                        return Err(AvilaError::generic(
                            "Aggregation input must be a column reference",
                        ))
                    }
                };

                let series = self.df.column(col_name)?;
                let result_values = self.aggregate_groups(series, groups, *func)?;

                // Generate result name
                let result_name = format!("{}_{:?}", col_name, func).to_lowercase();
                Ok(Series::new(result_name, result_values))
            }
            Expr::Alias { expr, name } => {
                let mut result = self.apply_aggregation(expr, groups)?;
                result = result.rename(name.clone());
                Ok(result)
            }
            _ => Err(AvilaError::generic(format!(
                "Expression type not supported in aggregation: {:?}",
                expr
            ))),
        }
    }

    /// Aggregate values within groups
    fn aggregate_groups(
        &self,
        series: &Series,
        groups: &[(GroupKey, Vec<usize>)],
        func: AggFunc,
    ) -> Result<Vec<f64>> {
        groups
            .iter()
            .map(|(_, indices)| {
                let values: Result<Vec<f64>> = indices.iter().map(|&i| series.get_f64(i)).collect();
                let values = values?;

                match func {
                    AggFunc::Sum => Ok(values.iter().sum()),
                    AggFunc::Mean => Ok(values.iter().sum::<f64>() / values.len() as f64),
                    AggFunc::Min => values
                        .iter()
                        .cloned()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .ok_or_else(|| AvilaError::generic("No values to aggregate")),
                    AggFunc::Max => values
                        .iter()
                        .cloned()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .ok_or_else(|| AvilaError::generic("No values to aggregate")),
                    AggFunc::Count => Ok(values.len() as f64),
                    AggFunc::Std => {
                        let mean = values.iter().sum::<f64>() / values.len() as f64;
                        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
                            / values.len() as f64;
                        Ok(variance.sqrt())
                    }
                    AggFunc::Var => {
                        let mean = values.iter().sum::<f64>() / values.len() as f64;
                        Ok(values.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
                            / values.len() as f64)
                    }
                    AggFunc::Median => {
                        let mut sorted = values.clone();
                        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let mid = sorted.len() / 2;
                        if sorted.len() % 2 == 0 {
                            Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
                        } else {
                            Ok(sorted[mid])
                        }
                    }
                }
            })
            .collect()
    }
}

/// Group key for hashing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GroupKey(Vec<OrderedFloat>);

impl GroupKey {
    fn new(values: Vec<f64>) -> Self {
        Self(values.into_iter().map(OrderedFloat).collect())
    }
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

impl From<f64> for OrderedFloat {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::col;

    #[test]
    fn test_group_by_sum() {
        let df = DataFrame::new(vec![
            Series::new("category", vec![1.0, 1.0, 2.0, 2.0]),
            Series::new("value", vec![10.0, 20.0, 30.0, 40.0]),
        ])
        .unwrap();

        let result = df
            .group_by(&["category"])
            .unwrap()
            .agg(&[col("value").sum()])
            .unwrap();

        assert_eq!(result.height(), 2);
        assert_eq!(result.width(), 2);
    }

    #[test]
    fn test_group_by_mean() {
        let df = DataFrame::new(vec![
            Series::new("group", vec![1.0, 1.0, 1.0, 2.0, 2.0]),
            Series::new("val", vec![2.0, 4.0, 6.0, 10.0, 20.0]),
        ])
        .unwrap();

        let result = df
            .group_by(&["group"])
            .unwrap()
            .agg(&[col("val").mean()])
            .unwrap();

        assert_eq!(result.height(), 2);
    }
}
