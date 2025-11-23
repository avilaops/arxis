//! Billing and cost tracking

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
        .route("/", get(billing_page))
        .route("/usage", get(get_usage))
        .route("/invoices", get(get_invoices))
        .with_state(state)
}

async fn billing_page() -> impl IntoResponse {
    Html(BILLING_HTML)
}

#[derive(Serialize)]
struct UsageData {
    current_month: MonthlyUsage,
    breakdown: Vec<ServiceUsage>,
}

#[derive(Serialize)]
struct MonthlyUsage {
    period: String,
    total_cost_brl: f64,
    estimated_cost_brl: f64,
}

#[derive(Serialize)]
struct ServiceUsage {
    service: String,
    usage: String,
    cost_brl: f64,
}

async fn get_usage(State(_state): State<Arc<AppState>>) -> Result<Json<UsageData>> {
    // TODO: Query actual billing data
    let usage = UsageData {
        current_month: MonthlyUsage {
            period: "Novembro 2024".to_string(),
            total_cost_brl: 125.50,
            estimated_cost_brl: 180.00,
        },
        breakdown: vec![
            ServiceUsage {
                service: "AvilaDB".to_string(),
                usage: "3 databases, 15M operations".to_string(),
                cost_brl: 75.00,
            },
            ServiceUsage {
                service: "Storage".to_string(),
                usage: "128 GB, 50K requests".to_string(),
                cost_brl: 35.50,
            },
            ServiceUsage {
                service: "Queue".to_string(),
                usage: "5 topics, 1.2M messages".to_string(),
                cost_brl: 15.00,
            },
        ],
    };

    Ok(Json(usage))
}

#[derive(Serialize)]
struct Invoice {
    id: String,
    period: String,
    amount_brl: f64,
    status: String,
    due_date: String,
}

async fn get_invoices(State(_state): State<Arc<AppState>>) -> Result<Json<Vec<Invoice>>> {
    // TODO: Query actual invoices
    let invoices = vec![
        Invoice {
            id: "inv_2024_11".to_string(),
            period: "Novembro 2024".to_string(),
            amount_brl: 125.50,
            status: "current".to_string(),
            due_date: "2024-12-01".to_string(),
        },
        Invoice {
            id: "inv_2024_10".to_string(),
            period: "Outubro 2024".to_string(),
            amount_brl: 98.30,
            status: "paid".to_string(),
            due_date: "2024-11-01".to_string(),
        },
    ];

    Ok(Json(invoices))
}

const BILLING_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Billing - AVL Console</title>
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
        .summary {
            background: linear-gradient(135deg, #00d4ff 0%, #0088ff 100%);
            border-radius: 8px;
            padding: 2rem;
            margin-bottom: 2rem;
            color: #0a0e1a;
        }
        .summary h2 { font-size: 2.5rem; margin-bottom: 0.5rem; }
        .summary p { opacity: 0.8; }
        .section {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 2rem;
        }
        .section-title {
            font-size: 1.25rem;
            margin-bottom: 1rem;
            color: #00d4ff;
        }
        .usage-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem;
            border-bottom: 1px solid #1a1f2e;
        }
        .usage-item:last-child { border-bottom: none; }
        .service-name { font-weight: bold; }
        .service-usage { color: #8b92a0; font-size: 0.875rem; }
        .cost { font-size: 1.25rem; font-weight: bold; color: #00d4ff; }
        .invoice-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem;
            border-bottom: 1px solid #1a1f2e;
        }
        .invoice-item:last-child { border-bottom: none; }
        .status-paid { color: #00ff88; }
        .status-current { color: #ffa500; }
    </style>
</head>
<body>
    <div class="header">
        <h1>💰 Billing</h1>
    </div>

    <div class="container">
        <div class="summary" id="summary">
            <p>Uso do mês atual</p>
            <h2 id="currentCost">R$ -</h2>
            <p>Estimativa do mês: <strong id="estimatedCost">R$ -</strong></p>
        </div>

        <div class="section">
            <h3 class="section-title">Detalhamento por Serviço</h3>
            <div id="breakdown"></div>
        </div>

        <div class="section">
            <h3 class="section-title">Faturas</h3>
            <div id="invoices"></div>
        </div>
    </div>

    <script>
        async function loadUsage() {
            const res = await fetch('/billing/usage');
            const data = await res.json();

            document.getElementById('currentCost').textContent =
                'R$ ' + data.current_month.total_cost_brl.toFixed(2);
            document.getElementById('estimatedCost').textContent =
                'R$ ' + data.current_month.estimated_cost_brl.toFixed(2);

            const breakdown = document.getElementById('breakdown');
            breakdown.innerHTML = data.breakdown.map(item => `
                <div class="usage-item">
                    <div>
                        <div class="service-name">${item.service}</div>
                        <div class="service-usage">${item.usage}</div>
                    </div>
                    <div class="cost">R$ ${item.cost_brl.toFixed(2)}</div>
                </div>
            `).join('');
        }

        async function loadInvoices() {
            const res = await fetch('/billing/invoices');
            const invoices = await res.json();
            const container = document.getElementById('invoices');
            container.innerHTML = invoices.map(inv => `
                <div class="invoice-item">
                    <div>
                        <strong>${inv.period}</strong><br>
                        <span style="color: #8b92a0; font-size: 0.875rem;">
                            Vencimento: ${inv.due_date}
                        </span>
                    </div>
                    <div>
                        <div class="cost">R$ ${inv.amount_brl.toFixed(2)}</div>
                        <div class="status-${inv.status}" style="font-size: 0.875rem;">
                            ${inv.status === 'paid' ? 'Paga' : 'Em aberto'}
                        </div>
                    </div>
                </div>
            `).join('');
        }

        loadUsage();
        loadInvoices();
    </script>
</body>
</html>"#;
