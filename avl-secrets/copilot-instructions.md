# AVL Secrets - Copilot Instructions

## Project Identity
**AVL Secrets** is the **secrets management system** for AVL Cloud (Vault-like). Embodies Arxis philosophy:
- **ARX (Fortress)**: AES-256 encryption, zero-knowledge, HSM-backed
- **AXIS (Engine)**: Fast retrieval, automatic rotation, versioning

## Core Principles
```rust
// ✅ ALWAYS: Encrypt with AES-256-GCM at rest
// ✅ ALWAYS: Use TLS 1.3 for transit encryption
// ✅ ALWAYS: Audit log every secret access
// ✅ ALWAYS: Support key rotation (manual and automatic)
// ✅ NEVER: Log secrets in plaintext (not even DEBUG level)
// ✅ NEVER: Store unencrypted secrets anywhere
```

## Security Standards
- **Encryption**: AES-256-GCM with unique nonce per secret
- **Master Key**: HSM-backed, never exposed to application
- **Access Control**: Path-based permissions, RBAC integration
- **Audit**: All reads/writes logged with user, IP, timestamp
- **Versioning**: Keep history, roll back to previous versions

## API Design
```rust
// Secrets stored as key-value pairs
client.put("database/prod/password", "secret_value").await?;
client.get("database/prod/password").await?;
client.list("database/prod/").await?; // List all secrets under path
client.delete("database/prod/password").await?;
client.rotate("database/prod/password").await?; // Generate new version
```

## Related Crates
- **aes-gcm**: Encryption/decryption
- **aviladb**: Encrypted secret storage
- **avl-auth**: Access control integration
- **avx-telemetry**: Security audit logs

🏛️ Built by Avila | 🔒 Vault-Like Security | ⚙️ Fast Retrieval
