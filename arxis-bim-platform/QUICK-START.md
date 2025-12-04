# 🚀 QUICK START - ARXIS BIM PLATFORM

Plataforma BIM 100% Rust pronta para rodar em **5 minutos**!

---

## 📋 Pré-requisitos

```bash
# 1. Rust (versão 1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Docker & Docker Compose
# Windows: https://docs.docker.com/desktop/install/windows-install/
# Linux: sudo apt install docker.io docker-compose

# 3. Verificar instalação
rustc --version
cargo --version
docker --version
docker-compose --version
```

---

## ⚡ Início Rápido

### 1. Subir Infraestrutura

```bash
cd arxis-bim-platform

# Subir PostgreSQL, Redis, RabbitMQ, MinIO, Elasticsearch
docker-compose up -d

# Verificar status
docker-compose ps

# Aguardar inicialização (30s)
sleep 30
```

### 2. Compilar Projeto

```bash
# Compilar tudo (Release otimizado)
cargo build --release

# Ou individualmente:
cargo build -p avila-bim-core --release
cargo build -p avila-ifc --release
cargo build -p avila-gltf --release
cargo build -p avila-bim-converter --release
```

### 3. Executar Converter Worker

```bash
# Terminal 1: Worker de conversão
export RABBITMQ_URL="amqp://guest:guest@localhost:5672"
export S3_ENDPOINT="http://localhost:9000"
export S3_BUCKET="bim-models"
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/bim_platform"

cargo run --bin converter-worker --release
```

### 4. Testar Conversão (CLI)

```bash
# Terminal 2: Enviar job de teste
cd avila-bim-converter

# Criar arquivo de teste (exemplo simplificado IFC)
cat > test_model.ifc << 'EOF'
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
FILE_NAME('test_model.ifc','2024-12-04T10:00:00',('Author'),('Org'),'App','Soft','');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;
#1 = IFCPROJECT('2O_RrAJHv7xv2dl5cNZYOF',#2,'Test Project',$,$,$,$,$,$);
#2 = IFCOWNERHISTORY(#3,#4,$,$,$,$,$,$);
#3 = IFCPERSON($,'Doe','John',$,$,$,$,$);
#4 = IFCORGANIZATION($,'ARXIS',$,$,$);
ENDSEC;
END-ISO-10303-21;
EOF

# Upload para MinIO
aws s3 cp test_model.ifc s3://bim-models/test/test_model.ifc \
  --endpoint-url http://localhost:9000 \
  --no-verify-ssl

# Publicar job na fila RabbitMQ (via Python script)
python3 << 'PYTHON'
import pika
import json
import uuid

connection = pika.BlockingConnection(pika.ConnectionParameters('localhost'))
channel = connection.channel()
channel.queue_declare(queue='bim_conversion_jobs', durable=True)

job = {
    "model_id": str(uuid.uuid4()),
    "project_id": str(uuid.uuid4()),
    "ifc_s3_key": "test/test_model.ifc",
    "output_glb_key": "test/test_model.glb",
    "retry_count": 0
}

channel.basic_publish(
    exchange='',
    routing_key='bim_conversion_jobs',
    body=json.dumps(job),
    properties=pika.BasicProperties(delivery_mode=2)
)

print(f"✅ Job published: {job['model_id']}")
connection.close()
PYTHON
```

---

## 🧪 Testes Unitários

```bash
# Testar todos os crates
cargo test --workspace

# Testar apenas avila-bim-core
cargo test -p avila-bim-core

# Testar com output detalhado
cargo test -p avila-ifc -- --nocapture

# Benchmarks (performance)
cargo bench --workspace
```

---

## 🔍 Verificar Resultados

### MinIO Console
```
http://localhost:9001
Usuário: minioadmin
Senha: minioadmin

Navegar para bucket: bim-models/test/test_model.glb
```

### RabbitMQ Management
```
http://localhost:15672
Usuário: guest
Senha: guest

Ver fila: bim_conversion_jobs (deve estar vazia após processamento)
```

### PostgreSQL
```bash
psql -h localhost -U postgres -d bim_platform -c "SELECT * FROM models;"
```

### Elasticsearch
```bash
curl http://localhost:9200/bim_metadata/_search?pretty
```

---

## 📊 Exemplo Completo: Converter IFC Real

```bash
# 1. Baixar modelo IFC exemplo (ex: do Open IFC Model Repository)
wget https://github.com/IFCjs/test-ifc-files/raw/main/IFC2X3/simple-project.ifc -O example.ifc

# 2. Upload para MinIO
aws s3 cp example.ifc s3://bim-models/examples/simple-project.ifc \
  --endpoint-url http://localhost:9000

# 3. Publicar job
python3 publish_job.py --ifc-key examples/simple-project.ifc

# 4. Acompanhar logs do worker
tail -f converter-worker.log

# 5. Download do GLB gerado
aws s3 cp s3://bim-models/examples/simple-project.glb ./output.glb \
  --endpoint-url http://localhost:9000

# 6. Visualizar GLB (usar online viewer)
# https://gltf-viewer.donmccurdy.com/
# ou
# https://sandbox.babylonjs.com/
```

---

## 🐛 Debugging

### Ver logs do Docker
```bash
docker-compose logs -f postgres
docker-compose logs -f rabbitmq
docker-compose logs -f minio
```

### Conectar ao PostgreSQL
```bash
docker exec -it arxis-postgres psql -U postgres -d bim_platform

# Ver tabelas
\dt

# Ver schema
\d models

# Query
SELECT id, name, status FROM models;
```

### Conectar ao RabbitMQ
```bash
# Ver filas
docker exec arxis-rabbitmq rabbitmqctl list_queues

# Ver consumers
docker exec arxis-rabbitmq rabbitmqctl list_consumers
```

### Conectar ao MinIO
```bash
# Listar buckets
docker exec arxis-minio mc ls local/

# Listar objetos
docker exec arxis-minio mc ls local/bim-models/
```

---

## 🚦 Health Checks

```bash
# PostgreSQL
pg_isready -h localhost -U postgres

# Redis
redis-cli ping

# RabbitMQ
curl -u guest:guest http://localhost:15672/api/health/checks/alarms

# MinIO
curl http://localhost:9000/minio/health/live

# Elasticsearch
curl http://localhost:9200/_cluster/health
```

---

## 🧹 Limpeza

```bash
# Parar e remover containers
docker-compose down

# Remover volumes (CUIDADO: apaga dados!)
docker-compose down -v

# Limpar build artifacts
cargo clean
```

---

## 📖 Próximos Passos

1. **Implementar microserviços** (auth, projects, ingestion)
2. **Criar frontend WASM** (web viewer 3D)
3. **Deploy no Kubernetes** (produção)
4. **Adicionar testes de integração**
5. **Configurar CI/CD** (GitHub Actions)

Ver: [IMPLEMENTATION-COMPLETE.md](./IMPLEMENTATION-COMPLETE.md) para detalhes completos.

---

## 💬 Suporte

- **Issues**: https://github.com/avilaops/arxis-bim-platform/issues
- **Docs**: `/docs/`
- **Discord**: [Avila Community](#)
