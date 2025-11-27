# 🚀 Avila Analytics - Sistema Completo de Analytics Comportamental

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](docker-compose.yml)

**Sistema de análise comportamental digital de alta performance, powered by AvilaDB** 🇧🇷

## ✨ Features

### 📊 Core Analytics
- ✅ Event tracking em tempo real
- ✅ Funnel analysis completo
- ✅ Cohort retention analysis
- ✅ RFM segmentation
- ✅ User profiling avançado
- ✅ Session management

### 🤖 Machine Learning
- ✅ Churn prediction
- ✅ Conversion probability
- ✅ Product recommendations
- ✅ Anomaly detection
- ✅ Next best action

### 🏭 Industry 4.0
- ✅ IoT sensor data ingestion
- ✅ Predictive maintenance
- ✅ OEE (Overall Equipment Effectiveness)
- ✅ Digital twins
- ✅ Production optimization
- ✅ Quality inspection
- ✅ Energy consumption tracking

### 🌐 API REST
- ✅ Event ingestion (single & batch)
- ✅ Analytics endpoints
- ✅ User profiling
- ✅ Real-time metrics
- ✅ WebSocket support
- ✅ Data export (CSV, JSON, Parquet)

### 🔐 Enterprise Features
- ✅ API key authentication
- ✅ Rate limiting
- ✅ CORS support
- ✅ Health checks
- ✅ Metrics & monitoring
- ✅ Docker & Kubernetes ready

## 🚀 Quick Start

### Pré-requisitos
- Rust 1.75+ (`rustup install stable`)
- Docker & Docker Compose
- AvilaDB Emulator (incluído no docker-compose)

### Instalação

```bash
# Clone o repositório
git clone https://github.com/seu-repo/avila-analises
cd avila-analises

# Copiar .env
cp .env.example .env

# Iniciar com Docker Compose
docker-compose up -d

# Ou compilar localmente
cargo build --release

# Executar servidor
cargo run --bin server --release
```

A API estará disponível em `http://localhost:3000`

## 📖 Uso

### 1. Ingerir Eventos

```bash
curl -X POST http://localhost:3000/api/v1/events \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-key-123" \
  -d '{
    "event": {
      "event_id": "evt_123",
      "user_id": "user_001",
      "session_id": "sess_456",
      "timestamp": "2025-11-26T10:00:00Z",
      "event_type": {
        "type": "PageView",
        "data": {
          "url": "/products",
          "title": "Products",
          "duration_ms": 5000
        }
      },
      "metadata": {},
      "context": {
        "device": {
          "device_type": "Desktop",
          "os": "Windows",
          "browser": "Chrome",
          "screen_resolution": [1920, 1080]
        },
        "location": {
          "country": "BR",
          "city": "São Paulo",
          "timezone": "America/Sao_Paulo",
          "ip_address": "192.168.1.1"
        },
        "referrer": null,
        "user_agent": "Mozilla/5.0...",
        "viewport": { "width": 1920, "height": 1080 }
      }
    }
  }'
```

### 2. Análise de Funil

```bash
curl -X POST http://localhost:3000/api/v1/analytics/funnel \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-key-123" \
  -d '{
    "funnel_name": "E-commerce Conversion",
    "steps": [
      { "name": "Home", "condition_type": "page_view", "condition_value": "/" },
      { "name": "Product", "condition_type": "page_view", "condition_value": "/product" },
      { "name": "Cart", "condition_type": "add_to_cart", "condition_value": "" },
      { "name": "Purchase", "condition_type": "purchase", "condition_value": "" }
    ]
  }'
```

### 3. WebSocket Real-time

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onopen = () => {
  // Subscrever a canais
  ws.send(JSON.stringify({
    type: 'Subscribe',
    payload: {
      channels: ['events', 'metrics', 'alerts']
    }
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

### 4. Industry 4.0 - IoT

```bash
curl -X POST http://localhost:3000/api/v1/industry40/iot/ingest \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-key-123" \
  -d '{
    "device_id": "sensor_001",
    "device_type": "Temperature",
    "timestamp": "2025-11-26T10:00:00Z",
    "metrics": {
      "temperature": 75.5,
      "humidity": 60.2,
      "pressure": 1013.25
    }
  }'
```

### 5. Exportar Dados

```bash
curl -X POST http://localhost:3000/api/v1/export \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-key-123" \
  -d '{
    "format": "csv",
    "filters": {
      "user_ids": ["user_001", "user_002"],
      "start_time": "2025-11-01T00:00:00Z",
      "end_time": "2025-11-30T23:59:59Z"
    }
  }'
```

## 🏗️ Arquitetura

```
┌─────────────────┐
│   Client Apps   │
└────────┬────────┘
         │
    ┌────▼────┐
    │  Axum   │  REST API + WebSocket
    │  Server │
    └────┬────┘
         │
    ┌────▼─────────┬──────────┬──────────┐
    │              │          │          │
┌───▼───┐   ┌─────▼────┐ ┌──▼──┐  ┌────▼─────┐
│Storage│   │Analytics │ │ ML  │  │Industry  │
│(Avila │   │ Engine   │ │Model│  │   4.0    │
│  DB)  │   └──────────┘ └─────┘  └──────────┘
└───────┘
```

## 📊 Endpoints Principais

### Core Analytics
- `POST /api/v1/events` - Ingestão de eventos
- `POST /api/v1/events/batch` - Batch ingestion
- `POST /api/v1/analytics/funnel` - Análise de funil
- `POST /api/v1/analytics/cohort` - Análise de cohort
- `GET /api/v1/analytics/overview` - Overview geral

### Users
- `GET /api/v1/users/:id` - Perfil do usuário
- `GET /api/v1/users/:id/segment` - Segmento RFM
- `GET /api/v1/users/:id/predictions` - Predições ML

### Industry 4.0
- `POST /api/v1/industry40/iot/ingest` - Telemetria IoT
- `POST /api/v1/industry40/maintenance/predict` - Manutenção preditiva
- `POST /api/v1/industry40/oee/calculate` - Calcular OEE
- `GET /api/v1/industry40/twin/:device_id` - Digital twin

### Export & Monitoring
- `POST /api/v1/export` - Exportar dados
- `GET /health` - Health check
- `GET /metrics` - Métricas Prometheus
- `GET /ws` - WebSocket connection

## 🔧 Configuração

### Variáveis de Ambiente

```bash
# Server
HOST=0.0.0.0
PORT=3000
RUST_LOG=info

# Storage
USE_AVILADB=true
AVILADB_ENDPOINT=http://localhost:8000
AVILADB_KEY=development-key
AVILADB_DATABASE=analytics
AVILADB_COLLECTION=events

# Security
API_KEYS=dev-key-123,prod-key-xyz

# Features
ENABLE_CORS=true
RATE_LIMIT_PER_MINUTE=1000
```

## 🐳 Docker

```bash
# Desenvolvimento
docker-compose up -d

# Produção
docker-compose -f docker-compose.prod.yml up -d

# Com monitoramento
docker-compose --profile monitoring up -d
```

## 📈 Performance

- **Throughput**: > 10,000 eventos/segundo
- **Latência**: < 10ms (p99)
- **Storage**: AvilaDB com 4MB por documento
- **Scalability**: Horizontal via Kubernetes

## 🧪 Testes

```bash
# Unit tests
cargo test

# Integration tests
cargo test --features full

# Benchmarks
cargo bench

# Coverage
cargo tarpaulin --out Html
```

## 📦 Deploy

### Kubernetes

```bash
# Apply manifests
kubectl apply -f k8s/

# Check status
kubectl get pods -n analytics

# Scale
kubectl scale deployment avila-analytics --replicas=3
```

### AWS ECS/Fargate

```bash
# Build e push para ECR
docker build -t analytics-api .
docker tag analytics-api:latest 123456.dkr.ecr.sa-east-1.amazonaws.com/analytics-api
docker push 123456.dkr.ecr.sa-east-1.amazonaws.com/analytics-api

# Deploy via Terraform
terraform apply
```

## 🤝 Contribuindo

1. Fork o projeto
2. Crie sua branch (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📝 Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.

## 🎯 Roadmap

- [ ] Vector search nativo (embeddings)
- [ ] AutoML pipeline
- [ ] A/B testing framework
- [ ] Multi-tenancy completo
- [ ] GraphQL API
- [ ] Mobile SDKs (iOS/Android)
- [ ] Dashboard web React
- [ ] Alertas via Slack/Email
- [ ] Integração Segment.io

## 📞 Suporte

- 📧 Email: support@avila.cloud
- 💬 Discord: [Avila Community](https://discord.gg/avila)
- 📚 Docs: [docs.avila.cloud](https://docs.avila.cloud)

---

**Feito com ❤️ por Avila Cloud Team** 🇧🇷
