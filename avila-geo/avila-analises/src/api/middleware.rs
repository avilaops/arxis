use axum::{
    extract::Request,
    http::{header, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_http::cors::CorsLayer;
use tracing::{debug, warn};
use std::time::Instant;

/// Middleware de autenticação via API key
pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extrair API key do header
    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok());

    // Validar API key (em produção, verificar contra banco/cache)
    match api_key {
        Some(key) if is_valid_api_key(key) => {
            debug!("Request authenticated with API key");
            Ok(next.run(req).await)
        }
        Some(_) => {
            warn!("Invalid API key provided");
            Err(StatusCode::UNAUTHORIZED)
        }
        None => {
            warn!("No API key provided");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn is_valid_api_key(key: &str) -> bool {
    // Em produção: validar contra AvilaDB ou Redis
    key == "dev-key-123" || key.starts_with("avila_")
}

/// Middleware de logging de requisições
pub async fn request_logger_middleware(
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let latency = start.elapsed();
    let status = response.status();

    debug!(
        "{} {} - {} - {:?}",
        method,
        uri,
        status.as_u16(),
        latency
    );

    response
}

/// Criar layer de CORS
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::HeaderName::from_static("x-api-key"),
        ])
        .max_age(std::time::Duration::from_secs(3600))
}

/// Middleware de rate limiting (simplificado)
pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implementar rate limiting real com Redis ou Governor
    // Por enquanto, apenas passa através
    Ok(next.run(req).await)
}

/// Middleware para adicionar headers de segurança
pub async fn security_headers_middleware(
    req: Request,
    next: Next,
) -> Response {
    let mut response = next.run(req).await;

    let headers = response.headers_mut();
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );

    response
}

/// Middleware de compressão (já fornecido por tower-http)
/// Configurar via CompressionLayer no router
