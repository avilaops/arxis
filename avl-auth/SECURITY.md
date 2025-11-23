# Security Policy

## 🔒 Security at AVL Auth

Security is our top priority. AVL Auth is designed with multiple layers of security and follows industry best practices.

## 🛡️ Security Features

### Authentication
- Argon2id password hashing (winner of Password Hashing Competition)
- Configurable work factors for future-proofing
- Password strength validation
- Breach detection against common passwords
- Account lockout after failed attempts
- Rate limiting on authentication endpoints

### Token Security
- JWT with RS256/ES256 (asymmetric) by default
- Automatic key rotation
- Short-lived access tokens (15 min default)
- Secure refresh token rotation
- Token revocation support
- JWKS endpoint for public key distribution

### Session Security
- Device binding
- IP binding (optional)
- Idle timeout
- Absolute timeout
- Concurrent session limits
- Distributed session storage

### Access Control
- Role-Based Access Control (RBAC)
- Attribute-Based Access Control (ABAC)
- Fine-grained permissions
- Dynamic policy evaluation
- Principle of least privilege

### Data Protection
- AES-256-GCM encryption for sensitive data
- TLS 1.3 enforcement
- Secure key storage recommendations
- HSTS headers
- CORS configuration

### Monitoring & Detection
- Comprehensive audit logging
- Anomaly detection
- Geo-velocity checks
- Risk-based authentication
- Real-time alerting
- LGPD/GDPR compliance logging

## 🚨 Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please follow these steps:

### 1. **DO NOT** create a public GitHub issue

Security vulnerabilities should not be publicly disclosed until a fix is available.

### 2. Email us at security@avila.cloud

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)
- Your contact information

### 3. Response Timeline

- **Initial Response**: Within 24 hours
- **Severity Assessment**: Within 48 hours
- **Fix Development**: Based on severity
  - Critical: 1-3 days
  - High: 3-7 days
  - Medium: 7-14 days
  - Low: 14-30 days
- **Disclosure**: After fix is released and users have time to update

### 4. Bug Bounty

We currently do not have a formal bug bounty program, but we recognize and appreciate security researchers. Valid vulnerabilities will receive:
- Public acknowledgment (if desired)
- Swag and merchandise
- Potential monetary rewards for critical findings

## 🔐 Security Best Practices for Users

### 1. Key Management

```rust
// ❌ DON'T: Hardcode keys
let config = Config {
    jwt: JwtConfig {
        private_key: "-----BEGIN PRIVATE KEY-----...".to_string(),
        // ...
    },
    // ...
};

// ✅ DO: Load from secure storage
use std::env;
let config = Config {
    jwt: JwtConfig {
        private_key: env::var("JWT_PRIVATE_KEY")?,
        public_key: env::var("JWT_PUBLIC_KEY")?,
        // ...
    },
    // ...
};
```

### 2. Password Policies

```rust
// Configure strong password policies
let config = Config {
    password: PasswordConfig {
        min_length: 12,               // Minimum 12 characters
        require_uppercase: true,
        require_lowercase: true,
        require_numbers: true,
        require_special: true,
        argon2_memory_cost: 65536,    // 64 MB
        argon2_time_cost: 3,          // 3 iterations
        password_history: 5,          // Remember last 5
        // ...
    },
    // ...
};
```

### 3. Session Configuration

```rust
use std::time::Duration;

let config = Config {
    session: SessionConfig {
        idle_timeout: Duration::from_secs(1800),      // 30 minutes
        absolute_timeout: Duration::from_secs(43200),  // 12 hours
        max_concurrent_sessions: 5,
        device_binding: true,    // Highly recommended
        ip_binding: false,       // Optional, can cause issues with mobile
    },
    // ...
};
```

### 4. Rate Limiting

```rust
let config = Config {
    rate_limit: RateLimitConfig {
        login_attempts_per_minute: 5,
        registration_attempts_per_hour: 3,
        password_reset_attempts_per_hour: 3,
        lockout_threshold: 5,
        lockout_duration: Duration::from_secs(900), // 15 minutes
    },
    // ...
};
```

### 5. Risk-Based Authentication

```rust
let config = Config {
    risk: RiskConfig {
        enabled: true,
        mfa_threshold: 60,        // Require MFA if risk >= 60
        block_threshold: 90,      // Block if risk >= 90
        anomaly_detection: true,
        geo_velocity_check: true,
        max_travel_speed: 1000.0, // km/h
    },
    // ...
};
```

### 6. HTTPS Only

```rust
let config = Config {
    security: SecurityConfig {
        https_only: true,
        hsts_enabled: true,
        cors_enabled: true,
        cors_origins: vec!["https://your-domain.com".to_string()],
        // ...
    },
    // ...
};
```

### 7. Audit Logging

```rust
// Always enable audit logging in production
let config = Config {
    security: SecurityConfig {
        audit_enabled: true,
        // ...
    },
    // ...
};

// Regularly review audit logs
let activity = auth.audit_manager()
    .get_user_activity(&user_id, 30)
    .await?;

if activity.failed > 10 {
    // Alert security team
}
```

### 8. Regular Updates

```toml
# Keep dependencies updated
[dependencies]
avl-auth = "0.1"  # Use latest stable version
```

Check for updates regularly:
```bash
cargo update
cargo audit
```

## 🔍 Security Checklist

Before deploying to production:

- [ ] JWT keys are loaded from secure storage (not hardcoded)
- [ ] Strong password policy is configured
- [ ] HTTPS is enforced
- [ ] HSTS headers are enabled
- [ ] CORS is properly configured
- [ ] Rate limiting is enabled
- [ ] MFA is available for users
- [ ] Risk-based authentication is enabled
- [ ] Audit logging is enabled
- [ ] Session timeouts are configured
- [ ] Account lockout policies are set
- [ ] Dependencies are up to date
- [ ] Security headers are configured
- [ ] Error messages don't leak information
- [ ] Monitoring and alerting are set up

## 📚 Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
- [NIST Digital Identity Guidelines](https://pages.nist.gov/800-63-3/)
- [JWT Best Practices](https://datatracker.ietf.org/doc/html/rfc8725)

## 🏆 Security Acknowledgments

We would like to thank the following security researchers for responsibly disclosing vulnerabilities:

*None yet - be the first!*

## 📞 Contact

- **Security Issues**: security@avila.cloud
- **General Questions**: hello@avila.cloud
- **Twitter**: [@avilacloud](https://twitter.com/avilacloud)

---

**Remember**: Security is a continuous process, not a one-time setup. Stay vigilant and keep your systems updated.
