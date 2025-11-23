//! Gateway configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Complete gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Server configuration
    pub server: ServerConfig,

    /// Routes configuration
    pub routes: Vec<RouteConfig>,

    /// Middleware configuration
    #[serde(default)]
    pub middleware: MiddlewareConfig,

    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limiting: Option<RateLimitConfig>,

    /// Health check configuration
    #[serde(default)]
    pub health_check: HealthCheckConfig,

    /// TLS configuration
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    #[serde(default = "default_host")]
    pub host: String,

    /// Port to bind to
    #[serde(default = "default_port")]
    pub port: u16,

    /// Number of worker threads
    #[serde(default = "default_workers")]
    pub workers: usize,

    /// Request timeout in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

/// Route configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    /// Path pattern (e.g., "/api/users/*")
    pub path: String,

    /// Upstream service URL or load balancer
    pub upstream: UpstreamConfig,

    /// HTTP methods allowed for this route
    #[serde(default = "default_methods")]
    pub methods: Vec<String>,

    /// Strip path prefix before forwarding
    #[serde(default)]
    pub strip_path: bool,

    /// Timeout for this route in milliseconds
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Authentication required
    #[serde(default)]
    pub auth_required: bool,
}

/// Upstream configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpstreamConfig {
    /// Single upstream URL
    Single(String),

    /// Multiple upstreams for load balancing
    Multiple {
        /// List of upstream URLs
        urls: Vec<String>,

        /// Load balancing strategy
        #[serde(default)]
        strategy: LoadBalancingStrategy,
    },
}

/// Load balancing strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    #[default]
    RoundRobin,

    /// Least connections
    LeastConnections,

    /// Weighted round-robin
    Weighted,

    /// Random selection
    Random,
}

/// Middleware configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareConfig {
    /// Enable CORS
    #[serde(default)]
    pub enable_cors: bool,

    /// Enable compression
    #[serde(default)]
    pub enable_compression: bool,

    /// Enable rate limiting
    #[serde(default)]
    pub enable_rate_limiting: bool,

    /// Enable request logging
    #[serde(default = "default_true")]
    pub enable_logging: bool,

    /// Enable metrics collection
    #[serde(default = "default_true")]
    pub enable_metrics: bool,
}

impl Default for MiddlewareConfig {
    fn default() -> Self {
        Self {
            enable_cors: false,
            enable_compression: false,
            enable_rate_limiting: false,
            enable_logging: true,
            enable_metrics: true,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per second allowed
    pub requests_per_second: u32,

    /// Burst size (max requests in a short time)
    #[serde(default = "default_burst")]
    pub burst_size: u32,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Enable health checks
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Health check interval in seconds
    #[serde(default = "default_health_interval")]
    pub interval_seconds: u64,

    /// Health check timeout in milliseconds
    #[serde(default = "default_health_timeout")]
    pub timeout_ms: u64,

    /// Health check path
    #[serde(default = "default_health_path")]
    pub path: String,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: default_health_interval(),
            timeout_ms: default_health_timeout(),
            path: default_health_path(),
        }
    }
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Path to certificate file
    pub cert_path: String,

    /// Path to private key file
    pub key_path: String,
}

// Default value functions
fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_workers() -> usize {
    num_cpus::get()
}

fn default_timeout() -> u64 {
    30_000 // 30 seconds
}

fn default_methods() -> Vec<String> {
    vec!["GET", "POST", "PUT", "DELETE", "PATCH"]
        .into_iter()
        .map(String::from)
        .collect()
}

fn default_burst() -> u32 {
    20
}

fn default_true() -> bool {
    true
}

fn default_health_interval() -> u64 {
    30
}

fn default_health_timeout() -> u64 {
    3000
}

fn default_health_path() -> String {
    "/health".to_string()
}

impl GatewayConfig {
    /// Create a default configuration
    pub fn default() -> Self {
        Self {
            server: ServerConfig {
                host: default_host(),
                port: default_port(),
                workers: default_workers(),
                timeout_ms: default_timeout(),
            },
            routes: Vec::new(),
            middleware: MiddlewareConfig::default(),
            rate_limiting: None,
            health_check: HealthCheckConfig::default(),
            tls: None,
        }
    }

    /// Get request timeout duration
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_millis(self.server.timeout_ms)
    }
}
