//! Password hashing and validation with Argon2id

use crate::error::{AuthError, Result};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params, Version,
};
use rand::rngs::OsRng;

pub struct PasswordManager {
    argon2: Argon2<'static>,
    policy: PasswordPolicy,
}

#[derive(Clone)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special: bool,
    pub password_history: u32,
}

impl PasswordManager {
    pub fn new(policy: PasswordPolicy, memory_cost: u32, time_cost: u32, parallelism: u32) -> Result<Self> {
        let params = Params::new(memory_cost, time_cost, parallelism, None)
            .map_err(|e| AuthError::ConfigError(e.to_string()))?;

        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            params,
        );

        Ok(Self { argon2, policy })
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        self.validate_password(password)?;

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;

        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    pub fn validate_password(&self, password: &str) -> Result<()> {
        if password.len() < self.policy.min_length {
            return Err(AuthError::InvalidPassword(
                format!("Password must be at least {} characters", self.policy.min_length)
            ));
        }

        if self.policy.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(AuthError::InvalidPassword(
                "Password must contain at least one uppercase letter".to_string()
            ));
        }

        if self.policy.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(AuthError::InvalidPassword(
                "Password must contain at least one lowercase letter".to_string()
            ));
        }

        if self.policy.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err(AuthError::InvalidPassword(
                "Password must contain at least one number".to_string()
            ));
        }

        if self.policy.require_special {
            let special_chars = "!@#$%^&*()_+-=[]{}|;:',.<>?/~`";
            if !password.chars().any(|c| special_chars.contains(c)) {
                return Err(AuthError::InvalidPassword(
                    "Password must contain at least one special character".to_string()
                ));
            }
        }

        // Check for common weak passwords
        if self.is_common_password(password) {
            return Err(AuthError::InvalidPassword(
                "Password is too common, please choose a stronger password".to_string()
            ));
        }

        Ok(())
    }

    fn is_common_password(&self, password: &str) -> bool {
        // Top 100 most common passwords (simplified list)
        const COMMON_PASSWORDS: &[&str] = &[
            "password", "123456", "123456789", "12345678", "12345",
            "qwerty", "abc123", "password1", "111111", "iloveyou",
            "admin", "welcome", "monkey", "dragon", "letmein",
        ];

        COMMON_PASSWORDS.contains(&password.to_lowercase().as_str())
    }

    pub fn generate_strong_password(&self, length: usize) -> String {
        use rand::Rng;

        let mut rng = OsRng;
        let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=";

        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect()
    }

    pub fn calculate_strength(&self, password: &str) -> PasswordStrength {
        let mut score = 0;

        // Length
        if password.len() >= 12 { score += 20; }
        else if password.len() >= 10 { score += 15; }
        else if password.len() >= 8 { score += 10; }

        // Character variety
        if password.chars().any(|c| c.is_uppercase()) { score += 15; }
        if password.chars().any(|c| c.is_lowercase()) { score += 15; }
        if password.chars().any(|c| c.is_numeric()) { score += 15; }

        let special_chars = "!@#$%^&*()_+-=[]{}|;:',.<>?/~`";
        if password.chars().any(|c| special_chars.contains(c)) { score += 20; }

        // Entropy
        let unique_chars = password.chars().collect::<std::collections::HashSet<_>>().len();
        score += (unique_chars * 2).min(15);

        match score {
            0..=40 => PasswordStrength::Weak,
            41..=70 => PasswordStrength::Medium,
            71..=85 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}
