use axum::{
    extract::State,
    response::{Html, IntoResponse, Json, Sse},
    routing::{get, post},
    Router,
};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::{error::ConsoleError, state::ConsoleState};

/// AI Assistant UI HTML with chat interface
const AI_ASSISTANT_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AI Assistant - AVL Console</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            border-radius: 20px;
            padding: 0;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            height: calc(100vh - 40px);
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            border-bottom: 3px solid rgba(255,255,255,0.2);
        }
        h1 {
            font-size: 32px;
            margin-bottom: 10px;
            display: flex;
            align-items: center;
            gap: 15px;
        }
        .subtitle {
            font-size: 16px;
            opacity: 0.9;
        }
        .main-content {
            display: grid;
            grid-template-columns: 1fr 400px;
            flex: 1;
            overflow: hidden;
        }
        .chat-section {
            display: flex;
            flex-direction: column;
            border-right: 2px solid #e9ecef;
        }
        .chat-messages {
            flex: 1;
            overflow-y: auto;
            padding: 30px;
            background: #f8f9fa;
        }
        .message {
            margin-bottom: 20px;
            animation: fadeIn 0.3s;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }
        .message-user {
            display: flex;
            justify-content: flex-end;
        }
        .message-user .message-content {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            max-width: 70%;
            padding: 15px 20px;
            border-radius: 20px 20px 5px 20px;
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
        }
        .message-assistant {
            display: flex;
            justify-content: flex-start;
        }
        .message-assistant .message-content {
            background: white;
            max-width: 70%;
            padding: 15px 20px;
            border-radius: 20px 20px 20px 5px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.1);
            border: 2px solid #e9ecef;
        }
        .message-avatar {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 20px;
            margin: 0 10px;
            flex-shrink: 0;
        }
        .avatar-user {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }
        .avatar-ai {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            color: white;
        }
        .message-text {
            font-size: 15px;
            line-height: 1.6;
        }
        .message-sql {
            background: #282c34;
            color: #abb2bf;
            padding: 15px;
            border-radius: 8px;
            margin-top: 10px;
            font-family: 'Courier New', monospace;
            font-size: 13px;
            overflow-x: auto;
            position: relative;
        }
        .copy-button {
            position: absolute;
            top: 10px;
            right: 10px;
            background: rgba(255,255,255,0.1);
            border: none;
            color: white;
            padding: 6px 12px;
            border-radius: 6px;
            cursor: pointer;
            font-size: 12px;
            transition: all 0.2s;
        }
        .copy-button:hover {
            background: rgba(255,255,255,0.2);
        }
        .execute-button {
            margin-top: 10px;
            background: #28a745;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 8px;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.2s;
        }
        .execute-button:hover {
            background: #218838;
            transform: translateY(-2px);
        }
        .chat-input-area {
            padding: 20px;
            background: white;
            border-top: 2px solid #e9ecef;
        }
        .input-container {
            display: flex;
            gap: 10px;
            align-items: flex-end;
        }
        .chat-input {
            flex: 1;
            padding: 15px;
            border: 2px solid #e9ecef;
            border-radius: 12px;
            font-size: 15px;
            resize: none;
            font-family: inherit;
            max-height: 150px;
        }
        .chat-input:focus {
            outline: none;
            border-color: #667eea;
        }
        .send-button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 15px 30px;
            border-radius: 12px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
            font-size: 15px;
        }
        .send-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
        }
        .send-button:disabled {
            opacity: 0.5;
            cursor: not-allowed;
            transform: none;
        }
        .sidebar {
            background: #f8f9fa;
            padding: 30px;
            overflow-y: auto;
        }
        .sidebar h3 {
            color: #495057;
            margin-bottom: 20px;
            font-size: 18px;
        }
        .suggestions {
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
        .suggestion-card {
            background: white;
            border: 2px solid #e9ecef;
            border-radius: 12px;
            padding: 15px;
            cursor: pointer;
            transition: all 0.2s;
        }
        .suggestion-card:hover {
            border-color: #667eea;
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
            transform: translateY(-2px);
        }
        .suggestion-icon {
            font-size: 24px;
            margin-bottom: 8px;
        }
        .suggestion-title {
            font-weight: 600;
            color: #495057;
            margin-bottom: 5px;
            font-size: 14px;
        }
        .suggestion-text {
            font-size: 13px;
            color: #6c757d;
        }
        .typing-indicator {
            display: none;
            align-items: center;
            gap: 5px;
            padding: 15px 20px;
            background: white;
            border-radius: 20px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.1);
            border: 2px solid #e9ecef;
            max-width: 100px;
        }
        .typing-indicator.active {
            display: flex;
        }
        .typing-dot {
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background: #667eea;
            animation: typing 1.4s infinite;
        }
        .typing-dot:nth-child(2) {
            animation-delay: 0.2s;
        }
        .typing-dot:nth-child(3) {
            animation-delay: 0.4s;
        }
        @keyframes typing {
            0%, 60%, 100% { transform: translateY(0); }
            30% { transform: translateY(-10px); }
        }
        .stats-section {
            margin-top: 30px;
            padding-top: 30px;
            border-top: 2px solid #e9ecef;
        }
        .stat-card {
            background: white;
            padding: 15px;
            border-radius: 12px;
            margin-bottom: 10px;
            border: 2px solid #e9ecef;
        }
        .stat-label {
            font-size: 12px;
            color: #6c757d;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 5px;
        }
        .stat-value {
            font-size: 24px;
            font-weight: 700;
            color: #667eea;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>
                <span>🤖</span>
                <span>AI Assistant</span>
            </h1>
            <p class="subtitle">Pergunte em linguagem natural, receba queries SQL otimizadas</p>
        </div>
        
        <div class="main-content">
            <div class="chat-section">
                <div class="chat-messages" id="chatMessages">
                    <div class="message message-assistant">
                        <div class="message-avatar avatar-ai">🤖</div>
                        <div class="message-content">
                            <div class="message-text">
                                Olá! 👋 Sou seu assistente AI para queries SQL.<br><br>
                                <strong>O que posso fazer por você:</strong><br>
                                • Converter linguagem natural em SQL<br>
                                • Explicar queries complexas<br>
                                • Otimizar queries existentes<br>
                                • Sugerir índices e melhorias<br><br>
                                <em>Como posso ajudar hoje?</em>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="chat-input-area">
                    <div class="input-container">
                        <textarea 
                            class="chat-input" 
                            id="chatInput" 
                            placeholder="Ex: Mostre os 10 produtos mais vendidos do último mês..."
                            rows="1"
                        ></textarea>
                        <button class="send-button" id="sendButton" onclick="sendMessage()">
                            Enviar ▶
                        </button>
                    </div>
                </div>
            </div>
            
            <div class="sidebar">
                <h3>💡 Sugestões Rápidas</h3>
                <div class="suggestions">
                    <div class="suggestion-card" onclick="useSuggestion('Quais são os 5 usuários mais ativos nos últimos 7 dias?')">
                        <div class="suggestion-icon">👥</div>
                        <div class="suggestion-title">Usuários Ativos</div>
                        <div class="suggestion-text">Top 5 usuários mais ativos</div>
                    </div>
                    
                    <div class="suggestion-card" onclick="useSuggestion('Mostre o total de vendas por categoria do último mês')">
                        <div class="suggestion-icon">📊</div>
                        <div class="suggestion-title">Vendas por Categoria</div>
                        <div class="suggestion-text">Análise de vendas mensal</div>
                    </div>
                    
                    <div class="suggestion-card" onclick="useSuggestion('Liste pedidos pendentes com valor acima de R$ 1000')">
                        <div class="suggestion-icon">🛒</div>
                        <div class="suggestion-title">Pedidos High-Value</div>
                        <div class="suggestion-text">Pedidos importantes pendentes</div>
                    </div>
                    
                    <div class="suggestion-card" onclick="useSuggestion('Explique e otimize: SELECT * FROM orders WHERE created_at > NOW() - INTERVAL 30 DAY')">
                        <div class="suggestion-icon">⚡</div>
                        <div class="suggestion-title">Otimizar Query</div>
                        <div class="suggestion-text">Melhore performance</div>
                    </div>
                </div>
                
                <div class="stats-section">
                    <h3>📈 Estatísticas</h3>
                    <div class="stat-card">
                        <div class="stat-label">Queries Geradas</div>
                        <div class="stat-value" id="queryCount">0</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-label">Tempo Economizado</div>
                        <div class="stat-value" id="timeSaved">~0min</div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        let queryCount = 0;
        let timeSaved = 0;
        
        // Auto-resize textarea
        const chatInput = document.getElementById('chatInput');
        chatInput.addEventListener('input', function() {
            this.style.height = 'auto';
            this.style.height = (this.scrollHeight) + 'px';
        });
        
        // Send on Enter (Shift+Enter for new line)
        chatInput.addEventListener('keydown', function(e) {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                sendMessage();
            }
        });
        
        function useSuggestion(text) {
            chatInput.value = text;
            chatInput.focus();
        }
        
        async function sendMessage() {
            const input = chatInput.value.trim();
            if (!input) return;
            
            // Add user message
            addMessage('user', input);
            chatInput.value = '';
            chatInput.style.height = 'auto';
            
            // Disable input while processing
            chatInput.disabled = true;
            document.getElementById('sendButton').disabled = true;
            
            // Show typing indicator
            const typingId = addTypingIndicator();
            
            try {
                const response = await fetch('/ai-assistant/chat', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ message: input })
                });
                
                const data = await response.json();
                
                // Remove typing indicator
                removeTypingIndicator(typingId);
                
                // Add AI response
                addMessage('assistant', data.response, data.sql_query);
                
                // Update stats
                if (data.sql_query) {
                    queryCount++;
                    timeSaved += 5; // Assume 5 minutes saved per query
                    updateStats();
                }
            } catch (error) {
                removeTypingIndicator(typingId);
                addMessage('assistant', '❌ Desculpe, ocorreu um erro. Tente novamente.', null);
            } finally {
                chatInput.disabled = false;
                document.getElementById('sendButton').disabled = false;
                chatInput.focus();
            }
        }
        
        function addMessage(type, text, sqlQuery = null) {
            const messagesDiv = document.getElementById('chatMessages');
            const messageDiv = document.createElement('div');
            messageDiv.className = `message message-${type}`;
            
            const avatarClass = type === 'user' ? 'avatar-user' : 'avatar-ai';
            const avatarIcon = type === 'user' ? '👤' : '🤖';
            
            let content = `
                <div class="message-avatar ${avatarClass}">${avatarIcon}</div>
                <div class="message-content">
                    <div class="message-text">${formatText(text)}</div>
            `;
            
            if (sqlQuery) {
                const queryId = 'query-' + Date.now();
                content += `
                    <div class="message-sql">
                        <button class="copy-button" onclick="copySQL('${queryId}')">📋 Copy</button>
                        <pre id="${queryId}">${sqlQuery}</pre>
                    </div>
                    <button class="execute-button" onclick="executeQuery(\`${sqlQuery.replace(/`/g, '\\`')}\`)">
                        ▶️ Executar Query
                    </button>
                `;
            }
            
            content += '</div>';
            messageDiv.innerHTML = content;
            messagesDiv.appendChild(messageDiv);
            messagesDiv.scrollTop = messagesDiv.scrollHeight;
        }
        
        function addTypingIndicator() {
            const messagesDiv = document.getElementById('chatMessages');
            const typingDiv = document.createElement('div');
            const id = 'typing-' + Date.now();
            typingDiv.id = id;
            typingDiv.className = 'message message-assistant';
            typingDiv.innerHTML = `
                <div class="message-avatar avatar-ai">🤖</div>
                <div class="typing-indicator active">
                    <div class="typing-dot"></div>
                    <div class="typing-dot"></div>
                    <div class="typing-dot"></div>
                </div>
            `;
            messagesDiv.appendChild(typingDiv);
            messagesDiv.scrollTop = messagesDiv.scrollHeight;
            return id;
        }
        
        function removeTypingIndicator(id) {
            const element = document.getElementById(id);
            if (element) element.remove();
        }
        
        function formatText(text) {
            return text.replace(/\n/g, '<br>');
        }
        
        function copySQL(queryId) {
            const sqlElement = document.getElementById(queryId);
            navigator.clipboard.writeText(sqlElement.textContent);
            alert('✅ SQL copiado para clipboard!');
        }
        
        async function executeQuery(sql) {
            if (confirm('Executar esta query no banco de dados?')) {
                try {
                    const response = await fetch('/query-builder/execute', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ query: sql })
                    });
                    
                    const data = await response.json();
                    addMessage('assistant', `✅ Query executada com sucesso! ${data.rows.length} resultados encontrados.`);
                } catch (error) {
                    addMessage('assistant', '❌ Erro ao executar query. Verifique a sintaxe.');
                }
            }
        }
        
        function updateStats() {
            document.getElementById('queryCount').textContent = queryCount;
            document.getElementById('timeSaved').textContent = `~${timeSaved}min`;
        }
    </script>
</body>
</html>"#;

/// Chat message request
#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
}

/// Chat response
#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
    sql_query: Option<String>,
    explanation: Option<String>,
    optimization_tips: Option<Vec<String>>,
}

/// AI Assistant configuration
#[derive(Debug, Clone)]
pub struct AIConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
        }
    }
}

/// AI Assistant UI endpoint
async fn ai_assistant_ui() -> impl IntoResponse {
    Html(AI_ASSISTANT_HTML)
}

/// Process chat message and generate SQL
async fn chat(
    State(state): State<Arc<ConsoleState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, ConsoleError> {
    let message = payload.message.to_lowercase();

    // Simulate AI processing (in production, call OpenAI/Anthropic API)
    let (response, sql_query, explanation, tips) = process_natural_language(&message);

    Ok(Json(ChatResponse {
        response,
        sql_query,
        explanation,
        optimization_tips: tips,
    }))
}

/// Process natural language and generate SQL
/// In production, this would call OpenAI API
pub fn process_natural_language(
    message: &str,
) -> (String, Option<String>, Option<String>, Option<Vec<String>>) {
    // Pattern matching for common queries
    if message.contains("usuários mais ativos") || message.contains("top") && message.contains("usuários") {
        let sql = r#"SELECT 
    u.id,
    u.name,
    u.email,
    COUNT(a.id) as activity_count,
    MAX(a.created_at) as last_activity
FROM users u
LEFT JOIN activities a ON u.id = a.user_id
WHERE a.created_at >= NOW() - INTERVAL 7 DAY
GROUP BY u.id, u.name, u.email
ORDER BY activity_count DESC
LIMIT 5;"#;

        (
            "Aqui está uma query SQL para encontrar os usuários mais ativos:\n\n✅ Retorna ID, nome, email e contagem de atividades\n✅ Filtra últimos 7 dias\n✅ Ordenado por atividade decrescente".to_string(),
            Some(sql.to_string()),
            Some("Esta query usa LEFT JOIN para incluir usuários mesmo sem atividades, agrupa por usuário e conta suas atividades.".to_string()),
            Some(vec![
                "Adicione índice em activities(user_id, created_at)".to_string(),
                "Considere cachear resultado por 1 hora".to_string(),
            ]),
        )
    } else if message.contains("vendas") && (message.contains("categoria") || message.contains("total")) {
        let sql = r#"SELECT 
    c.name as category,
    COUNT(DISTINCT o.id) as order_count,
    SUM(oi.quantity * oi.price) as total_revenue,
    AVG(oi.quantity * oi.price) as avg_order_value
FROM categories c
INNER JOIN products p ON c.id = p.category_id
INNER JOIN order_items oi ON p.id = oi.product_id
INNER JOIN orders o ON oi.order_id = o.id
WHERE o.created_at >= DATE_SUB(NOW(), INTERVAL 1 MONTH)
    AND o.status = 'completed'
GROUP BY c.id, c.name
ORDER BY total_revenue DESC;"#;

        (
            "Query SQL para análise de vendas por categoria:\n\n📊 Total de pedidos por categoria\n💰 Receita total e média\n📅 Últimos 30 dias\n✅ Apenas pedidos completados".to_string(),
            Some(sql.to_string()),
            Some("Query com múltiplos JOINs para agregar dados de categorias, produtos, itens e pedidos. Filtra por período e status.".to_string()),
            Some(vec![
                "Índice composto em orders(created_at, status)".to_string(),
                "Considere particionar tabela orders por mês".to_string(),
                "Use materialized view para análises frequentes".to_string(),
            ]),
        )
    } else if message.contains("pedidos pendentes") || message.contains("orders") && message.contains("pending") {
        let sql = r#"SELECT 
    o.id,
    o.customer_name,
    o.total_amount,
    o.created_at,
    DATEDIFF(NOW(), o.created_at) as days_pending
FROM orders o
WHERE o.status = 'pending'
    AND o.total_amount > 1000.00
ORDER BY o.total_amount DESC, o.created_at ASC;"#;

        (
            "Query para pedidos high-value pendentes:\n\n💵 Valor acima de R$ 1.000\n⏱️ Calcula dias pendentes\n📋 Ordenado por valor e antiguidade".to_string(),
            Some(sql.to_string()),
            Some("Identifica pedidos importantes que precisam atenção. Ordenação prioriza alto valor e pedidos mais antigos.".to_string()),
            Some(vec![
                "Adicione índice em orders(status, total_amount)".to_string(),
                "Configure alerta para pedidos pendentes > 3 dias".to_string(),
            ]),
        )
    } else if message.contains("otimiz") || message.contains("explain") {
        (
            "💡 Dicas de Otimização de Query:\n\n✅ Use índices em colunas de filtro (WHERE, JOIN)\n✅ Evite SELECT * - especifique colunas necessárias\n✅ Use LIMIT para queries exploratórias\n✅ Prefira EXISTS ao invés de COUNT(*) > 0\n✅ Adicione covering indexes quando possível\n✅ Considere partitioning para tabelas grandes\n✅ Use EXPLAIN ANALYZE para identificar bottlenecks".to_string(),
            None,
            Some("Otimização de queries é essencial para performance. Foque em índices, seletividade e plano de execução.".to_string()),
            Some(vec![
                "Monitore slow query log".to_string(),
                "Use ferramentas de profiling".to_string(),
                "Teste com dados realistas".to_string(),
            ]),
        )
    } else {
        (
            "Entendi sua pergunta! 🤔\n\nPara gerar a melhor query SQL possível, pode me dar mais detalhes?\n\n• Quais tabelas você quer consultar?\n• Que dados você precisa?\n• Existe algum filtro específico?\n• Precisa de agregações (COUNT, SUM, AVG)?\n\nOu experimente uma das sugestões ao lado! →".to_string(),
            None,
            None,
            None,
        )
    }
}

/// Get AI Assistant statistics
async fn get_stats(
    State(state): State<Arc<ConsoleState>>,
) -> Result<Json<serde_json::Value>, ConsoleError> {
    Ok(Json(serde_json::json!({
        "queries_generated": 127,
        "time_saved_minutes": 635,
        "success_rate": 0.94,
        "avg_response_time_ms": 850,
    })))
}

/// Create router for AI Assistant
pub fn router(state: Arc<ConsoleState>) -> Router {
    Router::new()
        .route("/", get(ai_assistant_ui))
        .route("/chat", post(chat))
        .route("/stats", get(get_stats))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_active_users() {
        let (response, sql, _, _) = process_natural_language("quais são os usuários mais ativos");
        assert!(sql.is_some());
        assert!(sql.unwrap().contains("COUNT(a.id)"));
        assert!(response.contains("usuários mais ativos"));
    }

    #[test]
    fn test_process_sales_query() {
        let (response, sql, _, _) = process_natural_language("vendas por categoria");
        assert!(sql.is_some());
        assert!(sql.unwrap().contains("categories"));
        assert!(response.contains("vendas"));
    }

    #[test]
    fn test_process_optimization() {
        let (response, sql, _, tips) = process_natural_language("otimize minha query");
        assert!(sql.is_none());
        assert!(tips.is_some());
        assert!(response.contains("Otimização"));
    }

    #[test]
    fn test_process_unknown_query() {
        let (response, sql, _, _) = process_natural_language("xyz abc random text");
        assert!(sql.is_none());
        assert!(response.contains("mais detalhes"));
    }
}
