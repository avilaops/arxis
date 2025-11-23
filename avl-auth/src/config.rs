//! Configuration for AVL Auth

use crate::error::{AuthError, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// AvilaDB connection string
    pub database_url: String,

    /// Database name
    pub database_name: String,

    /// JWT configuration
    pub jwt: JwtConfig,

    /// Password policy
    pub password: PasswordConfig,

    /// Session configuration
    pub session: SessionConfig,

    /// MFA configuration
    pub mfa: MfaConfig,

    /// OAuth2 providers
    pub oauth2_providers: Vec<crate::models::OAuth2Provider>,

    /// Rate limiting
    pub rate_limit: RateLimitConfig,

    /// Security settings
    pub security: SecurityConfig,

    /// Risk engine settings
    pub risk: RiskConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT signing algorithm (RS256, RS384, RS512, ES256, ES384, HS256, HS512)
    pub algorithm: String,

    /// Private key for signing (PEM format)
    pub private_key: String,

    /// Public key for verification (PEM format)
    pub public_key: String,

    /// Token issuer
    pub issuer: String,

    /// Token audience
    pub audience: String,

    /// Access token expiration (seconds)
    #[serde(with = "humantime_serde")]
    pub access_token_ttl: Duration,

    /// Refresh token expiration (seconds)
    #[serde(with = "humantime_serde")]
    pub refresh_token_ttl: Duration,

    /// Enable automatic key rotation
    pub auto_rotate_keys: bool,

    /// Key rotation interval (days)
    #[serde(with = "humantime_serde")]
    pub rotation_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    /// Minimum password length
    pub min_length: usize,

    /// Require uppercase letters
    pub require_uppercase: bool,

    /// Require lowercase letters
    pub require_lowercase: bool,

    /// Require numbers
    pub require_numbers: bool,

    /// Require special characters
    pub require_special: bool,

    /// Argon2 memory cost (KiB)
    pub argon2_memory_cost: u32,

    /// Argon2 time cost (iterations)
    pub argon2_time_cost: u32,

    /// Argon2 parallelism
    pub argon2_parallelism: u32,

    /// Password history count
    pub password_history: u32,

    /// Password expiration (days, 0 = never)
    #[serde(with = "humantime_serde")]
    pub password_expiration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session idle timeout
    #[serde(with = "humantime_serde")]
    pub idle_timeout: Duration,

    /// Absolute session timeout
    #[serde(with = "humantime_serde")]
    pub absolute_timeout: Duration,

    /// Maximum concurrent sessions per user
    pub max_concurrent_sessions: u32,

    /// Enable device binding
    pub device_binding: bool,

    /// Enable IP binding
    pub ip_binding: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// TOTP issuer name
    pub totp_issuer: String,

    /// TOTP period (seconds)
    pub totp_period: u32,

    /// TOTP digits
    pub totp_digits: u32,

    /// WebAuthn RP ID
    pub webauthn_rp_id: String,

    /// WebAuthn RP name
    pub webauthn_rp_name: String,

    /// WebAuthn origin
    pub webauthn_origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Login attempts per minute
    pub login_attempts_per_minute: u32,

    /// Registration attempts per hour
    pub registration_attempts_per_hour: u32,

    /// Password reset attempts per hour
    pub password_reset_attempts_per_hour: u32,

    /// Account lockout threshold
    pub lockout_threshold: u32,

    /// Account lockout duration
    #[serde(with = "humantime_serde")]
    pub lockout_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable CORS
    pub cors_enabled: bool,

    /// Allowed CORS origins
    pub cors_origins: Vec<String>,

    /// Enable HTTPS only
    pub https_only: bool,

    /// Enable HSTS
    pub hsts_enabled: bool,

    /// Trusted proxy IPs
    pub trusted_proxies: Vec<String>,

    /// Enable audit logging
    pub audit_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    /// Enable risk-based authentication
    pub enabled: bool,

    /// Threshold for requiring MFA
    pub mfa_threshold: u8,

    /// Threshold for blocking
    pub block_threshold: u8,

    /// Enable anomaly detection
    pub anomaly_detection: bool,

    /// Enable geo-velocity check
    pub geo_velocity_check: bool,

    /// Maximum travel speed (km/h)
    pub max_travel_speed: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "http://localhost:8000".to_string(),
            database_name: "auth".to_string(),
            jwt: JwtConfig::default(),
            password: PasswordConfig::default(),
            session: SessionConfig::default(),
            mfa: MfaConfig::default(),
            oauth2_providers: vec![],
            rate_limit: RateLimitConfig::default(),
            security: SecurityConfig::default(),
            risk: RiskConfig::default(),
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            algorithm: "RS256".to_string(),
            private_key: String::new(),
            public_key: String::new(),
            issuer: "avl-auth".to_string(),
            audience: "avl-cloud".to_string(),
            access_token_ttl: Duration::from_secs(900), // 15 minutes
            refresh_token_ttl: Duration::from_secs(604800), // 7 days
            auto_rotate_keys: true,
            rotation_interval: Duration::from_secs(7776000), // 90 days
        }
    }
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
            argon2_memory_cost: 65536, // 64 MiB
            argon2_time_cost: 3,
            argon2_parallelism: 4,
            password_history: 5,
            password_expiration: Duration::from_secs(7776000), // 90 days
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(1800), // 30 minutes
            absolute_timeout: Duration::from_secs(43200), // 12 hours
            max_concurrent_sessions: 5,
            device_binding: true,
            ip_binding: false,
        }
    }
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            totp_issuer: "AVL Auth".to_string(),
            totp_period: 30,
            totp_digits: 6,
            webauthn_rp_id: "avila.cloud".to_string(),
            webauthn_rp_name: "AVL Cloud".to_string(),
            webauthn_origin: "https://auth.avila.cloud".to_string(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            login_attempts_per_minute: 5,
            registration_attempts_per_hour: 3,
            password_reset_attempts_per_hour: 3,
            lockout_threshold: 5,
            lockout_duration: Duration::from_secs(900), // 15 minutes
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            cors_enabled: true,
            cors_origins: vec!["https://avila.cloud".to_string()],
            https_only: true,
            hsts_enabled: true,
            trusted_proxies: vec![],
            audit_enabled: true,
        }
    }
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mfa_threshold: 60,
            block_threshold: 90,
            anomaly_detection: true,
            geo_velocity_check: true,
            max_travel_speed: 1000.0, // Roughly speed of a commercial aircraft
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("AVILADB_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            database_name: std::env::var("AVILADB_NAME")
                .unwrap_or_else(|_| "auth".to_string()),
            ..Default::default()
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.jwt.private_key.is_empty() {
            return Err(AuthError::ConfigError("JWT private key is required".to_string()));
        }

        if self.jwt.public_key.is_empty() {
            return Err(AuthError::ConfigError("JWT public key is required".to_string()));
        }

        if self.password.min_length < 8 {
            return Err(AuthError::ConfigError("Minimum password length must be at least 8".to_string()));
        }

        Ok(())
    }
}
