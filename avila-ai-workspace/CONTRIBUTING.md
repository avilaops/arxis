# 🤝 Contributing to AvilaDB

Obrigado pelo interesse em contribuir com AvilaDB! Este documento contém diretrizes para contribuições.

## 📜 Filosofia do Projeto

Antes de contribuir, entenda os princípios fundamentais:

1. **Zero Dependências Externas**: Não aceitamos PRs que adicionem dependências de terceiros (exceto `core`, `alloc`). Implementamos tudo do zero.

2. **Criptografia Soberana**: Priorizamos algoritmos provados por Bitcoin/Ethereum (secp256k1, Schnorr) sobre padrões governamentais (P-256, RSA).

3. **Constant-Time**: Todo código criptográfico DEVE ser constant-time para resistir a timing attacks.

4. **Performance First**: Otimizações SIMD, stack-only operations, zero heap allocation em hot paths.

5. **Matemática Transparente**: Preferimos implementações explícitas e auditáveis sobre "magia" de bibliotecas.

## 🚀 Getting Started

### 1. Setup Ambiente

```bash
# Clone o repositório
git clone https://github.com/avilaeng/aviladb.git
cd aviladb

# Build em modo debug
make debug

# Roda testes
make test

# Roda clippy (linter)
make lint
```

### 2. Estrutura do Projeto

```
avila-ai-workspace/
├── avila-nucleus/      # Operações atômicas (adc, sbb, mul)
├── avila-primitives/   # Fixed-size integers (U256, U512, etc)
├── avila-math/         # Matemática modular (Montgomery, etc)
├── avila-crypto/       # Criptografia (secp256k1, Schnorr, BLAKE3)
├── avila-quinn/        # Protocolo QUIC
└── aviladb-core/       # Database engine (LSM, MVCC)
```

**Dependências são bottom-up:**
```
aviladb-core → avila-quinn → avila-crypto → avila-math → avila-primitives → avila-nucleus
```

### 3. Encontrando Tarefas

- **Issues marcadas `good first issue`**: Ótimas para começar
- **TODOs no código**: Procure por `TODO:` ou `FIXME:`
- **TECHNICAL.md**: Lista implementações pendentes

## 📝 Processo de Contribuição

### 1. Crie um Issue (se não existir)

Antes de começar, abra um issue descrevendo:
- O que você quer implementar/consertar
- Por que é importante
- Abordagem planejada (para features grandes)

### 2. Fork & Branch

```bash
# Fork no GitHub, depois:
git clone https://github.com/SEU_USER/aviladb.git
cd aviladb

# Crie branch descritiva
git checkout -b feature/schnorr-batch-verification
# ou
git checkout -b fix/secp256k1-edge-case
```

**Convenção de nomes:**
- `feature/`: Novas funcionalidades
- `fix/`: Correções de bugs
- `perf/`: Melhorias de performance
- `docs/`: Apenas documentação
- `test/`: Apenas testes

### 3. Implemente a Mudança

#### Código Rust

**Estilo:**
```rust
// ✅ BOM: Constant-time, explicit, auditável
fn select_u64(a: u64, b: u64, condition: bool) -> u64 {
    let mask = (condition as u64).wrapping_neg(); // 0x0000 ou 0xFFFF
    (a & !mask) | (b & mask)
}

// ❌ RUIM: Timing leak via branch
fn select_u64_bad(a: u64, b: u64, condition: bool) -> u64 {
    if condition { b } else { a }  // Branch = timing attack!
}
```

**Documentação:**
```rust
/// Adiciona dois pontos na curva secp256k1.
///
/// # Algoritmo
/// P₁ + P₂ = P₃ onde:
/// - λ = (y₂ - y₁) / (x₂ - x₁)
/// - x₃ = λ² - x₁ - x₂
/// - y₃ = λ(x₁ - x₃) - y₁
///
/// # Segurança
/// - Constant-time em relação aos valores de entrada
/// - Não usa branches baseados em dados secretos
///
/// # Panics
/// Nunca. Retorna ponto no infinito se inválido.
///
/// # Exemplo
/// ```
/// use avila_crypto::curves::secp256k1::{Point, GENERATOR};
/// 
/// let p1 = GENERATOR;
/// let p2 = GENERATOR;
/// let p3 = p1.point_add(&p2);
/// ```
pub fn point_add(&self, other: &Point) -> Point {
    // implementação...
}
```

**Testes:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_addition_identity() {
        // G + O = G (O = ponto no infinito)
        let g = GENERATOR;
        let identity = Point::identity();
        let result = g.point_add(&identity);
        assert_eq!(result, g);
    }

    #[test]
    fn test_point_doubling() {
        // 2G = G + G
        let g = GENERATOR;
        let doubled = g.point_double();
        let added = g.point_add(&g);
        assert_eq!(doubled, added);
    }

    #[test]
    fn test_constant_time_property() {
        // Verifica que select_u64 não vaza timing
        use std::time::Instant;

        let iterations = 10_000_000;
        
        // Mede tempo para condition=true
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(select_u64(42, 99, true));
        }
        let time_true = start.elapsed();

        // Mede tempo para condition=false
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(select_u64(42, 99, false));
        }
        let time_false = start.elapsed();

        // Diferença deve ser < 1%
        let diff = (time_true.as_nanos() as f64 - time_false.as_nanos() as f64).abs();
        let avg = (time_true.as_nanos() + time_false.as_nanos()) as f64 / 2.0;
        let percent = (diff / avg) * 100.0;

        assert!(percent < 1.0, "Timing leak detected: {:.2}% difference", percent);
    }
}
```

#### Benchmarks

Para código de performance crítica, adicione benchmarks:

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_crypto::curves::secp256k1::{Point, GENERATOR};

fn bench_scalar_mul(c: &mut Criterion) {
    let g = GENERATOR;
    let scalar = [0xFFu8; 32]; // Pior caso

    c.bench_function("secp256k1_scalar_mul", |b| {
        b.iter(|| {
            black_box(g.scalar_mul(&scalar))
        })
    });
}

criterion_group!(benches, bench_scalar_mul);
criterion_main!(benches);
```

### 4. Valide Localmente

```bash
# Formata código
make format

# Roda linter
make lint

# Roda todos os testes
make test

# Benchmarks (se aplicável)
make bench

# Build em todas as configurações
make all
```

**Checklist antes de commit:**
- [ ] `make format` sem mudanças pendentes
- [ ] `make lint` sem warnings
- [ ] `make test` todos passando
- [ ] Código documentado com `///` docs
- [ ] Testes adicionados para novo código
- [ ] Benchmarks se for código hot-path
- [ ] `CHANGELOG.md` atualizado (se feature visível)

### 5. Commit & Push

**Convenção de commits:**

```bash
# Feature
git commit -m "feat(crypto): implementa batch verification para Schnorr

- Adiciona verify_batch() em schnorr.rs
- 15% mais rápido que verificações individuais
- Testes com vetores do BIP-340

Closes #42"

# Bugfix
git commit -m "fix(storage): corrige race condition em compaction

- Adiciona Mutex em CompactionState
- Testes de concorrência adicionados

Fixes #123"

# Performance
git commit -m "perf(math): usa Montgomery ladder em scalar_mul

- Reduz operações de 384 para 256
- 30% mais rápido em benchmarks
- Mantém constant-time"
```

**Prefixos:**
- `feat`: Nova funcionalidade
- `fix`: Correção de bug
- `perf`: Melhoria de performance
- `docs`: Apenas documentação
- `test`: Adiciona/melhora testes
- `refactor`: Refatoração sem mudança de comportamento
- `style`: Formatação, espaços, etc
- `chore`: Tarefas de manutenção

```bash
# Push para seu fork
git push origin feature/schnorr-batch-verification
```

### 6. Abra Pull Request

No GitHub:

1. Clique "New Pull Request"
2. Compare seu branch com `main`
3. Preencha template:

```markdown
## Descrição
Implementa batch verification para assinaturas Schnorr, permitindo verificar
múltiplas assinaturas em uma única operação usando randomização.

## Motivação
Ao validar blocos com centenas de transações, verificar cada assinatura
individualmente é lento. Batch verification é ~15% mais rápido.

## Mudanças
- Adiciona `verify_batch()` em `avila-crypto/src/signatures/schnorr.rs`
- Usa técnica de Bellare-Garay-Rabin com randomização
- Testes com vetores do BIP-340
- Benchmarks comparativos

## Checklist
- [x] Código formatado (`make format`)
- [x] Linter limpo (`make lint`)
- [x] Testes passando (`make test`)
- [x] Benchmarks adicionados
- [x] Documentação atualizada
- [x] CHANGELOG.md atualizado

## Performance
```
test bench_verify_individual ... bench:   1,234 ns/iter
test bench_verify_batch      ... bench:   1,050 ns/iter (-15%)
```

## Closes
Closes #42
```

### 7. Code Review

Mantenedores revisarão e poderão:

- **Aprovar**: PR será merged!
- **Request Changes**: Faça as mudanças solicitadas
- **Comment**: Discussão sobre abordagem

**Respondendo a feedback:**

```bash
# Faz mudanças solicitadas
git add .
git commit -m "fix: aplica feedback de review"
git push origin feature/schnorr-batch-verification

# PR é atualizado automaticamente
```

## 🧪 Testes

### Tipos de Testes

1. **Unit Tests**: Testam funções isoladas
   ```rust
   #[test]
   fn test_u256_addition() {
       let a = U256::from_u64(100);
       let b = U256::from_u64(200);
       let result = a.wrapping_add(&b);
       assert_eq!(result, U256::from_u64(300));
   }
   ```

2. **Property Tests**: Testam propriedades matemáticas
   ```rust
   #[test]
   fn test_modular_arithmetic_properties() {
       // a + b (mod p) = (a mod p) + (b mod p) (mod p)
       let a = U256::from_u64(123456);
       let b = U256::from_u64(789012);
       let p = U256::from_u64(1000);
       
       let lhs = (a.wrapping_add(&b)).mod_reduce(&p);
       let rhs = (a.mod_reduce(&p).wrapping_add(&b.mod_reduce(&p))).mod_reduce(&p);
       assert_eq!(lhs, rhs);
   }
   ```

3. **Crypto Test Vectors**: Use vetores conhecidos
   ```rust
   #[test]
   fn test_secp256k1_generator() {
       // Ponto gerador G da secp256k1
       let g = GENERATOR;
       assert_eq!(
           g.x.to_hex(),
           "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
       );
       assert_eq!(
           g.y.to_hex(),
           "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"
       );
   }
   ```

4. **Integration Tests**: Testam múltiplos componentes
   ```rust
   #[test]
   fn test_end_to_end_transaction() {
       let db = AvilaDB::new("test-db");
       let tx = db.begin_transaction();
       tx.put("key", "value");
       tx.commit();
       
       let value = db.get("key").unwrap();
       assert_eq!(value, "value");
   }
   ```

### Rodando Testes

```bash
# Todos os testes
make test

# Específico de um crate
cargo test -p avila-crypto

# Teste específico
cargo test -p avila-crypto test_schnorr_signature

# Com output verboso
make test-verbose

# Apenas integration tests
make test-integration
```

## 🚀 Performance

### Diretrizes

1. **Medir antes e depois**: Use benchmarks
   ```bash
   cargo bench --bench my_bench > before.txt
   # Faz mudança
   cargo bench --bench my_bench > after.txt
   ```

2. **Perfil antes de otimizar**:
   ```bash
   # Linux perf
   cargo build --release
   perf record -F 99 -g ./target/release/aviladb
   perf report
   ```

3. **SIMD quando apropriado**:
   ```rust
   #[cfg(target_arch = "x86_64")]
   use core::arch::x86_64::*;

   #[target_feature(enable = "avx2")]
   unsafe fn add_u64x4_avx2(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
       let va = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
       let vb = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
       let result = _mm256_add_epi64(va, vb);
       
       let mut output = [0u64; 4];
       _mm256_storeu_si256(output.as_mut_ptr() as *mut __m256i, result);
       output
   }
   ```

4. **Evite heap allocation em hot paths**:
   ```rust
   // ✅ BOM: Stack allocation
   let mut buffer = [0u8; 32];
   hash_function(&mut buffer);

   // ❌ RUIM: Heap allocation
   let mut buffer = vec![0u8; 32];
   hash_function(&mut buffer);
   ```

## 📚 Documentação

### Quando Documentar

- **Sempre**: Funções públicas (`pub fn`)
- **Sempre**: Structs e enums públicos
- **Opcionalmente**: Funções privadas complexas
- **Sempre**: Algoritmos não-óbvios

### Como Documentar

```rust
/// One-line summary que aparece na listagem.
///
/// Descrição mais longa com detalhes sobre o comportamento,
/// algoritmos usados, e considerações de segurança.
///
/// # Algoritmo
/// Descreve matematicamente o que faz:
/// - Passo 1: x = foo(a)
/// - Passo 2: y = bar(x, b)
///
/// # Segurança
/// - Constant-time em relação a `secret_key`
/// - Não vaza informação via timing
///
/// # Panics
/// Quando a função pode panic (evite isso!).
///
/// # Safety
/// Se for `unsafe`, explique precondições.
///
/// # Exemplo
/// ```
/// use avila_crypto::hash::blake3;
///
/// let data = b"Hello AvilaDB";
/// let hash = blake3::hash(data);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn my_function(a: u64, b: u64) -> u64 {
    // implementação
}
```

## ❌ O Que NÃO Fazer

1. **Adicionar dependências**: Implementamos tudo do zero
2. **Usar `unsafe` sem justificativa**: Apenas para SIMD intrinsics
3. **Branches baseados em dados secretos**: Timing attacks!
4. **Heap allocation desnecessária**: Stack sempre que possível
5. **Código "mágico" sem explicação**: Matemática deve ser clara
6. **Commits enormes**: Quebre em commits lógicos menores
7. **Ignorar clippy warnings**: Sempre resolva ou justifique `#[allow]`

## 🤔 Dúvidas?

- **Issues**: Abra uma issue com tag `question`
- **Discussions**: Use GitHub Discussions para design/arquitetura
- **Email**: dev@avila.inc

## 📜 Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas
sob a mesma licença do projeto (MIT ou Apache 2.0, a definir).

---

**Obrigado por contribuir com AvilaDB! 🇧🇷**

Juntos construímos infraestrutura soberana para o futuro.
