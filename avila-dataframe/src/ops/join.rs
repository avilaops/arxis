//! Join operations

use crate::core::{DataFrame, Series};
use crate::error::{AvilaError, Result};
use std::collections::HashMap;

/// Join type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    /// Inner join
    Inner,
    /// Left outer join
    Left,
    /// Right outer join
    Right,
    /// Full outer join
    Outer,
}

impl DataFrame {
    /// Join with another DataFrame on a key column
    pub fn join(
        &self,
        other: &DataFrame,
        left_on: &str,
        right_on: &str,
        how: JoinType,
    ) -> Result<Self> {
        self.join_impl(other, &[left_on], &[right_on], how)
    }

    /// Join with another DataFrame on multiple key columns
    pub fn join_on(
        &self,
        other: &DataFrame,
        left_on: &[&str],
        right_on: &[&str],
        how: JoinType,
    ) -> Result<Self> {
        if left_on.len() != right_on.len() {
            return Err(AvilaError::generic(
                "Left and right join keys must have same length",
            ));
        }
        self.join_impl(other, left_on, right_on, how)
    }

    /// Internal join implementation
    fn join_impl(
        &self,
        other: &DataFrame,
        left_on: &[&str],
        right_on: &[&str],
        how: JoinType,
    ) -> Result<Self> {
        // Verify join keys exist
        for key in left_on {
            self.column(key)?;
        }
        for key in right_on {
            other.column(key)?;
        }

        // Build join indices
        let join_pairs = match how {
            JoinType::Inner => self.build_inner_join_indices(other, left_on, right_on)?,
            JoinType::Left => self.build_left_join_indices(other, left_on, right_on)?,
            JoinType::Right => self.build_right_join_indices(other, left_on, right_on)?,
            JoinType::Outer => self.build_outer_join_indices(other, left_on, right_on)?,
        };

        // Build result DataFrame
        self.build_joined_dataframe(other, &join_pairs, left_on, right_on)
    }

    /// Build inner join indices
    fn build_inner_join_indices(
        &self,
        other: &DataFrame,
        left_on: &[&str],
        right_on: &[&str],
    ) -> Result<Vec<(Option<usize>, Option<usize>)>> {
        let mut result = Vec::new();
        let right_index = self.build_hash_index(other, right_on)?;

        for left_idx in 0..self.height() {
            let key = self.build_join_key(left_idx, left_on)?;
            if let Some(right_indices) = right_index.get(&key) {
                for &right_idx in right_indices {
                    result.push((Some(left_idx), Some(right_idx)));
                }
            }
        }

        Ok(result)
    }

    /// Build left join indices
    fn build_left_join_indices(
        &self,
        other: &DataFrame,
        left_on: &[&str],
        right_on: &[&str],
    ) -> Result<Vec<(Option<usize>, Option<usize>)>> {
        let mut result = Vec::new();
        let right_index = self.build_hash_index(other, right_on)?;

        for left_idx in 0..self.height() {
            let key = self.build_join_key(left_idx, left_on)?;
            if let Some(right_indices) = right_index.get(&key) {
                for &right_idx in right_indices {
                    result.push((Some(left_idx), Some(right_idx)));
                }
            } else {
                // Left row has no match, include with null right
                result.push((Some(left_idx), None));
            }
        }

        Ok(result)
    }

    /// Build right join indices
    fn build_right_join_indices(
        &self,
        other: &DataFrame,
        left_on: &[&str],
        right_on: &[&str],
    ) -> Result<Vec<(Option<usize>, Option<usize>)>> {
        let mut result = Vec::new();
        let left_index = self.build_hash_index(self, left_on)?;

        for right_idx in 0..other.height() {
            let key = self.build_join_key_from(other, right_idx, right_on)?;
            if let Some(left_indices) = left_index.get(&key) {
                for &left_idx in left_indices {
                    result.push((Some(left_idx), Some(right_idx)));
                }
            } else {
                // Right row has no match, include with null left
                result.push((None, Some(right_idx)));
            }
        }

        Ok(result)
    }

    /// Build outer join indices
    fn build_outer_join_indices(
        &self,
        other: &DataFrame,
        left_on: &[&str],
        right_on: &[&str],
    ) -> Result<Vec<(Option<usize>, Option<usize>)>> {
        let mut result = Vec::new();
        let right_index = self.build_hash_index(other, right_on)?;
        let mut matched_right = vec![false; other.height()];

        // Add all left rows with their matches
        for left_idx in 0..self.height() {
            let key = self.build_join_key(left_idx, left_on)?;
            if let Some(right_indices) = right_index.get(&key) {
                for &right_idx in right_indices {
                    result.push((Some(left_idx), Some(right_idx)));
                    matched_right[right_idx] = true;
                }
            } else {
                result.push((Some(left_idx), None));
            }
        }

        // Add unmatched right rows
        for (right_idx, &matched) in matched_right.iter().enumerate() {
            if !matched {
                result.push((None, Some(right_idx)));
            }
        }

        Ok(result)
    }

    /// Build hash index for a DataFrame
    fn build_hash_index(
        &self,
        df: &DataFrame,
        on: &[&str],
    ) -> Result<HashMap<JoinKey, Vec<usize>>> {
        let mut index: HashMap<JoinKey, Vec<usize>> = HashMap::new();

        for row_idx in 0..df.height() {
            let key = self.build_join_key_from(df, row_idx, on)?;
            index.entry(key).or_insert_with(Vec::new).push(row_idx);
        }

        Ok(index)
    }

    /// Build join key for current DataFrame
    fn build_join_key(&self, row_idx: usize, keys: &[&str]) -> Result<JoinKey> {
        self.build_join_key_from(self, row_idx, keys)
    }

    /// Build join key from any DataFrame
    fn build_join_key_from(
        &self,
        df: &DataFrame,
        row_idx: usize,
        keys: &[&str],
    ) -> Result<JoinKey> {
        let mut values = Vec::new();
        for key in keys {
            let series = df.column(key)?;
            let value = series.get_f64(row_idx)?;
            values.push(OrderedFloat(value));
        }
        Ok(JoinKey(values))
    }

    /// Build joined DataFrame from join indices
    fn build_joined_dataframe(
        &self,
        other: &DataFrame,
        join_pairs: &[(Option<usize>, Option<usize>)],
        left_on: &[&str],
        right_on: &[&str],
    ) -> Result<Self> {
        let mut result_columns = Vec::new();

        // Add left columns
        for series in &self.columns {
            let values: Result<Vec<f64>> = join_pairs
                .iter()
                .map(|(left_idx, _)| {
                    if let Some(idx) = left_idx {
                        series.get_f64(*idx)
                    } else {
                        Ok(f64::NAN)
                    }
                })
                .collect();
            result_columns.push(Series::new(series.name(), values?));
        }

        // Add right columns (excluding join keys if they match left keys)
        for series in &other.columns {
            // Skip if this is a join key that matches a left key
            if right_on.contains(&series.name()) && left_on.contains(&series.name()) {
                continue;
            }

            let values: Result<Vec<f64>> = join_pairs
                .iter()
                .map(|(_, right_idx)| {
                    if let Some(idx) = right_idx {
                        series.get_f64(*idx)
                    } else {
                        Ok(f64::NAN)
                    }
                })
                .collect();

            // Add suffix if column name conflicts
            let col_name = if self.column(series.name()).is_ok() {
                format!("{}_right", series.name())
            } else {
                series.name().to_string()
            };

            result_columns.push(Series::new(col_name, values?));
        }

        DataFrame::new(result_columns)
    }
}

/// Join key for hashing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JoinKey(Vec<OrderedFloat>);

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
    fn test_inner_join() {
        let left = DataFrame::new(vec![
            Series::new("id", vec![1.0, 2.0, 3.0]),
            Series::new("value", vec![10.0, 20.0, 30.0]),
        ])
        .unwrap();

        let right = DataFrame::new(vec![
            Series::new("id", vec![2.0, 3.0, 4.0]),
            Series::new("score", vec![100.0, 200.0, 300.0]),
        ])
        .unwrap();

        let result = left.join(&right, "id", "id", JoinType::Inner).unwrap();
        assert_eq!(result.height(), 2); // Only id 2 and 3 match
    }

    #[test]
    fn test_left_join() {
        let left = DataFrame::new(vec![
            Series::new("id", vec![1.0, 2.0, 3.0]),
            Series::new("value", vec![10.0, 20.0, 30.0]),
        ])
        .unwrap();

        let right = DataFrame::new(vec![
            Series::new("id", vec![2.0, 3.0]),
            Series::new("score", vec![100.0, 200.0]),
        ])
        .unwrap();

        let result = left.join(&right, "id", "id", JoinType::Left).unwrap();
        assert_eq!(result.height(), 3); // All left rows included
    }
}
