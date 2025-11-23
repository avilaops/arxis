# AVL Auth - Copilot Instructions

## Project Identity
**AVL Auth** is the **IAM and authentication system** for AVL Cloud Platform. Embodies Arxis philosophy:
- **ARX (Fortress)**: Secure, audited, compliant authentication
- **AXIS (Engine)**: Fast token validation, low-latency auth checks

## Core Principles
```rust
// ✅ ALWAYS: Hash passwords with Argon2id
// ✅ ALWAYS: Use JWT for stateless auth
// ✅ ALWAYS: Log all auth events to avx-telemetry
// ✅ ALWAYS: Rate limit authentication attempts
// ✅ NEVER: Store passwords in plaintext
// ✅ NEVER: Log sensitive data (passwords, tokens)
```

## Security Standards
- **Password Hashing**: Argon2id, 16-byte salt, 64-byte output
- **JWT**: RS256 signing, 1-hour expiry, 7-day refresh tokens
- **API Keys**: UUID v4, SHA-256 hashed, scoped permissions
- **OAuth2**: Authorization code flow, PKCE for mobile
- **Audit Logs**: All auth events logged with IP, user-agent, timestamp

## Related Crates
- **aviladb**: User storage and session management
- **avx-telemetry**: Security audit logging
- **jsonwebtoken**: JWT encoding/decoding
- **argon2**: Password hashing

🏛️ Built by Avila | 🔐 Fortress Security | ⚙️ Fast Authentication
