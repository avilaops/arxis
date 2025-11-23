//! Advanced JWT implementation with key rotation and multi-algorithm support

use crate::error::{AuthError, Result};
use crate::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct JwtManager {
    config: JwtConfig,
    keys: Arc<RwLock<KeyStore>>,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub algorithm: Algorithm,
    pub issuer: String,
    pub audience: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
}

struct KeyStore {
    active_key: KeyPair,
    retired_keys: Vec<KeyPair>,
    key_id_counter: u64,
}

struct KeyPair {
    id: String,
    encoding: EncodingKey,
    decoding: DecodingKey,
    algorithm: Algorithm,
    created_at: chrono::DateTime<Utc>,
}

impl JwtManager {
    pub fn new(config: JwtConfig, private_key: &str, public_key: &str) -> Result<Self> {
        let encoding = Self::create_encoding_key(&config.algorithm, private_key)?;
        let decoding = Self::create_decoding_key(&config.algorithm, public_key)?;

        let key_pair = KeyPair {
            id: "key_1".to_string(),
            encoding,
            decoding,
            algorithm: config.algorithm,
            created_at: Utc::now(),
        };

        let keys = Arc::new(RwLock::new(KeyStore {
            active_key: key_pair,
            retired_keys: Vec::new(),
            key_id_counter: 1,
        }));

        Ok(Self { config, keys })
    }

    fn create_encoding_key(algorithm: &Algorithm, private_key: &str) -> Result<EncodingKey> {
        match algorithm {
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                EncodingKey::from_rsa_pem(private_key.as_bytes())
                    .map_err(|e| AuthError::CryptoError(e.to_string()))
            }
            Algorithm::ES256 | Algorithm::ES384 => {
                EncodingKey::from_ec_pem(private_key.as_bytes())
                    .map_err(|e| AuthError::CryptoError(e.to_string()))
            }
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                Ok(EncodingKey::from_secret(private_key.as_bytes()))
            }
            _ => Err(AuthError::ConfigError(format!("Unsupported algorithm: {:?}", algorithm))),
        }
    }

    fn create_decoding_key(algorithm: &Algorithm, public_key: &str) -> Result<DecodingKey> {
        match algorithm {
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                DecodingKey::from_rsa_pem(public_key.as_bytes())
                    .map_err(|e| AuthError::CryptoError(e.to_string()))
            }
            Algorithm::ES256 | Algorithm::ES384 => {
                DecodingKey::from_ec_pem(public_key.as_bytes())
                    .map_err(|e| AuthError::CryptoError(e.to_string()))
            }
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                Ok(DecodingKey::from_secret(public_key.as_bytes()))
            }
            _ => Err(AuthError::ConfigError(format!("Unsupported algorithm: {:?}", algorithm))),
        }
    }

    pub async fn create_token(&self, claims: &Claims) -> Result<String> {
        let keys = self.keys.read().await;

        let mut header = Header::new(self.config.algorithm);
        header.kid = Some(keys.active_key.id.clone());

        encode(&header, claims, &keys.active_key.encoding)
            .map_err(|e| AuthError::CryptoError(e.to_string()))
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims> {
        let keys = self.keys.read().await;

        let mut validation = Validation::new(self.config.algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);

        // Try active key first
        if let Ok(token_data) = decode::<Claims>(token, &keys.active_key.decoding, &validation) {
            return Ok(token_data.claims);
        }

        // Try retired keys
        for key in &keys.retired_keys {
            if let Ok(token_data) = decode::<Claims>(token, &key.decoding, &validation) {
                return Ok(token_data.claims);
            }
        }

        Err(AuthError::InvalidToken("Unable to verify token signature".to_string()))
    }

    pub async fn rotate_keys(&self, new_private_key: &str, new_public_key: &str) -> Result<()> {
        let encoding = Self::create_encoding_key(&self.config.algorithm, new_private_key)?;
        let decoding = Self::create_decoding_key(&self.config.algorithm, new_public_key)?;

        let mut keys = self.keys.write().await;
        keys.key_id_counter += 1;

        let new_key = KeyPair {
            id: format!("key_{}", keys.key_id_counter),
            encoding,
            decoding,
            algorithm: self.config.algorithm,
            created_at: Utc::now(),
        };

        // Move current active key to retired
        let old_key = std::mem::replace(&mut keys.active_key, new_key);
        keys.retired_keys.push(old_key);

        // Keep only last 3 retired keys
        if keys.retired_keys.len() > 3 {
            keys.retired_keys.remove(0);
        }

        tracing::info!("JWT keys rotated successfully");
        Ok(())
    }

    pub async fn get_jwks(&self) -> Result<JwkSet> {
        let keys = self.keys.read().await;

        let mut jwks = Vec::new();

        // Add active key
        jwks.push(Jwk {
            kid: keys.active_key.id.clone(),
            kty: "RSA".to_string(), // Simplified, would need to determine from algorithm
            alg: format!("{:?}", keys.active_key.algorithm),
            use_: "sig".to_string(),
        });

        // Add retired keys
        for key in &keys.retired_keys {
            jwks.push(Jwk {
                kid: key.id.clone(),
                kty: "RSA".to_string(),
                alg: format!("{:?}", key.algorithm),
                use_: "sig".to_string(),
            });
        }

        Ok(JwkSet { keys: jwks })
    }

    pub fn create_claims(
        &self,
        user_id: Uuid,
        email: String,
        roles: Vec<String>,
        permissions: Vec<String>,
        session_id: Uuid,
        scopes: Vec<String>,
        device_id: Option<String>,
    ) -> Claims {
        let now = Utc::now();
        let exp = now + self.config.access_token_ttl;

        Claims {
            sub: user_id,
            email,
            roles,
            permissions,
            session_id,
            iat: now.timestamp(),
            exp: exp.timestamp(),
            nbf: now.timestamp(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            jti: Uuid::new_v4().to_string(),
            scopes,
            device_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwkSet {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    #[serde(rename = "use")]
    pub use_: String,
}
