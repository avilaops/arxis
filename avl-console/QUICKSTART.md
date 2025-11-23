# ⚡ AVL Console - Quick Start Guide

## 🚀 5-Minute Setup

### 1. Clone e Build (1 min)

```bash
cd avl-console
cargo build --release
```

### 2. Configure (1 min)

```bash
# Copie o exemplo de configuração
cp .env.example .env

# Edite as variáveis (opcional)
nano .env
```

### 3. Execute (30 segundos)

```bash
cargo run --example basic
```

### 4. Acesse (30 segundos)

Abra seu navegador em:
- 🖥️ **Dashboard**: http://localhost:8080/dashboard
- 🗄️ **AvilaDB**: http://localhost:8080/databases
- 💾 **Storage**: http://localhost:8080/storage
- 📈 **Observability**: http://localhost:8080/observability
- 💰 **Billing**: http://localhost:8080/billing

### 5. Login (1 min)

```
Usuário: admin
Senha: admin
```

## 🎯 Principais Comandos

```bash
# Desenvolvimento
cargo run --example basic                    # Executar
cargo watch -x 'run --example basic'         # Auto-reload

# Testes
cargo test                                   # Todos os testes
cargo test --lib                             # Testes da lib

# Build
cargo build                                  # Debug
cargo build --release                        # Release

# Verificação
cargo check                                  # Verificar
cargo clippy                                 # Lint
cargo fmt                                    # Formatar
```

## 📚 Estrutura do Projeto

```
avl-console/
├── src/              # Código-fonte
├── examples/         # Exemplos
├── tests/            # Testes
├── README.md         # Documentação principal
├── API.md            # Referência da API
├── DEVELOPMENT.md    # Guia de desenvolvimento
└── Cargo.toml        # Dependências
```

## 🔧 Configuração Avançada

### Portas Customizadas

```bash
AVL_CONSOLE_PORT=3000 cargo run --example basic
```

### Debug Mode

```bash
AVL_CONSOLE_DEBUG=true RUST_LOG=debug cargo run --example basic
```

### Rate Limiting

```bash
AVL_CONSOLE_RATE_LIMIT=200 cargo run --example basic
```

## 📊 Features Disponíveis

| Feature       | Rota             | Descrição                  |
| ------------- | ---------------- | -------------------------- |
| Dashboard     | `/dashboard`     | Métricas e overview        |
| AvilaDB       | `/databases`     | Explorer de banco de dados |
| Storage       | `/storage`       | Browser de arquivos        |
| Observability | `/observability` | Métricas e logs            |
| Billing       | `/billing`       | Custos e faturas           |
| API           | `/api`           | REST API                   |
| WebSocket     | `/ws`            | Real-time updates          |

## 🔐 Endpoints da API

### Health Check
```bash
curl http://localhost:8080/api/health
```

### Login
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}'
```

### Get Metrics
```bash
curl http://localhost:8080/dashboard/metrics \
  -H "Cookie: avl_session=YOUR_SESSION_ID"
```

## 🧪 Testar Features

### 1. Dashboard Real-Time
- Abra `/dashboard`
- Observe as métricas atualizando automaticamente

### 2. Query Editor
- Vá para `/databases`
- Execute: `SELECT * FROM users LIMIT 10`

### 3. Storage Browser
- Acesse `/storage`
- Navegue pelos buckets

### 4. Observability Charts
- Visite `/observability`
- Veja os gráficos de métricas

### 5. Billing Tracker
- Abra `/billing`
- Confira os custos por serviço

## 🎨 Personalizar UI

### Trocar Cores

Edite `src/dashboard.rs`, `src/database.rs`, etc:

```css
/* Accent color */
color: #00d4ff;  /* Mude para sua cor */
```

### Modificar Layout

Templates HTML estão inline nos arquivos:
- `src/dashboard.rs` → `DASHBOARD_HTML`
- `src/database.rs` → `DATABASE_LIST_HTML`
- etc.

## 🐛 Troubleshooting

### Porta em Uso
```bash
# Use outra porta
AVL_CONSOLE_PORT=8081 cargo run --example basic
```

### Erro de Compilação
```bash
# Limpe e recompile
cargo clean
cargo build
```

### Logs Detalhados
```bash
RUST_LOG=trace cargo run --example basic
```

## 📖 Próximos Passos

1. **Leia a [API.md](API.md)** para endpoints completos
2. **Consulte [DEVELOPMENT.md](DEVELOPMENT.md)** para contribuir
3. **Veja [SHOWCASE.md](SHOWCASE.md)** para design system
4. **Confira [STATUS.md](STATUS.md)** para features completas

## 🤝 Contribuir

```bash
# Fork e clone
git clone https://github.com/seu-usuario/arxis
cd arxis/avl-console

# Crie branch
git checkout -b feature/minha-feature

# Faça mudanças
# ... edite código ...

# Teste
cargo test

# Commit
git commit -am 'Add: minha feature'

# Push
git push origin feature/minha-feature

# Abra Pull Request
```

## 💡 Dicas

1. **Use `cargo watch`** para auto-reload durante desenvolvimento
2. **Configure RUST_LOG** para controlar verbosidade dos logs
3. **Teste com `cargo test`** antes de commit
4. **Use `cargo clippy`** para melhores práticas
5. **Consulte a documentação** inline com `cargo doc --open`

## 🆘 Ajuda

- **Issues**: https://github.com/avilaops/arxis/issues
- **Discord**: https://discord.gg/avilacloud
- **Email**: support@avila.cloud
- **Docs**: https://docs.avila.cloud

---

**⚡ AVL Console** - Quick Start em 5 minutos!

**🏛️ Built by Avila** | **🦀 Made with Rust** | **⚡ Powered by Axum**

---

**Pronto para começar? Execute:** `cargo run --example basic` 🚀
