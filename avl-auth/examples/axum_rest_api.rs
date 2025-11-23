//! Example: REST API with Axum and AVL Auth
//!
//! This example demonstrates how to build a secure REST API using Axum
//! with AVL Auth for authentication and authorization.

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use avl_auth::{AuthClient, Config, Credentials};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

// ==================== Application State ====================

#[derive(Clone)]
struct AppState {
    auth: Arc<AuthClient>,
}

// ==================== Request/Response Types ====================

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
    device_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: String,
    email: String,
    roles: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

// ==================== Auth Middleware ====================

async fn extract_token(headers: &HeaderMap) -> Result<String, AuthError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AuthError::Unauthorized("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::Unauthorized("Invalid authorization header".to_string()));
    }

    Ok(auth_header[7..].to_string())
}

// ==================== Handlers ====================

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AuthError> {
    let user_id = state.auth
        .register(req.email.clone(), req.password)
        .await?;

    Ok(Json(UserResponse {
        id: user_id.to_string(),
        email: req.email,
        roles: vec!["user".to_string()],
    }))
}

async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    // Extract IP address from headers (behind proxy)
    let ip_address = headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse().ok());

    let credentials = Credentials {
        email: req.email,
        password: req.password,
        device_id: req.device_id,
        ip_address,
    };

    let session = state.auth.login(credentials).await?;

    let expires_in = (session.expires_at - chrono::Utc::now()).num_seconds();

    Ok(Json(AuthResponse {
        access_token: session.access_token,
        refresh_token: session.refresh_token,
        token_type: session.token_type,
        expires_in,
    }))
}

async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, AuthError> {
    let token = extract_token(&headers).await?;
    let claims = state.auth.verify_token(&token).await?;

    state.auth.logout(&claims.session_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, AuthError> {
    let token = extract_token(&headers).await?;
    let claims = state.auth.verify_token(&token).await?;

    Ok(Json(UserResponse {
        id: claims.sub.to_string(),
        email: claims.email,
        roles: claims.roles,
    }))
}

async fn protected_resource(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(resource_id): Path<String>,
) -> Result<Json<serde_json::Value>, AuthError> {
    let token = extract_token(&headers).await?;
    let claims = state.auth.verify_token(&token).await?;

    // Check if user has permission to access this resource
    // This is a simplified example - in production, load user from DB
    // and use permission manager for fine-grained access control

    Ok(Json(serde_json::json!({
        "resource_id": resource_id,
        "accessed_by": claims.email,
        "data": "This is protected data"
    })))
}

async fn health_check() -> &'static str {
    "OK"
}

// ==================== Error Handling ====================

#[derive(Debug)]
enum AuthError {
    Auth(avl_auth::AuthError),
    Unauthorized(String),
}

impl From<avl_auth::AuthError> for AuthError {
    fn from(err: avl_auth::AuthError) -> Self {
        AuthError::Auth(err)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::Auth(err) => {
                let status = match err.status_code() {
                    401 => StatusCode::UNAUTHORIZED,
                    403 => StatusCode::FORBIDDEN,
                    404 => StatusCode::NOT_FOUND,
                    409 => StatusCode::CONFLICT,
                    429 => StatusCode::TOO_MANY_REQUESTS,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (status, err.to_string())
            }
            AuthError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        };

        let body = Json(ErrorResponse {
            error: status.canonical_reason().unwrap_or("Unknown").to_string(),
            message,
        });

        (status, body).into_response()
    }
}

// ==================== Main ====================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🔐 Starting AVL Auth REST API Example...\n");

    // Setup auth client
    let mut config = Config::default();

    let crypto = avl_auth::crypto::CryptoManager::new();
    let (private_key, public_key) = crypto.generate_rsa_keypair(2048)?;

    config.jwt.private_key = private_key;
    config.jwt.public_key = public_key;
    config.jwt.algorithm = "RS256".to_string();

    let auth = Arc::new(AuthClient::new(config).await?);

    let state = AppState { auth };

    // Build router
    let app = Router::new()
        // Public routes
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))

        // Protected routes
        .route("/auth/logout", post(logout))
        .route("/auth/profile", get(get_profile))
        .route("/api/resource/:id", get(protected_resource))

        // Add state
        .with_state(state)

        // Add CORS
        .layer(CorsLayer::permissive());

    let addr = "127.0.0.1:3000";
    println!("🚀 Server listening on http://{}\n", addr);
    println!("📝 API Endpoints:");
    println!("   POST   /auth/register   - Register new user");
    println!("   POST   /auth/login      - Login");
    println!("   POST   /auth/logout     - Logout (requires token)");
    println!("   GET    /auth/profile    - Get user profile (requires token)");
    println!("   GET    /api/resource/:id - Access protected resource (requires token)");
    println!("   GET    /health          - Health check\n");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ==================== Usage Example ====================
//
// # Register a new user
// curl -X POST http://localhost:3000/auth/register \
//   -H "Content-Type: application/json" \
//   -d '{"email":"user@example.com","password":"SecureP@ss123!"}'
//
// # Login
// curl -X POST http://localhost:3000/auth/login \
//   -H "Content-Type: application/json" \
//   -d '{"email":"user@example.com","password":"SecureP@ss123!"}'
//
// # Get profile (use token from login response)
// curl http://localhost:3000/auth/profile \
//   -H "Authorization: Bearer <your-token>"
//
// # Access protected resource
// curl http://localhost:3000/api/resource/123 \
//   -H "Authorization: Bearer <your-token>"
//
// # Logout
// curl -X POST http://localhost:3000/auth/logout \
//   -H "Authorization: Bearer <your-token>"
