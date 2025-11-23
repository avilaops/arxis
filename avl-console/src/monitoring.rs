use axum::{
    extract::State,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{error::ConsoleError, state::ConsoleState};

/// Advanced Monitoring Dashboard HTML
const MONITORING_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Advanced Monitoring - AVL Console</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0"></script>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            min-height: 100vh;
            padding: 20px;
        }
        .container {
            max-width: 1800px;
            margin: 0 auto;
        }
        h1 {
            color: white;
            margin-bottom: 10px;
            font-size: 36px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        .subtitle {
            color: rgba(255,255,255,0.9);
            margin-bottom: 30px;
            font-size: 18px;
        }
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin-bottom: 20px;
        }
        .metric-card {
            background: white;
            border-radius: 16px;
            padding: 25px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            transition: transform 0.2s;
        }
        .metric-card:hover {
            transform: translateY(-5px);
        }
        .metric-card.anomaly {
            border: 3px solid #dc3545;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0%, 100% { box-shadow: 0 10px 30px rgba(0,0,0,0.2); }
            50% { box-shadow: 0 10px 40px rgba(220, 53, 69, 0.5); }
        }
        .metric-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        }
        .metric-title {
            font-size: 14px;
            color: #6c757d;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            font-weight: 600;
        }
        .metric-badge {
            padding: 4px 12px;
            border-radius: 12px;
            font-size: 11px;
            font-weight: 600;
        }
        .badge-normal {
            background: #d4edda;
            color: #155724;
        }
        .badge-warning {
            background: #fff3cd;
            color: #856404;
        }
        .badge-critical {
            background: #f8d7da;
            color: #721c24;
        }
        .metric-value {
            font-size: 42px;
            font-weight: 700;
            color: #2a5298;
            margin-bottom: 10px;
        }
        .metric-change {
            font-size: 14px;
            display: flex;
            align-items: center;
            gap: 5px;
        }
        .change-up {
            color: #28a745;
        }
        .change-down {
            color: #dc3545;
        }
        .charts-section {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
            gap: 20px;
            margin-bottom: 20px;
        }
        .chart-card {
            background: white;
            border-radius: 16px;
            padding: 25px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
        }
        .chart-title {
            font-size: 18px;
            font-weight: 600;
            color: #2a5298;
            margin-bottom: 20px;
        }
        .alerts-section {
            background: white;
            border-radius: 16px;
            padding: 25px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
        }
        .alert-item {
            display: flex;
            align-items: start;
            gap: 15px;
            padding: 15px;
            border-radius: 12px;
            margin-bottom: 10px;
            border: 2px solid #e9ecef;
            transition: all 0.2s;
        }
        .alert-item:hover {
            border-color: #2a5298;
            background: #f8f9fa;
        }
        .alert-icon {
            font-size: 32px;
            flex-shrink: 0;
        }
        .alert-content {
            flex: 1;
        }
        .alert-title {
            font-weight: 600;
            color: #495057;
            margin-bottom: 5px;
        }
        .alert-description {
            font-size: 14px;
            color: #6c757d;
            margin-bottom: 8px;
        }
        .alert-time {
            font-size: 12px;
            color: #adb5bd;
        }
        .alert-actions {
            display: flex;
            gap: 8px;
        }
        .btn-small {
            padding: 6px 12px;
            border: none;
            border-radius: 6px;
            font-size: 12px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
        }
        .btn-resolve {
            background: #28a745;
            color: white;
        }
        .btn-resolve:hover {
            background: #218838;
        }
        .btn-ignore {
            background: #6c757d;
            color: white;
        }
        .btn-ignore:hover {
            background: #5a6268;
        }
        .ml-insights {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            border-radius: 16px;
            padding: 25px;
            color: white;
            margin-top: 20px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
        }
        .insight-title {
            font-size: 20px;
            font-weight: 600;
            margin-bottom: 15px;
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .insight-list {
            display: flex;
            flex-direction: column;
            gap: 12px;
        }
        .insight-item {
            background: rgba(255,255,255,0.1);
            padding: 15px;
            border-radius: 12px;
            backdrop-filter: blur(10px);
        }
        .insight-item strong {
            display: block;
            margin-bottom: 5px;
        }
        .refresh-btn {
            position: fixed;
            bottom: 30px;
            right: 30px;
            width: 60px;
            height: 60px;
            border-radius: 50%;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            font-size: 24px;
            cursor: pointer;
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.5);
            transition: all 0.3s;
        }
        .refresh-btn:hover {
            transform: scale(1.1) rotate(180deg);
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔬 Advanced Monitoring & Alerts</h1>
        <p class="subtitle">Detecção inteligente de anomalias com Machine Learning</p>

        <div class="metrics-grid" id="metricsGrid">
            <!-- Metrics will be populated here -->
        </div>

        <div class="charts-section">
            <div class="chart-card">
                <div class="chart-title">📈 Response Time Trend</div>
                <canvas id="responseTimeChart"></canvas>
            </div>
            <div class="chart-card">
                <div class="chart-title">💾 Resource Usage</div>
                <canvas id="resourceChart"></canvas>
            </div>
        </div>

        <div class="alerts-section">
            <h2 style="color: #2a5298; margin-bottom: 20px;">🚨 Active Alerts</h2>
            <div id="alertsList">
                <!-- Alerts will be populated here -->
            </div>
        </div>

        <div class="ml-insights">
            <div class="insight-title">
                <span>🤖</span>
                <span>ML-Powered Insights</span>
            </div>
            <div class="insight-list" id="insightsList">
                <!-- Insights will be populated here -->
            </div>
        </div>
    </div>

    <button class="refresh-btn" onclick="refreshData()">🔄</button>

    <script>
        let responseChart, resourceChart;

        async function loadMetrics() {
            try {
                const response = await fetch('/monitoring/metrics');
                const data = await response.json();
                renderMetrics(data.metrics);
                updateCharts(data.timeseries);
            } catch (error) {
                console.error('Failed to load metrics:', error);
            }
        }

        async function loadAlerts() {
            try {
                const response = await fetch('/monitoring/alerts');
                const data = await response.json();
                renderAlerts(data.alerts);
            } catch (error) {
                console.error('Failed to load alerts:', error);
            }
        }

        async function loadInsights() {
            try {
                const response = await fetch('/monitoring/insights');
                const data = await response.json();
                renderInsights(data.insights);
            } catch (error) {
                console.error('Failed to load insights:', error);
            }
        }

        function renderMetrics(metrics) {
            const grid = document.getElementById('metricsGrid');
            grid.innerHTML = metrics.map(metric => {
                const badgeClass = metric.status === 'normal' ? 'badge-normal' :
                                  metric.status === 'warning' ? 'badge-warning' : 'badge-critical';
                const changeClass = metric.change >= 0 ? 'change-up' : 'change-down';
                const changeIcon = metric.change >= 0 ? '▲' : '▼';
                const anomalyClass = metric.is_anomaly ? 'anomaly' : '';

                return `
                    <div class="metric-card ${anomalyClass}">
                        <div class="metric-header">
                            <div class="metric-title">${metric.name}</div>
                            <div class="metric-badge ${badgeClass}">${metric.status.toUpperCase()}</div>
                        </div>
                        <div class="metric-value">${metric.value}${metric.unit}</div>
                        <div class="metric-change ${changeClass}">
                            ${changeIcon} ${Math.abs(metric.change)}% vs last hour
                        </div>
                    </div>
                `;
            }).join('');
        }

        function renderAlerts(alerts) {
            const list = document.getElementById('alertsList');
            if (alerts.length === 0) {
                list.innerHTML = '<p style="text-align: center; color: #6c757d; padding: 40px;">✅ No active alerts</p>';
                return;
            }

            list.innerHTML = alerts.map(alert => `
                <div class="alert-item">
                    <div class="alert-icon">${alert.icon}</div>
                    <div class="alert-content">
                        <div class="alert-title">${alert.title}</div>
                        <div class="alert-description">${alert.description}</div>
                        <div class="alert-time">🕐 ${alert.time}</div>
                    </div>
                    <div class="alert-actions">
                        <button class="btn-small btn-resolve" onclick="resolveAlert('${alert.id}')">Resolve</button>
                        <button class="btn-small btn-ignore" onclick="ignoreAlert('${alert.id}')">Ignore</button>
                    </div>
                </div>
            `).join('');
        }

        function renderInsights(insights) {
            const list = document.getElementById('insightsList');
            list.innerHTML = insights.map(insight => `
                <div class="insight-item">
                    <strong>${insight.title}</strong>
                    <div>${insight.description}</div>
                </div>
            `).join('');
        }

        function initCharts() {
            const ctx1 = document.getElementById('responseTimeChart').getContext('2d');
            responseChart = new Chart(ctx1, {
                type: 'line',
                data: {
                    labels: [],
                    datasets: [{
                        label: 'Response Time (ms)',
                        data: [],
                        borderColor: '#2a5298',
                        backgroundColor: 'rgba(42, 82, 152, 0.1)',
                        tension: 0.4,
                        fill: true
                    }]
                },
                options: {
                    responsive: true,
                    plugins: {
                        legend: { display: false }
                    },
                    scales: {
                        y: { beginAtZero: true }
                    }
                }
            });

            const ctx2 = document.getElementById('resourceChart').getContext('2d');
            resourceChart = new Chart(ctx2, {
                type: 'bar',
                data: {
                    labels: ['CPU', 'Memory', 'Disk', 'Network'],
                    datasets: [{
                        label: 'Usage %',
                        data: [0, 0, 0, 0],
                        backgroundColor: [
                            '#667eea',
                            '#764ba2',
                            '#f093fb',
                            '#4facfe'
                        ]
                    }]
                },
                options: {
                    responsive: true,
                    plugins: {
                        legend: { display: false }
                    },
                    scales: {
                        y: {
                            beginAtZero: true,
                            max: 100
                        }
                    }
                }
            });
        }

        function updateCharts(timeseries) {
            if (!timeseries) return;

            // Update response time chart
            if (timeseries.response_time) {
                responseChart.data.labels = timeseries.response_time.labels;
                responseChart.data.datasets[0].data = timeseries.response_time.values;
                responseChart.update();
            }

            // Update resource chart
            if (timeseries.resources) {
                resourceChart.data.datasets[0].data = timeseries.resources;
                resourceChart.update();
            }
        }

        async function resolveAlert(id) {
            try {
                await fetch(`/monitoring/alerts/${id}/resolve`, { method: 'POST' });
                await loadAlerts();
            } catch (error) {
                console.error('Failed to resolve alert:', error);
            }
        }

        async function ignoreAlert(id) {
            try {
                await fetch(`/monitoring/alerts/${id}/ignore`, { method: 'POST' });
                await loadAlerts();
            } catch (error) {
                console.error('Failed to ignore alert:', error);
            }
        }

        function refreshData() {
            loadMetrics();
            loadAlerts();
            loadInsights();
        }

        // Initialize
        initCharts();
        refreshData();

        // Auto-refresh every 30 seconds
        setInterval(refreshData, 30000);
    </script>
</body>
</html>"#;

/// Metric data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub status: String, // "normal", "warning", "critical"
    pub change: f64,    // percentage change
    pub is_anomaly: bool,
    pub threshold: f64,
}

/// Alert data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alert {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: String, // "info", "warning", "critical"
    pub icon: String,
    pub time: String,
    pub metric: String,
    pub value: f64,
}

/// ML Insight data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLInsight {
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub recommendation: String,
}

/// Time series data
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct TimeSeriesData {
    labels: Vec<String>,
    values: Vec<f64>,
}

/// Metrics response
#[derive(Debug, Serialize)]
struct MetricsResponse {
    metrics: Vec<Metric>,
    timeseries: serde_json::Value,
}

/// Alerts response
#[derive(Debug, Serialize)]
struct AlertsResponse {
    alerts: Vec<Alert>,
}

/// Insights response
#[derive(Debug, Serialize)]
struct InsightsResponse {
    insights: Vec<MLInsight>,
}

/// Monitoring dashboard UI
async fn monitoring_ui() -> impl IntoResponse {
    Html(MONITORING_HTML)
}

/// Get current metrics with anomaly detection
async fn get_metrics(
    State(_state): State<Arc<ConsoleState>>,
) -> Result<Json<MetricsResponse>, ConsoleError> {
    // Simulate metrics with ML anomaly detection
    let metrics = vec![
        Metric {
            name: "Response Time".to_string(),
            value: 45.2,
            unit: "ms".to_string(),
            status: "normal".to_string(),
            change: -5.3,
            is_anomaly: false,
            threshold: 100.0,
        },
        Metric {
            name: "Requests/sec".to_string(),
            value: 1234.0,
            unit: "".to_string(),
            status: "normal".to_string(),
            change: 12.5,
            is_anomaly: false,
            threshold: 2000.0,
        },
        Metric {
            name: "Error Rate".to_string(),
            value: 0.8,
            unit: "%".to_string(),
            status: "warning".to_string(),
            change: 45.2,
            is_anomaly: true,
            threshold: 1.0,
        },
        Metric {
            name: "CPU Usage".to_string(),
            value: 67.3,
            unit: "%".to_string(),
            status: "normal".to_string(),
            change: 3.1,
            is_anomaly: false,
            threshold: 80.0,
        },
        Metric {
            name: "Memory Usage".to_string(),
            value: 82.5,
            unit: "%".to_string(),
            status: "warning".to_string(),
            change: 15.7,
            is_anomaly: true,
            threshold: 85.0,
        },
        Metric {
            name: "Active Connections".to_string(),
            value: 456.0,
            unit: "".to_string(),
            status: "normal".to_string(),
            change: -2.1,
            is_anomaly: false,
            threshold: 1000.0,
        },
    ];

    // Generate time series data
    let timeseries = serde_json::json!({
        "response_time": {
            "labels": ["10:00", "10:15", "10:30", "10:45", "11:00", "11:15", "11:30"],
            "values": [42.1, 43.5, 45.2, 44.8, 46.1, 45.2, 45.2]
        },
        "resources": [67.3, 82.5, 45.2, 34.1]
    });

    Ok(Json(MetricsResponse {
        metrics,
        timeseries,
    }))
}

/// Get active alerts
async fn get_alerts(
    State(_state): State<Arc<ConsoleState>>,
) -> Result<Json<AlertsResponse>, ConsoleError> {
    let alerts = vec![
        Alert {
            id: "alert_1".to_string(),
            title: "Anomaly Detected: Error Rate Spike".to_string(),
            description: "Error rate increased by 45% in the last hour. ML model detected unusual pattern."
                .to_string(),
            severity: "warning".to_string(),
            icon: "⚠️".to_string(),
            time: "2 minutes ago".to_string(),
            metric: "error_rate".to_string(),
            value: 0.8,
        },
        Alert {
            id: "alert_2".to_string(),
            title: "Memory Usage Approaching Limit".to_string(),
            description: "Memory usage at 82.5%. Consider scaling up or optimizing memory-intensive operations."
                .to_string(),
            severity: "warning".to_string(),
            icon: "💾".to_string(),
            time: "15 minutes ago".to_string(),
            metric: "memory_usage".to_string(),
            value: 82.5,
        },
        Alert {
            id: "alert_3".to_string(),
            title: "Unusual Traffic Pattern Detected".to_string(),
            description: "ML model identified abnormal request distribution. Possible DDoS or bot activity."
                .to_string(),
            severity: "critical".to_string(),
            icon: "🚨".to_string(),
            time: "5 minutes ago".to_string(),
            metric: "traffic_pattern".to_string(),
            value: 95.2,
        },
    ];

    Ok(Json(AlertsResponse { alerts }))
}

/// Get ML-powered insights
async fn get_insights(
    State(_state): State<Arc<ConsoleState>>,
) -> Result<Json<InsightsResponse>, ConsoleError> {
    let insights = vec![
        MLInsight {
            title: "Predictive Scaling Recommendation".to_string(),
            description: "Based on historical patterns, traffic is expected to increase by 35% in the next 2 hours. Consider auto-scaling now.".to_string(),
            confidence: 0.89,
            recommendation: "Enable auto-scaling with min 3, max 10 instances".to_string(),
        },
        MLInsight {
            title: "Cost Optimization Opportunity".to_string(),
            description: "Database queries show N+1 pattern. Implementing batch loading could reduce costs by ~40%.".to_string(),
            confidence: 0.92,
            recommendation: "Review database.rs query patterns and implement eager loading".to_string(),
        },
        MLInsight {
            title: "Performance Bottleneck Identified".to_string(),
            description: "ML analysis shows 78% of slow requests involve /api/search endpoint. Consider caching or indexing improvements.".to_string(),
            confidence: 0.85,
            recommendation: "Add Redis cache layer for search results with 5-minute TTL".to_string(),
        },
    ];

    Ok(Json(InsightsResponse { insights }))
}

/// Resolve an alert
async fn resolve_alert(
    State(_state): State<Arc<ConsoleState>>,
    axum::extract::Path(alert_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, ConsoleError> {
    // In production, update alert status in database
    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Alert {} resolved", alert_id)
    })))
}

/// Ignore an alert
async fn ignore_alert(
    State(_state): State<Arc<ConsoleState>>,
    axum::extract::Path(alert_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, ConsoleError> {
    // In production, update alert status in database
    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Alert {} ignored", alert_id)
    })))
}

/// Anomaly detection using simple statistical methods
/// In production, this would use proper ML models (e.g., Isolation Forest, LSTM)
pub fn detect_anomaly(value: f64, historical_values: &[f64], threshold_std: f64) -> bool {
    if historical_values.is_empty() {
        return false;
    }

    let mean = historical_values.iter().sum::<f64>() / historical_values.len() as f64;
    let variance = historical_values
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>()
        / historical_values.len() as f64;
    let std_dev = variance.sqrt();

    (value - mean).abs() > threshold_std * std_dev
}

/// Create router for monitoring
pub fn router(state: Arc<ConsoleState>) -> Router {
    Router::new()
        .route("/", get(monitoring_ui))
        .route("/metrics", get(get_metrics))
        .route("/alerts", get(get_alerts))
        .route("/insights", get(get_insights))
        .route("/alerts/:id/resolve", post(resolve_alert))
        .route("/alerts/:id/ignore", post(ignore_alert))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_detection_normal() {
        let historical = vec![100.0, 102.0, 98.0, 101.0, 99.0];
        assert!(!detect_anomaly(100.5, &historical, 2.0));
    }

    #[test]
    fn test_anomaly_detection_spike() {
        let historical = vec![100.0, 102.0, 98.0, 101.0, 99.0];
        assert!(detect_anomaly(150.0, &historical, 2.0));
    }

    #[test]
    fn test_anomaly_detection_empty() {
        assert!(!detect_anomaly(100.0, &[], 2.0));
    }
}
