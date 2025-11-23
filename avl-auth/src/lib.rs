//! # AVL Auth - World-Class Identity and Access Management
//!
//! The most advanced authentication and authorization system, built for
//! AVL Cloud Platform with native AvilaDB integration.
//!
//! ## Features
//!
//! - **JWT Authentication**: Multi-algorithm support with automatic key rotation
//! - **OAuth2/OIDC**: Complete flows for Google, GitHub, Microsoft, Apple
//! - **MFA**: TOTP, WebAuthn/FIDO2, biometric authentication
//! - **RBAC + ABAC**: Dynamic role and attribute-based access control
//! - **API Keys**: Scoped keys with rate limiting and auto-rotation
//! - **Zero Trust**: Continuous authentication and risk-based access
//! - **Anomaly Detection**: ML-powered threat detection
//! - **Audit Trail**: Complete LGPD/GDPR compliant logging
//! - **Session Management**: Distributed sessions with AvilaDB
//! - **Password Security**: Argon2id with configurable cost parameters
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avl_auth::{AuthClient, Credentials, Config};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::default();
//!     let auth = AuthClient::new(config).await?;
//!
//!     // Register with strong password policy
//!     let user_id = auth.register("user@example.com", "SecureP@ss123").await?;
//!
//!     // Login with device fingerprinting
//!     let session = auth.login(Credentials {
//!         email: "user@example.com".to_string(),
//!         password: "SecureP@ss123".to_string(),
//!         device_id: Some("device_123".to_string()),
//!         ip_address: Some("192.168.1.1".parse()?),
//!     }).await?;
//!
//!     // Verify token with automatic refresh
//!     let claims = auth.verify_token(&session.access_token).await?;
//!     println!("User: {}", claims.sub);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod crypto;
pub mod error;
pub mod jwt;
pub mod mfa;
pub mod oauth2;
pub mod password;
pub mod permissions;
pub mod session;
pub mod api_keys;
pub mod audit;
pub mod device_trust;
pub mod risk;
pub mod models;

// Re-exports
pub use client::AuthClient;
pub use config::Config;
pub use error::{AuthError, Result};
pub use models::*;

/// AVL Auth version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude with commonly used types
pub mod prelude {
    pub use crate::{
        AuthClient, Config, AuthError, Result,
        Credentials, User, Session, Claims,
        Role, Permission, Policy,
    };
}
