use axum::{
    extract::State,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{error::ConsoleError, state::ConsoleState};

/// Query Builder UI HTML with drag-and-drop interface
const QUERY_BUILDER_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Query Builder Visual - AVL Console</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        .container {
            max-width: 1800px;
            margin: 0 auto;
            background: white;
            border-radius: 20px;
            padding: 30px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        h1 {
            color: #667eea;
            margin-bottom: 10px;
            font-size: 32px;
        }
        .subtitle {
            color: #666;
            margin-bottom: 30px;
            font-size: 16px;
        }
        .builder-grid {
            display: grid;
            grid-template-columns: 250px 1fr;
            gap: 20px;
            margin-bottom: 20px;
        }
        .sidebar {
            background: #f8f9fa;
            border-radius: 12px;
            padding: 20px;
            border: 2px solid #e9ecef;
        }
        .sidebar h3 {
            color: #495057;
            margin-bottom: 15px;
            font-size: 14px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .component-list {
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .component {
            background: white;
            border: 2px solid #dee2e6;
            border-radius: 8px;
            padding: 12px;
            cursor: grab;
            transition: all 0.2s;
            font-size: 14px;
            font-weight: 500;
            color: #495057;
        }
        .component:hover {
            border-color: #667eea;
            background: #f0f3ff;
            transform: translateY(-2px);
        }
        .component:active {
            cursor: grabbing;
        }
        .component-icon {
            margin-right: 8px;
            font-size: 16px;
        }
        .canvas {
            background: white;
            border: 2px dashed #dee2e6;
            border-radius: 12px;
            min-height: 400px;
            padding: 20px;
            position: relative;
        }
        .canvas.drag-over {
            background: #f0f3ff;
            border-color: #667eea;
        }
        .canvas-placeholder {
            text-align: center;
            color: #adb5bd;
            padding: 60px 20px;
            pointer-events: none;
        }
        .query-component {
            background: white;
            border: 2px solid #667eea;
            border-radius: 8px;
            padding: 15px;
            margin-bottom: 10px;
            position: relative;
            transition: all 0.2s;
        }
        .query-component:hover {
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
        }
        .component-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
        }
        .component-title {
            font-weight: 600;
            color: #667eea;
            font-size: 14px;
        }
        .component-remove {
            background: #dc3545;
            color: white;
            border: none;
            border-radius: 4px;
            padding: 4px 10px;
            cursor: pointer;
            font-size: 12px;
        }
        .component-remove:hover {
            background: #c82333;
        }
        .component-content {
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
        .field-group {
            display: flex;
            gap: 10px;
            align-items: center;
        }
        .field-group label {
            font-size: 13px;
            color: #6c757d;
            min-width: 80px;
        }
        .field-group input,
        .field-group select {
            flex: 1;
            padding: 8px 12px;
            border: 1px solid #dee2e6;
            border-radius: 6px;
            font-size: 13px;
        }
        .field-group input:focus,
        .field-group select:focus {
            outline: none;
            border-color: #667eea;
        }
        .action-bar {
            display: flex;
            gap: 15px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 12px;
            align-items: center;
        }
        .btn {
            padding: 12px 24px;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
            font-size: 14px;
        }
        .btn-primary {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }
        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
        }
        .btn-secondary {
            background: white;
            color: #667eea;
            border: 2px solid #667eea;
        }
        .btn-secondary:hover {
            background: #f0f3ff;
        }
        .generated-query {
            flex: 1;
            background: #282c34;
            color: #abb2bf;
            padding: 15px;
            border-radius: 8px;
            font-family: 'Courier New', monospace;
            font-size: 13px;
            line-height: 1.6;
            overflow-x: auto;
        }
        .results-section {
            margin-top: 20px;
            background: #f8f9fa;
            border-radius: 12px;
            padding: 20px;
            display: none;
        }
        .results-section.active {
            display: block;
        }
        .results-table {
            width: 100%;
            border-collapse: collapse;
            background: white;
            border-radius: 8px;
            overflow: hidden;
        }
        .results-table th {
            background: #667eea;
            color: white;
            padding: 12px;
            text-align: left;
            font-size: 13px;
        }
        .results-table td {
            padding: 12px;
            border-bottom: 1px solid #dee2e6;
            font-size: 13px;
        }
        .results-table tbody tr:hover {
            background: #f8f9fa;
        }
        .loading {
            text-align: center;
            padding: 40px;
            color: #667eea;
        }
        .error-message {
            background: #f8d7da;
            color: #721c24;
            padding: 15px;
            border-radius: 8px;
            border: 1px solid #f5c6cb;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎨 Query Builder Visual</h1>
        <p class="subtitle">Construa queries SQL complexas com drag-and-drop - sem escrever código!</p>

        <div class="builder-grid">
            <div class="sidebar">
                <h3>📦 Componentes</h3>
                <div class="component-list">
                    <div class="component" draggable="true" data-type="select">
                        <span class="component-icon">🔍</span>SELECT
                    </div>
                    <div class="component" draggable="true" data-type="from">
                        <span class="component-icon">📋</span>FROM
                    </div>
                    <div class="component" draggable="true" data-type="where">
                        <span class="component-icon">🔎</span>WHERE
                    </div>
                    <div class="component" draggable="true" data-type="join">
                        <span class="component-icon">🔗</span>JOIN
                    </div>
                    <div class="component" draggable="true" data-type="groupby">
                        <span class="component-icon">📊</span>GROUP BY
                    </div>
                    <div class="component" draggable="true" data-type="orderby">
                        <span class="component-icon">⬆️</span>ORDER BY
                    </div>
                    <div class="component" draggable="true" data-type="limit">
                        <span class="component-icon">🔢</span>LIMIT
                    </div>
                </div>
            </div>

            <div class="canvas" id="canvas">
                <div class="canvas-placeholder">
                    <h3>👋 Arraste componentes aqui para começar</h3>
                    <p>Construa sua query visual arrastando blocos da esquerda</p>
                </div>
            </div>
        </div>

        <div class="action-bar">
            <button class="btn btn-primary" onclick="executeQuery()">▶️ Executar Query</button>
            <button class="btn btn-secondary" onclick="clearCanvas()">🗑️ Limpar</button>
            <div class="generated-query" id="generatedQuery">-- Sua query aparecerá aqui</div>
        </div>

        <div class="results-section" id="resultsSection">
            <h3 style="margin-bottom: 15px; color: #495057;">📊 Resultados</h3>
            <div id="resultsContent"></div>
        </div>
    </div>

    <script>
        let components = [];

        // Drag and Drop handlers
        document.querySelectorAll('.component').forEach(comp => {
            comp.addEventListener('dragstart', (e) => {
                e.dataTransfer.setData('componentType', e.target.dataset.type);
            });
        });

        const canvas = document.getElementById('canvas');
        canvas.addEventListener('dragover', (e) => {
            e.preventDefault();
            canvas.classList.add('drag-over');
        });

        canvas.addEventListener('dragleave', () => {
            canvas.classList.remove('drag-over');
        });

        canvas.addEventListener('drop', (e) => {
            e.preventDefault();
            canvas.classList.remove('drag-over');
            const type = e.dataTransfer.getData('componentType');
            addComponent(type);
        });

        function addComponent(type) {
            const id = Date.now();
            const component = { id, type, config: {} };
            components.push(component);
            renderCanvas();
            updateQuery();
        }

        function removeComponent(id) {
            components = components.filter(c => c.id !== id);
            renderCanvas();
            updateQuery();
        }

        function updateComponentConfig(id, key, value) {
            const comp = components.find(c => c.id === id);
            if (comp) {
                comp.config[key] = value;
                updateQuery();
            }
        }

        function renderCanvas() {
            const placeholder = canvas.querySelector('.canvas-placeholder');
            if (placeholder) placeholder.remove();

            canvas.innerHTML = components.map(comp => {
                const templates = {
                    select: `
                        <div class="field-group">
                            <label>Colunas:</label>
                            <input type="text" placeholder="*, id, nome, email"
                                   value="${comp.config.columns || '*'}"
                                   onchange="updateComponentConfig(${comp.id}, 'columns', this.value)">
                        </div>
                    `,
                    from: `
                        <div class="field-group">
                            <label>Tabela:</label>
                            <input type="text" placeholder="users, products, orders"
                                   value="${comp.config.table || ''}"
                                   onchange="updateComponentConfig(${comp.id}, 'table', this.value)">
                        </div>
                    `,
                    where: `
                        <div class="field-group">
                            <label>Campo:</label>
                            <input type="text" placeholder="age, status, category"
                                   value="${comp.config.field || ''}"
                                   onchange="updateComponentConfig(${comp.id}, 'field', this.value)">
                        </div>
                        <div class="field-group">
                            <label>Operador:</label>
                            <select onchange="updateComponentConfig(${comp.id}, 'operator', this.value)">
                                <option value="=" ${comp.config.operator === '=' ? 'selected' : ''}>=</option>
                                <option value="!=" ${comp.config.operator === '!=' ? 'selected' : ''}>!=</option>
                                <option value=">" ${comp.config.operator === '>' ? 'selected' : ''}>&gt;</option>
                                <option value="<" ${comp.config.operator === '<' ? 'selected' : ''}>&lt;</option>
                                <option value=">=" ${comp.config.operator === '>=' ? 'selected' : ''}>&gt;=</option>
                                <option value="<=" ${comp.config.operator === '<=' ? 'selected' : ''}>&lt;=</option>
                                <option value="LIKE" ${comp.config.operator === 'LIKE' ? 'selected' : ''}>LIKE</option>
                            </select>
                        </div>
                        <div class="field-group">
                            <label>Valor:</label>
                            <input type="text" placeholder="'value', 100, true"
                                   value="${comp.config.value || ''}"
                                   onchange="updateComponentConfig(${comp.id}, 'value', this.value)">
                        </div>
                    `,
                    join: `
                        <div class="field-group">
                            <label>Tipo:</label>
                            <select onchange="updateComponentConfig(${comp.id}, 'joinType', this.value)">
                                <option value="INNER" ${comp.config.joinType === 'INNER' ? 'selected' : ''}>INNER</option>
                                <option value="LEFT" ${comp.config.joinType === 'LEFT' ? 'selected' : ''}>LEFT</option>
                                <option value="RIGHT" ${comp.config.joinType === 'RIGHT' ? 'selected' : ''}>RIGHT</option>
                                <option value="FULL" ${comp.config.joinType === 'FULL' ? 'selected' : ''}>FULL</option>
                            </select>
                        </div>
                        <div class="field-group">
                            <label>Tabela:</label>
                            <input type="text" placeholder="other_table"
                                   value="${comp.config.table || ''}"
                                   onchange="updateComponentConfig(${comp.id}, 'table', this.value)">
                        </div>
                        <div class="field-group">
                            <label>Condição:</label>
                            <input type="text" placeholder="table1.id = table2.user_id"
                                   value="${comp.config.condition || ''}"
                                   onchange="updateComponentConfig(${comp.id}, 'condition', this.value)">
                        </div>
                    `,
                    groupby: `
                        <div class="field-group">
                            <label>Colunas:</label>
                            <input type="text" placeholder="category, status"
                                   value="${comp.config.columns || ''}"
                                   onchange="updateComponentConfig(${comp.id}, 'columns', this.value)">
                        </div>
                    `,
                    orderby: `
                        <div class="field-group">
                            <label>Coluna:</label>
                            <input type="text" placeholder="created_at, name"
                                   value="${comp.config.column || ''}"
                                   onchange="updateComponentConfig(${comp.id}, 'column', this.value)">
                        </div>
                        <div class="field-group">
                            <label>Ordem:</label>
                            <select onchange="updateComponentConfig(${comp.id}, 'order', this.value)">
                                <option value="ASC" ${comp.config.order === 'ASC' ? 'selected' : ''}>ASC</option>
                                <option value="DESC" ${comp.config.order === 'DESC' ? 'selected' : ''}>DESC</option>
                            </select>
                        </div>
                    `,
                    limit: `
                        <div class="field-group">
                            <label>Limite:</label>
                            <input type="number" placeholder="10, 100, 1000"
                                   value="${comp.config.limit || '100'}"
                                   onchange="updateComponentConfig(${comp.id}, 'limit', this.value)">
                        </div>
                    `
                };

                return `
                    <div class="query-component">
                        <div class="component-header">
                            <div class="component-title">${comp.type.toUpperCase()}</div>
                            <button class="component-remove" onclick="removeComponent(${comp.id})">✕</button>
                        </div>
                        <div class="component-content">
                            ${templates[comp.type] || ''}
                        </div>
                    </div>
                `;
            }).join('');

            if (components.length === 0) {
                canvas.innerHTML = `
                    <div class="canvas-placeholder">
                        <h3>👋 Arraste componentes aqui para começar</h3>
                        <p>Construa sua query visual arrastando blocos da esquerda</p>
                    </div>
                `;
            }
        }

        function updateQuery() {
            let query = '';

            components.forEach(comp => {
                switch(comp.type) {
                    case 'select':
                        query += `SELECT ${comp.config.columns || '*'}\n`;
                        break;
                    case 'from':
                        query += `FROM ${comp.config.table || 'table_name'}\n`;
                        break;
                    case 'where':
                        const op = comp.config.operator || '=';
                        query += `WHERE ${comp.config.field || 'field'} ${op} ${comp.config.value || 'value'}\n`;
                        break;
                    case 'join':
                        const joinType = comp.config.joinType || 'INNER';
                        query += `${joinType} JOIN ${comp.config.table || 'table_name'} ON ${comp.config.condition || 'condition'}\n`;
                        break;
                    case 'groupby':
                        query += `GROUP BY ${comp.config.columns || 'column'}\n`;
                        break;
                    case 'orderby':
                        const order = comp.config.order || 'ASC';
                        query += `ORDER BY ${comp.config.column || 'column'} ${order}\n`;
                        break;
                    case 'limit':
                        query += `LIMIT ${comp.config.limit || '100'}\n`;
                        break;
                }
            });

            document.getElementById('generatedQuery').textContent = query || '-- Sua query aparecerá aqui';
        }

        function clearCanvas() {
            components = [];
            renderCanvas();
            updateQuery();
            document.getElementById('resultsSection').classList.remove('active');
        }

        async function executeQuery() {
            const query = document.getElementById('generatedQuery').textContent;
            if (query === '-- Sua query aparecerá aqui') {
                alert('Adicione componentes para construir uma query primeiro!');
                return;
            }

            const resultsSection = document.getElementById('resultsSection');
            const resultsContent = document.getElementById('resultsContent');

            resultsSection.classList.add('active');
            resultsContent.innerHTML = '<div class="loading">⏳ Executando query...</div>';

            try {
                const response = await fetch('/query-builder/execute', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ query })
                });

                const data = await response.json();

                if (data.error) {
                    resultsContent.innerHTML = `<div class="error-message">❌ ${data.error}</div>`;
                } else {
                    renderResults(data);
                }
            } catch (error) {
                resultsContent.innerHTML = `<div class="error-message">❌ Erro: ${error.message}</div>`;
            }
        }

        function renderResults(data) {
            const resultsContent = document.getElementById('resultsContent');

            if (!data.rows || data.rows.length === 0) {
                resultsContent.innerHTML = '<p style="text-align: center; color: #6c757d;">Nenhum resultado encontrado</p>';
                return;
            }

            const columns = data.columns || Object.keys(data.rows[0]);

            let html = `
                <table class="results-table">
                    <thead>
                        <tr>${columns.map(col => `<th>${col}</th>`).join('')}</tr>
                    </thead>
                    <tbody>
                        ${data.rows.map(row => `
                            <tr>${columns.map(col => `<td>${row[col] !== null ? row[col] : '<em>null</em>'}</td>`).join('')}</tr>
                        `).join('')}
                    </tbody>
                </table>
                <p style="margin-top: 15px; color: #6c757d; font-size: 13px;">
                    ✅ ${data.rows.length} resultado(s) em ${data.execution_time_ms || 0}ms
                </p>
            `;

            resultsContent.innerHTML = html;
        }
    </script>
</body>
</html>"#;

/// Query execution request
#[derive(Debug, Deserialize)]
struct ExecuteQueryRequest {
    query: String,
}

/// Query execution response
#[derive(Debug, Serialize)]
struct ExecuteQueryResponse {
    columns: Vec<String>,
    rows: Vec<serde_json::Value>,
    execution_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Saved query template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub query: String,
    pub components: Vec<serde_json::Value>,
    pub created_at: String,
}

/// Query Builder UI endpoint
async fn query_builder_ui() -> impl IntoResponse {
    Html(QUERY_BUILDER_HTML)
}

/// Execute a visual query
async fn execute_query(
    State(_state): State<Arc<ConsoleState>>,
    Json(payload): Json<ExecuteQueryRequest>,
) -> Result<Json<ExecuteQueryResponse>, ConsoleError> {
    let start = std::time::Instant::now();

    // In production, this would execute against AvilaDB
    // For now, simulate with mock data
    let (columns, rows) = simulate_query_execution(&payload.query)?;

    let execution_time_ms = start.elapsed().as_millis() as u64;

    Ok(Json(ExecuteQueryResponse {
        columns,
        rows,
        execution_time_ms,
        error: None,
    }))
}

/// Simulate query execution (replace with real AvilaDB calls)
pub fn simulate_query_execution(
    query: &str,
) -> Result<(Vec<String>, Vec<serde_json::Value>), ConsoleError> {
    // Mock data based on query content
    if query.to_lowercase().contains("from users") {
        Ok((
            vec!["id".to_string(), "name".to_string(), "email".to_string()],
            vec![
                serde_json::json!({"id": 1, "name": "Alice", "email": "alice@example.com"}),
                serde_json::json!({"id": 2, "name": "Bob", "email": "bob@example.com"}),
                serde_json::json!({"id": 3, "name": "Charlie", "email": "charlie@example.com"}),
            ],
        ))
    } else if query.to_lowercase().contains("from products") {
        Ok((
            vec![
                "id".to_string(),
                "name".to_string(),
                "price".to_string(),
                "stock".to_string(),
            ],
            vec![
                serde_json::json!({"id": 1, "name": "Laptop", "price": 4999.99, "stock": 15}),
                serde_json::json!({"id": 2, "name": "Mouse", "price": 79.99, "stock": 150}),
                serde_json::json!({"id": 3, "name": "Keyboard", "price": 299.99, "stock": 75}),
            ],
        ))
    } else {
        Ok((
            vec!["result".to_string()],
            vec![serde_json::json!({"result": "Query executed successfully"})],
        ))
    }
}

/// List saved query templates
async fn list_templates(
    State(_state): State<Arc<ConsoleState>>,
) -> Result<Json<Vec<QueryTemplate>>, ConsoleError> {
    // In production, load from database
    let templates = vec![
        QueryTemplate {
            id: "tpl_1".to_string(),
            name: "Active Users".to_string(),
            description: "Get all active users from the last 30 days".to_string(),
            query: "SELECT * FROM users WHERE active = true AND last_login > @date".to_string(),
            components: vec![],
            created_at: "2024-11-23T10:00:00Z".to_string(),
        },
        QueryTemplate {
            id: "tpl_2".to_string(),
            name: "Top Products".to_string(),
            description: "Get top 10 products by sales".to_string(),
            query: "SELECT * FROM products ORDER BY sales DESC LIMIT 10".to_string(),
            components: vec![],
            created_at: "2024-11-23T11:00:00Z".to_string(),
        },
    ];

    Ok(Json(templates))
}

/// Save a query template
async fn save_template(
    State(_state): State<Arc<ConsoleState>>,
    Json(template): Json<QueryTemplate>,
) -> Result<Json<QueryTemplate>, ConsoleError> {
    // In production, save to database
    Ok(Json(template))
}

/// Create router for query builder
pub fn router(state: Arc<ConsoleState>) -> Router {
    Router::new()
        .route("/", get(query_builder_ui))
        .route("/execute", post(execute_query))
        .route("/templates", get(list_templates))
        .route("/templates", post(save_template))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_users_query() {
        let (columns, rows) = simulate_query_execution("SELECT * FROM users").unwrap();
        assert_eq!(columns.len(), 3);
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn test_simulate_products_query() {
        let (columns, rows) = simulate_query_execution("SELECT * FROM products").unwrap();
        assert_eq!(columns.len(), 4);
        assert_eq!(rows.len(), 3);
    }
}
