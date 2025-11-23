# 📦 Guia de Publicação no crates.io

Este documento explica como publicar `avl-auth` no crates.io como **avila**.

---

## ✅ Pré-requisitos

### 1. Conta no crates.io
- Acesse: https://crates.io
- Login com GitHub
- Obtenha seu token de API em: https://crates.io/me

### 2. Configurar Token Localmente
```bash
cargo login
# Cole seu token quando solicitado
```

O token será salvo em: `~/.cargo/credentials.toml`

---

## 📋 Checklist Pré-Publicação

### ✅ Verificações Obrigatórias

- [x] **Cargo.toml completo** com todos os metadados:
  - `name = "avl-auth"`
  - `version = "0.1.0"`
  - `authors`
  - `license = "MIT OR Apache-2.0"`
  - `description`
  - `repository`
  - `homepage`
  - `documentation`
  - `readme = "README.md"`
  - `keywords` (máx. 5)
  - `categories`

- [x] **README.md** presente na raiz
- [x] **LICENSE** ou `license` no Cargo.toml
- [x] **Build sem erros**: `cargo build`
- [x] **Testes passando**: `cargo test`
- [x] **Documentação válida**: `cargo doc --no-deps`
- [x] **Sem dependências locais** (`path = "../..."`)

### ⚠️ Estado Atual

**Status**: ✅ PRONTO PARA PUBLICAÇÃO

Dependências locais foram **comentadas** porque ainda não estão no crates.io:
- `aviladb` → será publicado em breve
- `avx-telemetry` → será publicado em breve
- `avila-compress` → será publicado em breve
- `avila-telemetry` → será publicado em breve

**Features temporariamente desabilitadas** até libs serem publicadas:
- `database`
- `telemetry`
- `compression`
- `analytics`

---

## 🚀 Comandos de Publicação

### 1. Verificar Pacote
```bash
cd avl-auth
cargo package --list
```

Isso mostra todos os arquivos que serão incluídos.

### 2. Build do Pacote
```bash
cargo package
```

Cria o arquivo `.crate` em `target/package/avl-auth-0.1.0.crate`

### 3. Publicar (DRY RUN)
```bash
cargo publish --dry-run
```

Simula publicação sem enviar.

### 4. Publicar no crates.io
```bash
cargo publish
```

🎉 **Pronto!** Sua crate estará disponível em: https://crates.io/crates/avl-auth

---

## 📝 Após Publicação

### Verificar Publicação
1. Acesse: https://crates.io/crates/avl-auth
2. Verifique:
   - README renderizado corretamente
   - Documentação linkada (docs.rs)
   - Badges funcionando
   - Links do repositório

### Documentação Automática (docs.rs)
A documentação será gerada automaticamente em:
https://docs.rs/avl-auth

Para forçar rebuild (se necessário):
1. Acesse: https://docs.rs/crate/avl-auth/0.1.0
2. Clique em "Rebuild"

### Divulgação
Compartilhe em:
- [x] GitHub Discussions do Arxis
- [x] Discord AVL Platform
- [x] Twitter/X (@avilacloud)
- [x] LinkedIn (Nicolas Ávila)
- [x] Reddit r/rust (opcional)

---

## 🔄 Atualizações Futuras

### Versionamento Semântico

```
MAJOR.MINOR.PATCH
  0  .  1  .  0
```

- **MAJOR**: Breaking changes (0.x.x = pré-1.0, pode quebrar)
- **MINOR**: Novas features (compatível)
- **PATCH**: Bug fixes (compatível)

### Publicar Nova Versão

1. **Atualizar versão** no `Cargo.toml`:
   ```toml
   version = "0.2.0"
   ```

2. **Atualizar CHANGELOG.md**:
   ```markdown
   ## [0.2.0] - 2025-11-24
   ### Added
   - Feature X
   - Feature Y
   ### Fixed
   - Bug Z
   ```

3. **Commit e Tag**:
   ```bash
   git commit -am "Release v0.2.0"
   git tag v0.2.0
   git push && git push --tags
   ```

4. **Publicar**:
   ```bash
   cargo publish
   ```

---

## 🎯 Roadmap de Publicações AVL

### Fase 1: Core (ATUAL)
1. ✅ `avl-auth` v0.1.0 - Autenticação e autorização

### Fase 2: Infraestrutura
2. ⏳ `aviladb` v0.1.0 - NoSQL distribuído
3. ⏳ `avx-telemetry` v0.1.0 - Logging estruturado
4. ⏳ `avila-compress` v0.1.0 - Compressão nativa

### Fase 3: Analytics
5. ⏳ `avila-telemetry` v0.1.0 - Time series
6. ⏳ `avila-ml` v0.1.0 - Machine learning

### Fase 4: Aplicações
7. ⏳ `avl-storage` v0.1.0 - Object storage
8. ⏳ `avl-queue` v0.1.0 - Message queue
9. ⏳ `avl-secrets` v0.1.0 - Secret management

### Fase 5: Re-enable Features
Quando as libs forem publicadas, **reativar no avl-auth**:

```toml
[dependencies]
aviladb = { version = "0.1", optional = true }
avx-telemetry = { version = "0.1", optional = true }
avila-compress = { version = "0.1", optional = true }
avila-telemetry = { version = "0.1", optional = true }

[features]
default = ["telemetry"]
full = ["database", "telemetry", "compression", "analytics"]
database = ["aviladb"]
telemetry = ["avx-telemetry"]
compression = ["avila-compress"]
analytics = ["avila-telemetry"]
```

Publicar `avl-auth` v0.2.0 com features completas.

---

## 🔒 Segurança

### Yanking (Remover) Versão
Se publicar versão com bug crítico:

```bash
cargo yank --vers 0.1.0
```

Isso impede novos projetos de usar, mas não quebra projetos existentes.

### Reverter Yank
```bash
cargo yank --vers 0.1.0 --undo
```

---

## ❓ Troubleshooting

### Erro: "name already taken"
- Nome `avl-auth` já existe?
- Escolha outro: `avilaauth`, `avila-auth`, `avl-identity`

### Erro: "failed to verify"
```bash
cargo clean
cargo build
cargo publish
```

### Erro: "rate limit"
- crates.io tem limite de 5 publicações/hora
- Aguarde 1 hora e tente novamente

### Documentação não gera
- Verifique se todos os links em `///` docs estão corretos
- Execute: `cargo doc --no-deps --open`

---

## 📞 Suporte

- **GitHub Issues**: https://github.com/avilaops/arxis/issues
- **Discord**: discord.avila.cloud
- **Email**: support@avila.cloud

---

**Boa sorte com a publicação! 🚀**

🇧🇷 Made in Brazil | ⚡ Optimized for LATAM | 🔐 World-class security
