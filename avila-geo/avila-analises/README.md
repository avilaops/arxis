# Sistema de Análise Comportamental Digital

Sistema completo de análise de comportamento digital em Rust, com captura, processamento, análise e predição de comportamento de usuários.

## 🚀 Características

- **Captura de Eventos**: Sistema robusto de tracking com enriquecimento de contexto
- **Análise de Funil**: Conversão por etapas com identificação de drop-off
- **Análise de Cohort**: Retenção e LTV por coortes temporais
- **Segmentação RFM**: Classificação automática de usuários (Champions, At Risk, etc.)
- **Machine Learning**: Predição de churn, conversão e recomendações
- **Dashboard em Tempo Real**: Monitoramento com alertas automáticos
- **Performance**: Processamento assíncrono com Tokio
- **Escalabilidade**: Pronto para integração com AvilaDB

## 📦 Instalação

```bash
cargo build --release
```

## 🎯 Uso

### Execução Simples

```bash
cargo run
```

### Exemplo de Código

```rust
use avila_analises::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Criar tracker
    let mut tracker = tracker::BehaviorTracker::new(30);

    // 2. Rastrear evento
    let event = BehaviorEvent {
        event_id: uuid::Uuid::new_v4().to_string(),
        user_id: "user123".to_string(),
        session_id: "session456".to_string(),
        timestamp: chrono::Utc::now(),
        event_type: EventType::PageView {
            url: "/products".to_string(),
            title: "Products".to_string(),
            duration_ms: 5000,
        },
        metadata: HashMap::new(),
        context: create_context(),
    };

    tracker.track_event(event).await?;

    // 3. Analisar funil
    let funnel_analyzer = funnel::FunnelAnalyzer::new();
    let funnel = funnel::FunnelBuilder::new("Checkout Flow")
        .add_page_view("Landing", "/")
        .add_cart("Add to Cart")
        .add_purchase("Purchase")
        .build();

    let events = tracker.get_event_store().get_all_events();
    let analysis = funnel_analyzer.analyze_funnel(&funnel, &events);

    // 4. Segmentar usuários
    let segmentation = segmentation::UserSegmentation::with_default_segments();
    let profiles = generate_user_profiles(&events);
    let segments = segmentation.classify_user(&profiles[0]);

    // 5. Predições
    let mut predictor = prediction::BehaviorPredictor::new();
    predictor.train_recommendation_model(&events);

    let churn_risk = predictor.predict_churn(&profiles[0]);
    let recommendations = predictor.recommend_products("user123", 5);

    Ok(())
}
```

## 🏗️ Arquitetura

### Módulos Principais

```
avila-analises/
├── models.rs          # Estruturas de dados
├── tracker.rs         # Sistema de captura
├── funnel.rs          # Análise de funil
├── cohort.rs          # Análise de coortes
├── segmentation.rs    # Segmentação RFM
├── prediction.rs      # Machine Learning
├── dashboard.rs       # Dashboard em tempo real
└── main.rs            # Exemplo de uso
```

### Fluxo de Dados

```
Evento → Validação → Enriquecimento → Processamento Real-Time
                                              ↓
                                        Event Store
                                              ↓
                              ┌───────────────┴───────────────┐
                              ↓                               ↓
                        Análise Batch                  Dashboard RT
                              ↓
                    ┌─────────┴─────────┐
                    ↓                   ↓
                  Funil              Cohort
                    ↓                   ↓
              Segmentação           Predição
```

## 📊 Tipos de Análise

### 1. Análise de Funil

Rastreia conversão através de etapas definidas:

```rust
let funnel = FunnelBuilder::new("E-commerce")
    .add_page_view("Product Page", "/product")
    .add_cart("Add to Cart")
    .add_page_view("Checkout", "/checkout")
    .add_purchase("Purchase Complete")
    .build();
```

**Métricas:**
- Taxa de conversão por etapa
- Tempo médio entre etapas
- Pontos de maior abandono

### 2. Análise de Cohort

Acompanha grupos de usuários ao longo do tempo:

```rust
let cohorts = analyzer.create_cohorts(&users, CohortPeriod::Weekly);
let analysis = analyzer.analyze_cohorts(&cohorts, &events, 12);
```

**Métricas:**
- Retenção por período
- Revenue per cohort
- Lifetime Value (LTV)
- Churn por cohort

### 3. Segmentação RFM

Classifica usuários em segmentos:

- **Champions**: Alto valor, compram frequentemente
- **Loyal**: Compras regulares
- **At Risk**: Risco de churn
- **New Customers**: Recém adquiridos
- **High Spenders**: Alto ticket médio
- **Window Shoppers**: Navegam mas não compram
- **Lost**: Inativos há muito tempo

### 4. Predições com ML

#### Churn Prediction
```rust
let churn_risk = predictor.predict_churn(&profile);
// 0.0 = Sem risco, 1.0 = Alto risco
```

#### Conversion Prediction
```rust
let conversion_prob = predictor.predict_conversion(&profile);
// Probabilidade de conversão
```

#### Recommendation Engine
```rust
let recommendations = predictor.recommend_products("user_id", 10);
// Top 10 produtos recomendados
```

## 🎯 Integração com AvilaDB

### Armazenamento de Eventos

```rust
use aviladb::{AvilaClient, Collection};

// Conectar ao AvilaDB
let client = AvilaClient::connect("http://localhost:8000").await?;
let db = client.database("analytics").await?;
let events_collection = db.collection("behavior_events").await?;

// Armazenar evento
let doc = serde_json::to_value(&event)?;
events_collection.insert(doc).await?;
```

### Consultas Eficientes

```rust
// Query com partition key
let user_events = events_collection
    .query("SELECT * FROM events WHERE userId = @user")
    .param("user", "user123")
    .execute()
    .await?;

// Agregação para métricas
let daily_stats = events_collection
    .query(r#"
        SELECT
            DATE(timestamp) as date,
            COUNT(*) as events,
            COUNT(DISTINCT userId) as users,
            SUM(CASE WHEN eventType = 'Purchase' THEN amount ELSE 0 END) as revenue
        FROM events
        WHERE timestamp > @start_date
        GROUP BY DATE(timestamp)
    "#)
    .param("start_date", start_date)
    .execute()
    .await?;
```

### Schema Recomendado

```json
{
  "id": "evt_123",
  "userId": "user_456",
  "sessionId": "sess_789",
  "timestamp": "2024-11-25T10:30:00Z",
  "eventType": "page_view",
  "data": {
    "url": "/products",
    "title": "Products",
    "duration_ms": 5000
  },
  "context": {
    "device": "desktop",
    "os": "Windows",
    "browser": "Chrome",
    "country": "BR"
  }
}
```

**Partition Key**: `userId` (ou HPK: `/userId/sessionId`)

## 🔧 Configuração

### Variáveis de Ambiente

```bash
# AvilaDB
AVILADB_ENDPOINT=http://localhost:8000
AVILADB_DATABASE=analytics
AVILADB_KEY=your-key-here

# Sistema
SESSION_TIMEOUT_MINUTES=30
ALERT_EMAIL=alerts@company.com
LOG_LEVEL=info
```

### Tuning de Performance

```rust
// Ajustar batch size para escrita
const BATCH_SIZE: usize = 1000;

// Configurar thread pool
tokio::runtime::Builder::new_multi_thread()
    .worker_threads(8)
    .enable_all()
    .build()?;
```

## 📈 Métricas e KPIs

### Métricas de Engajamento
- Usuários ativos (DAU/MAU)
- Tempo médio de sessão
- Páginas por sessão
- Taxa de bounce

### Métricas de Conversão
- Taxa de conversão global
- Conversão por funil
- Ticket médio
- Revenue per user

### Métricas de Retenção
- Retenção D1, D7, D30
- Churn rate
- Lifetime Value (LTV)
- Customer Acquisition Cost (CAC)

## 🧪 Testes

```bash
# Rodar todos os testes
cargo test

# Testes específicos
cargo test tracker::tests
cargo test funnel::tests
cargo test prediction::tests

# Com output detalhado
cargo test -- --nocapture
```

## 🚀 Deployment

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/avila-analises /usr/local/bin/
CMD ["avila-analises"]
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: behavior-analytics
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: analytics
        image: avila-analises:latest
        env:
        - name: AVILADB_ENDPOINT
          value: "http://aviladb-service:8000"
```

## 📊 Dashboard Example Output

```
╔══════════════════════════════════════════════════╗
║        BEHAVIOR ANALYTICS DASHBOARD              ║
╚══════════════════════════════════════════════════╝

📊 Real-Time Metrics:
  👥 Active Users (last minute): 42
  ⚡ Events/Second: 15.3
  💰 Revenue Today: R$ 12,450.50
  📈 Conversion Rate Today: 3.2%

🔥 Top Pages Today:
  1. /products (1,234 views)
  2. /home (987 views)
  3. /checkout (456 views)
```

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o projeto
2. Crie uma branch para sua feature
3. Commit suas mudanças
4. Push para a branch
5. Abra um Pull Request

## 📄 Licença

MIT License - veja LICENSE para detalhes

## 🔗 Links Úteis

- [AvilaDB Documentation](https://docs.avila.cloud/aviladb)
- [Best Practices](https://docs.avila.cloud/aviladb/best-practices)
- [API Reference](https://docs.avila.cloud/api)

## 💡 Casos de Uso

### E-commerce
- Análise de jornada de compra
- Recomendações personalizadas
- Detecção de abandono de carrinho
- Segmentação de clientes

### SaaS
- Onboarding flow analysis
- Feature adoption tracking
- Churn prediction
- User engagement scoring

### Gaming
- Player behavior tracking
- Session analysis
- Monetization optimization
- Retention cohorts

### Media & Content
- Content consumption patterns
- Engagement metrics
- Personalized recommendations
- Audience segmentation

---

**Desenvolvido com ❤️ para a plataforma AvilaDB** 🇧🇷
