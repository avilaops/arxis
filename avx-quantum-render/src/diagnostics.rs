//! Sistema de diagnóstico e métricas de performance

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Métricas de renderização
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderMetrics {
    /// Tempo total de renderização
    pub total_duration: Duration,

    /// Resolução da imagem
    pub resolution: (usize, usize),

    /// Configuração de samples
    pub samples_per_pixel: usize,

    /// Número de caminhos por sample
    pub num_paths: usize,

    /// Profundidade máxima
    pub max_depth: usize,

    /// Total de caminhos avaliados
    pub total_paths_evaluated: usize,

    /// Caminhos válidos
    pub valid_paths: usize,

    /// Taxa de aceitação
    pub acceptance_rate: f64,

    /// Intensidade média da imagem
    pub mean_intensity: f64,

    /// Intensidade mínima
    pub min_intensity: f64,

    /// Intensidade máxima
    pub max_intensity: f64,

    /// Throughput (pixels/segundo)
    pub throughput_pixels_per_sec: f64,

    /// Paralelização utilizada
    pub parallel_execution: bool,

    /// Timestamp da renderização
    pub timestamp: u64,

    /// Modo espectral
    pub spectral_mode: SpectralMode,
}

/// Modo de amostragem espectral
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpectralMode {
    /// Monocromático (um comprimento de onda)
    Monochromatic {
        /// Comprimento de onda em nanômetros
        wavelength: f64,
    },
    /// Múltiplas bandas espectrais
    Multispectral {
        /// Bandas espectrais
        bands: Vec<f64>,
    },
}

impl RenderMetrics {
    /// Cria novo conjunto de métricas
    pub fn new(resolution: (usize, usize), samples_per_pixel: usize, num_paths: usize) -> Self {
        Self {
            total_duration: Duration::from_secs(0),
            resolution,
            samples_per_pixel,
            num_paths,
            max_depth: 0,
            total_paths_evaluated: 0,
            valid_paths: 0,
            acceptance_rate: 0.0,
            mean_intensity: 0.0,
            min_intensity: 0.0,
            max_intensity: 0.0,
            throughput_pixels_per_sec: 0.0,
            parallel_execution: false,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            spectral_mode: SpectralMode::Monochromatic { wavelength: 550e-9 },
        }
    }

    /// Calcula estatísticas da imagem renderizada
    pub fn compute_image_stats(&mut self, image: &[Vec<f64>]) {
        let mut sum = 0.0;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let mut count = 0;

        for row in image {
            for &intensity in row {
                sum += intensity;
                min = min.min(intensity);
                max = max.max(intensity);
                count += 1;
            }
        }

        self.mean_intensity = if count > 0 { sum / count as f64 } else { 0.0 };
        self.min_intensity = min;
        self.max_intensity = max;
    }

    /// Calcula taxa de aceitação
    pub fn compute_acceptance_rate(&mut self) {
        if self.total_paths_evaluated > 0 {
            self.acceptance_rate = self.valid_paths as f64 / self.total_paths_evaluated as f64;
        }
    }

    /// Calcula throughput
    pub fn compute_throughput(&mut self) {
        let total_pixels = (self.resolution.0 * self.resolution.1) as f64;
        let seconds = self.total_duration.as_secs_f64();
        self.throughput_pixels_per_sec = if seconds > 0.0 {
            total_pixels / seconds
        } else {
            0.0
        };
    }

    /// Exporta para formato JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Exporta para documento AvilaDB
    pub fn to_aviladb_document(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "render_metrics",
            "timestamp": self.timestamp,
            "resolution": {
                "width": self.resolution.0,
                "height": self.resolution.1,
            },
            "config": {
                "samples_per_pixel": self.samples_per_pixel,
                "num_paths": self.num_paths,
                "max_depth": self.max_depth,
                "parallel": self.parallel_execution,
            },
            "performance": {
                "duration_ms": self.total_duration.as_millis(),
                "throughput_pixels_per_sec": self.throughput_pixels_per_sec,
                "total_paths": self.total_paths_evaluated,
                "valid_paths": self.valid_paths,
                "acceptance_rate": self.acceptance_rate,
            },
            "image": {
                "mean_intensity": self.mean_intensity,
                "min_intensity": self.min_intensity,
                "max_intensity": self.max_intensity,
            },
            "spectral": match &self.spectral_mode {
                SpectralMode::Monochromatic { wavelength } => serde_json::json!({
                    "mode": "monochromatic",
                    "wavelength_nm": wavelength * 1e9,
                }),
                SpectralMode::Multispectral { bands } => serde_json::json!({
                    "mode": "multispectral",
                    "bands_nm": bands.iter().map(|w| w * 1e9).collect::<Vec<_>>(),
                    "num_bands": bands.len(),
                }),
            },
        })
    }
}

/// Timer para medir performance
pub struct RenderTimer {
    start: Instant,
}

impl RenderTimer {
    /// Inicia timer
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Finaliza timer e retorna duração
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = RenderMetrics::new((800, 600), 100, 1000);
        assert_eq!(metrics.resolution, (800, 600));
        assert_eq!(metrics.samples_per_pixel, 100);
        assert_eq!(metrics.num_paths, 1000);
    }

    #[test]
    fn test_image_stats() {
        let mut metrics = RenderMetrics::new((2, 2), 10, 100);
        let image = vec![vec![0.1, 0.2], vec![0.3, 0.4]];

        metrics.compute_image_stats(&image);

        assert!((metrics.mean_intensity - 0.25).abs() < 1e-6);
        assert_eq!(metrics.min_intensity, 0.1);
        assert_eq!(metrics.max_intensity, 0.4);
    }

    #[test]
    fn test_acceptance_rate() {
        let mut metrics = RenderMetrics::new((100, 100), 10, 100);
        metrics.total_paths_evaluated = 1000;
        metrics.valid_paths = 800;

        metrics.compute_acceptance_rate();

        assert_eq!(metrics.acceptance_rate, 0.8);
    }

    #[test]
    fn test_aviladb_export() {
        let metrics = RenderMetrics::new((800, 600), 100, 1000);
        let doc = metrics.to_aviladb_document();

        assert_eq!(doc["type"], "render_metrics");
        assert_eq!(doc["resolution"]["width"], 800);
        assert_eq!(doc["resolution"]["height"], 600);
    }
}
