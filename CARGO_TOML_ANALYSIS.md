# Análise dos Cargo.toml - Padrões Identificados

## ✅ Padrões Consistentes Encontrados

### 1. **Autoria Padronizada**
```toml
authors = [
    "Nícolas Ávila <nicolas@avila.inc>",
    "Avila Development Team <dev@avila.inc>",
]
```
✅ **Usado em**: `avila-math`, `avila-ml`, `avila-clustering`, `avila-compress`
⚠️ **Variações**: `aviladb` usa apenas `Nicolas Ávila`, `avila-geo` usa `AVL Platform <team@avila.cloud>`

### 2. **Licença Dual MIT/Apache-2.0**
```toml
license = "MIT OR Apache-2.0"
```
✅ **Padrão na maioria dos crates**
⚠️ **Exceção**: `avila-dataframe` usa apenas `Apache-2.0`

### 3. **Repositório Unificado**
```toml
repository = "https://github.com/avilaops/arxis"
```
✅ **Usado consistentemente**
⚠️ **Variação**: `avila-geo` usa `https://github.com/avila-cloud/arxis`

### 4. **Homepage Consistente**
```toml
# Padrão 1: Produto/Marketing
homepage = "https://avila.inc"
homepage = "https://avila.cloud"
homepage = "https://arxis.avilaops.com"

# Padrão 2: Documentação técnica
homepage = "https://docs.avila.inc"
```

### 5. **Rust Edition & Version**
```toml
edition = "2021"
rust-version = "1.70"  # Apenas em alguns crates
```
✅ **Edition 2021** é padrão em todos
⚠️ **rust-version** definido apenas em `avila-clustering` e `avila-dataframe`

### 6. **Features Comuns**
```toml
[features]
default = []
full = ["feature1", "feature2", ...]
parallel = ["rayon"]
simd = ["wide"] ou ["dep:simd"]
gpu = ["cudarc"] ou ["wgpu"]
```

### 7. **Dev Dependencies Padrão**
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
approx = "0.5"
```

### 8. **Benchmarks Estruturados**
```toml
[[bench]]
name = "nome_bench"
harness = false
```

### 9. **Profile Otimizações**
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

## ⚠️ Inconsistências Identificadas

### 1. **Autoria Variada**
| Crate | Autor |
|-------|-------|
| `arxis_quaternions`, `avila-math`, `avila-ml` | `Nícolas Ávila + Team` |
| `aviladb` | `Nicolas Ávila` (sem acento) |
| `avila-geo` | `AVL Platform <team@avila.cloud>` |
| `avila-dataframe` | `Nícolas Ávila` (apenas) |

**Recomendação**: Padronizar como:
```toml
authors = [
    "Nícolas Ávila <nicolas@avila.inc>",
    "Avila Development Team <dev@avila.inc>",
]
```

### 2. **Homepage Diversa**
| Crate | Homepage |
|-------|----------|
| `avila-math`, `avila-ml` | `https://arxis.avilaops.com` |
| `avila-compress` | `https://avila.cloud` |
| `avila-clustering` | `https://avila.inc` |
| `aviladb` | `https://avila.cloud` |

**Recomendação**: Decidir entre:
- **Marketing**: `https://avila.inc` (principal)
- **Produto**: `https://avila.cloud` (para crates AVL)
- **Projeto**: `https://arxis.avilaops.com` (para crates Arxis)

### 3. **Repositório com Variação**
- **Maioria**: `https://github.com/avilaops/arxis`
- **avila-geo**: `https://github.com/avila-cloud/arxis`

**Recomendação**: Padronizar como `https://github.com/avilaops/arxis`

### 4. **Documentação Inconsistente**
```toml
# Alguns têm:
documentation = "https://docs.rs/nome-crate"

# Outros não têm o campo
```

**Recomendação**: Adicionar em todos:
```toml
documentation = "https://docs.rs/nome-crate"
```

### 5. **Descrições com Tamanhos Variados**
- `avila-compress`: **219 caracteres** ⚠️ (limite é 200)
- `arxis_quaternions`: **288 caracteres** ❌ **EXCEDE O LIMITE!**

**Ação Obrigatória**: Reduzir descrições para máximo de **200 caracteres**.

---

## 📋 Template Padrão Recomendado

```toml
[package]
name = "nome-do-crate"
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
authors = [
    "Nícolas Ávila <nicolas@avila.inc>",
    "Avila Development Team <dev@avila.inc>",
]
license = "MIT OR Apache-2.0"
description = "Descrição curta e objetiva (máx. 200 caracteres)"
repository = "https://github.com/avilaops/arxis"
homepage = "https://avila.inc"
documentation = "https://docs.rs/nome-do-crate"
readme = "README.md"
keywords = ["keyword1", "keyword2", "keyword3", "keyword4", "keyword5"]
categories = ["categoria1", "categoria2"]
exclude = ["target/", "*.swp", ".git*"]

[lib]
name = "nome_do_crate"
path = "src/lib.rs"

[dependencies]
# Dependências aqui

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
approx = "0.5"

[features]
default = []
full = ["all-features"]

[[bench]]
name = "benchmark_name"
harness = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[profile.bench]
inherits = "release"
```

---

## 🎯 Ações Prioritárias

### ❌ **BLOQUEADORES** (Impedem publicação)

1. **Descrição de `arxis_quaternions` excede 200 chars** (288 → 200)
2. **Descrição de `avila-compress` excede 200 chars** (219 → 200)

### ⚠️ **ALTA PRIORIDADE** (Melhoram qualidade)

3. Padronizar `authors` em todos os crates
4. Padronizar `homepage` (decidir estratégia)
5. Corrigir `repository` do `avila-geo`
6. Adicionar `documentation` em todos os crates
7. Adicionar `rust-version = "1.70"` em todos

### ✅ **MÉDIA PRIORIDADE** (Boas práticas)

8. Padronizar `keywords` (relevantes para busca)
9. Adicionar `categories` em todos
10. Padronizar `exclude` patterns
11. Adicionar `[package.metadata.docs.rs]` para melhor docs

---

## 📊 Resumo por Crate

| Crate | Versão | Autoria OK? | Licença OK? | Desc. OK? | Repo OK? |
|-------|--------|-------------|-------------|-----------|----------|
| `arxis_quaternions` | 0.2.0 | ✅ | ✅ | ❌ 288 chars | ✅ |
| `avila-math` | 0.1.0 | ✅ | ✅ | ✅ | ✅ |
| `avila-ml` | 1.0.0 | ✅ | ✅ | ✅ | ✅ |
| `avila-clustering` | 0.1.0 | ✅ | ✅ | ✅ | ✅ |
| `avila-compress` | 0.8.0 | ✅ | ✅ | ⚠️ 219 chars | ✅ |
| `avila-dataframe` | 0.2.0 | ⚠️ Só Nícolas | ⚠️ Só Apache | ✅ | ✅ |
| `avila-geo` | 0.1.0 | ⚠️ AVL Platform | ✅ | ✅ | ⚠️ avila-cloud |
| `aviladb` | 0.1.0 | ⚠️ Nicolas (sem acento) | ✅ | ✅ | ✅ |

---

## 🔧 Próximos Passos

1. **Corrigir bloqueadores** (descrições longas)
2. **Padronizar metadados** (authors, homepage, repo)
3. **Adicionar campos faltantes** (documentation, rust-version)
4. **Revisar keywords e categories** para SEO
5. **Testar publicação** com `cargo publish --dry-run`

---

**Data da análise**: 26/11/2025
**Crates analisados**: 8 principais do workspace Arxis
