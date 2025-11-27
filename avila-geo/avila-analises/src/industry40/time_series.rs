use serde::{Deserialize, Serialize};

/// Analisador de séries temporais para dados industriais
pub struct TimeSeriesAnalyzer {
    window_size: usize,
}

impl TimeSeriesAnalyzer {
    pub fn new(window_size: usize) -> Self {
        Self { window_size }
    }

    /// Detectar tendência
    pub fn detect_trend(&self, values: &[f64]) -> Trend {
        if values.len() < 2 {
            return Trend::Stable;
        }

        let first_half_avg = values[..values.len()/2].iter().sum::<f64>() / (values.len()/2) as f64;
        let second_half_avg = values[values.len()/2..].iter().sum::<f64>() / (values.len()/2) as f64;

        if second_half_avg > first_half_avg * 1.1 {
            Trend::Increasing
        } else if second_half_avg < first_half_avg * 0.9 {
            Trend::Decreasing
        } else {
            Trend::Stable
        }
    }

    /// Calcular média móvel
    pub fn moving_average(&self, values: &[f64]) -> Vec<f64> {
        values.windows(self.window_size)
            .map(|w| w.iter().sum::<f64>() / w.len() as f64)
            .collect()
    }
}

/// Detector de anomalias em séries temporais
pub struct AnomalyDetector {
    threshold_sigma: f64,
}

impl AnomalyDetector {
    pub fn new(threshold_sigma: f64) -> Self {
        Self { threshold_sigma }
    }

    /// Detectar anomalias usando z-score
    pub fn detect(&self, values: &[f64]) -> Vec<usize> {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        values.iter()
            .enumerate()
            .filter(|(_, v)| ((v - mean) / std_dev).abs() > self.threshold_sigma)
            .map(|(i, _)| i)
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Trend {
    Increasing,
    Decreasing,
    Stable,
}
