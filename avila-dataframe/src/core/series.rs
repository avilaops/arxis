//! Series: A single column of data

use arrow::array::{Array, ArrayRef, Float64Array, Int64Array, StringArray};
use arrow::datatypes::DataType;
use std::sync::Arc;
use crate::error::{Result, AvilaError};

/// A single column of data backed by Arrow arrays
#[derive(Debug, Clone)]
pub struct Series {
    name: String,
    data: ArrayRef,
}

impl Series {
    /// Create a new Series from a vector of f64
    pub fn new(name: impl Into<String>, data: Vec<f64>) -> Self {
        let array = Float64Array::from(data);
        Self {
            name: name.into(),
            data: Arc::new(array),
        }
    }

    /// Create a Series from i64 values
    pub fn from_i64(name: impl Into<String>, data: Vec<i64>) -> Self {
        let array = Int64Array::from(data);
        Self {
            name: name.into(),
            data: Arc::new(array),
        }
    }

    /// Create a Series from strings
    pub fn from_strings(name: impl Into<String>, data: Vec<&str>) -> Self {
        let array = StringArray::from(data);
        Self {
            name: name.into(),
            data: Arc::new(array),
        }
    }

    /// Create a Series from an Arrow array
    pub fn from_arrow(name: impl Into<String>, data: ArrayRef) -> Self {
        Self {
            name: name.into(),
            data,
        }
    }

    /// Get the name of the series
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the data type
    pub fn dtype(&self) -> &DataType {
        self.data.data_type()
    }

    /// Get the length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the underlying Arrow array
    pub fn array(&self) -> &ArrayRef {
        &self.data
    }

    /// Rename the series
    pub fn rename(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Get value as f64 at index (for numeric series)
    pub fn get_f64(&self, index: usize) -> Result<f64> {
        if index >= self.len() {
            return Err(AvilaError::generic(format!("Index {} out of bounds", index)));
        }

        match self.data.data_type() {
            DataType::Float64 => {
                let array = self.data.as_any().downcast_ref::<Float64Array>()
                    .ok_or_else(|| AvilaError::generic("Failed to downcast to Float64Array"))?;
                Ok(array.value(index))
            }
            DataType::Int64 => {
                let array = self.data.as_any().downcast_ref::<Int64Array>()
                    .ok_or_else(|| AvilaError::generic("Failed to downcast to Int64Array"))?;
                Ok(array.value(index) as f64)
            }
            _ => Err(AvilaError::type_error("numeric type", format!("{:?}", self.dtype())))
        }
    }

    /// Calculate mean (for numeric series)
    pub fn mean(&self) -> Result<f64> {
        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += self.get_f64(i)?;
        }
        Ok(sum / self.len() as f64)
    }

    /// Calculate sum (for numeric series)
    pub fn sum(&self) -> Result<f64> {
        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += self.get_f64(i)?;
        }
        Ok(sum)
    }

    /// Calculate standard deviation
    pub fn std(&self) -> Result<f64> {
        let mean = self.mean()?;
        let mut sum_sq_diff = 0.0;
        for i in 0..self.len() {
            let diff = self.get_f64(i)? - mean;
            sum_sq_diff += diff * diff;
        }
        Ok((sum_sq_diff / self.len() as f64).sqrt())
    }
}

impl std::fmt::Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Series: '{}' [{}]", self.name, self.dtype())?;
        writeln!(f, "Length: {}", self.len())?;

        let display_rows = std::cmp::min(10, self.len());
        for i in 0..display_rows {
            match self.dtype() {
                DataType::Float64 | DataType::Int64 => {
                    if let Ok(val) = self.get_f64(i) {
                        writeln!(f, "{}: {}", i, val)?;
                    }
                }
                _ => {
                    writeln!(f, "{}: ...", i)?;
                }
            }
        }

        if self.len() > display_rows {
            writeln!(f, "... ({} more rows)", self.len() - display_rows)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_series_creation() {
        let s = Series::new("test", vec![1.0, 2.0, 3.0]);
        assert_eq!(s.name(), "test");
        assert_eq!(s.len(), 3);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_series_mean() {
        let s = Series::new("values", vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_abs_diff_eq!(s.mean().unwrap(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_series_std() {
        let s = Series::new("values", vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
        let std = s.std().unwrap();
        assert_abs_diff_eq!(std, 2.0, epsilon = 0.1);
    }
}
