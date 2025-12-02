# 🎯 GUIA DE DESENVOLVIMENTO - NOTEBOOK 1 FUNDAÇÃO

## 📌 ISSUES GITHUB CRIADAS - TRACK AQUI!

**Todas issues criadas em:** https://github.com/avilaops/arxis/issues

### 🔹 ÁREA 1 - PRIMITIVOS (Layer 1)
- [#108 - avila-primitives](https://github.com/avilaops/arxis/issues/108) - Vec, String, HashMap do zero
- [#102 - avila-error](https://github.com/avilaops/arxis/issues/102) - Sistema de erros sem std
- [#106 - avila-id](https://github.com/avilaops/arxis/issues/106) - UUID/ULID sem uuid crate
- [#103 - avila-time](https://github.com/avilaops/arxis/issues/103) - Temporal sem chrono
- [#99 - avila-atom](https://github.com/avilaops/arxis/issues/99) - Atômicos sem std::sync
- [#107 - avila-cell](https://github.com/avilaops/arxis/issues/107) - Células sem std::cell
- [#105 - avila-nucleus](https://github.com/avilaops/arxis/issues/105) - Runtime sem std
- [#101 - avila-cell-core](https://github.com/avilaops/arxis/issues/101) - Arc/Mutex do zero

### 🔹 ÁREA 2 - TIPOS CORE (Layer 2)
- [#113 - avila-serde](https://github.com/avilaops/arxis/issues/113) - Serialização sem serde
- [#112 - avila-future](https://github.com/avilaops/arxis/issues/112) - Async sem tokio
- [#109 - avila-rand](https://github.com/avilaops/arxis/issues/109) - RNG sem rand crate
- [#111 - avila-rand-simple](https://github.com/avilaops/arxis/issues/111) - RNG rápido
- [#98 - avila-regex](https://github.com/avilaops/arxis/issues/98) - Regex sem regex crate
- [#110 - avila-crypto](https://github.com/avilaops/arxis/issues/110) - Crypto sem ring/openssl
- [#100 - avila-log](https://github.com/avilaops/arxis/issues/100) - Logging sem tracing
- [#104 - avila-term](https://github.com/avilaops/arxis/issues/104) - Terminal sem crossterm

---

## ⚠️ REGRA DE OURO: ABSOLUTAMENTE ZERO DEPENDÊNCIAS

**100% DO ZERO** - Nem `std`, nem crates externos, NEM NADA!

### 🚫 PROIBIDO TUDO:
- ❌ `std` (biblioteca padrão) → Implementar do ZERO
- ❌ `core` → Implementar do ZERO
- ❌ `alloc` → Implementar do ZERO
- ❌ `serde` → Implementar `avila-serde` do zero
- ❌ `tokio` → Implementar `avila-async` do zero
- ❌ `rand` → Implementar `avila-rand` do zero
- ❌ Qualquer crate externo!
- ❌ Qualquer `use std::*`
- ❌ Qualquer `use core::*`

### ✅ ÚNICO PERMITIDO:
- ✅ `#![no_std]` obrigatório
- ✅ Rust primitives (i32, u64, bool, etc)
- ✅ Dependências internas (outros módulos `avila-*` que você criar)

---

## 📋 WORKFLOW PARA CADA COPILOT

### Passo 1: Auditoria do Módulo (5-10 min)

**Abra seu módulo e analise:**

```
📁 avila-primitives/
├── Cargo.toml          ← Verificar dependências
├── src/
│   └── lib.rs         ← Verificar implementação
├── tests/             ← Verificar testes
└── examples/          ← Verificar exemplos
```

**Perguntas a responder:**

1. ✅ O módulo compila? (`cargo check`)
2. ✅ Tem testes? (`cargo test`)
3. ✅ Tem documentação? (rustdoc comments)
4. ✅ Tem exemplos funcionais?
5. ❌ Usa dependências externas? → **REMOVER**
6. ❌ Está incompleto? → **COMPLETAR**

---

### Passo 2: Remover TODAS Dependências (incluindo std)

**Exemplo: avila-error/Cargo.toml**

❌ **ERRADO:**
```toml
[dependencies]
thiserror = "1.0"  # NUNCA!
std = "*"          # NUNCA!
```

✅ **CORRETO:**
```toml
[package]
name = "avila-error"
version = "0.2.0"
edition = "2021"

[dependencies]
# ABSOLUTAMENTE VAZIO!
# Nem std, nem core, nem nada!

[features]
default = []
```

**No src/lib.rs:**
```rust
#![no_std]  // OBRIGATÓRIO!
#![no_main] // Se aplicável

// Implementar TUDO do zero
```

---

### Passo 3: Implementar do ZERO ABSOLUTO

**Exemplo: Sistema de Erros (avila-error)**

```rust
// src/lib.rs - IMPLEMENTAÇÃO 100% DO ZERO

#![no_std]  // OBRIGATÓRIO - sem std!
#![no_main] // OBRIGATÓRIO - sem runtime

/// Tipo de erro unificado para toda plataforma Avila
/// SEM usar std::error::Error, SEM usar core::fmt::Display
pub enum AvilaError {
    /// Erro genérico
    Generic { message: &'static str, code: u32 },
    /// Erro de I/O
    Io { kind: u8 },
    /// Erro de parsing
    Parse { position: usize },
}

impl AvilaError {
    /// Cria novo erro genérico
    #[inline]
    pub const fn new(message: &'static str) -> Self {
        AvilaError::Generic { message, code: 0 }
    }

    /// Retorna mensagem (implementação manual, sem Display)
    pub const fn message(&self) -> &'static str {
        match self {
            AvilaError::Generic { message, .. } => message,
            AvilaError::Io { .. } => "IO error",
            AvilaError::Parse { .. } => "Parse error",
        }
    }

    /// Converte para código numérico
    pub const fn code(&self) -> u32 {
        match self {
            AvilaError::Generic { code, .. } => *code,
            AvilaError::Io { kind } => 1000 + (*kind as u32),
            AvilaError::Parse { .. } => 2000,
        }
    }
}

// SEM traits std/core - tudo manual!
// SEM derive(Debug) - implementar manualmente se necessário
```

---

### Passo 4: Testes Completos

```rust
// tests/error_tests.rs

#[cfg(test)]
mod tests {
    use avila_error::*;

    #[test]
    fn test_error_creation() {
        let err = AvilaError::new("teste");
        assert!(matches!(err, AvilaError::Generic(_)));
    }

    #[test]
    fn test_error_display() {
        let err = AvilaError::new("erro de teste");
        assert_eq!(format!("{}", err), "erro de teste");
    }

    // Mínimo: 20 testes por módulo
}
```

---

### Passo 5: Documentação Completa

```rust
//! # Avila Error
//!
//! Sistema de erros nativo para toda plataforma Avila.
//!
//! ## Características
//! - Zero dependências externas
//! - `no_std` compatible
//! - Type-safe error handling
//!
//! ## Exemplo
//!
//! ```rust
//! use avila_error::AvilaError;
//!
//! fn pode_falhar() -> Result<(), AvilaError> {
//!     Err(AvilaError::new("algo deu errado"))
//! }
//! ```

/// Cria um novo erro genérico
///
/// # Exemplo
///
/// ```
/// let err = AvilaError::new("falha de validação");
/// ```
pub fn new(message: &'static str) -> AvilaError {
    // ...
}
```

---

## 🎯 TAREFAS ESPECÍFICAS POR MÓDULO

### 1.1 🔹 avila-primitives
**Objetivo:** Tipos primitivos base - A FUNDAÇÃO DE TUDO

**Implementar:**
- [ ] `Byte`, `Word`, `DWord`, `QWord`
- [ ] `Index`, `Offset`, `Size`
- [ ] `BitSet`, `BitVec`
- [ ] Operações bit-level otimizadas (assembly inline se necessário)
- [ ] **ZERO dependências - nem std, nem core!**
- [ ] Implementar próprio `Vec`, `String`, `HashMap` do zero
- [ ] Implementar próprio alocador de memória

**Exemplo:**
```rust
#![no_std]
#![no_main]

// Tipo primitivo básico
#[repr(transparent)]
pub struct Byte(pub u8);

impl Byte {
    pub const ZERO: Self = Byte(0);
    pub const MAX: Self = Byte(255);

    #[inline(always)]
    pub const fn new(value: u8) -> Self {
        Byte(value)
### 1.2 🔹 avila-error
**Objetivo:** Sistema de erros unificado - 100% do zero

**Implementar:**
- [ ] Enum de erros hierárquico
- [ ] **SEM traits std::error::Error ou core::fmt::Display**
- [ ] Implementar próprio sistema de formatação
- [ ] Macros `bail!`, `ensure!` do zero (proc-macro)
- [ ] Error context manual (sem backtrace std)
- [ ] **ZERO std, ZERO core, ZERO thiserror/anyhow**

```rust
#![no_std]
#![no_main]

pub enum AvilaError {
    Generic { msg: &'static str, code: u32 },
    // ... outros
}

// SEM impl Display - criar própria forma de exibir
impl AvilaError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Generic { msg, .. } => msg,
        }
    }
}
```m std::vec::Vec)
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub const fn new() -> Self {
        Vec { ptr: core::ptr::null_mut(), len: 0, cap: 0 }
    }

    // ... implementar push, pop, etc manualmente
}
```---

### 1.2 🔹 avila-error
**Objetivo:** Sistema de erros unificado

**Implementar:**
- [ ] Enum de erros hierárquico
- [ ] Traits `Error`, `Display`
- [ ] Macros `bail!`, `ensure!`
- [ ] Error context/backtrace
- [ ] **SEM thiserror/anyhow**

---

### 1.3 🔹 avila-id
**Objetivo:** IDs únicos (UUID, ULID, etc)

**Implementar:**
- [ ] UUID v4 (128-bit random)
- [ ] ULID (timestamp + random)
- [ ] Parsing de strings
- [ ] Serialização binária
- [ ] **SEM uuid crate**

---

### 1.4 🔹 avila-time
**Objetivo:** Operações temporais

**Implementar:**
- [ ] `Instant`, `Duration`
- [ ] Timestamp (Unix time)
- [ ] Parsing ISO 8601
- [ ] Timezone-aware
- [ ] **SEM chrono**

---

### 1.5 🔹 avila-atom
**Objetivo:** Tipos atômicos thread-safe

**Implementar:**
- [ ] `AtomicByte`, `AtomicWord`
- [ ] Lock-free operations
- [ ] Memory ordering
- [ ] **Apenas std::sync::atomic**

---

### 1.6 🔹 avila-cell
**Objetivo:** Cell types para interior mutability

**Implementar:**
- [ ] `RefCell`, `Cell` wrappers
- [ ] `OnceCell`, `LazyCell`
- [ ] Thread-safe variants
- [ ] **Apenas std::cell + custom**

---

### 1.7 🔹 avila-nucleus
**Objetivo:** Operações nucleares (crypto primitives)

**Implementar:**
- [ ] Bitwise operations SIMD
- [ ] XOR, AND, OR, NOT otimizados
- [ ] Rotação de bits
- [ ] **Zero dependências**

---

### 1.8 🔹 avila-cell-core
**Objetivo:** Core de células computacionais

**Implementar:**
- [ ] Estruturas celulares base
- [ ] Lifecycle management
- [ ] Communication patterns
- [ ] Usa apenas `avila-atom` + `avila-cell`

---

### 2.1 🔸 avila-serde
**Objetivo:** Serialização NATIVA (substitui serde)

**Implementar:**
- [ ] Traits `Serialize`, `Deserialize`
- [ ] JSON parser/writer
- [ ] Binary format (bincode-like)
- [ ] Derive macros (proc-macro)
- [ ] **SEM serde**

---

### 2.2 🔸 avila-future
**Objetivo:** Futures básicos

**Implementar:**
- [ ] Trait `Future` custom
- [ ] `Poll`, `Waker`, `Context`
- [ ] Combinators (`map`, `and_then`)
- [ ] **SEM tokio/futures crate**

---

### 2.3 🔸 avila-rand
**Objetivo:** Geração aleatória

**Implementar:**
- [ ] PRNG (PCG, Xorshift)
- [ ] Crypto-secure RNG
- [ ] Distributions (uniform, normal)
- [ ] **SEM rand crate**

---

### 2.4 🔸 avila-rand-simple
**Objetivo:** Rand simplificado para casos comuns

**Implementar:**
- [ ] `thread_rng()`
- [ ] `random::<T>()`
- [ ] Shuffle, sample
- [ ] Usa `avila-rand` internamente

---

### 2.5 🔸 avila-regex
**Objetivo:** Expressões regulares

**Implementar:**
- [ ] Parser de regex
- [ ] NFA/DFA engine
- [ ] Match, search, replace
- [ ] **SEM regex crate**

---

### 2.6 🔸 avila-crypto
**Objetivo:** Criptografia base

**Implementar:**
- [ ] SHA-256, SHA-512
- [ ] AES (encrypt/decrypt)
- [ ] HMAC
- [ ] **SEM ring/openssl**

---

### 2.7 🔸 avila-log
**Objetivo:** Sistema de logging
## 🎯 CRITÉRIOS DE CONCLUSÃO

Para cada módulo estar **100% completo**:

- [ ] ✅ `#![no_std]` e `#![no_main]` obrigatórios
- [ ] ✅ Zero `use std::*` no código
- [ ] ✅ Zero `use core::*` no código
- [ ] ✅ Cargo.toml [dependencies] completamente vazio
- [ ] ✅ Compila sem warnings
- [ ] ✅ Mínimo 20 testes (implementar test framework próprio)
- [ ] ✅ Documentação completa (sem rustdoc - criar próprio)
- [ ] ✅ Mínimo 2 exemplos funcionais
- [ ] ✅ README.md explicativo
- [ ] ✅ Performance benchmarks (criar framework próprio)
- [ ] ✅ Implementação 100% manual - NADA externo
- [ ] ANSI color codes
- [ ] Formatação (bold, italic)
- [ ] Estilo de texto
- [ ] **SEM colored/termcolor**

---

## 🎯 CRITÉRIOS DE CONCLUSÃO

Para cada módulo estar **100% completo**:

- [ ] ✅ Compila sem warnings (`cargo clippy`)
- [ ] ✅ Zero dependências externas
- [ ] ✅ Mínimo 20 testes (todos passando)
- [ ] ✅ Cobertura de testes >80%
- [ ] ✅ Documentação completa (rustdoc)
- [ ] ✅ Mínimo 2 exemplos funcionais
- [ ] ✅ README.md explicativo
- [ ] ✅ Performance benchmarks (se aplicável)
- [ ] ✅ `no_std` compatible (quando possível)

---

## 🔄 WORKFLOW DIÁRIO

### Manhã (3h)
1. Abrir workspace Notebook 1
2. Cada Copilot pega 1 módulo
3. Implementar funcionalidade core
4. Escrever testes

### Tarde (3h)
5. Documentar código
6. Criar exemplos
7. Executar benchmarks
8. Code review

### Noite (2h)
9. Integração entre módulos
10. Testes de integração
11. Atualizar manifestos

---

## 📊 TRACKING DE PROGRESSO

Atualize no `NOTEBOOK1-MANIFESTO.md`:

```markdown
## 📊 Status Atual

### Área 1 - Primitivos Base
- [x] 1.1 avila-primitives - ✅ 100%
- [ ] 1.2 avila-error - 🟡 50%
- [ ] 1.3 avila-id - 🔴 0%
...

### Área 2 - Tipos Core
- [ ] 2.1 avila-serde - 🔴 0%
...
```

---

## 🚀 COMANDO RÁPIDO

Para cada módulo:

```bash
# Verificar
cargo check

# Testar
cargo test

# Documentar
cargo doc --open

# Benchmark
cargo bench

# Publicar (quando pronto)
cargo publish
```

---

**LEMBRE-SE:** Tudo do zero! Dogfooding total! 🐕🍽️
