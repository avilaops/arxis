# AVL Platform Integration Guide

Complete guide for integrating AVL Auth with the entire Avila Cloud Platform ecosystem.

---

## 📦 Ecosystem Overview

AVL Auth is designed to work seamlessly with other Avila Cloud libraries:

| Library             | Purpose                        | Integration Benefit                                          |
| ------------------- | ------------------------------ | ------------------------------------------------------------ |
| **AvilaDB**         | Distributed NoSQL database     | User storage, session persistence, audit logs                |
| **AVX Telemetry**   | Structured logging & tracing   | Request tracking, error monitoring, performance metrics      |
| **Avila Telemetry** | Time series & analytics        | Risk prediction, anomaly detection, behavioral analysis      |
| **Avila Compress**  | Native compression (LZ4, Zstd) | Token compression, session optimization, bandwidth reduction |
| **AVL Storage**     | Object storage                 | Profile pictures, document storage, backup management        |
| **AVL Queue**       | Message queue                  | Async notifications, event processing, webhooks              |
| **AVL Secrets**     | Secret management              | API keys, encryption keys, OAuth credentials                 |

---

## 🚀 Quick Start

### 1. Enable Features

```toml
[dependencies]
avl-auth = { version = "0.1", features = ["full"] }
```

**Available features:**
- `default` = `["telemetry"]` - Basic telemetry
- `full` - All integrations enabled
- `database` - AvilaDB integration
- `telemetry` - AVX Telemetry structured logging
- `compression` - Avila Compress for token optimization
- `analytics` - Avila Telemetry time series analysis

### 2. Run Integration Example

```bash
# Full platform integration
cargo run --example avl_platform_integration --features full

# Individual features
cargo run --example avl_platform_integration --features database,telemetry
```

---

## 🗄️ AvilaDB Integration

### Why AvilaDB?

- **4MB document size** (10x larger than AWS DynamoDB)
- **Sub-10ms latency** in Brazil
- **Multi-region writes** at no extra cost
- **Vector search** for semantic user matching
- **40-60% cheaper** than AWS/Azure

### Configuration

```rust
use avl_auth::{Config, AuthClient};

let mut config = Config::default();
config.database_url = "http://localhost:8000".to_string();
config.database_name = "avl_auth".to_string();

// For production:
// config.database_url = "https://your-account.avila.cloud".to_string();

let client = AuthClient::new(config).await?;
```

### What Gets Stored

1. **User Profiles** (`users` collection)
   ```json
   {
     "id": "usr_abc123",
     "email": "user@example.com",
     "password_hash": "...",
     "roles": ["user", "premium"],
     "created_at": "2024-01-15T...",
     "metadata": {
       "last_login": "2024-01-20T...",
       "login_count": 42
     }
   }
   ```

2. **Sessions** (`sessions` collection)
   ```json
   {
     "session_id": "sess_xyz789",
     "user_id": "usr_abc123",
     "access_token_jti": "...",
     "refresh_token_jti": "...",
     "device_id": "mobile_app_v1",
     "ip_address": "191.36.8.1",
     "expires_at": "2024-01-20T..."
   }
   ```

3. **Audit Logs** (`audit_logs` collection)
   ```json
   {
     "timestamp": "2024-01-20T10:30:00Z",
     "event_type": "user.login",
     "user_id": "usr_abc123",
     "ip_address": "191.36.8.1",
     "success": true,
     "risk_score": 0.15
   }
   ```

### Vector Search for User Matching

```rust
// Semantic search for similar users (fraud detection)
let similar_users = aviladb
    .collection("users")
    .vector_search(
        "behavior_vector",
        user_behavior_embedding,
        10  // top 10 similar users
    )
    .await?;
```

---

## 📊 AVX Telemetry Integration

### Structured Logging

AVX Telemetry provides JSON-structured logs with automatic tracing context:

```rust
use tracing::{info, error};

// Automatically includes trace_id, span_id, user_id
info!(
    user_id = %user.id,
    email = %user.email,
    "User login successful"
);

error!(
    error = %e,
    user_id = %user.id,
    "Authentication failed"
);
```

### Distributed Tracing

```rust
use tracing::instrument;

#[instrument(skip(client))]
async fn authenticate_user(client: &AuthClient, email: String) -> Result<Session> {
    // Automatically tracked with trace_id
    let user = client.get_user_by_email(&email).await?;
    let session = client.create_session(&user).await?;
    Ok(session)
}
```

### Log Output Example

```json
{
  "timestamp": "2024-01-20T10:30:00.123Z",
  "level": "INFO",
  "message": "User login successful",
  "trace_id": "a1b2c3d4e5f6",
  "span_id": "12345678",
  "user_id": "usr_abc123",
  "email": "user@example.com",
  "fields": {
    "ip_address": "191.36.8.1",
    "device_id": "mobile_app_v1"
  }
}
```

---

## 🔍 Avila Telemetry Analytics

### Time Series Risk Analysis

Avila Telemetry provides NASA-grade time series analysis:

```rust
use avila_telemetry::{TimeSeries, ARIMA};

// Track login attempts over time
let mut login_series = TimeSeries::new("login_attempts");
login_series.add_point(timestamp, 1.0);

// ARIMA forecasting
let forecast = ARIMA::new()
    .fit(&login_series)
    .forecast(24)?;  // next 24 hours

if forecast.mean() > threshold {
    warn!("Unusual login activity predicted");
}
```

### Anomaly Detection

```rust
use avila_telemetry::AnomalyDetector;

let detector = AnomalyDetector::new()
    .with_sensitivity(0.95);

let is_anomaly = detector.detect(&user_behavior_vector)?;
if is_anomaly {
    // Trigger additional MFA challenge
    client.require_mfa(&session)?;
}
```

### Integration with Risk Engine

```rust
// AVL Auth Risk Engine + Avila Telemetry
let risk_factors = client.risk_engine()
    .assess_login(&credentials)
    .await?;

// Use time series to enhance risk score
let historical_pattern = telemetry
    .get_user_login_pattern(&user.id)
    .await?;

let enhanced_risk = risk_factors.score
    + historical_pattern.anomaly_score();
```

---

## 🗜️ Avila Compress Integration

### Token Compression

Reduces JWT size by 60-80% for mobile/IoT scenarios:

```rust
use avila_compress::{Compressor, Algorithm};

// Compress access token before sending
let token = client.create_token(&user)?;
let compressed = Compressor::new(Algorithm::LZ4)
    .compress(token.as_bytes())?;

// Token size: 1200 bytes -> 300 bytes
```

### Session Storage Optimization

```rust
// Compress session data in AvilaDB
let session_data = serde_json::to_vec(&session)?;
let compressed = Compressor::new(Algorithm::Zstd)
    .compress(&session_data)?;

aviladb.collection("sessions")
    .insert(doc! {
        "session_id": session.id,
        "data": compressed,  // 70% smaller
        "compressed": true
    })
    .await?;
```

### Benefits

- **Bandwidth Reduction**: 60-80% smaller payloads
- **Storage Savings**: Compress audit logs, session data
- **Faster Transmission**: Critical for mobile/IoT
- **Cost Optimization**: Lower network costs

---

## 🔗 Additional Integrations

### AVL Storage (Profile Pictures)

```rust
use avl_storage::StorageClient;

let storage = StorageClient::connect("https://storage.avila.cloud").await?;

// Upload profile picture
let picture_url = storage
    .bucket("user-profiles")
    .upload(user.id, profile_image)
    .await?;

// Update user record
client.update_user(&user.id, doc! {
    "profile_picture": picture_url
}).await?;
```

### AVL Queue (Async Notifications)

```rust
use avl_queue::QueueClient;

let queue = QueueClient::connect("https://queue.avila.cloud").await?;

// Send welcome email asynchronously
queue.publish("emails", json!({
    "type": "welcome",
    "user_id": user.id,
    "email": user.email
})).await?;
```

### AVL Secrets (Key Management)

```rust
use avl_secrets::SecretsClient;

let secrets = SecretsClient::connect("https://secrets.avila.cloud").await?;

// Store OAuth client secrets
secrets.set("oauth2/google/client_secret", google_secret).await?;

// Rotate JWT signing keys
let new_key = crypto.generate_rsa_keypair(2048)?;
secrets.set("jwt/signing_key/v2", new_key.0).await?;
```

---

## 🏗️ Architecture Patterns

### Pattern 1: Full Stack Authentication

```
┌─────────────────┐
│   Web/Mobile    │
│   Application   │
└────────┬────────┘
         │
    ┌────▼─────┐
    │ AVL Auth │
    └────┬─────┘
         │
    ┌────▼─────────────────┐
    │   AVL Platform       │
    ├──────────────────────┤
    │ • AvilaDB           │
    │ • AVX Telemetry     │
    │ • Avila Compress    │
    │ • AVL Storage       │
    │ • AVL Queue         │
    │ • AVL Secrets       │
    └─────────────────────┘
```

### Pattern 2: Microservices Gateway

```
┌─────────┐  ┌─────────┐  ┌─────────┐
│Service A│  │Service B│  │Service C│
└────┬────┘  └────┬────┘  └────┬────┘
     │            │            │
     └────────┬───┴────────────┘
              │
         ┌────▼─────┐
         │ AVL Auth │  (Shared authentication)
         │  Gateway │
         └────┬─────┘
              │
         ┌────▼─────┐
         │ AvilaDB  │  (Centralized user store)
         └──────────┘
```

### Pattern 3: Edge Authentication

```
┌──────────────────────────┐
│   CDN Edge (Cloudflare)  │
│   ┌──────────────────┐   │
│   │ AVL Auth Worker  │   │ (Lightweight auth at edge)
│   └────────┬─────────┘   │
└────────────┼─────────────┘
             │
        ┌────▼─────┐
        │ AvilaDB  │  (Global multi-region)
        └──────────┘
```

---

## 🎯 Best Practices

### 1. Feature Selection

**Development:**
```toml
avl-auth = { version = "0.1" }  # Basic features only
```

**Production:**
```toml
avl-auth = { version = "0.1", features = ["database", "telemetry"] }
```

**High-Scale:**
```toml
avl-auth = { version = "0.1", features = ["full"] }
```

### 2. Environment Configuration

```rust
use avl_auth::Config;

let config = match std::env::var("ENVIRONMENT")?.as_str() {
    "development" => Config {
        database_url: "http://localhost:8000".into(),
        enable_telemetry: false,
        ..Default::default()
    },
    "production" => Config {
        database_url: "https://prod.avila.cloud".into(),
        enable_telemetry: true,
        enable_compression: true,
        enable_analytics: true,
        ..Default::default()
    },
    _ => Config::default()
};
```

### 3. Error Handling

```rust
use avl_auth::{AuthClient, AuthError};
use tracing::error;

match client.login(credentials).await {
    Ok(session) => Ok(session),
    Err(AuthError::InvalidCredentials) => {
        error!("Login failed: invalid credentials");
        Err(AuthError::InvalidCredentials)
    }
    Err(AuthError::DatabaseError(e)) => {
        error!(error = %e, "AvilaDB connection failed");
        // Fallback to cached authentication
        client.login_offline(credentials).await
    }
    Err(e) => Err(e)
}
```

### 4. Performance Monitoring

```rust
use tracing::instrument;
use std::time::Instant;

#[instrument]
async fn timed_operation() {
    let start = Instant::now();

    let result = client.authenticate(&token).await;

    let duration = start.elapsed();
    if duration.as_millis() > 50 {
        warn!(duration_ms = duration.as_millis(), "Slow authentication");
    }
}
```

---

## 🇧🇷 Brazil-Optimized Deployment

### Latency Comparison

| Service  | AWS (São Paulo) | Azure (Brazil) | AVL Platform |
| -------- | --------------- | -------------- | ------------ |
| Auth     | 80-120ms        | 40-60ms        | **5-10ms**   |
| Database | 50-80ms         | 30-50ms        | **<10ms**    |
| Total    | 130-200ms       | 70-110ms       | **15-20ms**  |

### Cost Comparison (Monthly)

| Metric         | AWS      | Azure    | AVL Platform            | Savings |
| -------------- | -------- | -------- | ----------------------- | ------- |
| 1M auth ops    | USD 1.25 | USD 0.85 | **R$ 0,50** (~USD 0.10) | **92%** |
| 10GB storage   | USD 2.50 | USD 2.50 | **R$ 2,00** (~USD 0.40) | **84%** |
| 100GB transfer | USD 9.00 | USD 8.70 | **R$ 5,00** (~USD 1.00) | **89%** |

### Deployment Recommendation

```rust
// São Paulo region (primary)
let config_sp = Config {
    region: "sao-paulo".into(),
    database_url: "https://sp.avila.cloud".into(),
    ..Default::default()
};

// Rio de Janeiro region (failover)
let config_rj = Config {
    region: "rio-de-janeiro".into(),
    database_url: "https://rj.avila.cloud".into(),
    ..Default::default()
};

// Automatic failover
let client = AuthClient::multi_region(vec![config_sp, config_rj]).await?;
```

---

## 📚 Additional Resources

- [AvilaDB Documentation](https://docs.avila.cloud/aviladb)
- [AVX Telemetry Guide](https://docs.avila.cloud/avx-telemetry)
- [Avila Telemetry (Time Series)](https://docs.avila.cloud/avila-telemetry)
- [Avila Compress API](https://docs.avila.cloud/avila-compress)
- [AVL Platform Overview](https://docs.avila.cloud)

---

## 🎓 Training & Support

- **Discord**: [discord.avila.cloud](https://discord.avila.cloud)
- **Email**: support@avila.cloud
- **Workshops**: Monthly Rust + AVL Platform workshops in Portuguese

---

**Built with ❤️ in Brazil for LATAM developers**

🇧🇷 Otimizado para o Brasil | ⚡ Sub-10ms latency | 💰 40-60% mais barato
