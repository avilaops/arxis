# Avx Stack - Avila Experience Fabric

**Avx** = Ávila Experience Fabric - Stack Web oficial da Avila DevOps para fungo-deepweb.

## 🎯 Filosofia: Da Casca ao Cerne

O projeto **Arxis** trabalha da **casca → cerne**:
- Física fundamental (relatividade geral, ondas gravitacionais, tensores)
- Matemática pura (quaternions, geometria 4D, álgebra tensorial)
- Base científica que sustenta toda infraestrutura

Enquanto isso, a **stack Avx** é construída do **cerne → casca**:
- Infraestrutura deep (mesh, eventos, observabilidade)
- APIs e gateways
- Serviços de superfície

**Arxis fornece os fundamentos. Avx constrói sobre eles.**

## 📦 Estrutura do Workspace

```
arxis/
├── avx-config/       # Configuração unificada (stack=Avx, layer=deep, cluster, etc.)
├── avx-telemetry/    # Observabilidade (logs, métricas, traces com selo Avx)
├── avx-gateway/      # Porta de entrada deepweb (HTTP/gRPC interno)
├── avx-api-core/     # APIs internas de negócio
├── avx-events/       # Integração com event bus (Kafka/NATS/Rabbit)
└── avx-cli/          # CLI para gerar manifests K8s
```

## 🚀 Quick Start

### 1. Build workspace completo
```bash
cargo build --workspace
```

### 2. Rodar serviços individuais

**Gateway (porta 8080)**:
```bash
cargo run -p avx-gateway
```

**API Core (porta 8081)**:
```bash
cargo run -p avx-api-core
```

**Events (background worker)**:
```bash
cargo run -p avx-events
```

### 3. Gerar manifests Kubernetes

```bash
# Gateway
cargo run -p avx-cli -- k8s --service gateway --namespace avx-core --output k8s/gateway.yaml

# API Core com 3 réplicas
cargo run -p avx-cli -- k8s --service api-core --replicas 3 --output k8s/api-core.yaml

# Events com imagem custom
cargo run -p avx-cli -- k8s --service events \
  --image ghcr.io/avilaops/avx-events:prod \
  --namespace avx-data \
  --output k8s/events.yaml
```

### 4. Aplicar no cluster
```bash
kubectl apply -f k8s/
```

## 🏷️ Padrão de Naming Avx

Todos os serviços Avx seguem nomenclatura padronizada:

### Labels Kubernetes
```yaml
stack: Avx
layer: deep
env: dev|stg|prod
cluster: AVL-BR|AKS-US|EKS-EU
mesh: internal
```

### Headers HTTP
```
X-Avx-Stack: Avx
X-Avx-Layer: deep
X-Avx-Env: prod
X-Avx-Cluster: AVL-BR
X-Avx-Mesh: internal
X-Avx-Trace: <uuid>
```

### Variáveis de Ambiente
```bash
AVX__STACK=Avx
AVX__LAYER=deep
AVX__ENV=prod
AVX__CLUSTER=AVL-BR
AVX__MESH=internal
AVX__HTTP__BIND_ADDR=0.0.0.0:8080
```

## 🌐 Arquitetura Fungo-Deepweb

```
┌─────────────────────────────────────────┐
│  Superfície Web (público)               │
│  - Sites, APIs públicas                 │
└──────────────┬──────────────────────────┘
               │
      ┌────────▼─────────┐
      │  avx-gateway     │  ◀── Entrada deepweb
      │  :8080           │      (mTLS, mesh)
      └────────┬─────────┘
               │
      ┌────────▼──────────┐
      │  Service Mesh     │
      │  (Istio/Linkerd)  │
      └──┬──────────────┬─┘
         │              │
  ┌──────▼──────┐  ┌───▼────────┐
  │ avx-api-core│  │ avx-events │
  │ :8081       │  │ :8090      │
  └─────────────┘  └────────────┘
         │              │
    ┌────▼──────────────▼────┐
    │  Data Layer            │
    │  (DBs, Kafka, Cache)   │
    └────────────────────────┘
```

## 🔒 Zero Trust + Observabilidade

- **Autenticação**: mTLS obrigatório no mesh
- **Autorização**: RBAC por service account
- **Logs**: JSON estruturado com contexto Avx
- **Métricas**: Prometheus com labels Avx
- **Traces**: Distributed tracing com X-Avx-Trace

## 🧪 Testes

```bash
# Testar config
cargo test -p avx-config

# Build e testar gateway
cargo run -p avx-gateway &
curl http://localhost:8080/health
curl http://localhost:8080/deep/info

# Testar geração de manifests
cargo run -p avx-cli -- k8s --service gateway
```

## 📚 Integrações Futuras

- [ ] Kafka/NATS para avx-events
- [ ] PostgreSQL/Cockroach para persistência
- [ ] gRPC além de HTTP
- [ ] OpenTelemetry nativo
- [ ] Vault para secrets
- [ ] Multi-cloud (AVL + AKS + EKS + GKE)

## 🤝 Contribuindo

Stack Avx é parte do ecossistema Avila. Para contribuir:

1. Mantenha padrão de naming Avx
2. Adicione testes unitários
3. Documente com exemplos
4. Use `cargo fmt` e `cargo clippy`

## 📫 Contato

- **Email**: nicolas@avila.inc
- **WhatsApp**: +55 17 99781-1471
- **GitHub**: https://github.com/avilaops/arxis

---

**Avx** - Tecido nervoso da infraestrutura Avila 🧠
Construído com Rust 🦀 | Powered by AVL Cloud Platform ☁️
