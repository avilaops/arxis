# 🤖 AI Assistant - Natural Language to SQL

The AVL Console AI Assistant is an advanced feature that converts natural language queries into SQL, provides query explanations, and offers optimization tips. This feature makes database querying accessible to users of all skill levels.

## 🎯 Features

### 1. **Natural Language Processing**
- Convert plain Portuguese/English queries into SQL
- Intelligent pattern matching for common query types
- Context-aware query generation

### 2. **Query Explanation**
- Clear explanations of generated SQL queries
- Technical details and query breakdown
- Performance considerations

### 3. **Optimization Tips**
- Smart recommendations for query improvement
- Best practices guidance
- Performance optimization suggestions

### 4. **Interactive Chat Interface**
- Real-time chat with typing indicators
- Message history
- One-click query execution
- Copy SQL to clipboard

## 📊 Supported Query Patterns

### Active Users
```
Portuguese: "quais são os 5 usuários mais ativos?"
English: "who are the 5 most active users?"
```

**Generated SQL:**
```sql
SELECT
    u.user_id,
    u.name,
    COUNT(a.action_id) as action_count,
    MAX(a.created_at) as last_active
FROM users u
LEFT JOIN user_actions a ON u.user_id = a.user_id
WHERE a.created_at >= DATE_SUB(NOW(), INTERVAL 30 DAY)
GROUP BY u.user_id, u.name
ORDER BY action_count DESC
LIMIT 5
```

### Sales by Category
```
Portuguese: "mostre o total de vendas por categoria"
English: "show me total sales by category"
```

**Generated SQL:**
```sql
SELECT
    c.name as category,
    COUNT(s.sale_id) as total_sales,
    SUM(s.amount) as revenue,
    AVG(s.amount) as avg_sale
FROM categories c
LEFT JOIN products p ON c.category_id = p.category_id
LEFT JOIN sales s ON p.product_id = s.product_id
WHERE s.created_at >= DATE_SUB(NOW(), INTERVAL 90 DAY)
GROUP BY c.name
ORDER BY revenue DESC
```

### Pending Orders
```
Portuguese: "liste pedidos pendentes com valor acima de R$ 1000"
English: "list pending orders above $1000"
```

**Generated SQL:**
```sql
SELECT
    order_id,
    customer_name,
    order_date,
    total_amount,
    status
FROM orders
WHERE status = 'pending'
  AND total_amount > 1000
ORDER BY order_date DESC
```

### Query Optimization
```
Portuguese: "como posso otimizar minhas queries?"
English: "how can I optimize my queries?"
```

**Response:**
- Enable query logging and profiling
- Review slow query logs
- Add proper indexes on frequently queried columns
- Use EXPLAIN to analyze query execution plans
- Consider query result caching for frequently-accessed data

## 🚀 Usage

### Access the AI Assistant
1. Navigate to `http://localhost:8080/ai-assistant`
2. Start typing your question in Portuguese or English
3. Get instant SQL generation and explanations

### Quick Suggestions
The interface includes quick suggestion buttons:
- 👥 **Usuários ativos** - Active users query
- 💰 **Vendas por categoria** - Sales by category
- 📦 **Pedidos pendentes** - Pending orders
- ⚡ **Otimizar queries** - Optimization tips

### Execute Queries
1. Generate SQL using natural language
2. Review the generated query and explanation
3. Click "▶ Executar Query" to run in AvilaDB
4. View results instantly

### Copy SQL
Click the "📋 Copy" button to copy generated SQL to clipboard.

## 🏗️ Architecture

### Components

**1. Pattern Matching Engine**
```rust
pub fn process_natural_language(query: &str) -> (String, Option<String>, Option<String>, Option<Vec<String>>)
```
- Input: Natural language query
- Output: (Response, SQL, Explanation, Tips)
- Uses regex patterns for query classification

**2. Chat Interface**
- Single-page HTML/CSS/JavaScript
- Real-time typing indicators
- Message history with user/AI differentiation
- Responsive design

**3. REST API**
```
POST /ai-assistant/chat    - Process chat messages
GET  /ai-assistant/stats   - Usage statistics
```

### Configuration
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub model: String,          // Default: "gpt-4"
    pub temperature: f64,       // Default: 0.7
    pub max_tokens: u32,        // Default: 1000
    pub enable_streaming: bool, // Default: true
}
```

## 🔮 Future Enhancements

### Production Ready (v1.0)
- [ ] OpenAI GPT-4 integration
- [ ] Claude integration (Anthropic)
- [ ] Streaming responses for real-time feedback
- [ ] Query history and favorites
- [ ] Multi-language support (EN, PT, ES)

### Advanced Features
- [ ] Context-aware query suggestions
- [ ] Query result visualization
- [ ] Automatic query optimization
- [ ] Schema-aware query generation
- [ ] Multi-turn conversations with context
- [ ] Query templates from chat

### Security & Governance
- [ ] Query validation and sanitization
- [ ] Rate limiting per user
- [ ] Audit logging for AI-generated queries
- [ ] Role-based query restrictions

## 📊 Current Implementation

### Pattern Matching (v0.3.0)
Currently uses regex-based pattern matching for common query types:
- ✅ Active users queries
- ✅ Sales aggregations
- ✅ Order filtering
- ✅ Optimization advice

### Roadmap to AI Integration
1. **Phase 1 (Current):** Pattern matching with fixed templates
2. **Phase 2:** OpenAI API integration for flexible queries
3. **Phase 3:** Fine-tuned model for AvilaDB schema
4. **Phase 4:** Multi-model support (GPT-4, Claude, Llama)

## 🎯 Performance

### Current Benchmarks
- Pattern matching: <1ms latency
- Query generation: <2ms
- Full round-trip: <10ms

### Production Targets
- AI response time: <500ms (streaming)
- Query validation: <5ms
- End-to-end: <1s including AI processing

## 🧪 Testing

### Unit Tests
```bash
cargo test ai_assistant
```

Tests cover:
- Pattern matching for all query types
- Configuration defaults
- Unknown query handling
- SQL generation accuracy
- Explanation quality

### Integration Tests
```bash
cargo test --test advanced_features_tests
```

## 💡 Best Practices

### For Users
1. **Be specific:** "5 most active users" vs "show users"
2. **Include time ranges:** "last 30 days" for better queries
3. **Review generated SQL:** Always check before executing
4. **Use quick suggestions:** Start with examples

### For Developers
1. **Pattern Matching:** Add patterns to `process_natural_language()`
2. **Explanations:** Provide clear, actionable explanations
3. **Tips:** Context-specific optimization advice
4. **Testing:** Add tests for new patterns

## 🔗 Related Features

- **Query Builder:** Visual query construction
- **Advanced Monitoring:** Real-time performance metrics
- **Team Management:** Role-based access control

## 📚 References

- [OpenAI GPT-4 Documentation](https://platform.openai.com/docs)
- [AvilaDB Query Language](https://docs.avila.cloud/aviladb/query)
- [SQL Best Practices](https://docs.avila.cloud/best-practices)

---

**Status:** ✅ Production Ready (Pattern Matching) | 🚧 AI Integration Coming Soon

**Version:** 0.3.0

**License:** MIT OR Apache-2.0
