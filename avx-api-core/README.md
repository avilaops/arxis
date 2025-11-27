# avx-api-core

**Core API types and business logic for Avila Experience Fabric**

[![Crates.io](https://img.shields.io/crates/v/avx-api-core.svg)](https://crates.io/crates/avx-api-core)
[![Documentation](https://docs.rs/avx-api-core/badge.svg)](https://docs.rs/avx-api-core)
[![License](https://img.shields.io/crates/l/avx-api-core.svg)](https://github.com/avilaops/arxis#license)

Shared types, traits, models, and business logic for building AVX (Avila Experience) platform APIs. Provides common abstractions used across microservices and gateway layers.

## Features

- **Shared Models**: Common data structures for users, sessions, resources
- **API Traits**: Consistent interfaces for services
- **Error Handling**: Unified error types with HTTP status mapping
- **Validation**: Request/response validation utilities
- **Serialization**: JSON, MessagePack, and Protocol Buffers support
- **Pagination**: Cursor and offset-based pagination helpers
- **Rate Limiting**: Token bucket and quota abstractions
- **Authentication**: JWT claims, API key models

## Installation

```toml
[dependencies]
avx-api-core = "0.1"
```

## Quick Start

### Define API Models

```rust
use avx_api_core::models::{User, Resource, ApiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
        }
    }
}
```

### Use Common Traits

```rust
use avx_api_core::traits::{Repository, Service};
use async_trait::async_trait;

#[async_trait]
impl Repository<User> for UserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        // Implementation
    }

    async fn create(&self, user: User) -> Result<User, Error> {
        // Implementation
    }
}

#[async_trait]
impl Service for UserService {
    type Request = CreateUserRequest;
    type Response = UserResponse;

    async fn execute(&self, req: Self::Request) -> Result<Self::Response, Error> {
        // Business logic
        let user = self.repository.create(req.into()).await?;
        Ok(user.into())
    }
}
```

### Error Handling

```rust
use avx_api_core::error::{ApiError, ErrorCode};
use axum::response::{IntoResponse, Response};

pub enum UserError {
    NotFound(String),
    EmailTaken(String),
    ValidationFailed(String),
}

impl From<UserError> for ApiError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound(id) => ApiError {
                code: ErrorCode::NotFound,
                message: format!("User {} not found", id),
                details: None,
            },
            UserError::EmailTaken(email) => ApiError {
                code: ErrorCode::Conflict,
                message: format!("Email {} already taken", email),
                details: Some(serde_json::json!({"email": email})),
            },
            UserError::ValidationFailed(msg) => ApiError {
                code: ErrorCode::BadRequest,
                message: msg,
                details: None,
            },
        }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        ApiError::from(self).into_response()
    }
}
```

### Pagination

```rust
use avx_api_core::pagination::{Page, Pageable, CursorPage};

// Offset-based pagination
pub async fn list_users(
    page: Pageable,
) -> Result<Page<UserResponse>, ApiError> {
    let users = user_repo.find_all(page.offset(), page.size()).await?;
    let total = user_repo.count().await?;

    Ok(Page::new(users, page.page_number(), page.size(), total))
}

// Cursor-based pagination
pub async fn list_events(
    cursor: Option<String>,
    size: usize,
) -> Result<CursorPage<Event>, ApiError> {
    let (events, next_cursor) = event_repo
        .find_after(cursor.as_deref(), size)
        .await?;

    Ok(CursorPage::new(events, next_cursor))
}
```

### Validation

```rust
use avx_api_core::validation::{Validate, ValidationError};

impl Validate for CreateUserRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.email.is_empty() {
            return Err(ValidationError::field("email", "Email is required"));
        }

        if !self.email.contains('@') {
            return Err(ValidationError::field("email", "Invalid email format"));
        }

        if self.name.len() < 2 {
            return Err(ValidationError::field("name", "Name too short"));
        }

        Ok(())
    }
}

// Use in handler
#[axum::handler]
pub async fn create_user(
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    req.validate()?; // Returns 400 if validation fails

    let user = user_service.create(req).await?;
    Ok(Json(user))
}
```

## Common Types

### User Models

```rust
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum UserRole {
    Admin,
    User,
    Guest,
}
```

### Session Models

```rust
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: i64,
    pub metadata: serde_json::Value,
}
```

### Resource Models

```rust
pub struct Resource {
    pub id: String,
    pub owner_id: String,
    pub kind: ResourceKind,
    pub data: serde_json::Value,
    pub created_at: i64,
}

pub enum ResourceKind {
    Project,
    File,
    Collection,
}
```

## Authentication

```rust
use avx_api_core::auth::{Claims, JwtValidator};

pub struct Claims {
    pub sub: String,        // user_id
    pub exp: i64,           // expiration
    pub iat: i64,           // issued_at
    pub role: String,
}

// Validate JWT
let validator = JwtValidator::new("secret");
let claims = validator.validate(&token)?;

// Extract user from middleware
use axum::extract::Extension;

#[axum::handler]
pub async fn protected_route(
    Extension(claims): Extension<Claims>,
) -> String {
    format!("Hello, user {}!", claims.sub)
}
```

## Rate Limiting

```rust
use avx_api_core::rate_limit::{RateLimiter, Quota};

let limiter = RateLimiter::new(Quota {
    requests: 100,
    window: Duration::from_secs(60),
});

if limiter.check(&user_id).await? {
    // Process request
} else {
    return Err(ApiError::rate_limit_exceeded());
}
```

## Serialization

Supports multiple formats:

```rust
use avx_api_core::serde::{to_json, to_msgpack, from_json};

let user = User { /* ... */ };

// JSON
let json = to_json(&user)?;

// MessagePack (binary, more compact)
let msgpack = to_msgpack(&user)?;

// From JSON
let user: User = from_json(&json)?;
```

## Integration with AVX Ecosystem

Used by all AVX services:

```rust
use avx_api_core::prelude::*;
use avx_gateway::Gateway;
use avx_telemetry::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();

    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .layer(Extension(user_service));

    // Run with gateway
    Gateway::builder()
        .route("/api", app)
        .build()
        .await
        .unwrap()
        .serve()
        .await
        .unwrap();
}
```

## Testing Utilities

```rust
use avx_api_core::testing::{MockRepository, TestContext};

#[tokio::test]
async fn test_create_user() {
    let ctx = TestContext::new();
    let repo = MockRepository::new();

    repo.expect_create()
        .returning(|user| Ok(user));

    let service = UserService::new(repo);
    let result = service.create(CreateUserRequest {
        email: "test@example.com".into(),
        name: "Test".into(),
    }).await;

    assert!(result.is_ok());
}
```

## Part of AVX Ecosystem

`avx-api-core` provides the foundation for:

- **avx-gateway**: API routing and middleware
- **avx-telemetry**: Structured logging of API events
- **avx-config**: Configuration types and validation
- Custom microservices

## Examples

```bash
cargo run --example basic_api
cargo run --example with_validation
cargo run --example pagination
cargo run --example auth
```

## License

MIT OR Apache-2.0

See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

## Links

- **Repository**: https://github.com/avilaops/arxis
- **Documentation**: https://docs.rs/avx-api-core
- **Crates.io**: https://crates.io/crates/avx-api-core
- **AVX Platform**: https://avila.inc
- **Documentation**: https://docs.avila.inc
