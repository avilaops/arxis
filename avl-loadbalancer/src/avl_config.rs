//! AVL Config Module
//!
//! Configuration file parsing and serialization for AVL Cloud Platform.
//! Supports YAML and TOML formats with validation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Configuration file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    /// YAML format (.yaml, .yml)
    Yaml,
    /// TOML format (.toml)
    Toml,
}

impl ConfigFormat {
    /// Detect format from file extension
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext.to_lowercase().as_str() {
                "yaml" | "yml" => Some(Self::Yaml),
                "toml" => Some(Self::Toml),
                _ => None,
            })
    }
}

/// AVL configuration loader
pub struct AvlConfig;

impl AvlConfig {
    /// Load and deserialize configuration from file
    pub fn load<T, P>(path: P) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        
        let format = ConfigFormat::from_path(path)
            .ok_or_else(|| anyhow::anyhow!("Unknown config format for: {:?}", path))?;
        
        Self::parse(&content, format)
    }

    /// Parse configuration from string
    pub fn parse<T>(content: &str, format: ConfigFormat) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        match format {
            ConfigFormat::Yaml => Self::from_yaml(content),
            ConfigFormat::Toml => Self::from_toml(content),
        }
    }

    /// Parse YAML configuration
    pub fn from_yaml<T>(content: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(serde_yaml::from_str(content)?)
    }

    /// Parse TOML configuration
    pub fn from_toml<T>(content: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(toml::from_str(content)?)
    }

    /// Save configuration to file
    pub fn save<T, P>(value: &T, path: P) -> Result<()>
    where
        T: Serialize,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let format = ConfigFormat::from_path(path)
            .ok_or_else(|| anyhow::anyhow!("Unknown config format for: {:?}", path))?;
        
        let content = Self::serialize(value, format)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Serialize to string
    pub fn serialize<T>(value: &T, format: ConfigFormat) -> Result<String>
    where
        T: Serialize,
    {
        match format {
            ConfigFormat::Yaml => Self::to_yaml(value),
            ConfigFormat::Toml => Self::to_toml(value),
        }
    }

    /// Serialize to YAML
    pub fn to_yaml<T>(value: &T) -> Result<String>
    where
        T: Serialize,
    {
        Ok(serde_yaml::to_string(value)?)
    }

    /// Serialize to TOML
    pub fn to_toml<T>(value: &T) -> Result<String>
    where
        T: Serialize,
    {
        Ok(toml::to_string(value)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        port: u16,
        enabled: bool,
    }

    #[test]
    fn test_yaml_parsing() {
        let yaml = r#"
name: test-service
port: 8080
enabled: true
"#;
        let config: TestConfig = AvlConfig::from_yaml(yaml).unwrap();
        assert_eq!(config.name, "test-service");
        assert_eq!(config.port, 8080);
        assert!(config.enabled);
    }

    #[test]
    fn test_toml_parsing() {
        let toml = r#"
name = "test-service"
port = 8080
enabled = true
"#;
        let config: TestConfig = AvlConfig::from_toml(toml).unwrap();
        assert_eq!(config.name, "test-service");
        assert_eq!(config.port, 8080);
        assert!(config.enabled);
    }

    #[test]
    fn test_format_detection() {
        assert_eq!(ConfigFormat::from_path("config.yaml"), Some(ConfigFormat::Yaml));
        assert_eq!(ConfigFormat::from_path("config.yml"), Some(ConfigFormat::Yaml));
        assert_eq!(ConfigFormat::from_path("config.toml"), Some(ConfigFormat::Toml));
        assert_eq!(ConfigFormat::from_path("config.json"), None);
    }

    #[test]
    fn test_serialization() {
        let config = TestConfig {
            name: "test".to_string(),
            port: 3000,
            enabled: false,
        };

        let yaml = AvlConfig::to_yaml(&config).unwrap();
        assert!(yaml.contains("test"));
        assert!(yaml.contains("3000"));

        let toml = AvlConfig::to_toml(&config).unwrap();
        assert!(toml.contains("test"));
        assert!(toml.contains("3000"));
    }
}
