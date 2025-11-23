//! Multi-factor authentication with TOTP and WebAuthn

use crate::error::{AuthError, Result};
use crate::models::{TotpAlgorithm, TotpConfig, WebAuthnCredential};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

pub struct MfaManager {
    totp_issuer: String,
    totp_period: u32,
    totp_digits: u32,
}

impl MfaManager {
    pub fn new(issuer: String, period: u32, digits: u32) -> Self {
        Self {
            totp_issuer: issuer,
            totp_period: period,
            totp_digits: digits,
        }
    }

    // ==================== TOTP Implementation ====================

    pub fn generate_totp_secret(&self) -> String {
        let mut secret = vec![0u8; 20]; // 160 bits
        rand::thread_rng().fill_bytes(&mut secret);
        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &secret)
    }

    pub fn generate_totp_config(&self, account_name: &str, secret: Option<String>) -> TotpConfig {
        TotpConfig {
            secret: secret.unwrap_or_else(|| self.generate_totp_secret()),
            algorithm: TotpAlgorithm::SHA1,
            digits: self.totp_digits,
            period: self.totp_period,
            issuer: self.totp_issuer.clone(),
            account_name: account_name.to_string(),
        }
    }

    pub fn generate_totp_uri(&self, config: &TotpConfig) -> String {
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm={:?}&digits={}&period={}",
            urlencoding::encode(&config.issuer),
            urlencoding::encode(&config.account_name),
            config.secret,
            urlencoding::encode(&config.issuer),
            config.algorithm,
            config.digits,
            config.period
        )
    }

    pub fn verify_totp(&self, secret: &str, code: &str, tolerance: u64) -> Result<bool> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AuthError::Internal(e.to_string()))?
            .as_secs();

        // Check current time window and adjacent windows (tolerance)
        for i in 0..=tolerance {
            for sign in &[-1i64, 1i64] {
                let offset = (*sign as i64) * (i as i64);
                let time = (current_time as i64 + offset * self.totp_period as i64) as u64;
                let expected_code = self.generate_totp_code(secret, time)?;

                if expected_code == code {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn generate_totp_code(&self, secret: &str, time: u64) -> Result<String> {
        let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret)
            .ok_or_else(|| AuthError::CryptoError("Invalid TOTP secret".to_string()))?;

        let counter = time / self.totp_period as u64;
        let code = self.generate_hotp(&decoded, counter)?;

        Ok(format!("{:0width$}", code, width = self.totp_digits as usize))
    }

    fn generate_hotp(&self, key: &[u8], counter: u64) -> Result<u32> {
        use hmac::{Hmac, Mac};
        use sha1::Sha1;

        type HmacSha1 = Hmac<Sha1>;

        let mut mac = HmacSha1::new_from_slice(key)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;

        mac.update(&counter.to_be_bytes());
        let result = mac.finalize();
        let code = result.into_bytes();

        let offset = (code[19] & 0xf) as usize;
        let binary = ((code[offset] & 0x7f) as u32) << 24
            | ((code[offset + 1] & 0xff) as u32) << 16
            | ((code[offset + 2] & 0xff) as u32) << 8
            | ((code[offset + 3] & 0xff) as u32);

        let modulo = 10u32.pow(self.totp_digits);
        Ok(binary % modulo)
    }

    pub fn generate_backup_codes(&self, count: usize) -> Vec<String> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        (0..count)
            .map(|_| {
                format!(
                    "{:04}-{:04}",
                    rng.gen_range(0..10000),
                    rng.gen_range(0..10000)
                )
            })
            .collect()
    }

    // ==================== WebAuthn Implementation ====================
    // Note: This is a simplified WebAuthn implementation
    // For production, use a dedicated crate like `webauthn-rs`

    pub fn generate_webauthn_challenge(&self) -> String {
        let mut challenge = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut challenge);
        URL_SAFE_NO_PAD.encode(&challenge)
    }

    pub fn create_webauthn_credential(
        &self,
        credential_id: String,
        public_key: Vec<u8>,
        name: String,
    ) -> WebAuthnCredential {
        WebAuthnCredential {
            id: credential_id,
            public_key,
            counter: 0,
            name,
            created_at: chrono::Utc::now(),
            last_used_at: None,
        }
    }

    pub fn verify_webauthn_signature(
        &self,
        _credential: &WebAuthnCredential,
        authenticator_data: &[u8],
        _client_data_json: &[u8],
        _signature: &[u8],
    ) -> Result<bool> {
        // This is a placeholder for WebAuthn signature verification
        // In production, implement full FIDO2/WebAuthn spec

        // Verify authenticator data flags
        if authenticator_data.len() < 37 {
            return Ok(false);
        }

        let flags = authenticator_data[32];
        let user_present = (flags & 0x01) != 0;
        let user_verified = (flags & 0x04) != 0;

        if !user_present {
            return Ok(false);
        }

        // In real implementation:
        // 1. Parse and verify authenticator data
        // 2. Hash client data JSON
        // 3. Verify signature using public key
        // 4. Check and update counter

        Ok(user_verified)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaChallenge {
    pub challenge_id: String,
    pub challenge: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodes {
    pub codes: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
