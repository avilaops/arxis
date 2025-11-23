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

    /// FFT - Fast Fourier Transform
    /// Retorna o espectro de frequências como vetor de números complexos
    pub fn fft(&self) -> Result<Vec<crate::scientific::Complex<f64>>, String> {
        // Extrair valores numéricos
        let values: Vec<f64> = self
            .data
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();

        if values.is_empty() {
            return Err("Series não contém valores numéricos".to_string());
        }

        // Aplicar RFFT (otimizado para sinais reais)
        let spectrum = crate::scientific::rfft(&values);
        Ok(spectrum)
    }

    /// IFFT - Inverse Fast Fourier Transform
    /// Reconstrói o sinal a partir do espectro de frequências
    pub fn ifft(
        &self,
        spectrum: &[crate::scientific::Complex<f64>],
        n: usize,
    ) -> Result<Self, String> {
        use crate::scientific::fft_pure::irfft;

        let reconstructed = irfft(spectrum, n);

        Ok(Series::new_float(&self.name, reconstructed))
    }

    /// Power Spectrum - Espectro de potência
    /// Retorna uma nova Series com as magnitudes ao quadrado
    pub fn power_spectrum(&self, sample_rate: f64) -> Result<Self, String> {
        let spectrum = self.fft()?;
        let n = self.len();

        let power: Vec<f64> = spectrum
            .iter()
            .map(|z| {
                let mag_sq = z.magnitude_squared();
                mag_sq / (n * n) as f64 * 2.0
            })
            .collect();

        // Calcular frequências correspondentes
        let freq_bin = sample_rate / n as f64;
        let _frequencies: Vec<f64> = (0..power.len())
            .map(|k| k as f64 * freq_bin)
            .collect();

        Ok(Series::new_float(
            format!("{}_power_spectrum", self.name),
            power,
        ))
    }

    /// Magnitude Spectrum - Espectro de magnitude
    pub fn magnitude_spectrum(&self) -> Result<Self, String> {
        let spectrum = self.fft()?;

        let magnitudes: Vec<f64> = spectrum.iter().map(|z| z.magnitude()).collect();

        Ok(Series::new_float(
            format!("{}_magnitude", self.name),
            magnitudes,
        ))
    }

    /// Phase Spectrum - Espectro de fase
    pub fn phase_spectrum(&self) -> Result<Self, String> {
        let spectrum = self.fft()?;

        let phases: Vec<f64> = spectrum.iter().map(|z| z.phase()).collect();

        Ok(Series::new_float(format!("{}_phase", self.name), phases))
    }

    /// Convolução com outro sinal usando FFT
    pub fn convolve(&self, other: &Self) -> Result<Self, String> {
        use crate::scientific::convolve_fft;

        let values1: Vec<f64> = self
            .data
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();

        let values2: Vec<f64> = other
            .data
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();

        if values1.is_empty() || values2.is_empty() {
            return Err("Series devem conter valores numéricos".to_string());
        }

        let result = convolve_fft(&values1, &values2);

        Ok(Series::new_float(
            format!("{}_convolved", self.name),
            result,
        ))
    }

    /// Correlação cruzada com outro sinal usando FFT
    pub fn xcorr(&self, other: &Self) -> Result<Self, String> {
        use crate::scientific::xcorr_fft;

        let values1: Vec<f64> = self
            .data
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();

        let values2: Vec<f64> = other
            .data
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();

        if values1.is_empty() || values2.is_empty() {
            return Err("Series devem conter valores numéricos".to_string());
        }

        let result = xcorr_fft(&values1, &values2);

        Ok(Series::new_float(format!("{}_xcorr", self.name), result))
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
