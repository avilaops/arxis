//! Dashboard routes and handlers

use crate::{
    error::Result,
    state::{AppState, DashboardMetrics},
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(dashboard_page))
        .route("/metrics", get(get_metrics))
        .with_state(state)
}

pub async fn index() -> impl IntoResponse {
    Html(DASHBOARD_HTML)
}

async fn dashboard_page() -> impl IntoResponse {
    Html(DASHBOARD_HTML)
}

#[derive(Serialize)]
struct MetricsResponse {
    databases: usize,
    storage_buckets: usize,
    storage_size_gb: f64,
    active_connections: usize,
    requests_per_minute: u32,
    uptime_seconds: u64,
}

async fn get_metrics(State(state): State<Arc<AppState>>) -> Result<Json<MetricsResponse>> {
    // Try to get cached metrics
    if let Some(cached) = state.get_metrics().await {
        let age = cached.last_updated.elapsed().as_secs();
        if age < 60 {
            return Ok(Json(MetricsResponse {
                databases: cached.database_count,
                storage_buckets: cached.storage_buckets,
                storage_size_gb: cached.storage_size_bytes as f64 / 1_000_000_000.0,
                active_connections: cached.active_connections,
                requests_per_minute: cached.requests_per_minute,
                uptime_seconds: age,
            }));
        }
    }

    // Fetch fresh metrics
    // TODO: Query actual services
    let metrics = DashboardMetrics {
        database_count: 3,
        storage_buckets: 15,
        storage_size_bytes: 128_000_000_000,
        active_connections: 42,
        requests_per_minute: 1250,
        last_updated: std::time::Instant::now(),
    };

    state.update_metrics(metrics.clone()).await;

    Ok(Json(MetricsResponse {
        databases: metrics.database_count,
        storage_buckets: metrics.storage_buckets,
        storage_size_gb: metrics.storage_size_bytes as f64 / 1_000_000_000.0,
        active_connections: metrics.active_connections,
        requests_per_minute: metrics.requests_per_minute,
        uptime_seconds: 0,
    }))
}

const DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AVL Console - Dashboard</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: #0a0e1a;
            color: #e0e6ed;
            line-height: 1.6;
        }
        .header {
            background: #0f1419;
            border-bottom: 1px solid #1a1f2e;
            padding: 1rem 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .logo { font-size: 1.5rem; font-weight: bold; color: #00d4ff; }
        .container { max-width: 1400px; margin: 2rem auto; padding: 0 2rem; }
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }
        .metric-card {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
            transition: transform 0.2s;
        }
        .metric-card:hover { transform: translateY(-4px); }
        .metric-label {
            font-size: 0.875rem;
            color: #8b92a0;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .metric-value {
            font-size: 2rem;
            font-weight: bold;
            color: #00d4ff;
            margin-top: 0.5rem;
        }
        .section {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 2rem;
            margin-bottom: 2rem;
        }
        .section-title {
            font-size: 1.25rem;
            margin-bottom: 1rem;
            color: #00d4ff;
        }
        .activity-item {
            padding: 1rem;
            border-bottom: 1px solid #1a1f2e;
        }
        .activity-item:last-child { border-bottom: none; }
        .status-indicator {
            display: inline-block;
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background: #00ff88;
            margin-right: 0.5rem;
        }
    </style>
</head>
<body>
    <div class="header">
        <div class="logo">🖥️ AVL Console</div>
        <div>admin@avila.cloud</div>
    </div>

    <div class="container">
        <h1 style="margin-bottom: 2rem;">Dashboard</h1>

        <div class="metrics-grid" id="metrics">
            <div class="metric-card">
                <div class="metric-label">Bancos de Dados</div>
                <div class="metric-value" id="databases">-</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Storage Buckets</div>
                <div class="metric-value" id="buckets">-</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Storage (GB)</div>
                <div class="metric-value" id="storage">-</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Conexões Ativas</div>
                <div class="metric-value" id="connections">-</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Requisições/min</div>
                <div class="metric-value" id="requests">-</div>
            </div>
        </div>

        <div class="section">
            <h2 class="section-title">Atividade Recente</h2>
            <div class="activity-item">
                <span class="status-indicator"></span>
                user123 criou banco de dados "prod"
            </div>
            <div class="activity-item">
                <span class="status-indicator"></span>
                api-key-xyz fez upload de 15 arquivos
            </div>
            <div class="activity-item">
                <span class="status-indicator"></span>
                Fila "events" processou 50K mensagens
            </div>
        </div>

        <div class="section">
            <h2 class="section-title">Status dos Serviços</h2>
            <div class="activity-item">
                <span class="status-indicator"></span>
                AvilaDB: Operacional
            </div>
            <div class="activity-item">
                <span class="status-indicator"></span>
                Storage: Operacional
            </div>
            <div class="activity-item">
                <span class="status-indicator"></span>
                Observability: Operacional
            </div>
        </div>
    </div>

    <script>
        async function loadMetrics() {
            try {
                const res = await fetch('/dashboard/metrics');
                const data = await res.json();
                document.getElementById('databases').textContent = data.databases;
                document.getElementById('buckets').textContent = data.storage_buckets;
                document.getElementById('storage').textContent = data.storage_size_gb.toFixed(1);
                document.getElementById('connections').textContent = data.active_connections;
                document.getElementById('requests').textContent = data.requests_per_minute.toLocaleString();
            } catch (err) {
                console.error('Failed to load metrics:', err);
            }
        }

        loadMetrics();
        setInterval(loadMetrics, 5000);
    </script>
</body>
</html>"#;
