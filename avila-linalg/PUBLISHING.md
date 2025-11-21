# 🚀 Guia de Publicação - avila-linalg

## ✅ Checklist Pré-Publicação

Antes de publicar no crates.io, verifique:

- [x] **CHANGELOG.md** atualizado com versão 0.1.1
- [x] **Cargo.toml** com version = "0.1.1"
- [x] Testes passando: **12 testes OK**
- [x] Build release funcionando
- [x] Documentação completa:
  - [x] README.md
  - [x] ADVANCED.md
  - [x] STATUS.md
  - [x] MIGRATION.md
  - [x] ROADMAP.md
  - [x] SUMMARY.md
- [x] Exemplos funcionando:
  - [x] basic_usage.rs
  - [x] engine_aaa.rs

---

## 📦 Publicar no crates.io

### Opção 1: Via Workspace Root (Recomendado)

Se o repositório Arxis tem automação via GitHub Actions:

```bash
# 1. Commit todas as mudanças
git add .
git commit -m "feat(avila-linalg): Release v0.1.1 with quaternions and 4D transforms"

# 2. Push para main
git push origin main

# 3. Criar e publicar tag
git tag avila-linalg-v0.1.1
git push origin avila-linalg-v0.1.1

# 4. GitHub Actions vai publicar automaticamente
# Acompanhe em: https://github.com/avilaops/arxis/actions
```

### Opção 2: Publicação Manual

Se precisar publicar manualmente:

```bash
cd avila-linalg

# Verificar pacote (dry-run)
cargo publish --dry-run

# Publicar de verdade
cargo publish
```

**Nota:** Você precisa estar logado no crates.io:
```bash
cargo login <seu-token-do-crates.io>
```

---

## 📋 Verificações Automáticas

O `cargo publish` vai verificar automaticamente:

1. ✅ Todos os arquivos estão commitados
2. ✅ `Cargo.toml` tem campos obrigatórios:
   - `name`, `version`, `edition`
   - `license`
   - `description`
   - `repository`
3. ✅ README.md existe
4. ✅ Testes passam
5. ✅ Documentação compila

---

## 🔍 O Que Será Publicado

Execute `cargo package --list` para ver:

```
Cargo.toml
CHANGELOG.md
README.md
ADVANCED.md
STATUS.md
MIGRATION.md
ROADMAP.md
SUMMARY.md
src/lib.rs
src/vector.rs
src/matrix.rs
src/ops.rs
src/transform.rs
examples/basic_usage.rs
examples/engine_aaa.rs
```

**Não será incluído:**
- `.git/` (histórico git)
- `target/` (artefatos de build)
- Arquivos listados em `.gitignore`

---

## 📊 Após Publicação

### 1. Verificar no crates.io
- URL: https://crates.io/crates/avila-linalg
- Versão 0.1.1 deve aparecer em alguns minutos

### 2. Verificar Documentação
- URL: https://docs.rs/avila-linalg/0.1.1
- Docs são geradas automaticamente pelo docs.rs

### 3. Atualizar Dependentes (se houver)

Se outros projetos dependem de `avila-linalg`:

```toml
[dependencies]
avila-linalg = "0.1.1"
```

---

## 🐛 Problemas Comuns

### Erro: "crate already exists"

**Causa:** Versão 0.1.1 já foi publicada.

**Solução:** Incremente a versão:
```toml
version = "0.1.2"
```

### Erro: "uncommitted changes"

**Causa:** Há arquivos modificados não commitados.

**Solução:**
```bash
git add .
git commit -m "chore: Prepare for release"
```

Ou use `--allow-dirty` (não recomendado):
```bash
cargo publish --allow-dirty
```

### Erro: "failed to authenticate"

**Causa:** Token do crates.io inválido.

**Solução:**
1. Vá em https://crates.io/settings/tokens
2. Gere novo token
3. `cargo login <novo-token>`

### Documentação não compila

**Causa:** Erros em doc comments.

**Solução:**
```bash
cargo doc --no-deps --open
```

Corrija os erros e publique novamente.

---

## 📈 Métricas da Publicação

### v0.1.1

| Métrica               | Valor               |
| --------------------- | ------------------- |
| **Linhas de código**  | ~1240               |
| **Dependências**      | 1 (num-traits)      |
| **Tamanho do .crate** | ~50 KB (comprimido) |
| **Documentação**      | ~4500 linhas        |
| **Exemplos**          | 2                   |
| **Testes**            | 12                  |

### Comparação

| Biblioteca   | Tamanho .crate | Deps Transitivas |
| ------------ | -------------- | ---------------- |
| avila-linalg | ~50 KB         | 1 → ~10          |
| nalgebra     | ~500 KB        | 10 → ~40         |
| ndarray      | ~400 KB        | 8 → ~30          |

**avila-linalg é 10x menor!**

---

## 🎯 Próximos Passos Após Publicação

### 1. Anunciar

- [ ] Post no blog Avila
- [ ] Tweet/X: @AvilaCloud
- [ ] Reddit: r/rust
- [ ] Discord: Rust Brasil

### 2. Monitorar

- [ ] Downloads no crates.io
- [ ] Issues no GitHub
- [ ] Feedback da comunidade

### 3. Roadmap v0.2.0

- [ ] SVD (Singular Value Decomposition)
- [ ] Eigenvalues/Eigenvectors
- [ ] QR Decomposition
- [ ] Inversa 4×4 completa

---

## 📞 Contato

**Dúvidas sobre publicação?**

- 📧 Email: nicolas@avila.inc
- 💬 Discord: Avila Development Team
- 📝 Issues: https://github.com/avilaops/arxis/issues

---

## 🎓 Exemplo Completo de Publicação

```bash
# Situação: avila-linalg v0.1.1 pronto para publicar

# Passo 1: Garantir que está tudo commitado
cd avila-linalg
git status  # Deve estar limpo

# Passo 2: Verificar versão
cat Cargo.toml | grep version  # version = "0.1.1"

# Passo 3: Verificar testes
cargo test --quiet
# 12 passed ✅

# Passo 4: Verificar build
cargo build --release
# Finished release ✅

# Passo 5: Dry-run (simulação)
cargo publish --dry-run
# Uploading avila-linalg v0.1.1 ✅

# Passo 6: Publicar de verdade!
cargo publish

# Passo 7: Verificar
# Aguarde 1-2 minutos e acesse:
# https://crates.io/crates/avila-linalg

# Passo 8: Criar tag Git
git tag avila-linalg-v0.1.1
git push origin avila-linalg-v0.1.1

# DONE! 🎉
```

---

## 🎊 Sucesso!

Após publicação, sua biblioteca estará disponível para todo o ecossistema Rust:

```toml
# Qualquer projeto Rust pode usar:
[dependencies]
avila-linalg = "0.1.1"
```

```rust
use avila_linalg::prelude::*;

fn main() {
    let q = Quaternion::from_euler(0.0, PI/2.0, 0.0);
    println!("Quaternion: {:?}", q);
}
```

**🚀 100% Avila. Zero Bloat. Global. 🇧🇷**

---

**Versão do Guia:** 1.0
**Data:** 21 de Novembro de 2025
**Autor:** Nícolas Ávila <nicolas@avila.inc>
