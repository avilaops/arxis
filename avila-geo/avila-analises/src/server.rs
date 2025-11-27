mod models;
mod storage;
mod api;
mod tracker;
mod funnel;
mod cohort;
mod segmentation;
mod prediction;
mod dashboard;
mod websocket;
mod export;
mod industry40;
mod industry40;

use std::sync::Arc;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .json()
        .init();

    tracing::info!("🚀 Starting Avila Analytics API Server");

    // Configuração
    let config = api::state::AppConfig::default();

    // Inicializar storage (usar InMemory por padrão, trocar para AvilaDB em produção)
    let use_aviladb = std::env::var("USE_AVILADB").unwrap_or_else(|_| "false".to_string()) == "true";

    let event_store: Arc<dyn storage::EventStore> = if use_aviladb {
        tracing::info!("📦 Initializing AvilaDB storage...");
        let aviladb_config = storage::aviladb_store::AvilaDBConfig {
            endpoint: std::env::var("AVILADB_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            account_key: std::env::var("AVILADB_KEY")
                .unwrap_or_else(|_| "development-key".to_string()),
            database_name: std::env::var("AVILADB_DATABASE")
                .unwrap_or_else(|_| "analytics".to_string()),
            collection_name: std::env::var("AVILADB_COLLECTION")
                .unwrap_or_else(|_| "events".to_string()),
            throughput_units: std::env::var("AVILADB_THROUGHPUT")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        };

        let store = storage::AvilaDBStore::new(aviladb_config);

        // Inicializar database e collection
        if let Err(e) = store.initialize().await {
            tracing::error!("Failed to initialize AvilaDB: {}", e);
            return Err(anyhow::anyhow!("AvilaDB initialization failed: {}", e));
        }

        Arc::new(store)
    } else {
        tracing::info!("💾 Using in-memory storage (development mode)");
        Arc::new(storage::InMemoryStore::new())
    };

    // Criar estado da aplicação
    let app_state = api::AppState::new(event_store.clone(), config.clone());

    // Criar router
    let app = api::create_router(app_state);

    // Configurar endereço do servidor
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = format!("{}:{}", host, port);
    tracing::info!("🌐 Server listening on http://{}", addr);
    tracing::info!("📊 API version: {}", config.api_version);
    tracing::info!("🔐 API Key authentication enabled");

    // Banner
    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║   AVILA ANALYTICS API SERVER                         ║");
    println!("║   Powered by AvilaDB - High Performance Analytics    ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");
    println!("📍 Endpoints:");
    println!("   GET  /health              - Health check");
    println!("   GET  /metrics             - System metrics");
    println!("   POST /api/v1/events       - Ingest single event");
    println!("   POST /api/v1/events/batch - Ingest batch events");
    println!("   POST /api/v1/analytics/funnel  - Analyze funnel");
    println!("   POST /api/v1/analytics/cohort  - Analyze cohort");
    println!("   GET  /api/v1/users/:id    - Get user profile");
    println!("\n🔑 Use header: x-api-key: dev-key-123\n");

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Função auxiliar para gerar perfis (re-exportada de outro módulo)
pub fn generate_user_profiles(events: &[models::BehaviorEvent]) -> Vec<models::UserProfile> {
    use chrono::Utc;
    use std::collections::HashMap;

    let mut user_events: HashMap<String, Vec<&models::BehaviorEvent>> = HashMap::new();

    for event in events {
        user_events
            .entry(event.user_id.clone())
            .or_insert_with(Vec::new)
            .push(event);
    }

    user_events
        .iter()
        .map(|(user_id, user_events)| {
            let first_seen = user_events
                .iter()
                .map(|e| e.timestamp)
                .min()
                .unwrap_or_else(Utc::now);

            let last_seen = user_events
                .iter()
                .map(|e| e.timestamp)
                .max()
                .unwrap_or_else(Utc::now);

            let total_purchases = user_events
                .iter()
                .filter(|e| matches!(e.event_type, models::EventType::Purchase { .. }))
                .count();

            let total_spent: f64 = user_events
                .iter()
                .filter_map(|e| {
                    if let models::EventType::Purchase { amount, .. } = e.event_type {
                        Some(amount)
                    } else {
                        None
                    }
                })
                .sum();

            let days_since_last_purchase = if total_purchases > 0 {
                Some((Utc::now() - last_seen).num_days())
            } else {
                None
            };

            let engagement_score = (user_events.len() as f64 / 10.0).min(1.0);
            let churn_risk = if let Some(days) = days_since_last_purchase {
                (days as f64 / 90.0).min(1.0)
            } else {
                0.5
            };

            models::UserProfile {
                user_id: user_id.clone(),
                first_seen,
                last_seen,
                total_sessions: user_events
                    .iter()
                    .map(|e| &e.session_id)
                    .collect::<std::collections::HashSet<_>>()
                    .len(),
                total_events: user_events.len(),
                behaviors: models::UserBehaviors {
                    avg_session_duration_seconds: 120.0,
                    avg_pages_per_session: 3.5,
                    bounce_rate: 0.3,
                    conversion_rate: if total_purchases > 0 { 0.5 } else { 0.0 },
                    most_active_hours: vec![],
                    most_active_days: vec![],
                    total_purchases,
                    total_spent,
                    avg_order_value: if total_purchases > 0 {
                        total_spent / total_purchases as f64
                    } else {
                        0.0
                    },
                    days_since_last_purchase,
                    pages_viewed: std::collections::HashSet::new(),
                    search_queries: vec![],
                    clicked_products: vec![],
                },
                segments: vec![],
                engagement_score,
                loyalty_score: if total_purchases > 2 { 0.8 } else { 0.4 },
                conversion_probability: engagement_score * 0.7,
                churn_risk,
                interests: vec![],
                preferred_categories: HashMap::new(),
                browsing_patterns: models::BrowsingPatterns::default(),
            }
        })
        .collect()
}
