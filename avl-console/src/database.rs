//! AvilaDB Explorer - Database management and query interface

use crate::{error::Result, state::AppState};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(database_list_page))
        .route("/list", get(list_databases))
        .route("/:db_id", get(database_detail))
        .route("/:db_id/query", post(execute_query))
        .route("/:db_id/collections", get(list_collections))
        .with_state(state)
}

async fn database_list_page() -> impl IntoResponse {
    Html(DATABASE_LIST_HTML)
}

#[derive(Serialize)]
struct DatabaseInfo {
    id: String,
    name: String,
    region: String,
    collections: usize,
    size_bytes: u64,
    created_at: String,
}

async fn list_databases(State(_state): State<Arc<AppState>>) -> Result<Json<Vec<DatabaseInfo>>> {
    // TODO: Query actual AvilaDB service
    let databases = vec![
        DatabaseInfo {
            id: "db_prod_001".to_string(),
            name: "production".to_string(),
            region: "sa-east-1".to_string(),
            collections: 15,
            size_bytes: 2_500_000_000,
            created_at: "2024-11-01T10:00:00Z".to_string(),
        },
        DatabaseInfo {
            id: "db_dev_001".to_string(),
            name: "development".to_string(),
            region: "sa-east-1".to_string(),
            collections: 8,
            size_bytes: 500_000_000,
            created_at: "2024-11-15T14:30:00Z".to_string(),
        },
        DatabaseInfo {
            id: "db_test_001".to_string(),
            name: "testing".to_string(),
            region: "sa-east-1".to_string(),
            collections: 5,
            size_bytes: 100_000_000,
            created_at: "2024-11-20T09:00:00Z".to_string(),
        },
    ];

    Ok(Json(databases))
}

async fn database_detail(
    Path(db_id): Path<String>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<DatabaseInfo>> {
    // TODO: Query specific database
    Ok(Json(DatabaseInfo {
        id: db_id,
        name: "production".to_string(),
        region: "sa-east-1".to_string(),
        collections: 15,
        size_bytes: 2_500_000_000,
        created_at: "2024-11-01T10:00:00Z".to_string(),
    }))
}

#[derive(Deserialize)]
struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
struct QueryResult {
    rows: Vec<serde_json::Value>,
    count: usize,
    execution_time_ms: u64,
}

async fn execute_query(
    Path(_db_id): Path<String>,
    State(_state): State<Arc<AppState>>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<QueryResult>> {
    // TODO: Execute actual query on AvilaDB
    tracing::info!("Executing query: {}", req.query);

    Ok(Json(QueryResult {
        rows: vec![
            serde_json::json!({"id": 1, "name": "Item 1"}),
            serde_json::json!({"id": 2, "name": "Item 2"}),
        ],
        count: 2,
        execution_time_ms: 15,
    }))
}

#[derive(Serialize)]
struct CollectionInfo {
    name: String,
    partition_key: String,
    document_count: usize,
    size_bytes: u64,
}

async fn list_collections(
    Path(_db_id): Path<String>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<CollectionInfo>>> {
    // TODO: Query actual collections
    let collections = vec![
        CollectionInfo {
            name: "users".to_string(),
            partition_key: "userId".to_string(),
            document_count: 10_000,
            size_bytes: 50_000_000,
        },
        CollectionInfo {
            name: "orders".to_string(),
            partition_key: "customerId".to_string(),
            document_count: 50_000,
            size_bytes: 200_000_000,
        },
    ];

    Ok(Json(collections))
}

const DATABASE_LIST_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AvilaDB Explorer - AVL Console</title>
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
        .db-card {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1rem;
            cursor: pointer;
            transition: all 0.2s;
        }
        .db-card:hover {
            border-color: #00d4ff;
            transform: translateX(4px);
        }
        .db-name {
            font-size: 1.25rem;
            font-weight: bold;
            color: #00d4ff;
            margin-bottom: 0.5rem;
        }
        .db-info {
            display: flex;
            gap: 2rem;
            font-size: 0.875rem;
            color: #8b92a0;
        }
        .query-editor {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
            margin-top: 2rem;
        }
        textarea {
            width: 100%;
            min-height: 150px;
            background: #0a0e1a;
            border: 1px solid #1a1f2e;
            border-radius: 4px;
            padding: 1rem;
            color: #e0e6ed;
            font-family: 'Courier New', monospace;
            font-size: 0.875rem;
            resize: vertical;
        }
        button {
            background: #00d4ff;
            color: #0a0e1a;
            border: none;
            padding: 0.75rem 1.5rem;
            border-radius: 4px;
            font-weight: bold;
            cursor: pointer;
            margin-top: 1rem;
        }
        button:hover { background: #00b8e6; }
        .results {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
            margin-top: 1rem;
        }
        pre {
            background: #0a0e1a;
            padding: 1rem;
            border-radius: 4px;
            overflow-x: auto;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>🗄️ AvilaDB Explorer</h1>
    </div>

    <div class="container">
        <h2 style="margin-bottom: 1rem;">Seus Bancos de Dados</h2>
        <div id="databases"></div>

        <div class="query-editor">
            <h3 style="margin-bottom: 1rem;">Query Editor</h3>
            <textarea id="query" placeholder="SELECT * FROM users WHERE active = true">SELECT * FROM users LIMIT 10</textarea>
            <button onclick="executeQuery()">Executar Query</button>
        </div>

        <div class="results" id="results" style="display: none;">
            <h3>Resultados</h3>
            <pre id="resultData"></pre>
        </div>
    </div>

    <script>
        async function loadDatabases() {
            const res = await fetch('/databases/list');
            const databases = await res.json();
            const container = document.getElementById('databases');
            container.innerHTML = databases.map(db => `
                <div class="db-card">
                    <div class="db-name">${db.name}</div>
                    <div class="db-info">
                        <span>ID: ${db.id}</span>
                        <span>Região: ${db.region}</span>
                        <span>Coleções: ${db.collections}</span>
                        <span>Tamanho: ${(db.size_bytes / 1_000_000).toFixed(1)} MB</span>
                    </div>
                </div>
            `).join('');
        }

        async function executeQuery() {
            const query = document.getElementById('query').value;
            const res = await fetch('/databases/db_prod_001/query', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ query })
            });
            const result = await res.json();
            document.getElementById('results').style.display = 'block';
            document.getElementById('resultData').textContent = JSON.stringify(result, null, 2);
        }

        loadDatabases();
    </script>
</body>
</html>"#;
