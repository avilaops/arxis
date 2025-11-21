# 🚀 Workflow de Publicação - face3d-rs

## Para Contribuidores: LEIA ANTES DE PUBLICAR

### ⚠️ IMPORTANTE: Processo de Publicação

Este crate segue as melhores práticas de publicação no crates.io. Siga este guia para evitar erros.

---

## 📋 Checklist Pré-Publicação

Antes de publicar, verifique:

1. ✅ **Código sem warnings do Clippy**: `cargo clippy -- -D warnings`
2. ✅ **Todos os testes passando**: `cargo test --release`
3. ✅ **Documentação completa**: `cargo doc --no-deps --open`
4. ✅ **Exemplos funcionando**: `cargo run --example <nome>`
5. ✅ **Versão atualizada** em `Cargo.toml`
6. ✅ **CHANGELOG.md** atualizado com as mudanças
7. ✅ **README.md** atualizado se necessário
8. ✅ **Dry-run bem-sucedido**: `cargo publish --dry-run`

---

## 🎯 Como Publicar uma Nova Versão

### Passo 1: Preparar Release

```powershell
# 1. Certifique-se de estar na branch principal
git checkout main
git pull origin main

# 2. Execute todos os checks
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test --release
cargo doc --no-deps

# 3. Valide o pacote
cargo publish --dry-run
```

### Passo 2: Atualizar Versão

Edite `Cargo.toml`:
```toml
[package]
version = "0.2.0"  # Incremente conforme semantic versioning
```

**Semantic Versioning:**
- **MAJOR** (1.0.0): Mudanças incompatíveis na API
- **MINOR** (0.2.0): Novas funcionalidades compatíveis
- **PATCH** (0.1.1): Correções de bugs

### Passo 3: Atualizar CHANGELOG.md

```markdown
## [0.2.0] - 2025-11-21

### Added
- Nova funcionalidade X
- Suporte para Y

### Changed
- Melhoria na performance de Z

### Fixed
- Correção do bug #123
```

### Passo 4: Commit e Tag

```powershell
# Commit das mudanças
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.2.0"

# Criar tag
git tag v0.2.0

# Push
git push origin main
git push origin v0.2.0
```

### Passo 5: Publicar no crates.io

```powershell
# Login no crates.io (primeira vez)
cargo login <seu-token>

# Publicar
cargo publish
```

**Obter token:** https://crates.io/settings/tokens

---

## 🔐 Configuração Inicial (Primeira Publicação)

### 1. Criar Conta no crates.io

1. Acesse https://crates.io/
2. Login com GitHub
3. Vá em Settings → API Tokens
4. Crie novo token: `face3d-rs-publish`

### 2. Configurar Git

```powershell
git config --global user.name "Seu Nome"
git config --global user.email "seu@email.com"
```

### 3. Login no Cargo

```powershell
cargo login <token-do-crates-io>
```

O token fica salvo em `~/.cargo/credentials.toml`

---

## 🔍 Validações Importantes

### Antes de Publicar

```powershell
# Build de release
cargo build --release

# Testes em release mode
cargo test --release

# Verificar warnings
cargo clippy -- -D warnings

# Validar documentação
cargo doc --no-deps

# Verificar tamanho do pacote
cargo package --list
```

### Verificar Após Publicação

1. Acesse: https://crates.io/crates/face3d-rs
2. Verifique a documentação: https://docs.rs/face3d-rs
3. Teste instalação: `cargo install face3d-rs` (se aplicável)

---

## ❌ O Que NÃO Fazer

### 🚫 NUNCA publique sem testar:
```powershell
cargo publish  # ❌ Sem cargo test antes!
```

### 🚫 NUNCA publique com warnings:
```powershell
# ❌ Sempre resolva TODOS os warnings do Clippy primeiro
cargo clippy -- -D warnings  # Deve passar sem erros
```

### 🚫 NUNCA publique versão duplicada:
- Versões no crates.io são **imutáveis**
- Se errar, deve publicar uma nova versão

### 🚫 NUNCA publique sem atualizar CHANGELOG:
- Usuários precisam saber o que mudou

---

## 🐛 Problemas Comuns

### Erro: "crate version `X` is already uploaded"
**Solução:** Versão já existe. Incremente a versão no `Cargo.toml`.

```toml
version = "0.1.1"  # Era 0.1.0
```

### Erro: "failed to authenticate"
**Solução:** Token expirado. Renovar em https://crates.io/settings/tokens

```powershell
cargo login <novo-token>
```

### Erro: "package size exceeds limit"
**Solução:** Adicionar arquivos ao `.gitignore` ou `Cargo.toml`:

```toml
[package]
exclude = [
    "target/",
    "tests/data/*.h5",  # Arquivos grandes
    "*.png",
]
```

### Erro: "documentation failed to build"
**Solução:** Testar localmente:

```powershell
cargo doc --no-deps --open
```

Corrigir erros de documentação (links quebrados, exemplos inválidos).

### Build falhou no docs.rs
**Solução:** Adicionar metadados ao `Cargo.toml`:

```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

---

## 📦 Estrutura de Versões

### Desenvolvimento Local
```toml
version = "0.1.0-dev"  # Versão em desenvolvimento
```

### Release Candidate
```toml
version = "0.2.0-rc.1"  # Release candidate
```

### Produção
```toml
version = "0.2.0"  # Release estável
```

---

## 🎓 Exemplo Completo: Publicar 0.1.0 → 0.2.0

```powershell
# 1. Validar código atual
cargo fmt --all
cargo clippy -- -D warnings
cargo test --release
cargo doc --no-deps

# 2. Atualizar versões
# Editar Cargo.toml: version = "0.2.0"
# Editar CHANGELOG.md: adicionar seção [0.2.0]

# 3. Commit
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.2.0"

# 4. Dry run
cargo publish --dry-run

# 5. Tag
git tag v0.2.0
git push origin main
git push origin v0.2.0

# 6. Publicar
cargo publish

# 7. Verificar
# Aguarde 5-10 minutos para docs.rs processar
# Visite: https://crates.io/crates/face3d-rs/0.2.0
```

---

## 🔄 Automação Futura (GitHub Actions)

Para automatizar no futuro, criar `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Publish to crates.io
        run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
```

**Secrets necessários:**
- `CARGO_TOKEN` - Token do crates.io

---

## 📞 Recursos

- **Documentação Oficial:** https://doc.rust-lang.org/cargo/reference/publishing.html
- **Crates.io:** https://crates.io/crates/face3d-rs
- **Docs.rs:** https://docs.rs/face3d-rs
- **Repositório:** https://github.com/avilaops/face3d-rs

---

## 🎯 TL;DR - Quick Commands

```powershell
# Publicar nova versão
cargo clippy -- -D warnings  # Validar
cargo test --release         # Testar
# Editar Cargo.toml e CHANGELOG.md
git commit -am "chore: Bump version to X.Y.Z"
git tag vX.Y.Z
git push origin main --tags
cargo publish                # 🚀
```

---

**Lembrete:** face3d-rs é uma biblioteca matemática crítica. Cada release deve ser **bem testada** e **documentada**! 🛡️

---

*Última atualização: 2025-11-21*
*Autor: Nicolas @ Avila.inc*
