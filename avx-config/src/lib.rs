use config as cfg;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AvxConfigError {
    #[error("config load error: {0}")]
    Load(#[from] cfg::ConfigError),
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub bind_addr: String, // ex: "0.0.0.0:8080"
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvxConfig {
    pub stack: String,   // sempre "Avx"
    pub layer: String,   // "deep"
    pub env: String,     // "dev" | "stg" | "prod"
    pub cluster: String, // "AVL-BR", "AKS-US", etc.
    pub mesh: String,    // "internal"
    pub http: HttpConfig,
}

impl AvxConfig {
    pub fn load() -> Result<Self, AvxConfigError> {
        let builder = cfg::Config::builder()
            .add_source(cfg::File::with_name("avx").required(false))
            .add_source(cfg::Environment::with_prefix("AVX").separator("__"));

        let config = builder.build()?;
        Ok(config.try_deserialize()?)
    }

    pub fn with_defaults() -> Self {
        Self {
            stack: "Avx".into(),
            layer: "deep".into(),
            env: "dev".into(),
            cluster: "AVL-BR".into(),
            mesh: "internal".into(),
            http: HttpConfig {
                bind_addr: "0.0.0.0:8080".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = AvxConfig::with_defaults();
        assert_eq!(cfg.stack, "Avx");
        assert_eq!(cfg.layer, "deep");
        assert_eq!(cfg.cluster, "AVL-BR");
        assert_eq!(cfg.mesh, "internal");
    }
}
