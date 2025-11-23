# Security Policy

## 🔒 Security in AvilaDB

AvilaDB takes security seriously. This document outlines our security practices and how to report vulnerabilities.

---

## Supported Versions

Currently supported versions for security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

---

## Security Features

### 🔐 Data Encryption

**At Rest:**
- AES-256-GCM encryption for all stored data
- Encryption keys managed via AVL Secrets
- Per-document encryption available
- Automatic key rotation

**In Transit:**
- TLS 1.3 for all network communication
- Certificate pinning support
- HSTS headers enabled
- Perfect forward secrecy

### 🛡️ Access Control

**Authentication:**
- API key-based authentication
- AVL Auth integration for OAuth2/OIDC
- Short-lived access tokens (JWT)
- Refresh token rotation

**Authorization:**
- Role-Based Access Control (RBAC)
- Collection-level permissions
- Document-level permissions (optional)
- Attribute-Based Access Control (ABAC)

### 📊 Audit Logging

**What We Log:**
- All data access operations
- Permission changes
- Failed authentication attempts
- Query operations with user context
- Administrative actions

**Log Retention:**
- Standard: 90 days
- Compliance mode: 7 years
- Tamper-proof log storage
- LGPD/GDPR compliant

### 🔍 Security Monitoring

**Real-time Detection:**
- Anomaly detection via `avila-telemetry`
- Rate limiting per API key
- DDoS protection
- SQL injection prevention
- XSS prevention

**Alerting:**
- Slack/Discord webhooks
- Email notifications
- PagerDuty integration
- Custom webhook support

---

## Security Best Practices

### 1. API Key Management

```rust
// ❌ DON'T hardcode keys
let client = AvilaClient::connect("http://...").await?;

// ✅ DO use environment variables
let api_key = std::env::var("AVILADB_API_KEY")?;
let config = Config {
    access_key: Some(api_key),
    ..Default::default()
};
let client = AvilaClient::with_config(config).await?;
```

### 2. Use TLS in Production

```rust
// ❌ DON'T use HTTP in production
let config = Config {
    endpoint: "http://api.avila.cloud".into(),
    ..Default::default()
};

// ✅ DO use HTTPS
let config = Config {
    endpoint: "https://api.avila.cloud".into(),
    ..Default::default()
};
```

### 3. Validate User Input

```rust
// ❌ DON'T trust user input
let doc = Document::new()
    .set("userId", user_input);

// ✅ DO validate and sanitize
let validated_id = validate_user_id(&user_input)?;
let doc = Document::new()
    .set("userId", validated_id);
```

### 4. Use Least Privilege

```rust
// ❌ DON'T use admin keys everywhere
let config = Config {
    access_key: Some(admin_key),
    ..Default::default()
};

// ✅ DO use read-only keys when possible
let config = Config {
    access_key: Some(readonly_key),
    ..Default::default()
};
```

### 5. Enable Audit Logging

```rust
let config = Config {
    audit_logging: true,
    audit_verbose: true,
    ..Default::default()
};
```

---

## Reporting a Vulnerability

### 🚨 How to Report

If you discover a security vulnerability in AvilaDB:

**1. DO NOT open a public issue**

**2. Email us privately:**
- **Email**: security@avila.cloud
- **Subject**: [SECURITY] Brief description
- **Include**:
  - Detailed description of the vulnerability
  - Steps to reproduce
  - Potential impact
  - Suggested fix (if any)
  - Your contact information

**3. Use PGP encryption (optional but recommended):**
- Public key: https://avila.cloud/pgp-key.txt
- Fingerprint: `1234 5678 90AB CDEF 1234 5678 90AB CDEF 1234 5678`

### ⏱️ Response Timeline

| Timeframe | Action                                  |
| --------- | --------------------------------------- |
| 24 hours  | Initial acknowledgment                  |
| 72 hours  | Preliminary assessment                  |
| 7 days    | Detailed response with timeline         |
| 30 days   | Fix deployed (critical vulnerabilities) |
| 90 days   | Public disclosure (if applicable)       |

### 🏆 Bug Bounty Program

We offer rewards for responsibly disclosed vulnerabilities:

| Severity | Reward               |
| -------- | -------------------- |
| Critical | R$ 5,000 - R$ 10,000 |
| High     | R$ 2,000 - R$ 5,000  |
| Medium   | R$ 500 - R$ 2,000    |
| Low      | R$ 100 - R$ 500      |

**Eligibility:**
- First to report the vulnerability
- No public disclosure before fix
- Provide clear reproduction steps
- Act in good faith

---

## Security Updates

### Notifications

Subscribe to security advisories:
- **GitHub**: Watch repository for security alerts
- **Email**: security-announce@avila.cloud
- **RSS**: https://avila.cloud/security.rss
- **Discord**: #security-announcements channel

### Update Process

When a security issue is fixed:

1. **Patch Release**: New version published to crates.io
2. **Security Advisory**: GitHub Security Advisory created
3. **Notification**: Email sent to subscribers
4. **Documentation**: CHANGELOG.md updated
5. **Public Disclosure**: 7 days after patch (if appropriate)

### How to Update

```bash
# Update to latest patch
cargo update -p aviladb

# Or update to specific version
cargo update -p aviladb --precise 0.1.1
```

---

## Compliance

### 🇧🇷 LGPD (Lei Geral de Proteção de Dados)

AvilaDB is **LGPD compliant**:
- ✅ Data minimization
- ✅ Purpose limitation
- ✅ Right to erasure (delete operations)
- ✅ Data portability (export operations)
- ✅ Audit logging
- ✅ Encryption at rest and in transit

### 🇪🇺 GDPR (General Data Protection Regulation)

AvilaDB is **GDPR compliant**:
- ✅ Lawful basis for processing
- ✅ Data subject rights
- ✅ Data breach notification
- ✅ Privacy by design
- ✅ Data Protection Impact Assessment (DPIA)

### 🇺🇸 SOC 2 Type II

**Status**: In progress (Q1 2026)
- Security
- Availability
- Processing integrity
- Confidentiality
- Privacy

---

## Security Checklist

Before deploying AvilaDB to production:

- [ ] Use HTTPS endpoints
- [ ] Store API keys in environment variables or secrets manager
- [ ] Enable audit logging
- [ ] Configure rate limiting
- [ ] Set up monitoring and alerting
- [ ] Enable encryption at rest
- [ ] Use least privilege access keys
- [ ] Regular security updates
- [ ] Backup encryption keys
- [ ] Test disaster recovery procedures
- [ ] Review access logs monthly
- [ ] Implement network segmentation
- [ ] Use Web Application Firewall (WAF)
- [ ] Enable DDoS protection

---

## Additional Resources

- [AVL Security Center](https://avila.cloud/security)
- [Security Best Practices](https://docs.avila.cloud/security/best-practices)
- [Compliance Documentation](https://docs.avila.cloud/compliance)
- [Incident Response Plan](https://docs.avila.cloud/security/incident-response)

---

## Contact

- **Security Team**: security@avila.cloud
- **PGP Key**: https://avila.cloud/pgp-key.txt
- **Discord**: discord.avila.cloud (#security channel)
- **Phone (Emergency)**: +55 11 XXXX-XXXX (Business hours, Portuguese/English)

---

**🔒 Your data's security is our fortress.**

🇧🇷 Built in Brazil | 🛡️ Protected by AVL | ⚡ Monitored 24/7
