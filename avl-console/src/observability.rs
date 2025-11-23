//! Observability - Metrics, logs, and traces

use crate::{error::Result, state::AppState};
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
        .route("/", get(observability_page))
        .route("/metrics", get(get_metrics))
        .route("/logs", get(get_logs))
        .with_state(state)
}

async fn observability_page() -> impl IntoResponse {
    Html(OBSERVABILITY_HTML)
}

#[derive(Serialize)]
struct MetricPoint {
    timestamp: u64,
    value: f64,
}

#[derive(Serialize)]
struct MetricsData {
    cpu_usage: Vec<MetricPoint>,
    memory_usage: Vec<MetricPoint>,
    request_rate: Vec<MetricPoint>,
    error_rate: Vec<MetricPoint>,
}

async fn get_metrics(State(_state): State<Arc<AppState>>) -> Result<Json<MetricsData>> {
    // TODO: Query actual metrics from observability service
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut cpu_usage = Vec::new();
    let mut memory_usage = Vec::new();
    let mut request_rate = Vec::new();
    let mut error_rate = Vec::new();

    for i in 0..20 {
        let ts = now - (20 - i) * 60;
        cpu_usage.push(MetricPoint {
            timestamp: ts,
            value: 30.0 + (i as f64 * 2.5).sin() * 20.0,
        });
        memory_usage.push(MetricPoint {
            timestamp: ts,
            value: 60.0 + (i as f64 * 1.5).cos() * 15.0,
        });
        request_rate.push(MetricPoint {
            timestamp: ts,
            value: 1000.0 + (i as f64 * 3.0).sin() * 300.0,
        });
        error_rate.push(MetricPoint {
            timestamp: ts,
            value: 0.5 + (i as f64 * 0.5).sin().abs() * 1.5,
        });
    }

    Ok(Json(MetricsData {
        cpu_usage,
        memory_usage,
        request_rate,
        error_rate,
    }))
}

#[derive(Serialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
    service: String,
}

async fn get_logs(State(_state): State<Arc<AppState>>) -> Result<Json<Vec<LogEntry>>> {
    // TODO: Query actual logs
    let logs = vec![
        LogEntry {
            timestamp: "2024-11-23T15:30:01Z".to_string(),
            level: "INFO".to_string(),
            message: "Request processed successfully".to_string(),
            service: "aviladb".to_string(),
        },
        LogEntry {
            timestamp: "2024-11-23T15:30:05Z".to_string(),
            level: "WARN".to_string(),
            message: "High memory usage detected".to_string(),
            service: "storage".to_string(),
        },
        LogEntry {
            timestamp: "2024-11-23T15:30:10Z".to_string(),
            level: "ERROR".to_string(),
            message: "Connection timeout to external service".to_string(),
            service: "gateway".to_string(),
        },
    ];

    Ok(Json(logs))
}

const OBSERVABILITY_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Observability - AVL Console</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: #0a0e1a;
            color: #e0e6ed;
        }
        .header {
            background: #0f1419;
            border-bottom: 1px solid #1a1f2e;
            padding: 1rem 2rem;
        }
        .container { max-width: 1400px; margin: 2rem auto; padding: 0 2rem; }
        .chart-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
            gap: 2rem;
            margin-bottom: 2rem;
        }
        .chart-card {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
        }
        .chart-title {
            font-size: 1rem;
            font-weight: bold;
            margin-bottom: 1rem;
            color: #00d4ff;
        }
        .logs-section {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
        }
        .log-entry {
            padding: 0.75rem;
            border-bottom: 1px solid #1a1f2e;
            font-family: 'Courier New', monospace;
            font-size: 0.875rem;
        }
        .log-entry:last-child { border-bottom: none; }
        .log-info { color: #00d4ff; }
        .log-warn { color: #ffa500; }
        .log-error { color: #ff4444; }
    </style>
</head>
<body>
    <div class="header">
        <h1>📈 Observability</h1>
    </div>

    <div class="container">
        <h2 style="margin-bottom: 1.5rem;">Métricas em Tempo Real</h2>

        <div class="chart-grid">
            <div class="chart-card">
                <div class="chart-title">CPU Usage (%)</div>
                <canvas id="cpuChart"></canvas>
            </div>
            <div class="chart-card">
                <div class="chart-title">Memory Usage (%)</div>
                <canvas id="memoryChart"></canvas>
            </div>
            <div class="chart-card">
                <div class="chart-title">Request Rate (req/s)</div>
                <canvas id="requestChart"></canvas>
            </div>
            <div class="chart-card">
                <div class="chart-title">Error Rate (%)</div>
                <canvas id="errorChart"></canvas>
            </div>
        </div>

        <div class="logs-section">
            <h3 style="margin-bottom: 1rem;">Logs Recentes</h3>
            <div id="logs"></div>
        </div>
    </div>

    <script>
        const chartConfig = {
            type: 'line',
            options: {
                responsive: true,
                plugins: { legend: { display: false } },
                scales: {
                    y: { ticks: { color: '#8b92a0' }, grid: { color: '#1a1f2e' } },
                    x: { ticks: { color: '#8b92a0' }, grid: { color: '#1a1f2e' } }
                }
            }
        };

        async function loadMetrics() {
            const res = await fetch('/observability/metrics');
            const data = await res.json();

            updateChart('cpuChart', data.cpu_usage, '#00d4ff');
            updateChart('memoryChart', data.memory_usage, '#00ff88');
            updateChart('requestChart', data.request_rate, '#ffa500');
            updateChart('errorChart', data.error_rate, '#ff4444');
        }

        function updateChart(id, data, color) {
            const ctx = document.getElementById(id).getContext('2d');
            new Chart(ctx, {
                ...chartConfig,
                data: {
                    labels: data.map(p => new Date(p.timestamp * 1000).toLocaleTimeString()),
                    datasets: [{
                        data: data.map(p => p.value),
                        borderColor: color,
                        backgroundColor: color + '20',
                        fill: true,
                        tension: 0.4
                    }]
                }
            });
        }

        async function loadLogs() {
            const res = await fetch('/observability/logs');
            const logs = await res.json();
            const container = document.getElementById('logs');
            container.innerHTML = logs.map(log => `
                <div class="log-entry log-${log.level.toLowerCase()}">
                    [${log.timestamp}] [${log.level}] [${log.service}] ${log.message}
                </div>
            `).join('');
        }

        loadMetrics();
        loadLogs();
        setInterval(loadLogs, 5000);
    </script>
</body>
</html>"#;
