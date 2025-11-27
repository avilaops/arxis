# 🔒 AVL Secrets

**Secure Secrets Management for AVL Cloud Platform**

[![Crates.io](https://img.shields.io/crates/v/avl-secrets.svg)](https://crates.io/crates/avl-secrets)
[![Documentation](https://docs.rs/avl-secrets/badge.svg)](https://docs.rs/avl-secrets)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.inc)

🏛️ **Vault-Like Security** | ⚙️ **Fast Retrieval** | 🔐 **AES-256 Encryption**

---

## Features

- **Encrypted Storage**: AES-256-GCM encryption at rest
- **Key Rotation**: Automatic and manual key rotation
- **Versioning**: Keep history of secret changes
- **Access Control**: Fine-grained permissions
- **Audit Logs**: All access logged and traceable
- **CLI Integration**: Easy secret management via `avl` CLI

## Quick Start

```rust
use avl_secrets::{SecretsClient, Secret};

#[tokio::main]
async fn main() {
    let client = SecretsClient::connect("https://secrets.avila.cloud").await?;

    // Store secret
    client.put("database/password", "super_secret_pw").await?;

    // Retrieve secret
    let secret = client.get("database/password").await?;
    println!("Password: {}", secret.value);

    // List secrets
    let secrets = client.list("database/").await?;
    for name in secrets {
        println!("- {}", name);
    }
}
```

## Security

- **Encryption**: AES-256-GCM for data at rest
- **Transit Encryption**: TLS 1.3 for data in transit
- **Master Key**: Hardware Security Module (HSM) backed
- **Zero-Knowledge**: Secrets never logged in plaintext
- **Compliance**: SOC2, ISO 27001 compliant

🏛️ **Built by Avila** - Part of AVL Cloud Platform
