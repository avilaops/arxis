//! Main AuthClient implementation

use crate::api_keys::ApiKeyManager;
use crate::audit::AuditManager;
use crate::config::Config;
use crate::crypto::CryptoManager;
use crate::error::{AuthError, Result};
use crate::jwt::{JwtConfig, JwtManager};
use crate::mfa::MfaManager;
use crate::models::*;
use crate::oauth2::OAuth2Manager;
use crate::password::{PasswordManager, PasswordPolicy};
use crate::permissions::PermissionManager;
use crate::risk::{RiskConfig, RiskEngine};
use crate::session::{SessionConfig, SessionManager};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct AuthClient {
    config: Config,
    jwt: Arc<JwtManager>,
    password: Arc<PasswordManager>,
    oauth2: Arc<OAuth2Manager>,
    mfa: Arc<MfaManager>,
    permissions: Arc<PermissionManager>,
    sessions: Arc<SessionManager>,
    api_keys: Arc<ApiKeyManager>,
    risk: Arc<RiskEngine>,
    audit: Arc<AuditManager>,
    crypto: Arc<CryptoManager>,
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl AuthClient {
    pub async fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let jwt_config = JwtConfig {
            algorithm: parse_algorithm(&config.jwt.algorithm)
                .map_err(|e| AuthError::ConfigError(e))?,
            issuer: config.jwt.issuer.clone(),
            audience: config.jwt.audience.clone(),
            access_token_ttl: chrono::Duration::from_std(config.jwt.access_token_ttl)
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
            refresh_token_ttl: chrono::Duration::from_std(config.jwt.refresh_token_ttl)
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
        };

        let jwt = Arc::new(JwtManager::new(
            jwt_config,
            &config.jwt.private_key,
            &config.jwt.public_key,
        )?);

        let password_policy = PasswordPolicy {
            min_length: config.password.min_length,
            require_uppercase: config.password.require_uppercase,
            require_lowercase: config.password.require_lowercase,
            require_numbers: config.password.require_numbers,
            require_special: config.password.require_special,
            password_history: config.password.password_history,
        };

        let password = Arc::new(PasswordManager::new(
            password_policy,
            config.password.argon2_memory_cost,
            config.password.argon2_time_cost,
            config.password.argon2_parallelism,
        )?);

        let oauth2 = Arc::new(OAuth2Manager::new());
        for provider in &config.oauth2_providers {
            oauth2.register_provider(provider.clone()).await?;
        }

        let mfa = Arc::new(MfaManager::new(
            config.mfa.totp_issuer.clone(),
            config.mfa.totp_period,
            config.mfa.totp_digits,
        ));

        let permissions = Arc::new(PermissionManager::new());

        let session_config = SessionConfig {
            idle_timeout: chrono::Duration::from_std(config.session.idle_timeout)
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
            absolute_timeout: chrono::Duration::from_std(config.session.absolute_timeout)
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
            max_concurrent_sessions: config.session.max_concurrent_sessions,
            device_binding: config.session.device_binding,
            ip_binding: config.session.ip_binding,
        };

        let sessions = Arc::new(SessionManager::new(session_config));

        let api_keys = Arc::new(ApiKeyManager::new());

        let risk_config = RiskConfig {
            mfa_threshold: config.risk.mfa_threshold,
            block_threshold: config.risk.block_threshold,
            geo_velocity_enabled: config.risk.geo_velocity_check,
            max_travel_speed_kmh: config.risk.max_travel_speed,
        };

        let risk = Arc::new(RiskEngine::new(risk_config));

        let audit = Arc::new(AuditManager::new(90)); // 90-day retention

        let crypto = Arc::new(CryptoManager::new());

        Ok(Self {
            config,
            jwt,
            password,
            oauth2,
            mfa,
            permissions,
            sessions,
            api_keys,
            risk,
            audit,
            crypto,
            users: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // ==================== User Management ====================

    pub async fn register(&self, email: String, password: String) -> Result<Uuid> {
        // Validate email
        if !email.contains('@') {
            return Err(AuthError::InvalidPassword("Invalid email format".to_string()));
        }

        // Check if user already exists
        let users = self.users.read().await;
        if users.values().any(|u| u.email == email) {
            return Err(AuthError::UserAlreadyExists(email));
        }
        drop(users);

        // Hash password
        let password_hash = self.password.hash_password(&password)?;

        let user = User {
            id: Uuid::new_v4(),
            email: email.clone(),
            email_verified: false,
            password_hash,
            display_name: None,
            avatar_url: None,
            roles: vec!["user".to_string()],
            permissions: vec![],
            metadata: HashMap::new(),
            mfa_enabled: false,
            mfa_secret: None,
            webauthn_credentials: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
            login_count: 0,
            failed_login_attempts: 0,
            locked_until: None,
            password_changed_at: Utc::now(),
            status: UserStatus::Active,
        };

        let user_id = user.id;

        let mut users = self.users.write().await;
        users.insert(user_id, user);

        self.audit.log(
            Some(user_id),
            None,
            "user.register".to_string(),
            "user".to_string(),
            AuditResult::Success,
            None,
            None,
            HashMap::new(),
            0,
        ).await;

        tracing::info!(user_id = %user_id, email = %email, "User registered");

        Ok(user_id)
    }

    pub async fn login(&self, credentials: Credentials) -> Result<Session> {
        let users = self.users.read().await;

        let user = users
            .values()
            .find(|u| u.email == credentials.email)
            .cloned()
            .ok_or(AuthError::InvalidCredentials)?;

        drop(users);

        // Check if account is locked
        if let Some(locked_until) = user.locked_until {
            if Utc::now() < locked_until {
                return Err(AuthError::AccountLocked(
                    format!("Account locked until {}", locked_until)
                ));
            }
        }

        // Verify password
        let password_valid = self.password.verify_password(&credentials.password, &user.password_hash)?;

        if !password_valid {
            self.handle_failed_login(&user.id).await?;

            self.audit.log(
                Some(user.id),
                None,
                "user.login".to_string(),
                "session".to_string(),
                AuditResult::Failure,
                credentials.ip_address,
                None,
                HashMap::new(),
                50,
            ).await;

            return Err(AuthError::InvalidCredentials);
        }

        // Risk assessment
        let risk_assessment = self.risk.assess_risk(
            &user,
            credentials.ip_address,
            credentials.device_id.as_deref(),
            None,
        ).await?;

        match risk_assessment.recommended_action {
            RiskAction::Deny => {
                self.audit.log(
                    Some(user.id),
                    None,
                    "user.login".to_string(),
                    "session".to_string(),
                    AuditResult::Blocked,
                    credentials.ip_address,
                    None,
                    HashMap::new(),
                    risk_assessment.score,
                ).await;

                return Err(AuthError::SuspiciousActivity(
                    "Login blocked due to high risk score".to_string()
                ));
            }
            RiskAction::RequireMfa if !user.mfa_enabled => {
                return Err(AuthError::MfaRequired);
            }
            _ => {}
        }

        // Update behavior profile
        self.risk.update_behavior_profile(
            &user.id,
            credentials.ip_address,
            credentials.device_id.clone(),
            true,
        ).await;

        // Create JWT claims
        let claims = self.jwt.create_claims(
            user.id,
            user.email.clone(),
            user.roles.clone(),
            user.permissions.clone(),
            Uuid::new_v4(),
            vec!["*".to_string()],
            credentials.device_id.clone(),
        );

        let access_token = self.jwt.create_token(&claims).await?;
        let refresh_token = self.crypto.generate_token(32);

        // Create session
        let session = self.sessions.create_session(
            user.id,
            access_token,
            refresh_token,
            chrono::Duration::from_std(self.config.jwt.access_token_ttl)
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
            chrono::Duration::from_std(self.config.jwt.refresh_token_ttl)
                .map_err(|e| AuthError::ConfigError(e.to_string()))?,
            credentials.device_id,
            credentials.ip_address,
            None,
            vec!["*".to_string()],
        ).await?;

        // Update user
        let mut users = self.users.write().await;
        if let Some(u) = users.get_mut(&user.id) {
            u.last_login_at = Some(Utc::now());
            u.login_count += 1;
            u.failed_login_attempts = 0;
        }

        self.audit.log(
            Some(user.id),
            Some(session.id),
            "user.login".to_string(),
            "session".to_string(),
            AuditResult::Success,
            credentials.ip_address,
            None,
            HashMap::new(),
            risk_assessment.score,
        ).await;

        Ok(session)
    }

    async fn handle_failed_login(&self, user_id: &Uuid) -> Result<()> {
        let mut users = self.users.write().await;

        if let Some(user) = users.get_mut(user_id) {
            user.failed_login_attempts += 1;

            if user.failed_login_attempts >= self.config.rate_limit.lockout_threshold {
                user.locked_until = Some(Utc::now() + Duration::from_std(self.config.rate_limit.lockout_duration).unwrap());
                tracing::warn!(user_id = %user_id, "Account locked due to failed login attempts");
            }
        }

        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims> {
        self.jwt.verify_token(token).await
    }

    pub async fn logout(&self, session_id: &Uuid) -> Result<()> {
        self.sessions.delete_session(session_id).await?;

        self.audit.log(
            None,
            Some(*session_id),
            "user.logout".to_string(),
            "session".to_string(),
            AuditResult::Success,
            None,
            None,
            HashMap::new(),
            0,
        ).await;

        Ok(())
    }

    // ==================== Accessors ====================

    pub fn jwt_manager(&self) -> &JwtManager {
        &self.jwt
    }

    pub fn password_manager(&self) -> &PasswordManager {
        &self.password
    }

    pub fn oauth2_manager(&self) -> &OAuth2Manager {
        &self.oauth2
    }

    pub fn mfa_manager(&self) -> &MfaManager {
        &self.mfa
    }

    pub fn permission_manager(&self) -> &PermissionManager {
        &self.permissions
    }

    pub fn session_manager(&self) -> &SessionManager {
        &self.sessions
    }

    pub fn api_key_manager(&self) -> &ApiKeyManager {
        &self.api_keys
    }

    pub fn risk_engine(&self) -> &RiskEngine {
        &self.risk
    }

    pub fn audit_manager(&self) -> &AuditManager {
        &self.audit
    }

    pub fn crypto_manager(&self) -> &CryptoManager {
        &self.crypto
    }
}

// Helper function para converter string para Algorithm
fn parse_algorithm(s: &str) -> std::result::Result<jsonwebtoken::Algorithm, String> {
    match s {
        "HS256" => Ok(jsonwebtoken::Algorithm::HS256),
        "HS384" => Ok(jsonwebtoken::Algorithm::HS384),
        "HS512" => Ok(jsonwebtoken::Algorithm::HS512),
        "RS256" => Ok(jsonwebtoken::Algorithm::RS256),
        "RS384" => Ok(jsonwebtoken::Algorithm::RS384),
        "RS512" => Ok(jsonwebtoken::Algorithm::RS512),
        "ES256" => Ok(jsonwebtoken::Algorithm::ES256),
        "ES384" => Ok(jsonwebtoken::Algorithm::ES384),
        _ => Err(format!("Unknown algorithm: {}", s)),
    }
}
