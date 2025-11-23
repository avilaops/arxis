/// Production configuration with environment variable support
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    /// Server configuration
    pub server: ServerConfig,

    /// AvilaDB configuration
    pub aviladb: AvilaDBConfig,

    /// AVL Auth configuration
    pub auth: AuthConfig,

    /// AVL Telemetry configuration
    pub telemetry: TelemetryConfig,

    /// AI configuration
    pub ai: AIConfig,

    /// Feature flags
    pub features: FeatureFlags,

    /// Security settings
    pub security: SecurityConfig,

    /// Rate limiting
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub metrics_port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvilaDBConfig {
    pub endpoint: String,
    pub database: String,
    pub collection: String,
    pub api_key: Option<String>,
    pub region: String,
    pub throughput_units: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub endpoint: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub enabled: bool,
    pub jwt_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub endpoint: String,
    pub enabled: bool,
    pub service_name: String,
    pub service_version: String,
    pub environment: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub backend: String,
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub ai_assistant: bool,
    pub vector_search: bool,
    pub query_safety: bool,
    pub rate_limiting: bool,
    pub metrics: bool,
    pub tracing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub session_secret: String,
    pub cors_origins: Vec<String>,
    pub allowed_hosts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst: u32,
    pub token_bucket_size: u64,
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            aviladb: AvilaDBConfig::default(),
            auth: AuthConfig::default(),
            telemetry: TelemetryConfig::default(),
            ai: AIConfig::default(),
            features: FeatureFlags::default(),
            security: SecurityConfig::default(),
            rate_limit: RateLimitConfig::default(),
        }
    }
}

impl ProductionConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig {
                host: env::var("AVL_CONSOLE_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("AVL_CONSOLE_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(3000),
                metrics_port: env::var("AVL_METRICS_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(9090),
                debug: env::var("AVL_CONSOLE_DEBUG")
                    .ok()
                    .and_then(|d| d.parse().ok())
                    .unwrap_or(false),
            },
            aviladb: AvilaDBConfig {
                endpoint: env::var("AVILADB_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:8000".to_string()),
                database: env::var("AVILADB_DATABASE")
                    .unwrap_or_else(|_| "console_db".to_string()),
                collection: env::var("AVILADB_COLLECTION")
                    .unwrap_or_else(|_| "vectors".to_string()),
                api_key: env::var("AVILADB_API_KEY").ok(),
                region: env::var("AVILADB_REGION")
                    .unwrap_or_else(|_| "saopaulo-1".to_string()),
                throughput_units: env::var("AVILADB_THROUGHPUT_UNITS")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(400),
            },
            auth: AuthConfig {
                endpoint: env::var("AVL_AUTH_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:8080".to_string()),
                client_id: env::var("AVL_AUTH_CLIENT_ID")
                    .unwrap_or_else(|_| "avl-console-client".to_string()),
                client_secret: env::var("AVL_AUTH_CLIENT_SECRET").ok(),
                enabled: env::var("AVL_AUTH_ENABLED")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
                jwt_secret: env::var("AVL_AUTH_JWT_SECRET").ok(),
            },
            telemetry: TelemetryConfig {
                endpoint: env::var("AVL_TELEMETRY_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:4317".to_string()),
                enabled: env::var("AVL_TELEMETRY_ENABLED")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
                service_name: env::var("AVL_SERVICE_NAME")
                    .unwrap_or_else(|_| "avl-console".to_string()),
                service_version: env::var("AVL_SERVICE_VERSION")
                    .unwrap_or_else(|_| "0.3.0".to_string()),
                environment: env::var("AVL_SERVICE_ENVIRONMENT")
                    .unwrap_or_else(|_| "production".to_string()),
                region: env::var("AVL_SERVICE_REGION")
                    .unwrap_or_else(|_| "saopaulo-1".to_string()),
            },
            ai: AIConfig {
                backend: env::var("AI_BACKEND")
                    .unwrap_or_else(|_| "pattern".to_string()),
                openai_api_key: env::var("OPENAI_API_KEY").ok(),
                anthropic_api_key: env::var("ANTHROPIC_API_KEY").ok(),
            },
            features: FeatureFlags {
                ai_assistant: env::var("ENABLE_AI_ASSISTANT")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
                vector_search: env::var("ENABLE_VECTOR_SEARCH")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
                query_safety: env::var("ENABLE_QUERY_SAFETY")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
                rate_limiting: env::var("ENABLE_RATE_LIMITING")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
                metrics: env::var("ENABLE_METRICS")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
                tracing: env::var("ENABLE_TRACING")
                    .ok()
                    .and_then(|e| e.parse().ok())
                    .unwrap_or(true),
            },
            security: SecurityConfig {
                session_secret: env::var("SESSION_SECRET")
                    .unwrap_or_else(|_| "change-me-in-production".to_string()),
                cors_origins: env::var("CORS_ORIGINS")
                    .unwrap_or_else(|_| "*".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                allowed_hosts: env::var("ALLOWED_HOSTS")
                    .unwrap_or_else(|_| "localhost".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            rate_limit: RateLimitConfig {
                requests_per_minute: env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
                    .ok()
                    .and_then(|r| r.parse().ok())
                    .unwrap_or(60),
                burst: env::var("RATE_LIMIT_BURST")
                    .ok()
                    .and_then(|b| b.parse().ok())
                    .unwrap_or(10),
                token_bucket_size: env::var("RATE_LIMIT_TOKEN_BUCKET_SIZE")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(100000),
            },
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.security.session_secret == "change-me-in-production" {
            return Err("SESSION_SECRET must be changed in production".to_string());
        }

        if self.auth.enabled && self.auth.jwt_secret.is_none() {
            return Err("AVL_AUTH_JWT_SECRET is required when auth is enabled".to_string());
        }

        if self.aviladb.api_key.is_none() {
            eprintln!("Warning: AVILADB_API_KEY not set - using unauthenticated access");
        }

        Ok(())
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            metrics_port: 9090,
            debug: false,
        }
    }
}

impl Default for AvilaDBConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8000".to_string(),
            database: "console_db".to_string(),
            collection: "vectors".to_string(),
            api_key: None,
            region: "saopaulo-1".to_string(),
            throughput_units: 400,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8080".to_string(),
            client_id: "avl-console-client".to_string(),
            client_secret: None,
            enabled: false,
            jwt_secret: None,
        }
    }
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:4317".to_string(),
            enabled: true,
            service_name: "avl-console".to_string(),
            service_version: "0.3.0".to_string(),
            environment: "development".to_string(),
            region: "saopaulo-1".to_string(),
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            backend: "pattern".to_string(),
            openai_api_key: None,
            anthropic_api_key: None,
        }
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            ai_assistant: true,
            vector_search: true,
            query_safety: true,
            rate_limiting: true,
            metrics: true,
            tracing: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            session_secret: "change-me-in-production".to_string(),
            cors_origins: vec!["*".to_string()],
            allowed_hosts: vec!["localhost".to_string()],
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst: 10,
            token_bucket_size: 100000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ProductionConfig::default();
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.aviladb.database, "console_db");
        assert!(config.features.ai_assistant);
    }

    #[test]
    fn test_validation_fails_default_secret() {
        let config = ProductionConfig::default();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_fails_missing_jwt() {
        let mut config = ProductionConfig::default();
        config.security.session_secret = "valid-secret-32-characters-long".to_string();
        config.auth.enabled = true;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_succeeds() {
        let mut config = ProductionConfig::default();
        config.security.session_secret = "valid-secret-32-characters-long".to_string();
        config.auth.enabled = false;
        assert!(config.validate().is_ok());
    }
}
