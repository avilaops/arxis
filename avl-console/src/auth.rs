//! Authentication module

use crate::{
    error::{ConsoleError, Result},
    state::AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(current_user))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    session_id: String,
    user: UserInfo,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    // TODO: Call avl-auth service to validate credentials
    // For now, mock authentication
    if req.username == "admin" && req.password == "admin" {
        let user = UserInfo {
            id: "user_001".to_string(),
            username: req.username.clone(),
            email: format!("{}@avila.cloud", req.username),
            role: "admin".to_string(),
        };

        let session_id = generate_session_id();
        state.store_session(session_id.clone(), user.id.clone()).await;

        let response = LoginResponse {
            session_id: session_id.clone(),
            user,
        };

        // Set session cookie
        let cookie = format!(
            "avl_session={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400",
            session_id
        );

        Ok((
            StatusCode::OK,
            [("Set-Cookie", cookie)],
            Json(response),
        ))
    } else {
        Err(ConsoleError::Authentication(
            "Invalid credentials".to_string(),
        ))
    }
}

async fn logout(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // TODO: Extract session from cookie and remove it
    let cookie = "avl_session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0";

    (StatusCode::OK, [("Set-Cookie", cookie)], "Logged out")
}

async fn current_user(State(_state): State<Arc<AppState>>) -> Result<Json<UserInfo>> {
    // TODO: Extract user from session
    Ok(Json(UserInfo {
        id: "user_001".to_string(),
        username: "admin".to_string(),
        email: "admin@avila.cloud".to_string(),
        role: "admin".to_string(),
    }))
}

fn generate_session_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("sess_{:x}", timestamp)
}
