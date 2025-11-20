# AVX Telemetry - Roadmap de Desenvolvimento

**Projeto**: Integração completa de observabilidade e telemetria no ecossistema AVX
**Owner**: Nicolas Ávila (nicolas@avila.inc)
**Status**: Em desenvolvimento ativo
**Última atualização**: 20 de novembro de 2025

---

## 📋 Visão Geral

O **avx-telemetry** é a camada de observabilidade do Avila Experience Fabric (AVX), integrando:
- **avila-telemetry**: Time series analysis, anomaly detection, forecasting
- **Distributed tracing**: Rastreamento de requisições através de microserviços
- **Metrics**: Latência, throughput, error rates, saturation
- **Alerting**: Detecção automática de anomalias e degradação de qualidade

---

## 🎯 Objetivos

### Curto Prazo (1-2 semanas)
1. ✅ Integrar avila-telemetry no avx-telemetry
2. ✅ Implementar AvxMetrics com latency tracking e anomaly detection
3. 🔄 Corrigir erros de compilação e testes
4. 🔄 Adicionar endpoints de métricas em avx-gateway e avx-api-core
5. ⏳ Implementar middleware de latency tracking automático

### Médio Prazo (3-4 semanas)
1. ⏳ Integrar com AvilaDB para armazenamento de métricas históricas
2. ⏳ Dashboard de observabilidade (Grafana ou custom)
3. ⏳ Sistema de alertas com thresholds configuráveis
4. ⏳ Forecasting automático de recursos (CPU, memória, requests)
5. ⏳ Exportar métricas para Prometheus/OpenTelemetry

### Longo Prazo (2-3 meses)
1. ⏳ ML-based anomaly detection (além de Z-score e IQR)
2. ⏳ Auto-scaling baseado em forecasting
3. ⏳ Distributed tracing completo (OpenTelemetry)
4. ⏳ SLO/SLA tracking e error budgets
5. ⏳ Chaos engineering integration para testes

---

## 🛠️ Tarefas Imediatas (Para Você)

### 1. Corrigir Compilação do avx-telemetry ⚠️ URGENTE

**Problema atual**: `cargo build -p avx-telemetry` falhando com erros de tipo.

**Tarefas**:
```bash
# 1. Verificar estado atual
cd C:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\1.2.7 - Identidade visual\Arxis
cargo build -p avx-telemetry

# 2. Corrigir tipos de retorno em lib.rs
# - Mudar String para TelemetryError onde necessário
# - Usar .forecast() ao invés de .predict()
# - Garantir que todos os re-exports estão corretos

# 3. Compilar e testar
cargo test -p avx-telemetry

# 4. Testar integração
cargo build -p avx-gateway
cargo build -p avx-api-core
```

**Arquivos para revisar**:
- `avx-telemetry/src/lib.rs` (já corrigido parcialmente)
- `avx-telemetry/Cargo.toml` (verificar dependências)
- `avx-gateway/src/main.rs` (endpoints de métricas)
- `avx-api-core/src/main.rs` (endpoint de forecast)

---

### 2. Adicionar Testes Unitários

**Local**: `avx-telemetry/src/lib.rs`

**Adicionar testes para**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ✅ Já existe: test_avx_metrics_tracking
    // ✅ Já existe: test_quality_assessment

    // ⏳ ADICIONAR:
    #[test]
    fn test_forecast_metric() {
        // Testar forecasting com dados históricos
    }

    #[test]
    fn test_avx_context_serialization() {
        // Garantir que AvxContext serializa corretamente para JSON
    }

    #[test]
    fn test_init_tracing() {
        // Verificar inicialização de tracing sem panic
    }
}
```

---

### 3. Implementar Middleware de Latency Tracking

**Local**: `avx-gateway/src/main.rs`

**Objetivo**: Automaticamente coletar latências de todas as requisições.

```rust
// Criar uma nova estrutura que armazena latências
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct LatencyCollector {
    latencies: Arc<Mutex<Vec<f64>>>,
}

impl LatencyCollector {
    fn new() -> Self {
        Self {
            latencies: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn record(&self, latency_ms: f64) {
        if let Ok(mut latencies) = self.latencies.lock() {
            latencies.push(latency_ms);

            // Manter apenas últimas 1000 requisições
            if latencies.len() > 1000 {
                latencies.drain(0..100);
            }
        }
    }

    fn get_anomalies(&self, metrics: &AvxMetrics) -> Result<Vec<Anomaly>, TelemetryError> {
        let latencies = self.latencies.lock().unwrap();
        metrics.track_latencies(latencies.clone())
    }
}
```

**Integrar no middleware**:
```rust
// Modificar AvxHeaderMiddleware para incluir LatencyCollector
// No método call(), registrar tempo de início e fim
// Após response, chamar collector.record(duration_ms)
```

---

### 4. Endpoint de Métricas em Tempo Real

**Local**: `avx-gateway/src/main.rs`

**Adicionar novo endpoint**:
```rust
async fn metrics_realtime(
    State(state): State<AppState>
) -> axum::Json<serde_json::Value> {
    // Coletar métricas dos últimos 5 minutos
    let latencies = state.collector.latencies.lock().unwrap();

    let stats = if !latencies.is_empty() {
        let ts = TimeSeries::new(latencies.clone());
        let s = ts.statistics();
        serde_json::json!({
            "mean_ms": s.mean,
            "std_dev_ms": s.std_dev,
            "min_ms": s.min,
            "max_ms": s.max,
            "count": latencies.len()
        })
    } else {
        serde_json::json!({
            "error": "No data available"
        })
    };

    axum::Json(serde_json::json!({
        "service": "avx-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "metrics": stats
    }))
}

// Adicionar rota
.route("/metrics/realtime", get(metrics_realtime))
```

---

### 5. Integração com AvilaDB (Armazenamento de Métricas)

**Criar novo módulo**: `avx-telemetry/src/storage.rs`

```rust
use avila_telemetry::{TimeSeries, Anomaly};
use chrono::{DateTime, Utc};

pub struct MetricsStorage {
    // TODO: Integrar com AvilaDB client
    service_name: String,
}

impl MetricsStorage {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    /// Salvar métricas no AvilaDB
    pub async fn save_metrics(
        &self,
        timestamp: DateTime<Utc>,
        latencies: Vec<f64>,
        anomalies: Vec<Anomaly>,
    ) -> Result<(), String> {
        // TODO: Implementar quando AvilaDB Rust SDK estiver pronto
        // Estrutura do documento:
        // {
        //   "service": "avx-gateway",
        //   "timestamp": "2025-11-20T12:00:00Z",
        //   "metrics": {
        //     "latencies": [10.2, 11.5, ...],
        //     "anomalies": [...],
        //     "quality_score": 0.98
        //   }
        // }

        Ok(())
    }

    /// Buscar métricas históricas
    pub async fn query_metrics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<TimeSeries>, String> {
        // TODO: Query AvilaDB por timestamp range
        Ok(vec![])
    }
}
```

---

## 📊 Estrutura de Dados para Métricas

### Documento de Métrica (AvilaDB)

```json
{
  "id": "metric-avx-gateway-20251120-120000",
  "service": "avx-gateway",
  "timestamp": "2025-11-20T12:00:00Z",
  "context": {
    "stack": "Avx",
    "layer": "deep",
    "env": "prod",
    "cluster": "AVL-BR",
    "mesh": "internal"
  },
  "metrics": {
    "latency": {
      "mean_ms": 12.5,
      "p50_ms": 11.0,
      "p95_ms": 18.3,
      "p99_ms": 25.7,
      "max_ms": 95.3
    },
    "traffic": {
      "requests_per_second": 1250,
      "bytes_per_second": 5242880,
      "active_connections": 342
    },
    "errors": {
      "error_rate": 0.002,
      "total_errors": 3,
      "error_budget_remaining": 0.998
    },
    "anomalies": [
      {
        "timestamp": "2025-11-20T12:00:05Z",
        "value": 95.3,
        "type": "ZScore",
        "score": 4.5,
        "threshold": 3.0
      }
    ],
    "quality": {
      "accuracy": 0.98,
      "completeness": 0.97,
      "consistency": 0.96,
      "validity": 0.99,
      "overall_score": 0.975,
      "meets_nasa_standards": true
    }
  },
  "forecast": {
    "next_5min": [1280, 1310, 1295, 1320, 1305],
    "model": "ARIMA(1,1,1)",
    "confidence": 0.85
  }
}
```

---

## 🔧 Ferramentas e Dependências

### Rust Crates Necessárias

```toml
[dependencies]
# Já instaladas
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt", "json"] }
serde = { version = "1", features = ["derive"] }
avila-telemetry = { path = "../avila-telemetry" }

# A adicionar (próximas sprints)
prometheus = "0.13"  # Exportar métricas Prometheus
opentelemetry = "0.20"  # Distributed tracing
opentelemetry-jaeger = "0.19"  # Backend para traces
tokio = { version = "1", features = ["full"] }
```

### Infraestrutura Externa

**Para desenvolvimento local**:
```bash
# Prometheus (métricas)
docker run -p 9090:9090 prom/prometheus

# Grafana (dashboards)
docker run -p 3000:3000 grafana/grafana

# Jaeger (tracing)
docker run -p 16686:16686 -p 6831:6831/udp jaegertracing/all-in-one
```

---

## 📈 Métricas de Sucesso (KPIs)

### Técnicas
- ✅ 100% dos serviços AVX com telemetria integrada
- ⏳ Latência P99 < 50ms para APIs críticas
- ⏳ Detecção de anomalias em < 5 segundos
- ⏳ 99.9% uptime para sistema de métricas
- ⏳ Forecasting com erro < 10% MAPE

### Operacionais
- ⏳ Dashboard de observabilidade funcional
- ⏳ Alertas configurados para todos os serviços críticos
- ⏳ Retenção de métricas: 30 dias no AvilaDB
- ⏳ Documentação completa de APIs e uso

---

## 🚀 Como Contribuir

### Setup Inicial
```bash
# 1. Clone o repositório
git clone https://github.com/avilaops/arxis
cd arxis

# 2. Build avx-telemetry
cargo build -p avx-telemetry

# 3. Run testes
cargo test -p avx-telemetry

# 4. Run serviços localmente
cargo run -p avx-gateway &
cargo run -p avx-api-core &

# 5. Testar endpoints
curl http://localhost:8080/metrics/anomalies
curl http://localhost:8080/metrics/quality
curl http://localhost:8081/core/forecast
```

### Workflow de Desenvolvimento
1. **Branch**: Criar feature branch (`git checkout -b feature/avx-telemetry-xxx`)
2. **Desenvolver**: Implementar feature + testes
3. **Testar**: `cargo test -p avx-telemetry` (todos devem passar)
4. **Lint**: `cargo clippy -- -D warnings`
5. **Format**: `cargo fmt --all`
6. **Commit**: Mensagens descritivas (Conventional Commits)
7. **Push**: `git push origin feature/avx-telemetry-xxx`
8. **PR**: Abrir Pull Request com descrição detalhada

### Code Review Checklist
- [ ] Todos os testes passam (`cargo test`)
- [ ] Sem warnings de clippy (`cargo clippy`)
- [ ] Código formatado (`cargo fmt`)
- [ ] Documentação atualizada (doc comments `///`)
- [ ] Exemplos funcionam
- [ ] Performance aceitável (benchmarks se necessário)

---

## 📚 Recursos e Documentação

### Leitura Obrigatória
1. **avila-telemetry**: `avila-telemetry/README.md` (time series, anomaly detection)
2. **Tracing**: [Guia de tracing em Rust](https://tokio.rs/tokio/topics/tracing)
3. **Observability**: [Google SRE Book - Monitoring](https://sre.google/sre-book/monitoring-distributed-systems/)
4. **Forecasting**: [ARIMA Models](https://otexts.com/fpp3/arima.html)

### Referências Técnicas
- **NASA Standards**: NASA-STD-8739.8A (Software Quality)
- **Google Four Golden Signals**: Latency, Traffic, Errors, Saturation
- **OpenTelemetry**: [Specification](https://opentelemetry.io/docs/specs/otel/)
- **Prometheus**: [Best Practices](https://prometheus.io/docs/practices/naming/)

---

## 🐛 Troubleshooting

### Erro: "could not compile `avx-telemetry`"
```bash
# Verificar dependências
cargo tree -p avx-telemetry

# Limpar cache
cargo clean
cargo build -p avx-telemetry

# Verificar tipos de retorno
# TelemetryError vs String - usar TelemetryError!
```

### Erro: "method `predict` not found"
```rust
// ❌ Errado
arima.predict(steps)

// ✅ Correto
let result = arima.forecast(steps)?;
result.point_forecast
```

### Tracing não aparece nos logs
```bash
# Definir nível de log
export RUST_LOG=info  # Linux/Mac
$env:RUST_LOG="info"  # PowerShell

# Logs em JSON
export RUST_LOG=avx_telemetry=debug,avx_gateway=debug
```

---

## 📞 Contato e Suporte

**Lead**: Nicolas Ávila
**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: [@avilaops](https://github.com/avilaops)

**Daily Standups**: 10h00 BRT
**Sprint Planning**: Segundas 14h00 BRT
**Retrospective**: Sextas 16h00 BRT

---

## 🎯 Sprint Atual (Semana 1-2)

### Sprint Goal
> Integrar avila-telemetry em todos os serviços AVX com anomaly detection e forecasting funcionais.

### Tasks Prioritárias
1. **[P0 - BLOCKER]** Corrigir compilação avx-telemetry
2. **[P1 - HIGH]** Adicionar testes unitários (cobertura > 80%)
3. **[P1 - HIGH]** Implementar LatencyCollector automático
4. **[P2 - MEDIUM]** Endpoint de métricas em tempo real
5. **[P2 - MEDIUM]** Documentação de APIs (Swagger/OpenAPI)
6. **[P3 - LOW]** Dashboard Grafana básico

### Definition of Done
- [ ] Código compila sem warnings
- [ ] Todos os testes passam
- [ ] Cobertura de testes ≥ 80%
- [ ] Documentação atualizada
- [ ] Code review aprovado
- [ ] Deploy em ambiente de dev funcionando

---

**Última atualização**: 20 de novembro de 2025
**Versão**: 1.0.0
**Status**: 🟡 Em Progresso
