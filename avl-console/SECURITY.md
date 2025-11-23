# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.3.x   | :white_check_mark: |
| 0.2.x   | :white_check_mark: |
| < 0.2   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: security@avila.cloud

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Security Measures

### Input Validation
- All user inputs are validated and sanitized
- SQL injection protection via query safety module
- XSS protection through proper escaping
- CSRF protection with tokens

### Authentication & Authorization
- JWT-based authentication via AVL Auth
- Role-based access control (RBAC)
- Session management with secure cookies
- Password hashing with Argon2

### Network Security
- TLS/HTTPS enforced in production
- CORS properly configured
- Rate limiting per user/IP
- WebSocket authentication

### Data Protection
- Encryption at rest (AES-256)
- Encryption in transit (TLS 1.3)
- Secure key management
- PII data handling compliant with LGPD/GDPR

### Monitoring & Logging
- Comprehensive audit logs
- Real-time security alerts
- Anomaly detection
- Failed login attempt tracking

### Dependencies
- Regular security audits with `cargo-audit`
- Automated dependency updates
- Minimal dependency footprint
- Only trusted crates from crates.io

### Container Security
- Non-root user in containers
- Read-only root filesystem where possible
- Security scanning with Trivy
- Minimal base images (Debian slim)

### Kubernetes Security
- Pod security policies
- Network policies
- Resource quotas
- RBAC for service accounts

## Security Best Practices

### For Developers
1. Run `cargo audit` before committing
2. Use `cargo clippy` with pedantic lints
3. Enable all compiler warnings
4. Write security-focused tests
5. Review PRs for security issues
6. Follow OWASP guidelines

### For Operators
1. Change all default secrets
2. Enable TLS/HTTPS
3. Configure firewall rules
4. Set up monitoring and alerts
5. Regular backups
6. Incident response plan
7. Security audits

### Environment Variables
Never commit these to version control:
- `SESSION_SECRET`
- `AVL_AUTH_JWT_SECRET`
- `AVILADB_API_KEY`
- `OPENAI_API_KEY`
- `ANTHROPIC_API_KEY`

## Vulnerability Disclosure Policy

We follow coordinated vulnerability disclosure:

1. **Report** - Researcher reports vulnerability privately
2. **Acknowledge** - We acknowledge receipt within 48 hours
3. **Investigate** - We investigate and validate the issue
4. **Fix** - We develop and test a fix
5. **Release** - We release a patched version
6. **Disclose** - Public disclosure after 90 days or patch release

## Bug Bounty Program

We currently do not have a formal bug bounty program, but we recognize and appreciate security researchers who help us keep AVL Console secure.

Rewards:
- Public acknowledgment (if desired)
- CVE credit
- Swag and merchandise
- Possible monetary rewards for critical vulnerabilities

## Security Compliance

AVL Console follows these security standards:

- **OWASP Top 10** - Web application security
- **CWE Top 25** - Most dangerous software weaknesses
- **LGPD** - Brazilian data protection law
- **GDPR** - European data protection regulation
- **SOC 2** - Security, availability, and confidentiality (in progress)
- **ISO 27001** - Information security management (planned)

## Incident Response

In case of a security incident:

1. **Identify** - Detect and identify the incident
2. **Contain** - Isolate affected systems
3. **Eradicate** - Remove the threat
4. **Recover** - Restore systems to normal operation
5. **Lessons Learned** - Document and improve

Contact: security@avila.cloud

## Security Updates

Security updates are released as soon as possible after a vulnerability is confirmed. Subscribe to our security advisories:

- GitHub Security Advisories
- Email: security-announce@avila.cloud
- RSS Feed: https://avila.cloud/security/feed

## Hall of Fame

We thank the following security researchers for their contributions:

<!-- Contributors will be listed here -->

---

Last updated: November 23, 2025
