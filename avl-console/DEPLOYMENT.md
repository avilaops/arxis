# Production Deployment Guide - AVL Console

## 🚀 Deployment Overview

AVL Console v0.3.0 está pronto para produção com integração completa ao ecossistema AVL Platform.

## 📋 Pre-requisitos

### Sistema
- **Docker** 20.10+ e Docker Compose 2.0+
- **Linux/macOS/Windows** com PowerShell 5.1+
- **Memória**: Mínimo 4GB RAM (8GB recomendado)
- **Storage**: Mínimo 20GB de espaço em disco

### Configuração
1. **Secrets configurados** (ver seção Security)
2. **API Keys** para AvilaDB
3. **JWT Secrets** para autenticação
4. **Session secrets** para segurança

## 🔐 Security Checklist

Antes de fazer deploy em produção:

- [ ] `SESSION_SECRET` mudado do padrão
- [ ] `AVL_AUTH_JWT_SECRET` configurado
- [ ] `AVILADB_API_KEY` obtido da AVL Cloud
- [ ] CORS configurado com origens específicas
- [ ] Rate limiting ativado
- [ ] HTTPS/TLS configurado (via reverse proxy)
- [ ] Firewall rules configuradas
- [ ] Logs de audit habilitados

## 🎯 Quick Deploy

### Linux/macOS

```bash
# 1. Clone e configure
git clone https://github.com/avilaops/arxis.git
cd arxis/avl-console

# 2. Configure environment
cp .env.example .env
nano .env  # Edit with your values

# 3. Deploy
chmod +x deploy.sh
./deploy.sh
```

### Windows

```powershell
# 1. Clone e configure
git clone https://github.com/avilaops/arxis.git
cd arxis\avl-console

# 2. Configure environment
Copy-Item .env.example .env
notepad .env  # Edit with your values

# 3. Deploy
.\deploy.ps1
```

## 🏗️ Architecture

### Stack Completo

```
┌─────────────────┐
│  Load Balancer  │  (Nginx/Traefik)
│   HTTPS/TLS     │
└────────┬────────┘
         │
┌────────▼────────┐
│  AVL Console    │  :3000 (HTTP)
│  + Metrics      │  :9090 (Metrics)
└────────┬────────┘
         │
    ┌────┴─────┬──────────┬──────────────┐
    │          │          │              │
┌───▼──┐  ┌───▼──┐  ┌────▼─────┐  ┌────▼─────┐
│AvilaDB│  │Auth  │  │Telemetry │  │  Redis   │
│:8000  │  │:8080 │  │:4317/8888│  │  :6379   │
└───────┘  └──────┘  └──────────┘  └──────────┘
```

### Serviços Incluídos

1. **AVL Console** - Portal principal
   - Features: AI Assistant, Query Builder, Monitoring
   - Ports: 3000 (HTTP), 9090 (Metrics)

2. **AvilaDB** - Banco de dados distribuído
   - Persistent storage para vetores e documentos
   - Port: 8000

3. **AVL Auth** - Autenticação e autorização
   - JWT tokens, OAuth2, RBAC
   - Port: 8080

4. **AVX Telemetry** - Observabilidade
   - Metrics, logs, traces
   - Ports: 4317 (gRPC), 4318 (HTTP), 8888 (Metrics)

5. **Redis** - Cache e sessions
   - Cache de queries, rate limiting
   - Port: 6379

6. **Prometheus** - Metrics collection
   - Time-series database
   - Port: 9091

7. **Grafana** - Dashboards
   - Visualização de métricas
   - Port: 3001

## ⚙️ Configuration

### Required Environment Variables

```bash
# Core
AVL_CONSOLE_HOST=0.0.0.0
AVL_CONSOLE_PORT=3000

# Security (MUST CHANGE IN PRODUCTION!)
SESSION_SECRET=<generate-with: openssl rand -base64 32>
AVL_AUTH_JWT_SECRET=<generate-with: openssl rand -base64 32>
AVILADB_API_KEY=<get-from-avila-cloud-console>

# Services
AVILADB_ENDPOINT=http://aviladb:8000
AVL_AUTH_ENDPOINT=http://avl-auth:8080
AVL_TELEMETRY_ENDPOINT=http://avx-telemetry:4317
```

### Optional - AI Backend

```bash
# Pattern matching (default - no API key needed)
AI_BACKEND=pattern

# OpenAI integration
AI_BACKEND=openai
OPENAI_API_KEY=sk-...

# Anthropic integration
AI_BACKEND=anthropic
ANTHROPIC_API_KEY=sk-ant-...
```

### Feature Flags

```bash
ENABLE_AI_ASSISTANT=true        # Natural language queries
ENABLE_VECTOR_SEARCH=true       # RAG with embeddings
ENABLE_QUERY_SAFETY=true        # SQL injection prevention
ENABLE_RATE_LIMITING=true       # Per-user rate limits
ENABLE_METRICS=true             # Prometheus metrics
ENABLE_TRACING=true             # Distributed tracing
```

## 📊 Monitoring

### Health Checks

```bash
# AVL Console
curl http://localhost:3000/health

# AvilaDB
curl http://localhost:8000/health

# AVL Auth
curl http://localhost:8080/health

# AVX Telemetry
curl http://localhost:8888/health
```

### Metrics

```bash
# Prometheus metrics
curl http://localhost:9090/metrics

# Grafana dashboards
open http://localhost:3001
```

### Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f avl-console

# Last 100 lines
docker-compose logs --tail=100 avl-console
```

## 🔧 Troubleshooting

### Service não inicia

```bash
# Check logs
docker-compose logs <service-name>

# Restart service
docker-compose restart <service-name>

# Full restart
docker-compose down && docker-compose up -d
```

### Port conflicts

```bash
# Check what's using the port
netstat -tulpn | grep <port>  # Linux
lsof -i :<port>               # macOS
netstat -ano | findstr <port> # Windows

# Change port in .env or docker-compose.yml
```

### Database connection issues

```bash
# Check AvilaDB is running
docker-compose ps aviladb

# Verify network
docker network inspect avl-platform

# Test connection
docker exec -it avl-console curl http://aviladb:8000/health
```

## 🔄 Updates

### Update to new version

```bash
# Pull latest images
docker-compose pull

# Restart with new images
docker-compose up -d

# Verify update
docker-compose ps
```

### Rollback

```bash
# Stop current version
docker-compose down

# Specify older version in docker-compose.yml
# image: avilacloud/avl-console:0.2.0

# Start
docker-compose up -d
```

## 💾 Backup & Recovery

### Backup data

```bash
# Backup volumes
docker run --rm -v avl-console-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/avl-console-backup.tar.gz /data

# Backup AvilaDB
docker exec aviladb aviladb backup /backup/aviladb-backup
```

### Restore data

```bash
# Restore volumes
docker run --rm -v avl-console-data:/data -v $(pwd):/backup \
  alpine tar xzf /backup/avl-console-backup.tar.gz -C /

# Restore AvilaDB
docker exec aviladb aviladb restore /backup/aviladb-backup
```

## 🌐 Production Best Practices

### 1. Reverse Proxy

Use Nginx ou Traefik com TLS:

```nginx
# nginx.conf
upstream avl_console {
    server localhost:3000;
}

server {
    listen 443 ssl http2;
    server_name console.avila.cloud;

    ssl_certificate /etc/ssl/certs/avila.crt;
    ssl_certificate_key /etc/ssl/private/avila.key;

    location / {
        proxy_pass http://avl_console;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /ws {
        proxy_pass http://avl_console;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### 2. Resource Limits

```yaml
# docker-compose.yml
services:
  avl-console:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '1'
          memory: 1G
```

### 3. Auto-scaling

```bash
# Docker Swarm
docker swarm init
docker stack deploy -c docker-compose.yml avl

# Kubernetes
kubectl apply -f k8s/deployment.yaml
kubectl autoscale deployment avl-console --cpu-percent=70 --min=2 --max=10
```

### 4. Monitoring Alerts

Configure Prometheus alerts:

```yaml
# alerts.yml
groups:
  - name: avl_console
    rules:
      - alert: HighLatency
        expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 1
        for: 5m
        annotations:
          summary: "AVL Console high latency"

      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
        for: 5m
        annotations:
          summary: "AVL Console high error rate"
```

## 📞 Support

- **Documentation**: https://docs.avila.cloud/console
- **GitHub Issues**: https://github.com/avilaops/arxis/issues
- **Email**: support@avila.cloud
- **Discord**: https://discord.gg/avilacloud

---

**AVL Console v0.3.0** - Production-ready! 🚀
