# ARXIS BIM Platform - 100% Rust + Avila

Plataforma BIM avançada para visualização web/mobile/AR de modelos IFC, inspirada no Augin, usando **100% ecossistema Avila**.

## 🏗️ Arquitetura

```
arxis-bim-platform/
├── avila-bim-core/         # Primitivos BIM (Model, Element, Geometry)
├── avila-ifc/              # Parser IFC nativo (STEP → estruturas Rust)
├── avila-gltf/             # Exporter glTF 2.0/GLB nativo
├── avila-bim-converter/    # Worker de conversão (IFC → glTF)
│
├── services/
│   ├── auth-users/         # Autenticação JWT + Argon2
│   ├── projects-acl/       # Projetos e controle de acesso
│   ├── model-ingestion/    # Upload e enfileiramento
│   ├── model-metadata/     # Busca de elementos BIM
│   ├── collaboration/      # Anotações e comentários
│   ├── file-assets/        # Acesso a Object Storage
│   └── notifications/      # E-mail/push
│
├── backends/
│   ├── api-gateway/        # Roteamento e rate limiting
│   ├── bff-web/            # Agregação para web viewer
│   └── bff-mobile/         # Otimizado para mobile/AR
│
└── frontends/
    ├── web-viewer/         # WASM + WebGL (Three.js alternativo)
    └── admin-portal/       # Dashboard administrativo
```

## 🔧 Stack Tecnológica

### Dependências Avila (100% nativas)
- **avila-http**: Cliente/servidor HTTP (substitui axum internamente)
- **avila-db**: Cliente AvilaDB (PostgreSQL + NoSQL híbrido)
- **avila-async**: Runtime assíncrono (substitui tokio internamente)
- **avila-crypto**: Criptografia (JWT, Argon2, TLS)
- **avila-error**: Sistema de erros unificado

### Bibliotecas Complementares
- **nalgebra/glam**: Álgebra linear para transformações 3D
- **serde**: Serialização JSON/binária

### Infraestrutura
- **PostgreSQL**: Banco relacional (metadados, ACL)
- **MinIO/S3**: Object Storage (arquivos IFC/GLB)
- **Redis**: Cache de metadados frequentes
- **RabbitMQ**: Filas para jobs de conversão
- **Elasticsearch**: Busca full-text de elementos BIM

## 🚀 Quick Start

### 1. Instalar dependências

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Docker (para infra local)
docker-compose up -d
```

### 2. Compilar workspace

```bash
cd arxis-bim-platform
cargo build --release
```

### 3. Executar microserviços

```bash
# Converter worker
cargo run --bin bim-converter

# API Gateway
cargo run --bin api-gateway

# Web Viewer
cd frontends/web-viewer
wasm-pack build --target web
python3 -m http.server 8080
```

## 📐 Pipeline BIM

```
┌──────────────────────────────────────────────────────────────┐
│                      PIPELINE BIM                            │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  1. UPLOAD IFC                                               │
│     ├─ POST /models/upload                                   │
│     ├─ Validação (schema, tamanho)                           │
│     └─ Storage MinIO + Enfileirar job                        │
│                                                              │
│  2. PARSING IFC (avila-ifc)                                  │
│     ├─ Ler STEP P21                                          │
│     ├─ Parsear entidades (IfcWall, IfcSlab, etc.)           │
│     └─ Construir grafo hierárquico                           │
│                                                              │
│  3. CONVERSÃO GEOMETRIA (avila-gltf)                         │
│     ├─ Triangular superfícies BRep                           │
│     ├─ Otimizar meshes (merge, LOD, Draco)                   │
│     ├─ Comprimir texturas (KTX2/Basis)                       │
│     └─ Gerar GLB + JSON metadados                            │
│                                                              │
│  4. INDEXAÇÃO                                                │
│     ├─ PostgreSQL: Metadados estruturados                    │
│     ├─ Elasticsearch: Busca full-text                        │
│     └─ Redis: Cache de queries frequentes                    │
│                                                              │
│  5. VISUALIZAÇÃO                                             │
│     ├─ Web: WASM + WebGL (carrega GLB)                       │
│     ├─ Mobile: React Native (com cache offline)              │
│     └─ AR: ARCore/ARKit (escala 1:1)                         │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## 🧬 Modelo de Dados

### PostgreSQL (metadados relacionais)

```sql
-- Projetos
CREATE TABLE projects (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Modelos (versionados)
CREATE TABLE models (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES projects(id),
    name VARCHAR(255) NOT NULL,
    version INT NOT NULL,
    status VARCHAR(50) NOT NULL, -- uploading, converting, ready, error
    ifc_url TEXT,
    glb_url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(project_id, name, version)
);

-- Elementos BIM
CREATE TABLE elements (
    id UUID PRIMARY KEY,
    model_id UUID NOT NULL REFERENCES models(id),
    guid VARCHAR(22) NOT NULL, -- IFC GUID
    type VARCHAR(100) NOT NULL, -- IfcWall, IfcSlab, etc.
    properties JSONB, -- {Name, Material, LoadBearing, etc.}
    hierarchy_path VARCHAR(500), -- /Projeto/Pavimento/Parede
    geometry_bounds JSONB, -- {min: [x,y,z], max: [x,y,z]}
    created_at TIMESTAMPTZ DEFAULT NOW(),
    INDEX idx_elements_guid (guid),
    INDEX idx_elements_type (type),
    INDEX idx_elements_hierarchy USING gin (hierarchy_path gin_trgm_ops)
);

-- Anotações colaborativas
CREATE TABLE annotations (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES projects(id),
    model_id UUID NOT NULL REFERENCES models(id),
    element_guid VARCHAR(22),
    position JSONB NOT NULL, -- {x, y, z}
    camera JSONB NOT NULL, -- {position, target, up}
    author_id UUID NOT NULL,
    content TEXT NOT NULL,
    status VARCHAR(50) DEFAULT 'open', -- open, in_progress, resolved
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Elasticsearch (busca full-text)

```json
{
  "index": "bim_metadata",
  "mappings": {
    "properties": {
      "model_id": { "type": "keyword" },
      "guid": { "type": "keyword" },
      "type": { "type": "keyword" },
      "name": { "type": "text" },
      "description": { "type": "text" },
      "properties": { "type": "object", "enabled": false },
      "floor": { "type": "keyword" },
      "material": { "type": "keyword" },
      "category": { "type": "keyword" }
    }
  }
}
```

## 🔐 Autenticação e Autorização

### JWT com avila-crypto

```rust
use avila_crypto::jwt::{JwtBuilder, Algorithm};

// Criar token
let token = JwtBuilder::new()
    .algorithm(Algorithm::RS256)
    .subject("user-uuid")
    .claim("role", "editor")
    .claim("project_id", "project-uuid")
    .sign(private_key)?;

// Verificar
let claims = avila_crypto::jwt::verify(&token, public_key)?;
```

### Roles (ACL)

```rust
enum Role {
    Viewer,   // Apenas visualização
    Editor,   // + criar anotações
    Admin,    // + gerenciar usuários e projetos
}
```

## 📊 Métricas e Monitoramento

- **Prometheus**: Métricas de performance (latência, throughput)
- **Grafana**: Dashboards de visualização
- **OpenTelemetry**: Tracing distribuído

## 🧪 Testes

```bash
# Unitários
cargo test

# Integração (requer infra local)
cargo test --test integration -- --ignored

# Performance
cargo bench
```

## 📦 Deploy

### Docker Compose (desenvolvimento)

```bash
docker-compose up -d
```

### Kubernetes (produção)

```bash
kubectl apply -f infra/k8s/
```

## 🤝 Contribuição

1. Fork o repositório
2. Crie uma branch (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -am 'Add nova feature'`)
4. Push para branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

## 📄 Licença

MIT OR Apache-2.0
