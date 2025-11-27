# Guia de Publicação no Crates.io

## 🔐 Autenticação Configurada

O token do crates.io já está configurado e salvo em `~/.cargo/credentials.toml`.

```powershell
# Token salvo em: C:\Users\Administrador\.cargo\credentials.toml
```

## 📦 Checklist Antes de Publicar

### 1. Verificar Cargo.toml
```toml
[package]
name = "seu-crate"           # Nome único no crates.io
version = "0.1.0"             # Semver: MAJOR.MINOR.PATCH
edition = "2021"              # Edição Rust
authors = ["Your Name <email@example.com>"]
license = "MIT OR Apache-2.0" # Licença obrigatória
description = "Descrição curta (máx. 200 chars)"
repository = "https://github.com/avilaops/arxis"
readme = "README.md"
keywords = ["max", "5", "keywords"]
categories = ["category"]     # Ver: https://crates.io/categories
```

### 2. Verificar Arquivos Essenciais
- ✅ `README.md` - Documentação principal
- ✅ `LICENSE-MIT` e `LICENSE-APACHE` - Licenças
- ✅ `Cargo.toml` - Metadados completos
- ✅ `.gitignore` - Excluir arquivos desnecessários

### 3. Adicionar Badges (Opcional)
```markdown
[![Crates.io](https://img.shields.io/crates/v/seu-crate.svg)](https://crates.io/crates/seu-crate)
[![Documentation](https://docs.rs/seu-crate/badge.svg)](https://docs.rs/seu-crate)
[![License](https://img.shields.io/crates/l/seu-crate.svg)](LICENSE)
```

## 🚀 Comandos de Publicação

### Verificar antes de publicar
```powershell
# Build e testes
cargo build --release
cargo test --all-features

# Verificar se está tudo OK
cargo package --list

# Dry-run (simula publicação)
cargo publish --dry-run
```

### Publicar
```powershell
# Publicar no crates.io
cargo publish

# Publicar crate específico (workspace)
cargo publish -p nome-do-crate
```

### Publicar com features específicas
```powershell
cargo publish --features "feature1,feature2"
cargo publish --all-features
```

## 📋 Estrutura de Workspace

Se você tem um workspace com múltiplos crates:

```toml
# Cargo.toml raiz
[workspace]
members = [
    "avila",
    "avila-geo",
    "avila-clustering",
    # ... outros crates
]
```

### Publicar na ordem correta
```powershell
# 1. Publicar dependências primeiro
cd avila
cargo publish

# 2. Depois os crates que dependem delas
cd ../avila-geo
cargo publish

# 3. Continue na ordem de dependência
```

## ⚠️ Atenções Importantes

### Versionamento
- **MAJOR** (1.x.x): Mudanças incompatíveis
- **MINOR** (x.1.x): Novas funcionalidades compatíveis
- **PATCH** (x.x.1): Correções de bugs

### Limitações do crates.io
- ✅ Tamanho máximo: **10 MB** (compactado)
- ✅ Nome do crate: **alfanumérico + hífens** (sem underscore)
- ✅ Descrição: **máximo 200 caracteres**
- ✅ Keywords: **máximo 5**
- ⚠️ **Não é possível deletar versões publicadas** (apenas yankar)

### Yanking (Retirar versão)
```powershell
# Marcar versão como não recomendada
cargo yank --vers 0.1.0

# Desfazer yank
cargo yank --vers 0.1.0 --undo
```

## 🔍 Verificar Status de Publicação

### Verificar crate no crates.io
```powershell
# Via navegador
Start-Process "https://crates.io/crates/seu-crate"

# Via API
Invoke-RestMethod "https://crates.io/api/v1/crates/seu-crate"
```

### Verificar documentação
```powershell
# Após publicação, docs aparecem em:
Start-Process "https://docs.rs/seu-crate"
```

## 📝 Workflow Recomendado

```powershell
# 1. Atualizar versão no Cargo.toml
# 2. Atualizar CHANGELOG.md
# 3. Commit e tag
git add .
git commit -m "Release v0.2.0"
git tag -a v0.2.0 -m "Version 0.2.0"

# 4. Testar
cargo test --all-features
cargo clippy -- -D warnings

# 5. Dry-run
cargo publish --dry-run

# 6. Publicar
cargo publish

# 7. Push para GitHub
git push origin main --tags
```

## 🛠️ Troubleshooting

### Erro: "crate name already exists"
- O nome já está em uso. Escolha outro nome único.

### Erro: "missing license file"
- Adicione `LICENSE-MIT` e `LICENSE-APACHE` na raiz.

### Erro: "description too long"
- Máximo de 200 caracteres no campo `description`.

### Erro: "failed to verify package"
- Execute `cargo clean` e tente novamente.
- Verifique se todas as dependências estão corretas.

## 📚 Links Úteis

- **Crates.io**: https://crates.io
- **Docs.rs**: https://docs.rs
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **Publishing Guide**: https://doc.rust-lang.org/cargo/reference/publishing.html
- **Manifest Format**: https://doc.rust-lang.org/cargo/reference/manifest.html

## 🎯 Crates do Arxis Prontos para Publicação

Com base no workspace atual:

1. **avila** - Core library
2. **avila-geo** - Geospatial processing
3. **avila-clustering** - Clustering algorithms
4. **avila-compress** - Compression utilities
5. **avila-dataframe** - DataFrame operations
6. **avila-math** - Mathematical functions
7. **avila-ml** - Machine learning
8. **avila-arrow** - Apache Arrow integration
9. **avila-telemetry** - Observability
10. **aviladb** - Database client

---

**Token configurado**: ✅
**Pronto para publicar**: ✅
**Documentação atualizada**: 26/11/2025
