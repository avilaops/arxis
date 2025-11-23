//! API Key management with scopes and rate limiting

use crate::error::{AuthError, Result};
use crate::models::ApiKey;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct ApiKeyManager {
    keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    rate_limiter: Arc<RateLimiter>,
    argon2: Argon2<'static>,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            rate_limiter: Arc::new(RateLimiter::new()),
            argon2: Argon2::default(),
        }
    }

    pub async fn generate_api_key(
        &self,
        user_id: Uuid,
        name: String,
        description: Option<String>,
        scopes: Vec<String>,
        rate_limit: Option<u32>,
        expires_in: Option<Duration>,
    ) -> Result<(String, ApiKey)> {
        // Generate a cryptographically secure API key
        let key = self.generate_key_string();
        let prefix = &key[..8]; // First 8 chars for identification

        // Hash the key for storage
        let salt = SaltString::generate(&mut rand::thread_rng());
        let key_hash = self.argon2
            .hash_password(key.as_bytes(), &salt)?
            .to_string();

        let now = Utc::now();
        let api_key = ApiKey {
            id: Uuid::new_v4(),
            key_hash,
            prefix: prefix.to_string(),
            user_id,
            name,
            description,
            scopes,
            rate_limit,
            expires_at: expires_in.map(|d| now + d),
            last_used_at: None,
            created_at: now,
            revoked: false,
        };

        let mut keys = self.keys.write().await;
        keys.insert(api_key.id.to_string(), api_key.clone());

        Ok((key, api_key))
    }

    fn generate_key_string(&self) -> String {
        let random_part: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect();

        format!("avl_{}", random_part)
    }

    pub async fn verify_api_key(&self, key: &str) -> Result<ApiKey> {
        if !key.starts_with("avl_") {
            return Err(AuthError::InvalidApiKey);
        }

        let prefix = &key[..8.min(key.len())];

        let keys = self.keys.read().await;

        // Find keys with matching prefix
        for api_key in keys.values() {
            if !api_key.prefix.starts_with(prefix) {
                continue;
            }

            if api_key.revoked {
                return Err(AuthError::InvalidApiKey);
            }

            if let Some(expires_at) = api_key.expires_at {
                if Utc::now() > expires_at {
                    return Err(AuthError::ApiKeyExpired);
                }
            }

            // Verify hash
            let parsed_hash = PasswordHash::new(&api_key.key_hash)
                .map_err(|e| AuthError::CryptoError(e.to_string()))?;

            if self.argon2.verify_password(key.as_bytes(), &parsed_hash).is_ok() {
                // Check rate limit
                if let Some(limit) = api_key.rate_limit {
                    if !self.rate_limiter.check_limit(&api_key.id.to_string(), limit).await {
                        return Err(AuthError::RateLimitExceeded);
                    }
                }

                return Ok(api_key.clone());
            }
        }

        Err(AuthError::InvalidApiKey)
    }

    pub async fn revoke_api_key(&self, key_id: &Uuid) -> Result<()> {
        let mut keys = self.keys.write().await;

        if let Some(api_key) = keys.get_mut(&key_id.to_string()) {
            api_key.revoked = true;
            Ok(())
        } else {
            Err(AuthError::InvalidApiKey)
        }
    }

    pub async fn update_last_used(&self, key_id: &Uuid) -> Result<()> {
        let mut keys = self.keys.write().await;

        if let Some(api_key) = keys.get_mut(&key_id.to_string()) {
            api_key.last_used_at = Some(Utc::now());
            Ok(())
        } else {
            Err(AuthError::InvalidApiKey)
        }
    }

    pub async fn list_user_keys(&self, user_id: &Uuid) -> Vec<ApiKey> {
        let keys = self.keys.read().await;
        keys.values()
            .filter(|k| k.user_id == *user_id && !k.revoked)
            .cloned()
            .collect()
    }

    pub async fn rotate_api_key(&self, old_key_id: &Uuid) -> Result<(String, ApiKey)> {
        // Clone necessary data before releasing lock
        let (user_id, name, description, scopes, rate_limit, expires_duration) = {
            let keys = self.keys.read().await;

            let old_key = keys
                .get(&old_key_id.to_string())
                .ok_or(AuthError::InvalidApiKey)?;

            (
                old_key.user_id,
                old_key.name.clone(),
                old_key.description.clone(),
                old_key.scopes.clone(),
                old_key.rate_limit,
                old_key.expires_at.map(|exp| exp - Utc::now()),
            )
        };

        // Generate new key with same properties
        let (new_key, new_api_key) = self.generate_api_key(
            user_id,
            name,
            description,
            scopes,
            rate_limit,
            expires_duration,
        ).await?;

        // Revoke old key
        self.revoke_api_key(old_key_id).await?;

        Ok((new_key, new_api_key))
    }
}

// ==================== Rate Limiter ====================

struct RateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
}

struct TokenBucket {
    tokens: f64,
    capacity: f64,
    refill_rate: f64,
    last_refill: DateTime<Utc>,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn check_limit(&self, key: &str, limit: u32) -> bool {
        let mut buckets = self.buckets.write().await;

        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket {
                tokens: limit as f64,
                capacity: limit as f64,
                refill_rate: limit as f64 / 60.0, // Refill per second
                last_refill: Utc::now(),
            }
        });

        // Refill tokens based on time elapsed
        let now = Utc::now();
        let elapsed = (now - bucket.last_refill).num_seconds() as f64;
        bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity);
        bucket.last_refill = now;

        // Check if we have at least 1 token
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    async fn reset(&self, key: &str) {
        let mut buckets = self.buckets.write().await;
        buckets.remove(key);
    }

    async fn get_remaining(&self, key: &str) -> Option<u32> {
        let buckets = self.buckets.read().await;
        buckets.get(key).map(|b| b.tokens as u32)
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_and_verify_api_key() {
        let manager = ApiKeyManager::new();

        let user_id = Uuid::new_v4();
        let (key, api_key) = manager
            .generate_api_key(
                user_id,
                "Test Key".to_string(),
                None,
                vec!["read".to_string()],
                Some(100),
                None,
            )
            .await
            .unwrap();

        assert!(key.starts_with("avl_"));

        let verified = manager.verify_api_key(&key).await.unwrap();
        assert_eq!(verified.user_id, user_id);
    }

    #[tokio::test]
    async fn test_revoke_api_key() {
        let manager = ApiKeyManager::new();

        let user_id = Uuid::new_v4();
        let (key, api_key) = manager
            .generate_api_key(
                user_id,
                "Test Key".to_string(),
                None,
                vec![],
                None,
                None,
            )
            .await
            .unwrap();

        manager.revoke_api_key(&api_key.id).await.unwrap();

        let result = manager.verify_api_key(&key).await;
        assert!(result.is_err());
    }
}
