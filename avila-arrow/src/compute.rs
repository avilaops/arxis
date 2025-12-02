//! Compute kernels for array operations
//!
//! This module provides optimized operations for arrays, including:
//! - Aggregations (sum, mean, min, max)
//! - Filters (filter by boolean mask)
//! - Sorting

use crate::array::*;
use crate::error::{ArrowError, Result};

// ==================== AGGREGATIONS ====================

/// Sum of all elements in an Int64Array
pub fn sum_i64(array: &Int64Array) -> i64 {
    array.values().iter().sum()
}

/// Sum of all elements in a Float64Array
pub fn sum_f64(array: &Float64Array) -> f64 {
    array.values().iter().sum()
}

/// Mean of all elements in an Int64Array
pub fn mean_i64(array: &Int64Array) -> Option<f64> {
    if array.is_empty() {
        return None;
    }
    Some(sum_i64(array) as f64 / array.len() as f64)
}

/// Mean of all elements in a Float64Array
pub fn mean_f64(array: &Float64Array) -> Option<f64> {
    if array.is_empty() {
        return None;
    }
    Some(sum_f64(array) / array.len() as f64)
}

/// Minimum value in an Int64Array
pub fn min_i64(array: &Int64Array) -> Option<i64> {
    array.values().iter().min().copied()
}

/// Minimum value in a Float64Array
pub fn min_f64(array: &Float64Array) -> Option<f64> {
    array.values().iter().fold(None, |acc, &x| {
        Some(acc.map_or(x, |a| if x < a { x } else { a }))
    })
}

/// Maximum value in an Int64Array
pub fn max_i64(array: &Int64Array) -> Option<i64> {
    array.values().iter().max().copied()
}

/// Maximum value in a Float64Array
pub fn max_f64(array: &Float64Array) -> Option<f64> {
    array.values().iter().fold(None, |acc, &x| {
        Some(acc.map_or(x, |a| if x > a { x } else { a }))
    })
}

// ==================== FILTERS ====================

/// Filter an Int64Array by a boolean mask
pub fn filter_i64(array: &Int64Array, mask: &BooleanArray) -> Result<Int64Array> {
    if array.len() != mask.len() {
        return Err(ArrowError::SchemaMismatch {
            expected: format!("mask length {}", array.len()),
            actual: format!("mask length {}", mask.len()),
        });
    }

    let filtered: Vec<i64> = array.values()
        .iter()
        .zip(mask.values().iter())
        .filter_map(|(&value, &keep)| if keep { Some(value) } else { None })
        .collect();

    Ok(Int64Array::new(filtered))
}

/// Filter a Float64Array by a boolean mask
pub fn filter_f64(array: &Float64Array, mask: &BooleanArray) -> Result<Float64Array> {
    if array.len() != mask.len() {
        return Err(ArrowError::SchemaMismatch {
            expected: format!("mask length {}", array.len()),
            actual: format!("mask length {}", mask.len()),
        });
    }

    let filtered: Vec<f64> = array.values()
        .iter()
        .zip(mask.values().iter())
        .filter_map(|(&value, &keep)| if keep { Some(value) } else { None })
        .collect();

    Ok(Float64Array::new(filtered))
}

/// Filter a Utf8Array by a boolean mask
pub fn filter_utf8(array: &Utf8Array, mask: &BooleanArray) -> Result<Utf8Array> {
    if array.len() != mask.len() {
        return Err(ArrowError::SchemaMismatch {
            expected: format!("mask length {}", array.len()),
            actual: format!("mask length {}", mask.len()),
        });
    }

    let filtered: Vec<String> = array.values()
        .iter()
        .zip(mask.values().iter())
        .filter_map(|(value, &keep)| if keep { Some(value.clone()) } else { None })
        .collect();

    Ok(Utf8Array::new(filtered))
}

// ==================== COMPARISONS ====================

/// Greater than comparison for Int64Array
pub fn gt_i64(array: &Int64Array, scalar: i64) -> BooleanArray {
    let mask: Vec<bool> = array.values().iter().map(|&x| x > scalar).collect();
    BooleanArray::new(mask)
}

/// Less than comparison for Int64Array
pub fn lt_i64(array: &Int64Array, scalar: i64) -> BooleanArray {
    let mask: Vec<bool> = array.values().iter().map(|&x| x < scalar).collect();
    BooleanArray::new(mask)
}

/// Equal comparison for Int64Array
pub fn eq_i64(array: &Int64Array, scalar: i64) -> BooleanArray {
    let mask: Vec<bool> = array.values().iter().map(|&x| x == scalar).collect();
    BooleanArray::new(mask)
}

/// Greater than comparison for Float64Array
pub fn gt_f64(array: &Float64Array, scalar: f64) -> BooleanArray {
    let mask: Vec<bool> = array.values().iter().map(|&x| x > scalar).collect();
    BooleanArray::new(mask)
}

/// Less than comparison for Float64Array
pub fn lt_f64(array: &Float64Array, scalar: f64) -> BooleanArray {
    let mask: Vec<bool> = array.values().iter().map(|&x| x < scalar).collect();
    BooleanArray::new(mask)
}

// ==================== SORTING ====================

/// Sort an Int64Array in ascending order
pub fn sort_i64(array: &Int64Array) -> Int64Array {
    let mut sorted = array.values().to_vec();
    sorted.sort_unstable();
    Int64Array::new(sorted)
}

/// Sort a Float64Array in ascending order
pub fn sort_f64(array: &Float64Array) -> Float64Array {
    let mut sorted = array.values().to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    Float64Array::new(sorted)
}

/// Sort a Utf8Array in lexicographic order
pub fn sort_utf8(array: &Utf8Array) -> Utf8Array {
    let mut sorted = array.values().to_vec();
    sorted.sort();
    Utf8Array::new(sorted)
}

// ==================== ARITHMETIC ====================

/// Add two Int64Arrays element-wise
pub fn add_i64(left: &Int64Array, right: &Int64Array) -> Result<Int64Array> {
    if left.len() != right.len() {
        return Err(ArrowError::SchemaMismatch {
            expected: format!("array length {}", left.len()),
            actual: format!("array length {}", right.len()),
        });
    }

    let result: Vec<i64> = left.values()
        .iter()
        .zip(right.values().iter())
        .map(|(&a, &b)| a + b)
        .collect();

    Ok(Int64Array::new(result))
}

/// Add two Float64Arrays element-wise
pub fn add_f64(left: &Float64Array, right: &Float64Array) -> Result<Float64Array> {
    if left.len() != right.len() {
        return Err(ArrowError::SchemaMismatch {
            expected: format!("array length {}", left.len()),
            actual: format!("array length {}", right.len()),
        });
    }

    let result: Vec<f64> = left.values()
        .iter()
        .zip(right.values().iter())
        .map(|(&a, &b)| a + b)
        .collect();

    Ok(Float64Array::new(result))
}

/// Subtract two Int64Arrays element-wise
pub fn sub_i64(left: &Int64Array, right: &Int64Array) -> Result<Int64Array> {
    if left.len() != right.len() {
        return Err(ArrowError::SchemaMismatch {
            expected: format!("array length {}", left.len()),
            actual: format!("array length {}", right.len()),
        });
    }

    let result: Vec<i64> = left.values()
        .iter()
        .zip(right.values().iter())
        .map(|(&a, &b)| a - b)
        .collect();

    Ok(Int64Array::new(result))
}

/// Multiply two Float64Arrays element-wise
pub fn mul_f64(left: &Float64Array, right: &Float64Array) -> Result<Float64Array> {
    if left.len() != right.len() {
        return Err(ArrowError::SchemaMismatch {
            expected: format!("array length {}", left.len()),
            actual: format!("array length {}", right.len()),
        });
    }

    let result: Vec<f64> = left.values()
        .iter()
        .zip(right.values().iter())
        .map(|(&a, &b)| a * b)
        .collect();

    Ok(Float64Array::new(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_i64() {
        let array = Int64Array::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(sum_i64(&array), 15);
    }

    #[test]
    fn test_sum_f64() {
        let array = Float64Array::from(vec![1.0, 2.0, 3.0]);
        assert_eq!(sum_f64(&array), 6.0);
    }

    #[test]
    fn test_mean_i64() {
        let array = Int64Array::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(mean_i64(&array), Some(3.0));
    }

    #[test]
    fn test_mean_f64() {
        let array = Float64Array::from(vec![1.0, 2.0, 3.0]);
        assert_eq!(mean_f64(&array), Some(2.0));
    }

    #[test]
    fn test_min_max_i64() {
        let array = Int64Array::from(vec![5, 2, 8, 1, 9]);
        assert_eq!(min_i64(&array), Some(1));
        assert_eq!(max_i64(&array), Some(9));
    }

    #[test]
    fn test_min_max_f64() {
        let array = Float64Array::from(vec![5.5, 2.2, 8.8, 1.1, 9.9]);
        assert_eq!(min_f64(&array), Some(1.1));
        assert_eq!(max_f64(&array), Some(9.9));
    }

    #[test]
    fn test_filter_i64() {
        let array = Int64Array::from(vec![1, 2, 3, 4, 5]);
        let mask = BooleanArray::from(vec![true, false, true, false, true]);
        let filtered = filter_i64(&array, &mask).unwrap();
        assert_eq!(filtered.values(), &[1, 3, 5]);
    }

    #[test]
    fn test_filter_f64() {
        let array = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0]);
        let mask = BooleanArray::from(vec![true, true, false, true]);
        let filtered = filter_f64(&array, &mask).unwrap();
        assert_eq!(filtered.values(), &[1.0, 2.0, 4.0]);
    }

    #[test]
    fn test_gt_lt_i64() {
        let array = Int64Array::from(vec![1, 5, 10, 15]);
        let gt_mask = gt_i64(&array, 5);
        assert_eq!(gt_mask.values(), &[false, false, true, true]);

        let lt_mask = lt_i64(&array, 10);
        assert_eq!(lt_mask.values(), &[true, true, false, false]);
    }

    #[test]
    fn test_sort_i64() {
        let array = Int64Array::from(vec![5, 2, 8, 1, 9]);
        let sorted = sort_i64(&array);
        assert_eq!(sorted.values(), &[1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_add_i64() {
        let left = Int64Array::from(vec![1, 2, 3]);
        let right = Int64Array::from(vec![10, 20, 30]);
        let result = add_i64(&left, &right).unwrap();
        assert_eq!(result.values(), &[11, 22, 33]);
    }

    #[test]
    fn test_add_f64() {
        let left = Float64Array::from(vec![1.1, 2.2, 3.3]);
        let right = Float64Array::from(vec![10.0, 20.0, 30.0]);
        let result = add_f64(&left, &right).unwrap();
        assert!((result.value(0).unwrap() - 11.1).abs() < 1e-10);
    }

    #[test]
    fn test_sub_i64() {
        let left = Int64Array::from(vec![10, 20, 30]);
        let right = Int64Array::from(vec![1, 2, 3]);
        let result = sub_i64(&left, &right).unwrap();
        assert_eq!(result.values(), &[9, 18, 27]);
    }

    #[test]
    fn test_mul_f64() {
        let left = Float64Array::from(vec![2.0, 3.0, 4.0]);
        let right = Float64Array::from(vec![5.0, 6.0, 7.0]);
        let result = mul_f64(&left, &right).unwrap();
        assert_eq!(result.values(), &[10.0, 18.0, 28.0]);
    }

    #[test]
    fn test_filter_mismatch() {
        let array = Int64Array::from(vec![1, 2, 3]);
        let mask = BooleanArray::from(vec![true, false]); // Wrong length
        assert!(filter_i64(&array, &mask).is_err());
    }
}
