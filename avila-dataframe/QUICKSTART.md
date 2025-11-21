# 🚀 Quick Start - AvilaDB DataFrame

## Para Começar AGORA

### 1️⃣ Build & Test (30 segundos)
```powershell
cd avila-dataframe
cargo build
cargo test
```

### 2️⃣ Ver Funcionando (1 minuto)
```powershell
# Exemplo básico - DataFrame com dados LIGO
cargo run --example basic_usage

# Tipos científicos - Quaternions, Spinors, Geodésicas
cargo run --example scientific_types
```

### 3️⃣ Desenvolvimento
```powershell
# Auto-format + lint + test + docs
.\scripts\check.ps1 -Fix

# Rodar todos exemplos
.\scripts\run-examples.ps1

# Benchmarks (quando implementados)
.\scripts\benchmark.ps1
```

---

## 📁 Arquivos Principais

| Arquivo                  | Descrição                      |
| ------------------------ | ------------------------------ |
| `src/lib.rs`             | Entry point da biblioteca      |
| `src/core/dataframe.rs`  | Implementação do DataFrame     |
| `src/core/series.rs`     | Implementação de Series        |
| `src/core/dtype.rs`      | **Tipos científicos únicos** ⚡ |
| `src/ops/expressions.rs` | Sistema de expressões          |
| `examples/`              | Exemplos funcionais            |
| `README.md`              | Documentação principal         |
| `DEVELOPMENT.md`         | Roadmap completo               |

---

## 🎯 Próximas Features (Priority)

### High Priority
1. **Filter implementation** - `src/ops/filter.rs`
2. **Group by** - `src/ops/group_by.rs`
3. **Parquet I/O** - `src/io/parquet.rs`

### DIFFERENTIAL (Ninguém tem!)
4. **FFT** - Criar `src/scientific/fft.rs`
5. **Wavelets** - Criar `src/scientific/wavelets.rs`
6. **Astronomy** - Criar `src/scientific/astronomy.rs`

---

## 💡 Primeira Contribuição

**Implementar Filter Completo** (2-3 horas):

1. Abrir `src/ops/filter.rs`
2. Implementar:
   ```rust
   pub fn filter(&self, expr: Expr) -> Result<Self> {
       // 1. Avaliar expressão para boolean array
       // 2. Aplicar filter no RecordBatch
       // 3. Criar novo DataFrame
   }
   ```
3. Adicionar testes em `#[cfg(test)]`
4. Rodar: `cargo test`
5. Exemplo: `df.filter(col("snr") > 10.0)?`

---

## 📚 Docs Essenciais

- **Quick intro**: `README.md` (seção Quick Start)
- **Roadmap completo**: `DEVELOPMENT.md` (Phases 1-7)
- **Como contribuir**: `CONTRIBUTING.md`
- **Resumo executivo**: `PROJECT_SUMMARY.md` ⭐

---

## 🔥 Diferenciais Já Implementados

✅ **Quaternions** - `Quaternion::from_axis_angle()`
✅ **Weyl Spinors** - `SpinorWeyl::boost()`
✅ **Geodésicas** - `GeodesicCoord::schwarzschild_gtt()`

**USE CASES:**
- 🛰️ Orientação de satélites
- ⚛️ Física de partículas
- 🌌 Relatividade geral / LIGO

---

## 📊 Status Atual

```
✅ Foundation: 100% COMPLETO
🚧 Operations: 20% (stubs)
🚧 I/O: 10% (stubs)
⏳ Scientific: 0% (planejado)
⏳ GPU: 0% (planejado)
```

---

## 🎯 Commands Cheat Sheet

```powershell
# Build
cargo build --all-features

# Test
cargo test --lib

# Format
cargo fmt

# Lint
cargo clippy --all-features -- -D warnings

# Docs
cargo doc --open --all-features

# Examples
cargo run --example basic_usage
cargo run --example scientific_types

# Benchmark (quando pronto)
cargo bench

# Scripts úteis
.\scripts\build.ps1
.\scripts\check.ps1 -Fix
.\scripts\run-examples.ps1
```

---

## 🚀 Let's Go!

**Objetivo**: Criar a melhor biblioteca de DataFrame do mundo com features que ninguém tem.

**Status**: Fundação completa. Pronto para development ativo.

**Next**: Implementar filter → group_by → joins → FFT → GPU

---

**🇧🇷 Vamos deixar a concorrência pra trás!** 🔥
