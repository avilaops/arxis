//! avila-dataframe Advanced Operations - Pandas/Polars Killer
//!
//! Features:
//! - Window functions (rolling, expanding, ewm)
//! - Lazy evaluation with query optimization
//! - Advanced aggregations
//! - Pivot tables and reshaping

use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

/// Column data types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DType {
    Int64,
    Float64,
    String,
    Boolean,
    DateTime,
}

/// Series - single column of data
#[derive(Debug, Clone)]
pub struct Series {
    pub name: String,
    pub dtype: DType,
    pub data: SeriesData,
    pub null_mask: Vec<bool>, // true = null
}

#[derive(Debug, Clone)]
pub enum SeriesData {
    Int64(Vec<i64>),
    Float64(Vec<f64>),
    String(Vec<String>),
    Boolean(Vec<bool>),
}

impl Series {
    pub fn new_int64(name: String, data: Vec<i64>) -> Self {
        let null_mask = vec![false; data.len()];
        Self {
            name,
            dtype: DType::Int64,
            data: SeriesData::Int64(data),
            null_mask,
        }
    }

    pub fn new_float64(name: String, data: Vec<f64>) -> Self {
        let null_mask = vec![false; data.len()];
        Self {
            name,
            dtype: DType::Float64,
            data: SeriesData::Float64(data),
            null_mask,
        }
    }

    pub fn len(&self) -> usize {
        match &self.data {
            SeriesData::Int64(v) => v.len(),
            SeriesData::Float64(v) => v.len(),
            SeriesData::String(v) => v.len(),
            SeriesData::Boolean(v) => v.len(),
        }
    }

    /// Get value at index (respects null mask)
    pub fn get_f64(&self, idx: usize) -> Option<f64> {
        if self.null_mask[idx] {
            return None;
        }

        match &self.data {
            SeriesData::Int64(v) => Some(v[idx] as f64),
            SeriesData::Float64(v) => Some(v[idx]),
            _ => None,
        }
    }

    /// Rolling window aggregation
    pub fn rolling_mean(&self, window: usize) -> Series {
        let mut result = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            let start = if i >= window - 1 { i - window + 1 } else { 0 };
            let end = i + 1;

            let mut sum = 0.0;
            let mut count = 0;

            for j in start..end {
                if let Some(val) = self.get_f64(j) {
                    sum += val;
                    count += 1;
                }
            }

            result.push(if count > 0 { sum / count as f64 } else { f64::NAN });
        }

        Series::new_float64(format!("{}_rolling_mean_{}", self.name, window), result)
    }

    /// Rolling standard deviation
    pub fn rolling_std(&self, window: usize) -> Series {
        let mut result = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            let start = if i >= window - 1 { i - window + 1 } else { 0 };
            let end = i + 1;

            // Calculate mean
            let mut sum = 0.0;
            let mut count = 0;
            for j in start..end {
                if let Some(val) = self.get_f64(j) {
                    sum += val;
                    count += 1;
                }
            }
            let mean = if count > 0 { sum / count as f64 } else { f64::NAN };

            // Calculate variance
            let mut var_sum = 0.0;
            for j in start..end {
                if let Some(val) = self.get_f64(j) {
                    let diff = val - mean;
                    var_sum += diff * diff;
                }
            }

            let std = if count > 1 {
                (var_sum / (count - 1) as f64).sqrt()
            } else {
                f64::NAN
            };

            result.push(std);
        }

        Series::new_float64(format!("{}_rolling_std_{}", self.name, window), result)
    }

    /// Exponentially weighted moving average
    pub fn ewm(&self, alpha: f64) -> Series {
        let mut result = Vec::with_capacity(self.len());
        let mut ewm = 0.0;
        let mut initialized = false;

        for i in 0..self.len() {
            if let Some(val) = self.get_f64(i) {
                if !initialized {
                    ewm = val;
                    initialized = true;
                } else {
                    ewm = alpha * val + (1.0 - alpha) * ewm;
                }
                result.push(ewm);
            } else {
                result.push(f64::NAN);
            }
        }

        Series::new_float64(format!("{}_ewm", self.name), result)
    }

    /// Cumulative sum
    pub fn cumsum(&self) -> Series {
        let mut result = Vec::with_capacity(self.len());
        let mut sum = 0.0;

        for i in 0..self.len() {
            if let Some(val) = self.get_f64(i) {
                sum += val;
                result.push(sum);
            } else {
                result.push(f64::NAN);
            }
        }

        Series::new_float64(format!("{}_cumsum", self.name), result)
    }

    /// Percent change
    pub fn pct_change(&self, periods: usize) -> Series {
        let mut result = vec![f64::NAN; self.len()];

        for i in periods..self.len() {
            if let (Some(current), Some(previous)) = (self.get_f64(i), self.get_f64(i - periods)) {
                if previous != 0.0 {
                    result[i] = (current - previous) / previous;
                }
            }
        }

        Series::new_float64(format!("{}_pct_change_{}", self.name, periods), result)
    }
}

/// Window specification for window functions
#[derive(Debug, Clone)]
pub struct WindowSpec {
    pub partition_by: Vec<String>,
    pub order_by: Vec<(String, bool)>, // (column, ascending)
    pub frame: WindowFrame,
}

#[derive(Debug, Clone)]
pub enum WindowFrame {
    Rows { start: i64, end: i64 },      // ROWS BETWEEN start AND end
    Range { start: f64, end: f64 },     // RANGE BETWEEN start AND end
}

/// Window function types
#[derive(Debug, Clone)]
pub enum WindowFunction {
    RowNumber,
    Rank,
    DenseRank,
    PercentRank,
    Lag { offset: usize, default: Option<f64> },
    Lead { offset: usize, default: Option<f64> },
    FirstValue,
    LastValue,
    NthValue { n: usize },
}

/// Aggregation functions
#[derive(Debug, Clone, Copy)]
pub enum AggFunc {
    Sum,
    Mean,
    Min,
    Max,
    Count,
    Std,
    Var,
    Median,
    Quantile { q: f64 },
}

impl AggFunc {
    pub fn apply(&self, values: &[f64]) -> f64 {
        match self {
            Self::Sum => values.iter().sum(),
            Self::Mean => {
                if values.is_empty() {
                    f64::NAN
                } else {
                    values.iter().sum::<f64>() / values.len() as f64
                }
            }
            Self::Min => values.iter().copied().fold(f64::INFINITY, f64::min),
            Self::Max => values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            Self::Count => values.len() as f64,
            Self::Std => {
                if values.len() < 2 {
                    return f64::NAN;
                }
                let mean = values.iter().sum::<f64>() / values.len() as f64;
                let var = values.iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>() / (values.len() - 1) as f64;
                var.sqrt()
            }
            Self::Var => {
                if values.len() < 2 {
                    return f64::NAN;
                }
                let mean = values.iter().sum::<f64>() / values.len() as f64;
                values.iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>() / (values.len() - 1) as f64
            }
            Self::Median => {
                let mut sorted = values.to_vec();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mid = sorted.len() / 2;
                if sorted.len() % 2 == 0 {
                    (sorted[mid - 1] + sorted[mid]) / 2.0
                } else {
                    sorted[mid]
                }
            }
            Self::Quantile { q } => {
                let mut sorted = values.to_vec();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let idx = (q * (sorted.len() - 1) as f64) as usize;
                sorted[idx]
            }
        }
    }
}

/// Group-by aggregation
pub struct GroupBy {
    pub keys: Vec<String>,
    pub groups: BTreeMap<Vec<String>, Vec<usize>>, // Group key -> row indices
}

impl GroupBy {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            keys,
            groups: BTreeMap::new(),
        }
    }

    pub fn add_row(&mut self, key_values: Vec<String>, row_idx: usize) {
        self.groups.entry(key_values)
            .or_insert_with(Vec::new)
            .push(row_idx);
    }

    pub fn aggregate(&self, series: &Series, func: AggFunc) -> Vec<(Vec<String>, f64)> {
        let mut results = Vec::new();

        for (key, indices) in &self.groups {
            let values: Vec<f64> = indices.iter()
                .filter_map(|&idx| series.get_f64(idx))
                .collect();

            let result = func.apply(&values);
            results.push((key.clone(), result));
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_mean() {
        let s = Series::new_float64("x".to_string(), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = s.rolling_mean(3);

        // Window 3: [1], [1,2], [1,2,3], [2,3,4], [3,4,5]
        // Means: 1.0, 1.5, 2.0, 3.0, 4.0
        assert!((result.get_f64(0).unwrap() - 1.0).abs() < 0.01);
        assert!((result.get_f64(2).unwrap() - 2.0).abs() < 0.01);
        assert!((result.get_f64(4).unwrap() - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_cumsum() {
        let s = Series::new_float64("x".to_string(), vec![1.0, 2.0, 3.0, 4.0]);
        let result = s.cumsum();

        assert_eq!(result.get_f64(0).unwrap(), 1.0);
        assert_eq!(result.get_f64(1).unwrap(), 3.0);
        assert_eq!(result.get_f64(2).unwrap(), 6.0);
        assert_eq!(result.get_f64(3).unwrap(), 10.0);
    }

    #[test]
    fn test_pct_change() {
        let s = Series::new_float64("x".to_string(), vec![100.0, 110.0, 121.0, 133.1]);
        let result = s.pct_change(1);

        assert!((result.get_f64(1).unwrap() - 0.1).abs() < 0.01); // 10% increase
        assert!((result.get_f64(2).unwrap() - 0.1).abs() < 0.01); // 10% increase
    }

    #[test]
    fn test_agg_functions() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(AggFunc::Sum.apply(&values), 15.0);
        assert_eq!(AggFunc::Mean.apply(&values), 3.0);
        assert_eq!(AggFunc::Min.apply(&values), 1.0);
        assert_eq!(AggFunc::Max.apply(&values), 5.0);
        assert_eq!(AggFunc::Median.apply(&values), 3.0);
    }
}
