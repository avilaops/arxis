# AVL Console Development Guide

## 🏗️ Project Structure

```
avl-console/
├── src/
│   ├── lib.rs                 # Main library entry point
│   ├── api.rs                 # REST API routes
│   ├── auth.rs                # Authentication & authorization
│   ├── billing.rs             # Billing & cost tracking
│   ├── config.rs              # Configuration management
│   ├── dashboard.rs           # Dashboard routes & UI
│   ├── database.rs            # AvilaDB Explorer
│   ├── error.rs               # Error types
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs            # Auth middleware
│   │   └── rate_limit.rs      # Rate limiting
│   ├── observability.rs       # Metrics, logs, traces
│   ├── state.rs               # Application state
│   ├── storage.rs             # Storage browser
│   ├── templates.rs           # Template filters
│   └── websocket.rs           # WebSocket handlers
├── examples/
│   └── basic.rs               # Basic usage example
├── tests/
│   └── integration_tests.rs   # Integration tests
├── static/                    # Static assets (CSS, JS, images)
├── templates/                 # HTML templates (Askama)
├── Cargo.toml
└── README.md
```

## 🚀 Development Setup

### Prerequisites

- Rust 1.75+ (stable)
- Docker (optional, for emulator)
- Node.js 18+ (optional, for frontend tooling)

### Clone and Build

```bash
git clone https://github.com/avilaops/arxis
cd arxis/avl-console
cargo build
```

### Run Development Server

```bash
# With hot-reload (using cargo-watch)
cargo install cargo-watch
cargo watch -x 'run --example basic'

# Or manually
cargo run --example basic
```

### Run Tests

```bash
# All tests
cargo test

# With coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Specific test
cargo test test_console_creation
```

## 🎨 Frontend Development

The console uses **server-side rendering** with Askama templates and vanilla JavaScript for interactivity.

### Adding a New Page

1. **Create template** in `templates/`:

```html
<!-- templates/my_page.html -->
<!DOCTYPE html>
<html>
<head>
    <title>My Page</title>
</head>
<body>
    <h1>{{ title }}</h1>
</body>
</html>
```

2. **Define struct** in your module:

```rust
use askama::Template;

#[derive(Template)]
#[template(path = "my_page.html")]
struct MyPageTemplate {
    title: String,
}
```

3. **Add route handler**:

```rust
async fn my_page() -> Result<Html<String>> {
    let template = MyPageTemplate {
        title: "My Page".to_string(),
    };
    Ok(Html(template.render()?))
}
```

### Styling Guidelines

- Use **inline CSS** or add to `static/styles.css`
- Dark theme: `#0a0e1a` (background), `#00d4ff` (accent)
- Responsive design with mobile-first approach
- Use system fonts: `-apple-system, BlinkMacSystemFont, 'Segoe UI'`

## 🔌 Adding New API Endpoints

### 1. Define Request/Response Types

```rust
#[derive(Deserialize)]
struct CreateResourceRequest {
    name: String,
    config: serde_json::Value,
}

#[derive(Serialize)]
struct CreateResourceResponse {
    id: String,
    name: String,
    created_at: String,
}
```

### 2. Implement Handler

```rust
async fn create_resource(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateResourceRequest>,
) -> Result<Json<CreateResourceResponse>> {
    // Validate input
    if req.name.is_empty() {
        return Err(ConsoleError::InvalidInput("Name required".into()));
    }

    // TODO: Create resource

    Ok(Json(CreateResourceResponse {
        id: "res_123".to_string(),
        name: req.name,
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}
```

### 3. Register Route

```rust
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/resources", post(create_resource))
        .with_state(state)
}
```

## 🔐 Authentication Flow

### Session Management

```rust
// Create session
let session_id = generate_session_id();
state.store_session(session_id.clone(), user_id.clone()).await;

// Validate session
if let Some(user_id) = state.get_session(&session_id).await {
    // Authenticated
}

// Remove session
state.remove_session(&session_id).await;
```

### Adding Protected Routes

Routes are automatically protected by the `AuthLayer` middleware. To bypass auth:

```rust
// In middleware/auth.rs
if path.starts_with("/static") || path == "/login" || path == "/public" {
    return Ok(next.run(req).await);
}
```

## 🌐 WebSocket Development

### Sending Real-Time Updates

```rust
// In your handler
let message = WsMessage {
    msg_type: "update".to_string(),
    payload: Some(serde_json::to_string(&data)?),
};

// Send to specific user (implement broadcast mechanism)
send_to_user(&user_id, message).await?;
```

### Client-Side Connection

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    console.log('Received:', msg);
};

ws.send(JSON.stringify({
    type: 'subscribe',
    payload: 'metrics'
}));
```

## 📊 Adding Metrics

### Define Metric

```rust
#[derive(Serialize)]
struct CustomMetric {
    name: String,
    value: f64,
    timestamp: u64,
}
```

### Update Dashboard

```rust
async fn get_custom_metrics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CustomMetric>>> {
    // TODO: Query metrics service
    Ok(Json(vec![]))
}
```

## 🧪 Testing Best Practices

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feature() {
        let config = ConsoleConfig::default();
        let console = Console::new(config).await.unwrap();
        // Test logic
    }
}
```

### Integration Tests

```rust
// tests/integration_tests.rs
#[tokio::test]
async fn test_api_endpoint() {
    let config = ConsoleConfig::default();
    let console = Console::new(config).await.unwrap();
    let app = console.router();

    // Use axum-test or similar for HTTP testing
}
```

## 🐛 Debugging

### Enable Debug Logging

```bash
RUST_LOG=avl_console=debug,tower_http=debug cargo run --example basic
```

### Inspect State

```rust
tracing::debug!("Current sessions: {:?}", state.sessions.read().await);
tracing::debug!("WS connections: {:?}", state.ws_connections.read().await);
```

### Performance Profiling

```bash
cargo install cargo-flamegraph
cargo flamegraph --example basic
```

## 📦 Release Process

### Version Bump

```bash
# Update Cargo.toml version
vim Cargo.toml

# Update CHANGELOG.md
vim CHANGELOG.md

# Commit
git add .
git commit -m "Release v0.2.0"
git tag v0.2.0
git push origin main --tags
```

### Build Release

```bash
cargo build --release
strip target/release/avl-console
```

### Publish to crates.io

```bash
cargo login
cargo publish --dry-run
cargo publish
```

## 🤝 Contributing Guidelines

1. **Fork** the repository
2. Create a **feature branch**: `git checkout -b feature/amazing`
3. **Write tests** for new features
4. Ensure **all tests pass**: `cargo test`
5. **Format code**: `cargo fmt`
6. **Lint**: `cargo clippy -- -D warnings`
7. **Commit** with clear message: `git commit -am 'Add amazing feature'`
8. **Push**: `git push origin feature/amazing`
9. Open a **Pull Request**

### Code Style

- Follow Rust API guidelines
- Use `rustfmt` with default settings
- Document public APIs with doc comments
- Keep functions under 50 lines when possible
- Use meaningful variable names

### Commit Messages

```
feat: Add new dashboard widget
fix: Correct WebSocket connection handling
docs: Update API documentation
test: Add integration tests for auth
refactor: Simplify error handling
```

## 📚 Resources

- [Axum Documentation](https://docs.rs/axum)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Askama Templates](https://docs.rs/askama)
- [AVL Cloud Docs](https://docs.avila.cloud)

---

Happy coding! 🚀
