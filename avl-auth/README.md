# 🔐 AVL Auth

**Identity and Access Management for AVL Cloud Platform**

[![Crates.io](https://img.shields.io/crates/v/avl-auth.svg)](https://crates.io/crates/avl-auth)
[![Documentation](https://docs.rs/avl-auth/badge.svg)](https://docs.rs/avl-auth)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.cloud)

🏛️ **Fortress Security** | ⚙️ **Fast Authentication** | 🇧🇷 **Built for Brazil**

---

## Features

- **JWT Authentication**: Stateless token-based auth
- **OAuth2/OIDC**: Integration with Google, GitHub, Microsoft
- **RBAC**: Role-Based Access Control
- **API Keys**: Service-to-service authentication
- **MFA**: Multi-factor authentication support
- **Session Management**: Secure session handling
- **Password Hashing**: Argon2 for secure passwords

## Quick Start

```rust
use avl_auth::{AuthClient, Credentials};

#[tokio::main]
async fn main() {
    let auth = AuthClient::connect("https://auth.avila.cloud").await?;

    // Register user
    auth.register("user@example.com", "secure_password").await?;

    // Login
    let token = auth.login(Credentials {
        email: "user@example.com",
        password: "secure_password",
    }).await?;

    // Verify token
    let claims = auth.verify_token(&token).await?;
    println!("User ID: {}", claims.sub);
}
```

## Architecture

- **JWT Tokens**: Secure, stateless authentication
- **AvilaDB Backend**: User storage and sessions
- **OAuth2 Flows**: Authorization code, implicit, client credentials
- **API Key Management**: Generate, rotate, revoke keys
- **Audit Logs**: All auth events logged via `avx-telemetry`

## Security

- **Password Hashing**: Argon2id with salt
- **Token Expiration**: Configurable JWT expiry
- **Refresh Tokens**: Long-lived, securely stored
- **Rate Limiting**: Brute-force protection
- **2FA**: TOTP support for enhanced security

🏛️ **Built by Avila** - Part of AVL Cloud Platform
