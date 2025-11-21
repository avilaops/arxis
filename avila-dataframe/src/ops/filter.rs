//! Filter operations

use crate::core::{DataFrame, Series};
use crate::error::{AvilaError, Result};
use crate::ops::expressions::{Expr, LiteralValue, Operator};
use arrow::array::BooleanArray;

impl DataFrame {
    /// Filter rows based on a boolean expression
    pub fn filter(&self, expr: Expr) -> Result<Self> {
        if self.is_empty() {
            return Ok(self.clone());
        }

        // Evaluate expression to get boolean mask
        let mask = self.evaluate_boolean_expr(&expr)?;

        // Apply filter to all columns
        let filtered_columns: Result<Vec<Series>> = self
            .columns
            .iter()
            .map(|series| filter_series(series, &mask))
            .collect();

        DataFrame::new(filtered_columns?)
    }

    /// Evaluate a boolean expression to get a mask array
    fn evaluate_boolean_expr(&self, expr: &Expr) -> Result<BooleanArray> {
        match expr {
            Expr::BinaryOp { left, op, right } => self.evaluate_binary_op(left, *op, right),
            _ => Err(AvilaError::generic(
                "Filter expression must be a boolean comparison",
            )),
        }
    }

    /// Evaluate binary operation
    fn evaluate_binary_op(&self, left: &Expr, op: Operator, right: &Expr) -> Result<BooleanArray> {
        use Operator::*;

        match op {
            Gt | GtEq | Lt | LtEq | Eq | NotEq => self.evaluate_comparison(left, op, right),
            And | Or => {
                let left_mask = self.evaluate_boolean_expr(left)?;
                let right_mask = self.evaluate_boolean_expr(right)?;

                let result: Vec<bool> = (0..left_mask.len())
                    .map(|i| {
                        let l = left_mask.value(i);
                        let r = right_mask.value(i);
                        match op {
                            And => l && r,
                            Or => l || r,
                            _ => unreachable!(),
                        }
                    })
                    .collect();

                Ok(BooleanArray::from(result))
            }
            _ => Err(AvilaError::generic(format!(
                "Operator {:?} not supported in filter",
                op
            ))),
        }
    }

    /// Evaluate comparison expression
    fn evaluate_comparison(&self, left: &Expr, op: Operator, right: &Expr) -> Result<BooleanArray> {
        // Get left values
        let left_values = self.evaluate_numeric_expr(left)?;

        // Get right values (could be column or literal)
        let right_values = self.evaluate_numeric_expr(right)?;

        if left_values.len() != right_values.len() {
            return Err(AvilaError::ShapeMismatch(format!(
                "Left and right expressions have different lengths: {} vs {}",
                left_values.len(),
                right_values.len()
            )));
        }

        // Apply comparison
        let result: Vec<bool> = (0..left_values.len())
            .map(|i| {
                let l = left_values[i];
                let r = right_values[i];
                match op {
                    Operator::Gt => l > r,
                    Operator::GtEq => l >= r,
                    Operator::Lt => l < r,
                    Operator::LtEq => l <= r,
                    Operator::Eq => (l - r).abs() < f64::EPSILON,
                    Operator::NotEq => (l - r).abs() >= f64::EPSILON,
                    _ => false,
                }
            })
            .collect();

        Ok(BooleanArray::from(result))
    }

    /// Evaluate numeric expression to get values
    fn evaluate_numeric_expr(&self, expr: &Expr) -> Result<Vec<f64>> {
        match expr {
            Expr::Column(name) => {
                let series = self.column(name)?;
                (0..series.len()).map(|i| series.get_f64(i)).collect()
            }
            Expr::Literal(lit) => {
                let val = match lit {
                    LiteralValue::Float64(v) => *v,
                    LiteralValue::Int64(v) => *v as f64,
                    _ => {
                        return Err(AvilaError::type_error(
                            "numeric literal",
                            format!("{:?}", lit),
                        ))
                    }
                };
                Ok(vec![val; self.height()])
            }
            Expr::BinaryOp { left, op, right } => {
                let left_vals = self.evaluate_numeric_expr(left)?;
                let right_vals = self.evaluate_numeric_expr(right)?;

                let result: Vec<f64> = (0..left_vals.len())
                    .map(|i| {
                        let l = left_vals[i];
                        let r = right_vals[i];
                        match op {
                            Operator::Add => l + r,
                            Operator::Sub => l - r,
                            Operator::Mul => l * r,
                            Operator::Div => l / r,
                            _ => f64::NAN,
                        }
                    })
                    .collect();

                Ok(result)
            }
            _ => Err(AvilaError::generic(format!(
                "Cannot evaluate expression as numeric: {:?}",
                expr
            ))),
        }
    }
}

/// Filter a series by a boolean mask
fn filter_series(series: &Series, mask: &BooleanArray) -> Result<Series> {
    if series.len() != mask.len() {
        return Err(AvilaError::ShapeMismatch(format!(
            "Series length {} != mask length {}",
            series.len(),
            mask.len()
        )));
    }

    // Collect indices where mask is true
    let indices: Vec<usize> = (0..mask.len()).filter(|&i| mask.value(i)).collect();

    // Take values at those indices
    let filtered_array = arrow::compute::take(
        series.array().as_ref(),
        &arrow::array::UInt64Array::from(indices.iter().map(|&i| i as u64).collect::<Vec<_>>()),
        None,
    )?;

    Ok(Series::from_arrow(series.name(), filtered_array))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{col, lit};

    #[test]
    fn test_filter_gt() {
        let df = DataFrame::new(vec![
            Series::new("a", vec![1.0, 2.0, 3.0, 4.0, 5.0]),
            Series::new("b", vec![10.0, 20.0, 30.0, 40.0, 50.0]),
        ])
        .unwrap();

        let filtered = df.filter(col("a").gt(lit(2.5))).unwrap();
        assert_eq!(filtered.height(), 3);
    }

    #[test]
    fn test_filter_eq() {
        let df = DataFrame::new(vec![Series::new("x", vec![1.0, 2.0, 3.0, 2.0])]).unwrap();

        let filtered = df.filter(col("x").eq(lit(2.0))).unwrap();
        assert_eq!(filtered.height(), 2);
    }
}
