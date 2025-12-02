// Configuration management for Avila Copilot

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Copilot configuration with all tunable parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Model configuration
    pub model: ModelConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
    /// Features configuration
    pub features: FeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Path to model files
    pub path: PathBuf,
    /// Model architecture
    pub architecture: String,
    /// Model precision (fp16, fp32, int8)
    pub precision: ModelPrecision,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModelPrecision {
    FP16,
    FP32,
    INT8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum latency in milliseconds
    pub max_latency_ms: u64,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Number of worker threads
    pub num_threads: usize,
    /// Enable SIMD optimizations
    pub enable_simd: bool,
    /// Batch size for inference
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    /// Enable bug detection
    pub bug_detection: bool,
    /// Enable auto documentation
    pub auto_documentation: bool,
    /// Enable test generation
    pub test_generation: bool,
    /// Enable refactoring suggestions
    pub refactoring: bool,
    /// Enable semantic search
    pub semantic_search: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: ModelConfig {
                path: PathBuf::from("./models"),
                architecture: "transformer".to_string(),
                precision: ModelPrecision::FP16,
            },
            performance: PerformanceConfig {
                max_latency_ms: 50,
                cache_size_mb: 1024,
                num_threads: num_cpus::get(),
                enable_simd: true,
                batch_size: 32,
            },
            features: FeaturesConfig {
                bug_detection: true,
                auto_documentation: true,
                test_generation: true,
                refactoring: true,
                semantic_search: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
