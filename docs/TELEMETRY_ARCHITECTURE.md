# Arquitetura de Telemetria - Arxis Project

## Visão Geral

O projeto Arxis utiliza uma **arquitetura em camadas** para telemetria e observabilidade, separando responsabilidades entre bibliotecas científicas core e camadas de integração de plataforma.

## Estrutura de Duas Camadas

### 🔬 Camada Científica: `avila-telemetry`

**Propósito**: Biblioteca científica pura para análise de séries temporais e telemetria.

**Características**:
- ✅ **Zero dependências de infraestrutura** (apenas ndarray, statrs, FFT)
- ✅ **Reutilizável** em qualquer contexto (CLI, API, embedded, científico)
- ✅ **Foco em algoritmos**: análise temporal, forecasting, anomaly detection
- ✅ **Padrões NASA**: Data Quality Assessment com threshold ≥0.95

**Módulos**:
```
avila-telemetry/
├── anomaly.rs          # Detecção de anomalias (Z-score, IQR, isolation forest)
├── decomposition.rs    # STL, trend/seasonal decomposition
├── forecasting.rs      # ARIMA, exponential smoothing, Prophet-like
├── features.rs         # Feature engineering para séries temporais
├── time_series.rs      # Core: TimeSeries struct + operações
├── observability.rs    # Data Quality Assessment (NASA standards)
└── models/             # Modelos estatísticos (ARIMA, etc)
```

**Casos de Uso**:
- Análise científica de dados do LISA (ondas gravitacionais)
- Processamento de telemetria de telescópios/sensores
- Detecção de anomalias em séries temporais arbitrárias
- Forecasting de métricas científicas
- Qualquer aplicação que precise de análise temporal **sem** infraestrutura AVX

**Testes**: 22 testes de integração

---

### 🌐 Camada de Integração: `avx-telemetry`

**Propósito**: Wrapper fino que adiciona infraestrutura AVX ao `avila-telemetry`.

**Características**:
- ✅ **Depende de** `avila-telemetry` (path dependency)
- ✅ **Re-exporta tipos** para o ecossistema AVX
- ✅ **Adiciona observabilidade**: tracing estruturado, contexto AVX
- ✅ **Middleware**: integração com axum/tower para APIs
- ✅ **Storage**: persistência e cache para telemetria

**Estrutura**:
```rust
// avx-telemetry/src/lib.rs
pub use avila_telemetry::{
    anomaly::{Anomaly, AnomalyDetector},
    forecasting::Forecaster,
    models::ARIMA,
    observability::DataQualityAssessment,
    TelemetryError, TimeSeries,
};

pub struct AvxContext {
    pub stack: String,      // "Avx"
    pub layer: String,      // "deep", "platform", "edge"
    pub env: String,        // "prod", "staging", "dev"
    pub cluster: String,    // "AVL-BR", "AVL-US", "AVL-EU"
    pub mesh: String,       // "internal", "external"
}

pub struct AvxMetrics {
    // Wrapper que adiciona contexto AVX
}
```

**Funcionalidades Adicionais**:
1. **Tracing Estruturado**: JSON logs com contexto AVX
   ```rust
   init_tracing(&ctx);  // Configura tracing-subscriber
   ```

2. **Middleware (feature flag)**:
   - Headers AVX customizados
   - Integração com axum/tower
   - Request/response logging

3. **Storage**:
   - Persistência de métricas
   - Cache de forecasts
   - Histórico de anomalias

**Casos de Uso**:
- `avx-gateway`: API gateway com observabilidade
- `avx-api-core`: Endpoints de telemetria/health
- Serviços AVX que precisam tracking + contexto de plataforma
- Integração com AvilaDB para armazenar métricas

**Testes**: Cobertura completa de wrapper + integration tests

---

## Padrão de Nomenclatura

### Prefixo `avila-*`
**Bibliotecas Core/Científicas**:
- `avila-math`: Kernel matemático (tensores, geometria, quaternions)
- `avila-telemetry`: Time series analysis + forecasting
- Características: reutilizáveis, zero infraestrutura, foco algorítmico

### Prefixo `avx-*`
**Camadas de Plataforma/Integração**:
- `avx-telemetry`: Wrapper com observabilidade AVX
- `avx-gateway`: API gateway
- `avx-api-core`: Core da API AVX
- `avx-quantum-render`: Rendering QED com integração AVX
- `avx-image`: Computer Vision + OCR
- Características: dependem de `avila-*`, adicionam infraestrutura, específicas da plataforma

---

## Fluxo de Dados

```
┌─────────────────────────────────────────────────────┐
│  Aplicação (avx-gateway, avx-api-core)              │
└───────────────────┬─────────────────────────────────┘
                    │ usa
                    ▼
┌─────────────────────────────────────────────────────┐
│  avx-telemetry                                      │
│  ┌──────────────────────────────────────────────┐  │
│  │ AvxContext + Tracing + Middleware + Storage  │  │
│  └───────────────────┬──────────────────────────┘  │
│                      │ re-exports + wraps           │
│                      ▼                              │
│  ┌──────────────────────────────────────────────┐  │
│  │ pub use avila_telemetry::*                   │  │
│  └──────────────────────────────────────────────┘  │
└───────────────────┬─────────────────────────────────┘
                    │ depende
                    ▼
┌─────────────────────────────────────────────────────┐
│  avila-telemetry (Core Científico)                  │
│  ┌──────────────────────────────────────────────┐  │
│  │ TimeSeries, AnomalyDetector, ARIMA           │  │
│  │ STL, Forecaster, DataQualityAssessment       │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

---

## Exemplo de Uso

### 1. Uso Científico (sem infraestrutura AVX)
```rust
use avila_telemetry::{TimeSeries, AnomalyDetector};

// Análise pura de séries temporais
let data = vec![10.0, 12.0, 11.0, 100.0]; // outlier
let ts = TimeSeries::new(data);
let detector = AnomalyDetector::new(3.0, 1.5);
let anomalies = detector.detect_zscore(&ts)?;
```

### 2. Uso em Serviço AVX (com observabilidade)
```rust
use avx_telemetry::{AvxContext, AvxMetrics, init_tracing};

// Setup contexto AVX
let ctx = AvxContext {
    stack: "Avx".into(),
    layer: "deep".into(),
    env: "prod".into(),
    cluster: "AVL-BR".into(),
    mesh: "internal".into(),
};

init_tracing(&ctx);  // JSON logs estruturados

// Track latências com contexto AVX
let metrics = AvxMetrics::new();
let anomalies = metrics.track_latencies(vec![10.0, 12.0, 11.0, 100.0])?;

// Quality assessment (NASA standards)
let quality = metrics.assess_quality(0.99, 0.98, 0.97, 50, 0.96);
assert!(quality.meets_nasa_standards());

// Forecast com ARIMA
let predictions = metrics.forecast_metric(historical_data, 5)?;
```

### 3. Uso em Gateway (com middleware)
```rust
use avx_telemetry::middleware::AvxHeaderLayer;
use axum::Router;

let app = Router::new()
    .route("/health", get(health_handler))
    .layer(AvxHeaderLayer::new()); // Adiciona headers AVX
```

---

## Decisões Arquiteturais

### Por que Duas Camadas?

1. **Separação de Responsabilidades**:
   - Core científico não deve saber sobre HTTP, tracing, storage
   - Infraestrutura não deve reimplementar algoritmos científicos

2. **Reutilização**:
   - `avila-telemetry` pode ser usado em CLI, embedded, notebooks Jupyter
   - Não força dependências pesadas (axum, tower, tracing)

3. **Testabilidade**:
   - Algoritmos científicos testáveis isoladamente
   - Infraestrutura testável com mocks

4. **Manutenibilidade**:
   - Mudanças em algoritmos não afetam infraestrutura
   - Mudanças em infraestrutura não afetam algoritmos

### Alternativas Consideradas

❌ **Monolito**: `avila-telemetry` com tudo (algoritmos + infraestrutura)
- Problema: dependências pesadas, acoplamento, difícil reutilizar

❌ **Features Flags**: `avila-telemetry` com features para infraestrutura
- Problema: complexidade de build, #[cfg] hell, API confusa

✅ **Camadas Separadas**: Core + Wrapper
- Vantagens: clean, testável, reutilizável, manutenível

---

## Integrações

### AvilaDB
`avx-telemetry` pode armazenar métricas no AvilaDB:
```rust
// Armazenar forecasts
db.collection("forecasts").insert({
    "metric": "latency",
    "predictions": predictions,
    "timestamp": Utc::now(),
    "cluster": ctx.cluster,
})?;

// Query histórico de anomalias
db.collection("anomalies")
    .query("SELECT * FROM anomalies WHERE cluster = @cluster")
    .param("cluster", "AVL-BR")
    .execute()?;
```

### LISA Pipeline
`avila-telemetry` é usado para análise de dados do LISA:
```rust
// src/physics/lisa_analysis.rs
use avila_telemetry::{TimeSeries, forecasting::Forecaster};

let strain_data = lisa_pipeline.get_strain()?;
let ts = TimeSeries::new(strain_data);
let forecast = ts.forecast_arima(1, 1, 1, 100)?;
```

---

## Roadmap

### Curto Prazo (Q4 2025)
- [ ] Adicionar `avx-telemetry/storage` para AvilaDB
- [ ] Implementar middleware completo (AvxHeaderLayer)
- [ ] Dashboards de métricas com avx-quantum-render

### Médio Prazo (Q1 2026)
- [ ] Streaming de métricas (Kafka/Pulsar integration)
- [ ] ML-based anomaly detection (isolation forest, LSTM)
- [ ] Distributed tracing (OpenTelemetry)

### Longo Prazo (2026)
- [ ] Auto-tuning de thresholds com RL
- [ ] Forecasting ensemble (ARIMA + Prophet + LSTM)
- [ ] Real-time alerting com WebSockets

---

## Referências

- [AvilaDB Best Practices](../CONTACT.md)
- [LISA Project Status](../LISA_PROJECT_STATUS.md)
- [Scientific Architecture](../SCIENTIFIC_ARCHITECTURE.md)
- [avila-telemetry README](../avila-telemetry/README.md)

**Mantido por**: Arxis Team @ Avilaops
**Última atualização**: 2025-11-20
