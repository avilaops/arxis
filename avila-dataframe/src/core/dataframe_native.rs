//! Core: DataFrame - Tabela de dados

use super::series_native::{Series, Value};
use crate::error::{AvilaError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Alias para compatibilidade
pub type Column = Series;

/// DataFrame: Tabela 2D de dados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrame {
    pub columns: Vec<Series>,
}

impl DataFrame {
    /// Criar DataFrame vazio
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    /// Criar DataFrame de Series
    pub fn from_series(columns: Vec<Series>) -> Result<Self> {
        if columns.is_empty() {
            return Ok(Self::new());
        }

        // Verificar que todas têm mesmo tamanho
        let len = columns[0].len();
        for col in &columns {
            if col.len() != len {
                return Err(AvilaError::shape_mismatch(format!(
                    "Todas as colunas devem ter mesmo tamanho: esperado {}, encontrado {}",
                    len,
                    col.len()
                )));
            }
        }

        Ok(Self { columns })
    }

    /// Número de linhas
    pub fn height(&self) -> usize {
        self.columns.first().map(|c| c.len()).unwrap_or(0)
    }

    /// Número de colunas
    pub fn width(&self) -> usize {
        self.columns.len()
    }

    /// Shape (linhas, colunas)
    pub fn shape(&self) -> (usize, usize) {
        (self.height(), self.width())
    }

    /// Está vazio?
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    /// Nomes das colunas
    pub fn column_names(&self) -> Vec<&str> {
        self.columns.iter().map(|c| c.name()).collect()
    }

    /// Obter coluna por nome
    pub fn column(&self, name: &str) -> Result<&Series> {
        self.columns
            .iter()
            .find(|c| c.name() == name)
            .ok_or_else(|| AvilaError::column_not_found(name))
    }

    /// Obter coluna mutável
    pub fn column_mut(&mut self, name: &str) -> Result<&mut Series> {
        self.columns
            .iter_mut()
            .find(|c| c.name() == name)
            .ok_or_else(|| AvilaError::column_not_found(name))
    }

    /// Adicionar coluna
    pub fn add_column(&mut self, series: Series) -> Result<()> {
        if !self.is_empty() && series.len() != self.height() {
            return Err(AvilaError::shape_mismatch(format!(
                "Coluna deve ter {} linhas, tem {}",
                self.height(),
                series.len()
            )));
        }
        self.columns.push(series);
        Ok(())
    }

    /// Selecionar colunas
    pub fn select(&self, names: &[&str]) -> Result<Self> {
        let mut new_columns = Vec::new();
        for name in names {
            new_columns.push(self.column(name)?.clone());
        }
        Ok(Self {
            columns: new_columns,
        })
    }

    /// Filtrar linhas por mask booleana
    pub fn filter(&self, mask: &[bool]) -> Result<Self> {
        if mask.len() != self.height() {
            return Err(AvilaError::shape_mismatch(format!(
                "Mask deve ter {} elementos, tem {}",
                self.height(),
                mask.len()
            )));
        }

        let filtered_columns = self.columns.iter().map(|col| col.filter(mask)).collect();

        Ok(Self {
            columns: filtered_columns,
        })
    }

    /// Pegar primeiras N linhas
    pub fn head(&self, n: usize) -> Self {
        let n = n.min(self.height());
        let mask: Vec<bool> = (0..self.height()).map(|i| i < n).collect();
        self.filter(&mask).unwrap()
    }

    /// Pegar últimas N linhas
    pub fn tail(&self, n: usize) -> Self {
        let n = n.min(self.height());
        let start = self.height().saturating_sub(n);
        let mask: Vec<bool> = (0..self.height()).map(|i| i >= start).collect();
        self.filter(&mask).unwrap()
    }

    /// Obter linha como Vec<Value>
    pub fn row(&self, idx: usize) -> Result<Vec<Value>> {
        if idx >= self.height() {
            return Err(AvilaError::index_out_of_bounds(idx, self.height()));
        }

        Ok(self
            .columns
            .iter()
            .map(|col| col.get(idx).cloned().unwrap_or(Value::Null))
            .collect())
    }

    /// Iterar sobre linhas
    pub fn rows(&self) -> impl Iterator<Item = Vec<Value>> + '_ {
        (0..self.height()).map(move |i| self.row(i).unwrap())
    }

    /// Estatísticas descritivas
    pub fn describe(&self) -> Self {
        let stats = vec!["count", "mean", "min", "max"];
        let mut result_columns = vec![Series::new_str(
            "stat",
            stats.iter().map(|s| s.to_string()).collect(),
        )];

        for col in &self.columns {
            let count = col.len() as f64;
            let mean = col.mean().unwrap_or(f64::NAN);
            let min = col.min().unwrap_or(f64::NAN);
            let max = col.max().unwrap_or(f64::NAN);

            result_columns.push(Series::new_float(col.name(), vec![count, mean, min, max]));
        }

        Self {
            columns: result_columns,
        }
    }

    /// FFT em uma coluna específica - Retorna espectro de magnitude
    pub fn fft_column(&self, column: &str) -> Result<Self> {
        let series = self.column(column)?;
        let magnitude_spectrum = series.magnitude_spectrum().map_err(|e| {
            AvilaError::InvalidOperation(format!("FFT falhou: {}", e))
        })?;

        let mut new_df = DataFrame::new();
        new_df.add_column(magnitude_spectrum)?;
        Ok(new_df)
    }

    /// Spectrogram de uma coluna
    pub fn spectrogram_column(
        &self,
        column: &str,
        window_size: usize,
        hop_size: usize,
        sample_rate: f64,
    ) -> Result<Self> {
        use crate::scientific::spectrogram::{stft, WindowType};

        let series = self.column(column)?;
        let values: Vec<f64> = series
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();

        if values.is_empty() {
            return Err(AvilaError::InvalidOperation(
                "Coluna não contém valores numéricos".to_string(),
            ));
        }

        let (spectrogram, _frequencies, times) =
            stft(&values, window_size, hop_size, sample_rate, WindowType::Hann);

        // Criar DataFrame com resultados
        let mut result = DataFrame::new();

        // Adicionar coluna de tempos
        result.add_column(Series::new_float("time", times))?;

        // Adicionar colunas para cada bin de frequência
        if !spectrogram.is_empty() {
            let n_freqs = spectrogram[0].len();
            for freq_idx in 0..n_freqs {
                let freq_data: Vec<f64> = spectrogram
                    .iter()
                    .map(|frame| frame[freq_idx])
                    .collect();
                result.add_column(Series::new_float(
                    format!("freq_{}", freq_idx),
                    freq_data,
                ))?;
            }
        }

        Ok(result)
    }

    /// Power Spectrum de uma coluna
    pub fn power_spectrum_column(&self, column: &str, sample_rate: f64) -> Result<Self> {
        let series = self.column(column)?;
        let power_series = series.power_spectrum(sample_rate).map_err(|e| {
            AvilaError::InvalidOperation(format!(
                "Power spectrum falhou: {}",
                e
            ))
        })?;

        let mut new_df = DataFrame::new();
        new_df.add_column(power_series)?;
        Ok(new_df)
    }

    /// Convolução entre duas colunas
    pub fn convolve_columns(&self, col1: &str, col2: &str) -> Result<Self> {
        let series1 = self.column(col1)?;
        let series2 = self.column(col2)?;

        let result_series = series1.convolve(series2).map_err(|e| {
            AvilaError::InvalidOperation(format!("Convolução falhou: {}", e))
        })?;

        let mut new_df = DataFrame::new();
        new_df.add_column(result_series)?;
        Ok(new_df)
    }

    /// Correlação cruzada entre duas colunas
    pub fn xcorr_columns(&self, col1: &str, col2: &str) -> Result<Self> {
        let series1 = self.column(col1)?;
        let series2 = self.column(col2)?;

        let result_series = series1.xcorr(series2).map_err(|e| {
            AvilaError::InvalidOperation(format!(
                "Correlação cruzada falhou: {}",
                e
            ))
        })?;

        let mut new_df = DataFrame::new();
        new_df.add_column(result_series)?;
        Ok(new_df)
    }
}


impl Default for DataFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "DataFrame [{} rows × {} cols]",
            self.height(),
            self.width()
        )?;
        writeln!(f, "{}", "─".repeat(80))?;

        // Header
        write!(f, "│ ")?;
        for col in &self.columns {
            write!(f, "{:>12} │ ", col.name())?;
        }
        writeln!(f)?;
        writeln!(f, "{}", "─".repeat(80))?;

        // Rows (primeiras 10)
        let display_rows = self.height().min(10);
        for i in 0..display_rows {
            write!(f, "│ ")?;
            for col in &self.columns {
                if let Some(val) = col.get(i) {
                    write!(f, "{:>12} │ ", format!("{}", val))?;
                }
            }
            writeln!(f)?;
        }

        if self.height() > display_rows {
            writeln!(f, "... ({} mais linhas)", self.height() - display_rows)?;
        }

        writeln!(f, "{}", "─".repeat(80))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe_creation() {
        let df = DataFrame::from_series(vec![
            Series::new_float("a", vec![1.0, 2.0, 3.0]),
            Series::new_float("b", vec![4.0, 5.0, 6.0]),
        ])
        .unwrap();

        assert_eq!(df.shape(), (3, 2));
        assert_eq!(df.column_names(), vec!["a", "b"]);
    }

    #[test]
    fn test_select() {
        let df = DataFrame::from_series(vec![
            Series::new_float("a", vec![1.0, 2.0]),
            Series::new_float("b", vec![3.0, 4.0]),
            Series::new_float("c", vec![5.0, 6.0]),
        ])
        .unwrap();

        let selected = df.select(&["a", "c"]).unwrap();
        assert_eq!(selected.width(), 2);
        assert_eq!(selected.column_names(), vec!["a", "c"]);
    }

    #[test]
    fn test_filter() {
        let df = DataFrame::from_series(vec![
            Series::new_float("x", vec![1.0, 2.0, 3.0, 4.0]),
            Series::new_float("y", vec![5.0, 6.0, 7.0, 8.0]),
        ])
        .unwrap();

        let filtered = df.filter(&[true, false, true, false]).unwrap();
        assert_eq!(filtered.height(), 2);
    }
}
