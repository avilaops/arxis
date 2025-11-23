//! # AVL Console - Developer Portal & Web Dashboard
//!
//! **World-class developer portal** for AVL Cloud Platform.
//!
//! ## Features
//!
//! - **Dashboard**: Real-time resource overview with WebSocket updates
//! - **AvilaDB Explorer**: Interactive query editor with syntax highlighting
//! - **Storage Browser**: Full S3-compatible file management
//! - **Observability**: Metrics, logs, traces with advanced filtering
//! - **Billing**: Cost tracking, usage analytics, budget alerts
//! - **API Explorer**: Interactive API testing with OpenAPI specs
//! - **User Management**: Teams, permissions, audit logs
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │              AVL Console Frontend               │
//! │  (Server-Side Rendered + Real-Time WebSocket)   │
//! └─────────────────────────────────────────────────┘
//!                        ↓
//! ┌─────────────────────────────────────────────────┐
//! │              Axum REST API Layer                │
//! │  (Authentication, Rate Limiting, CORS)          │
//! └─────────────────────────────────────────────────┘
//!                        ↓
//! ┌──────────────┬──────────────┬──────────────────┐
//! │   AvilaDB    │   Storage    │   Observability  │
//! │   Explorer   │   Browser    │   Dashboard      │
//! └──────────────┴──────────────┴──────────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avl_console::{Console, ConsoleConfig};
//! use std::net::SocketAddr;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = ConsoleConfig::from_env()?;
//!     let console = Console::new(config).await?;
//!     let addr: SocketAddr = "127.0.0.1:8080".parse()?;
//!
//!     console.serve(addr).await?;
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod auth;
pub mod config;
pub mod config_production;
pub mod dashboard;
pub mod database;
pub mod error;
pub mod middleware;
pub mod observability;
pub mod storage;
pub mod billing;
pub mod templates;
pub mod websocket;
pub mod state;
pub mod query_builder;
pub mod query_history;
pub mod monitoring;
pub mod teams;
pub mod ai_assistant;
pub mod ai_engine;
pub mod ai_metrics;
pub mod embeddings;
pub mod query_safety;
pub mod rate_limiter;
pub mod vector_persistence;
pub mod streaming;
pub mod ml;
pub mod ml_persistence;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
};

pub use config::ConsoleConfig;
pub use error::{ConsoleError, Result};
pub use state::AppState;

/// AVL Console - Main application structure
///
/// Manages the entire console lifecycle including:
/// - HTTP server with Axum
/// - WebSocket connections for real-time updates
/// - Authentication and authorization
/// - Resource management (databases, storage, etc.)
#[derive(Clone)]
pub struct Console {
    state: Arc<AppState>,
    config: ConsoleConfig,
}

impl Console {
    /// Create a new Console instance
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use avl_console::{Console, ConsoleConfig};
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = ConsoleConfig::default();
    /// let console = Console::new(config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(config: ConsoleConfig) -> Result<Self> {
        let state = Arc::new(AppState::new(&config).await?);

        Ok(Self { state, config })
    }

    /// Build the Axum router with all routes and middleware
    pub fn router(&self) -> Router {
        Router::new()
            // API routes
            .nest("/api", api::routes(self.state.clone()))
            // Dashboard routes
            .nest("/dashboard", dashboard::routes(self.state.clone()))
            // Database explorer
            .nest("/databases", database::routes(self.state.clone()))
            // Storage browser
            .nest("/storage", storage::routes(self.state.clone()))
            // Observability
            .nest("/observability", observability::routes(self.state.clone()))
            // Billing
            .nest("/billing", billing::routes(self.state.clone()))
            // Query Builder
            .nest("/query-builder", query_builder::router(self.state.clone()))
            // Advanced Monitoring
            .nest("/monitoring", monitoring::router(self.state.clone()))
            // Team Management & RBAC
            .nest("/teams", teams::router(self.state.clone()))
            // AI Assistant
            .nest("/ai-assistant", ai_assistant::router(self.state.clone()))
            // Machine Learning (Avila ML Integration)
            .merge(ml::ml_routes().with_state(self.state.clone()))
            // WebSocket endpoint
            .nest("/ws", websocket::routes(self.state.clone()))
            // Static files
            .nest_service("/static", ServeDir::new("static"))
            // Root redirect to dashboard
            .fallback(dashboard::index)
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CompressionLayer::new())
                    .layer(CorsLayer::permissive())
                    .layer(middleware::auth::AuthLayer::new(self.state.clone()))
                    .layer(middleware::rate_limit::RateLimitLayer::new())
            )
    }

    /// Start the console server
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use avl_console::{Console, ConsoleConfig};
    /// # use std::net::SocketAddr;
    /// # async fn example() -> anyhow::Result<()> {
    /// let console = Console::new(ConsoleConfig::default()).await?;
    /// let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    /// console.serve(addr).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn serve(self, addr: impl Into<SocketAddr>) -> Result<()> {
        let addr = addr.into();
        let router = self.router();

        tracing::info!("🖥️  AVL Console starting on http://{}", addr);
        tracing::info!("📊 Dashboard: http://{}/dashboard", addr);
        tracing::info!("🗄️  AvilaDB Explorer: http://{}/databases", addr);
        tracing::info!("💾 Storage Browser: http://{}/storage", addr);
        tracing::info!("📈 Observability: http://{}/observability", addr);
        tracing::info!("🎨 Query Builder: http://{}/query-builder", addr);
        tracing::info!("🔬 Advanced Monitoring: http://{}/monitoring", addr);
        tracing::info!("👥 Team Management: http://{}/teams", addr);
        tracing::info!("🤖 AI Assistant: http://{}/ai-assistant", addr);
        tracing::info!("🧠 Machine Learning: http://{}/ml", addr);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| ConsoleError::Server(e.to_string()))?;

        axum::serve(listener, router)
            .await
            .map_err(|e| ConsoleError::Server(e.to_string()))?;

        Ok(())
    }

    /// Get reference to application state
    pub fn state(&self) -> &Arc<AppState> {
        &self.state
    }

    /// Get configuration
    pub fn config(&self) -> &ConsoleConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_console_creation() {
        let config = ConsoleConfig::default();
        let console = Console::new(config).await;
        assert!(console.is_ok());
    }
}
