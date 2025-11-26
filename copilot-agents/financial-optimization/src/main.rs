use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod portugal_tax;
mod vat_optimizer;
mod optimization;
mod financial_models;
mod corporate_structure;
mod simulators;
mod api;
mod errors;

use api::*;
use errors::AppError;

#[derive(Clone)]
pub struct AppState {
    // Add database pool or other shared state here if needed
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "financial_optimization_agent=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let state = Arc::new(AppState {});

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))

        // Tax Optimization
        .route("/api/tax/portugal/irc", post(calculate_irc))
        .route("/api/tax/portugal/sifide", post(calculate_sifide))
        .route("/api/tax/portugal/patent-box", post(calculate_patent_box))
        .route("/api/tax/effective-rate", post(calculate_effective_tax_rate))

        // VAT Optimization
        .route("/api/vat/cross-border", post(optimize_cross_border_vat))
        .route("/api/vat/recovery", post(optimize_vat_recovery))

        // Cost Optimization
        .route("/api/optimization/linear-programming", post(linear_programming_allocation))
        .route("/api/optimization/break-even", post(break_even_analysis))

        // Financial Models
        .route("/api/valuation/dcf", post(dcf_valuation))
        .route("/api/valuation/npv", post(npv_calculation))
        .route("/api/valuation/irr", post(irr_calculation))

        // Corporate Structure
        .route("/api/structure/optimize", post(optimize_corporate_structure))
        .route("/api/structure/evaluate", post(evaluate_structure))

        // Simulation & Forecasting
        .route("/api/simulation/monte-carlo", post(monte_carlo_simulation))
        .route("/api/forecast/revenue", post(revenue_forecast))
        .route("/api/sensitivity/analysis", post(sensitivity_analysis))

        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    info!("🚀 Financial Optimization Agent listening on {}", listener.local_addr().unwrap());
    info!("📊 Specialized in: Tax Optimization, Cost Minimization, ROI Analysis");
    info!("🇵🇹 Portugal Tax System: IRC, SIFIDE, Patent Box, VAT");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Financial Optimization Specialist Agent - AVL Cloud Platform"
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "financial-optimization-agent",
        "version": env!("CARGO_PKG_VERSION"),
        "capabilities": [
            "Tax Optimization (Portugal)",
            "VAT Optimization",
            "Linear Programming",
            "Monte Carlo Simulation",
            "DCF Valuation",
            "Corporate Structure Optimization",
            "Break-Even Analysis",
            "Sensitivity Analysis"
        ]
    }))
}
