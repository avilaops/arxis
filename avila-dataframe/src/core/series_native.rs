//! Core: Series - Coluna de dados tipada

use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipos de dados suportados
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Float64,
    Int64,
    String,
    Bool,
    DateTime,
}

/// Valor individual em uma Series
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Float(f64),
    Int(i64),
    Str(String),
    Bool(bool),
    DateTime(i64), // Unix timestamp
    Null,
}

impl Value {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(*v),
            Value::Int(v) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            Value::Float(v) => Some(*v as i64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Float(v) => write!(f, "{}", v),
            Value::Int(v) => write!(f, "{}", v),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::DateTime(ts) => write!(f, "{}", ts),
            Value::Null => write!(f, "null"),
        }
    }
}

/// Series: Uma coluna de dados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub name: String,
    pub dtype: DataType,
    pub data: Vec<Value>,
}

impl Series {
    /// Criar Series de floats
    pub fn new_float(name: impl Into<String>, data: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            dtype: DataType::Float64,
            data: data.into_iter().map(Value::Float).collect(),
        }
    }

    /// Criar Series de integers
    pub fn new_int(name: impl Into<String>, data: Vec<i64>) -> Self {
        Self {
            name: name.into(),
            dtype: DataType::Int64,
            data: data.into_iter().map(Value::Int).collect(),
        }
    }

    /// Criar Series de strings
    pub fn new_str(name: impl Into<String>, data: Vec<String>) -> Self {
        Self {
            name: name.into(),
            dtype: DataType::String,
            data: data.into_iter().map(Value::Str).collect(),
        }
    }

    /// Criar Series de bools
    pub fn new_bool(name: impl Into<String>, data: Vec<bool>) -> Self {
        Self {
            name: name.into(),
            dtype: DataType::Bool,
            data: data.into_iter().map(Value::Bool).collect(),
        }
    }

    /// Tamanho da Series
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Obter valor em índice
    pub fn get(&self, idx: usize) -> Option<&Value> {
        self.data.get(idx)
    }

    /// Nome da Series
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Tipo de dados
    pub fn dtype(&self) -> &DataType {
        &self.dtype
    }

    /// Converter para Vec<f64> (se possível)
    pub fn to_vec_f64(&self) -> Option<Vec<f64>> {
        self.data.iter().map(|v| v.as_f64()).collect()
    }

    /// Converter para Vec<i64> (se possível)
    pub fn to_vec_i64(&self) -> Option<Vec<i64>> {
        self.data.iter().map(|v| v.as_i64()).collect()
    }

    /// Aplicar função element-wise
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(&Value) -> Value,
    {
        Self {
            name: self.name.clone(),
            dtype: self.dtype.clone(),
            data: self.data.iter().map(f).collect(),
        }
    }

    /// Filtrar valores
    pub fn filter(&self, mask: &[bool]) -> Self {
        assert_eq!(mask.len(), self.len(), "Mask deve ter mesmo tamanho");

        Self {
            name: self.name.clone(),
            dtype: self.dtype.clone(),
            data: self
                .data
                .iter()
                .zip(mask)
                .filter(|(_, &keep)| keep)
                .map(|(v, _)| v.clone())
                .collect(),
        }
    }

    /// Soma (para numéricos)
    pub fn sum(&self) -> Option<f64> {
        self.data
            .iter()
            .filter_map(|v| v.as_f64())
            .reduce(|a, b| a + b)
    }

    /// Média
    pub fn mean(&self) -> Option<f64> {
        let sum = self.sum()?;
        let count = self.data.iter().filter(|v| !v.is_null()).count();
        Some(sum / count as f64)
    }

    /// Mínimo
    pub fn min(&self) -> Option<f64> {
        self.data.iter().filter_map(|v| v.as_f64()).reduce(f64::min)
    }

    /// Máximo
    pub fn max(&self) -> Option<f64> {
        self.data.iter().filter_map(|v| v.as_f64()).reduce(f64::max)
    }
}

impl fmt::Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Series: {} ({:?})", self.name, self.dtype)?;
        for (i, val) in self.data.iter().enumerate().take(10) {
            writeln!(f, "  [{}] {}", i, val)?;
        }
        if self.len() > 10 {
            writeln!(f, "  ... ({} mais)", self.len() - 10)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_float() {
        let s = Series::new_float("values", vec![1.0, 2.0, 3.0]);
        assert_eq!(s.len(), 3);
        assert_eq!(s.sum(), Some(6.0));
        assert_eq!(s.mean(), Some(2.0));
    }

    #[test]
    fn test_series_filter() {
        let s = Series::new_float("values", vec![1.0, 2.0, 3.0, 4.0]);
        let filtered = s.filter(&[true, false, true, false]);
        assert_eq!(filtered.len(), 2);
    }
}
