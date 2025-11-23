//! Authentication module

pub mod jwt;
pub mod api_key;

pub use jwt::{JwtAuth, JwtConfig};
pub use api_key::ApiKeyAuth;

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower::{Layer, Service};

/// Authentication layer
#[derive(Clone)]
pub struct AuthLayer {
    auth_type: AuthType,
}

/// Authentication type
#[derive(Clone)]
pub enum AuthType {
    /// JWT authentication
    Jwt(JwtAuth),

    /// API key authentication
    ApiKey(ApiKeyAuth),
}

impl AuthLayer {
    /// Create JWT authentication layer
    pub fn jwt(config: JwtConfig) -> Self {
        Self {
            auth_type: AuthType::Jwt(JwtAuth::new(config)),
        }
    }

    /// Create API key authentication layer
    pub fn api_key(keys: Vec<String>) -> Self {
        Self {
            auth_type: AuthType::ApiKey(ApiKeyAuth::new(keys)),
        }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            auth_type: self.auth_type.clone(),
        }
    }
}

/// Authentication middleware
#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    auth_type: AuthType,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
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
        let auth_result = match &self.auth_type {
            AuthType::Jwt(jwt_auth) => jwt_auth.validate(&req),
            AuthType::ApiKey(api_key_auth) => api_key_auth.validate(&req),
        };

        let mut inner = self.inner.clone();

        Box::pin(async move {
            if auth_result {
                inner.call(req).await
            } else {
                Ok((
                    StatusCode::UNAUTHORIZED,
                    "Authentication failed",
                ).into_response())
            }
        })
    }
}
