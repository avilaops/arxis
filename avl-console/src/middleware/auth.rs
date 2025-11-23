//! Authentication middleware

use crate::{error::ConsoleError, state::AppState};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tower::{Layer, Service};

/// Authentication layer
#[derive(Clone)]
pub struct AuthLayer {
    state: Arc<AppState>,
}

impl AuthLayer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    state: Arc<AppState>,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let state = self.state.clone();

        // Extract path before moving req
        let path = req.uri().path().to_string();        // Extract session cookie before moving req
        let cookies = req
            .headers()
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let future = self.inner.call(req);

        Box::pin(async move {
            if path.starts_with("/static") || path == "/login" || path == "/health" {
                return future.await;
            }

            let session_id = extract_session_id(&cookies);

            if let Some(sid) = session_id {
                if let Some(_user_id) = state.get_session(&sid).await {
                    // User is authenticated
                    return future.await;
                }
            }

            // Not authenticated - return 401
            Ok(ConsoleError::Authentication("Session expired or invalid".to_string())
                .into_response())
        })
    }
}

fn extract_session_id(cookies: &str) -> Option<String> {
    cookies
        .split(';')
        .find_map(|cookie| {
            let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == "avl_session" {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
}

/// Extract authenticated user from request
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    let path = req.uri().path();
    if path.starts_with("/static") || path == "/login" || path == "/health" {
        return Ok(next.run(req).await);
    }

    // Extract session cookie
    let cookies = req
        .headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let session_id = extract_session_id(cookies);

    if let Some(sid) = session_id {
        if let Some(_user_id) = state.get_session(&sid).await {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_session_id() {
        let cookies = "avl_session=abc123; other=value";
        assert_eq!(extract_session_id(cookies), Some("abc123".to_string()));

        let cookies = "other=value";
        assert_eq!(extract_session_id(cookies), None);
    }
}
