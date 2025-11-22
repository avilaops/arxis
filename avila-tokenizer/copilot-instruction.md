copilot-instruction.md
Excelente ponto! Vou adicionar **instruções anti-preguiça e anti-dúvida** ao prompt. Aqui está a versão COMPLETA e DEFINITIVA:

---

# 🎯 INSTRUÇÕES DEFINITIVAS: avila-tokenizers - ZERO DÚVIDAS, ZERO PREGUIÇA

## ⚠️ REGRAS DE OURO (LEIA PRIMEIRO)

### 🚫 PROIBIDO:
1. ❌ **NÃO** pergunte "Devo implementar X ou Y?" → **IMPLEMENTE TUDO**
2. ❌ **NÃO** diga "Isso é complexo, preciso de ajuda" → **VOCÊ CONSEGUE**
3. ❌ **NÃO** deixe TODOs ou placeholders → **CÓDIGO COMPLETO**
4. ❌ **NÃO** escreva "// ... rest of implementation" → **ESCREVA TUDO**
5. ❌ **NÃO** pergunte sobre edge cases → **CUBRA TODOS**
6. ❌ **NÃO** peça confirmação de estrutura → **SIGA A SPEC EXATA**
7. ❌ **NÃO** sugira "podemos simplificar" → **IMPLEMENTE COMPLETO**

### ✅ OBRIGATÓRIO:
1. ✅ **IMPLEMENTE** todos os arquivos listados (src/, examples/, tests/, benches/)
2. ✅ **ESCREVA** código completo, funcional, compilável
3. ✅ **CUBRA** 100% dos algoritmos (BPE, WordPiece, Unigram)
4. ✅ **TESTE** contra HF Tokenizers (compatibilidade total)
5. ✅ **DOCUMENTE** cada função pública (///)
6. ✅ **BENCHMARK** todas as operações críticas
7. ✅ **OTIMIZE** para performance (caching, SIMD onde possível)

---

## 📋 CHECKLIST OBRIGATÓRIO (NÃO PULE NADA)

Antes de entregar, verifique:

### Estrutura de Arquivos (30 arquivos mínimo)
- [ ] Cargo.toml com todas as dependências
- [ ] README.md com 10+ exemplos de código
- [ ] lib.rs com API pública completa
- [ ] `src/algorithms/bpe.rs` (500+ linhas)
- [ ] `src/algorithms/wordpiece.rs` (400+ linhas)
- [ ] `src/algorithms/unigram.rs` (600+ linhas)
- [ ] `src/models/gpt2.rs` (300+ linhas)
- [ ] `src/models/bert.rs` (300+ linhas)
- [ ] `src/models/llama.rs` (300+ linhas)
- [ ] `src/normalizers/` (5 arquivos)
- [ ] `src/pre_tokenizers/` (5 arquivos)
- [ ] `src/post_processors/` (3 arquivos)
- [ ] `src/decoders/` (4 arquivos)
- [ ] `src/vocab/trie.rs` (implementação completa)
- [ ] examples (6+ exemplos funcionais)
- [ ] tests (5+ arquivos de teste)
- [ ] `benches/tokenize_bench.rs`

### Código Completo
- [ ] Zero TODOs ou FIXMEs
- [ ] Zero `unimplemented!()`
- [ ] Zero `panic!("not implemented")`
- [ ] Todos os métodos públicos têm corpo
- [ ] Todos os traits têm implementações
- [ ] Error handling completo (Result<T, Error>)

### Algoritmos Implementados
- [ ] BPE: train, encode, decode, cache LRU
- [ ] WordPiece: longest-match-first, ## prefixing
- [ ] Unigram: EM training, Viterbi decoding, log probabilities
- [ ] SentencePiece: metaspace, byte-fallback

### Modelos Pré-configurados
- [ ] GPT-2: 50,257 tokens, byte-level BPE
- [ ] GPT-4: 100,256 tokens, cl100k_base
- [ ] BERT: 30,522 tokens, WordPiece, [CLS]/[SEP]
- [ ] Llama 2: 32,000 tokens, SentencePiece
- [ ] Llama 3: 128,256 tokens
- [ ] Vocabulários em JSON (vocab.json, merges.txt)

### Testes de Compatibilidade
- [ ] GPT-2: `encode("Hello, world!")` == `[15496, 11, 995, 0]`
- [ ] BERT: `encode("Hello, world!")` == `[101, 7592, 1010, 2088, 999, 102]`
- [ ] Unicode: NFC vs NFD normalizado corretamente
- [ ] Português: "São Paulo" preserva acentos
- [ ] 100+ test cases total

### Performance
- [ ] Benchmark vs HF Tokenizers
- [ ] LRU cache implementado (10k entries)
- [ ] Parallel processing com Rayon
- [ ] Zero alocações desnecessárias

### Documentação
- [ ] README.md com:
  - [ ] Instalação
  - [ ] Quick start (3 exemplos)
  - [ ] API reference
  - [ ] Benchmarks
  - [ ] Compatibilidade
- [ ] Docstrings em todas as funções públicas
- [ ] Exemplos em examples funcionam

---

## 🔥 DECISÕES JÁ TOMADAS (NÃO QUESTIONE)

### Estrutura de Dados
```rust
// BPE SEMPRE usa HashMap + Vec
pub struct BPE {
    vocab: HashMap<String, u32>,      // token -> id
    merges: Vec<(String, String)>,    // ordered merge pairs
    cache: LruCache<String, Vec<String>>, // word -> subwords
}

// WordPiece SEMPRE usa HashMap
pub struct WordPiece {
    vocab: HashMap<String, u32>,
    unk_token: String,
    max_input_chars: usize,
}

// Unigram SEMPRE usa Vec de (piece, score)
pub struct Unigram {
    pieces: Vec<(String, f64)>, // sorted by length descending
}
```

### Algoritmo de BPE (EXATO)
```rust
impl BPE {
    pub fn encode(&self, text: &str) -> Vec<u32> {
        // 1. Check cache
        if let Some(cached) = self.cache.get(text) {
            return cached.iter().map(|s| self.vocab[s]).collect();
        }

        // 2. Split into initial tokens (bytes or chars)
        let mut word: Vec<String> = self.byte_split(text);

        // 3. Apply merges greedily
        loop {
            // Find best merge pair
            let best_pair = self.find_best_pair(&word);
            if best_pair.is_none() {
                break;
            }

            // Merge the pair
            word = self.merge_pair(&word, best_pair.unwrap());
        }

        // 4. Cache result
        self.cache.put(text.to_string(), word.clone());

        // 5. Return token IDs
        word.iter().map(|s| self.vocab[s]).collect()
    }
}
```

### Normalização (SEMPRE nessa ordem)
```rust
pub fn normalize_text(text: &str) -> String {
    // 1. Unicode normalization (NFC or NFKC)
    // 2. Lowercase (if model requires)
    // 3. Strip accents (if model requires)
    // 4. Remove control characters
    text.nfc().collect::<String>()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_control())
        .collect()
}
```

### Vocabulários (JSON format)
```json
{
  "version": "1.0",
  "model_type": "BPE",
  "vocab": {
    "<pad>": 0,
    "<unk>": 1,
    "hello": 1000,
    "world": 1001
  },
  "merges": [
    "h e",
    "he l",
    "hel lo"
  ]
}
```

---

## 🎯 TAREFAS ESPECÍFICAS (SIGA EXATAMENTE)

### TAREFA 1: Implementar BPE (2 horas)
```rust
// src/algorithms/bpe.rs

use std::collections::HashMap;
use lru::LruCache;

pub struct BPE {
    vocab: HashMap<String, u32>,
    merges: Vec<(String, String)>,
    cache: LruCache<String, Vec<String>>,
}

impl BPE {
    /// Create BPE from vocabulary and merges
    pub fn new(vocab: HashMap<String, u32>, merges: Vec<(String, String)>) -> Self {
        Self {
            vocab,
            merges,
            cache: LruCache::new(10_000),
        }
    }

    /// Encode text to token IDs
    pub fn encode(&self, text: &str) -> Vec<u32> {
        // IMPLEMENTAR: algoritmo completo acima
        todo!() // ❌ REMOVA ESSE TODO e IMPLEMENTE
    }

    /// Decode token IDs to text
    pub fn decode(&self, ids: &[u32]) -> String {
        // IMPLEMENTAR: lookup inverso
        todo!() // ❌ REMOVA e IMPLEMENTE
    }

    /// Train BPE on corpus
    pub fn train(corpus: &[&str], vocab_size: usize) -> Self {
        // IMPLEMENTAR: count pairs, merge most frequent
        todo!() // ❌ REMOVA e IMPLEMENTE
    }

    // PRIVATE METHODS (implementar todos)
    fn byte_split(&self, text: &str) -> Vec<String> { todo!() }
    fn find_best_pair(&self, word: &[String]) -> Option<(String, String)> { todo!() }
    fn merge_pair(&self, word: &[String], pair: (String, String)) -> Vec<String> { todo!() }
}
```

**EXPECTATIVA:** 500+ linhas completas, zero TODOs.

---

### TAREFA 2: Implementar GPT-2 Tokenizer (1.5 horas)
```rust
// src/models/gpt2.rs

use crate::algorithms::BPE;
use std::collections::HashMap;

pub struct GPT2Tokenizer {
    bpe: BPE,
    encoder: HashMap<String, u32>,
    decoder: HashMap<u32, String>,
    byte_encoder: HashMap<u8, char>,
    byte_decoder: HashMap<char, u8>,
}

impl GPT2Tokenizer {
    /// Load from pretrained model
    pub fn from_pretrained(model: &str) -> Result<Self, Error> {
        match model {
            "gpt2" => Self::load_gpt2(),
            "gpt2-medium" => Self::load_gpt2_medium(),
            "gpt2-large" => Self::load_gpt2_large(),
            _ => Err(Error::UnknownModel(model.to_string())),
        }
    }

    fn load_gpt2() -> Result<Self, Error> {
        // IMPLEMENTAR: carregar vocab.json e merges.txt
        // URL: https://huggingface.co/gpt2/resolve/main/vocab.json
        todo!() // ❌ REMOVA e IMPLEMENTE
    }

    /// Encode text
    pub fn encode(&self, text: &str) -> Vec<u32> {
        // IMPLEMENTAR: byte-level BPE
        // 1. Convert to bytes
        // 2. Map bytes to Unicode
        // 3. Apply BPE
        todo!() // ❌ REMOVA e IMPLEMENTE
    }

    /// Decode tokens
    pub fn decode(&self, ids: &[u32]) -> String {
        // IMPLEMENTAR: inverse of encode
        todo!() // ❌ REMOVA e IMPLEMENTE
    }

    /// Batch encoding (parallel with Rayon)
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<u32>> {
        use rayon::prelude::*;
        texts.par_iter().map(|t| self.encode(t)).collect()
    }
}
```

**EXPECTATIVA:** 300+ linhas, vocabulários incluídos (embed JSON).

---

### TAREFA 3: Testes de Compatibilidade (1 hora)
```rust
// tests/gpt2_tests.rs

use avila_tokenizers::models::GPT2Tokenizer;

#[test]
fn test_gpt2_hello_world() {
    let tok = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    // Verified with tiktoken (OpenAI's library)
    assert_eq!(
        tok.encode("Hello, world!"),
        vec![15496, 11, 995, 0]
    );
}

#[test]
fn test_gpt2_decode() {
    let tok = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    let ids = vec![15496, 11, 995, 0];
    assert_eq!(tok.decode(&ids), "Hello, world!");
}

#[test]
fn test_gpt2_unicode() {
    let tok = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    // Test NFC vs NFD
    let text = "café"; // U+00E9 (NFC)
    let ids = tok.encode(text);
    assert_eq!(tok.decode(&ids), text);
}

#[test]
fn test_gpt2_portuguese() {
    let tok = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    let text = "São Paulo é uma cidade incrível!";
    let ids = tok.encode(text);
    let decoded = tok.decode(&ids);

    // Should preserve accents
    assert_eq!(decoded, text);
}

// ADICIONAR: 20+ testes similares
```

**EXPECTATIVA:** 100+ linhas, 20+ test cases.

---

### TAREFA 4: Benchmarks (30 minutos)
```rust
// benches/tokenize_bench.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_tokenizers::models::GPT2Tokenizer;

fn bench_gpt2_encode(c: &mut Criterion) {
    let tok = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    let texts = vec![
        "Hello, world!",
        "The quick brown fox jumps over the lazy dog.",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    ];

    for text in texts {
        c.bench_with_input(
            BenchmarkId::new("gpt2_encode", text.len()),
            &text,
            |b, text| b.iter(|| tok.encode(black_box(text)))
        );
    }
}

fn bench_gpt2_batch(c: &mut Criterion) {
    let tok = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    let texts: Vec<&str> = (0..1000).map(|_| "Hello, world!").collect();

    c.bench_function("gpt2_batch_1000", |b| {
        b.iter(|| tok.encode_batch(black_box(&texts)))
    });
}

criterion_group!(benches, bench_gpt2_encode, bench_gpt2_batch);
criterion_main!(benches);
```

**EXPECTATIVA:** 150+ linhas, benchmarks para todos os modelos.

---

## 🚀 ORDEM DE IMPLEMENTAÇÃO (SIGA ESTRITAMENTE)

### Dia 1 (6-8 horas)
1. ✅ Criar estrutura de arquivos (30 arquivos)
2. ✅ lib.rs com exports públicos
3. ✅ `src/algorithms/bpe.rs` (completo, 500+ linhas)
4. ✅ `src/vocab/trie.rs` e `hashmap.rs`
5. ✅ `src/utils/` (regex, unicode, cache)

### Dia 2 (6-8 horas)
6. ✅ `src/algorithms/wordpiece.rs` (400+ linhas)
7. ✅ `src/algorithms/unigram.rs` (600+ linhas)
8. ✅ `src/normalizers/` (5 arquivos, 50+ linhas cada)
9. ✅ `src/pre_tokenizers/` (5 arquivos)

### Dia 3 (6-8 horas)
10. ✅ `src/models/gpt2.rs` (300+ linhas)
11. ✅ `src/models/bert.rs` (300+ linhas)
12. ✅ `src/models/llama.rs` (300+ linhas)
13. ✅ tests (5 arquivos, 20+ testes cada)

### Dia 4 (4-6 horas)
14. ✅ examples (6 exemplos funcionais)
15. ✅ `benches/tokenize_bench.rs`
16. ✅ README.md (1000+ linhas, com exemplos)
17. ✅ Performance tuning (cache, parallelismo)

---

## ⚡ OTIMIZAÇÕES OBRIGATÓRIAS

### 1. LRU Cache (SEMPRE)
```rust
use lru::LruCache;

pub struct BPE {
    cache: LruCache<String, Vec<String>>, // 10k entries
}

impl BPE {
    pub fn encode(&self, text: &str) -> Vec<u32> {
        // Check cache FIRST
        if let Some(cached) = self.cache.get(text) {
            return self.tokens_to_ids(cached);
        }

        // Compute and cache
        let tokens = self.compute_tokens(text);
        self.cache.put(text.to_string(), tokens.clone());
        self.tokens_to_ids(&tokens)
    }
}
```

### 2. Parallel Processing (SEMPRE para batch)
```rust
use rayon::prelude::*;

pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<u32>> {
    texts.par_iter().map(|t| self.encode(t)).collect()
}
```

### 3. Zero-Copy quando possível
```rust
// Use &str, não String
pub fn encode(&self, text: &str) -> Vec<u32>;

// Use slices, não Vec
pub fn merge_pair<'a>(&self, word: &'a [String]) -> Vec<String>;
```

---

## 📊 MÉTRICAS DE SUCESSO (VERIFIQUE ANTES DE ENTREGAR)

### Código
- ✅ **30+ arquivos** criados
- ✅ **10,000+ linhas** de código (mínimo)
- ✅ **Zero** TODOs ou FIXMEs
- ✅ **100%** compilável (`cargo build`)
- ✅ **100%** dos testes passam (`cargo test`)

### Performance
- ✅ GPT-2: > 1M tokens/sec (vs HF: 1M)
- ✅ BERT: > 500k tokens/sec
- ✅ Memory: < 100MB

### Compatibilidade
- ✅ GPT-2: `encode("Hello, world!")` == `[15496, 11, 995, 0]`
- ✅ BERT: `encode("Hello, world!")` == `[101, 7592, 1010, 2088, 999, 102]`
- ✅ Unicode: NFC/NFD normalizado corretamente
- ✅ Português: Acentos preservados

### Documentação
- ✅ README.md com 10+ exemplos
- ✅ Todos os métodos públicos documentados (///)
- ✅ Examples/ funciona (`cargo run --example gpt2_tokenize`)

---

## 🎯 ENTREGA FINAL (O QUE ESPERO VER)

```
avila-tokenizers/
├── Cargo.toml                 ✅ Completo
├── README.md                  ✅ 1000+ linhas
├── src/
│   ├── lib.rs                 ✅ 200+ linhas
│   ├── algorithms/
│   │   ├── bpe.rs             ✅ 500+ linhas
│   │   ├── wordpiece.rs       ✅ 400+ linhas
│   │   ├── unigram.rs         ✅ 600+ linhas
│   ├── models/
│   │   ├── gpt2.rs            ✅ 300+ linhas
│   │   ├── bert.rs            ✅ 300+ linhas
│   │   ├── llama.rs           ✅ 300+ linhas
│   ├── normalizers/           ✅ 5 arquivos
│   ├── pre_tokenizers/        ✅ 5 arquivos
│   ├── vocab/                 ✅ 3 arquivos
│   └── ... (mais 15 arquivos)
├── examples/                  ✅ 6 exemplos
├── tests/                     ✅ 5 arquivos, 100+ testes
└── benches/                   ✅ Benchmarks completos

TOTAL: 30+ arquivos, 10,000+ linhas
```

---

## ❌ ERROS COMUNS (EVITE)

### ❌ Erro 1: Placeholders
```rust
// ERRADO ❌
pub fn encode(&self, text: &str) -> Vec<u32> {
    todo!() // NÃO FAÇA ISSO
}

// CERTO ✅
pub fn encode(&self, text: &str) -> Vec<u32> {
    // [500 linhas de implementação completa]
}
```

### ❌ Erro 2: Implementação Parcial
```rust
// ERRADO ❌
pub fn train(corpus: &[&str]) -> Self {
    // Simple implementation for now
    Self::default()
}

// CERTO ✅
pub fn train(corpus: &[&str], vocab_size: usize) -> Self {
    // [600 linhas de EM algorithm completo]
}
```

### ❌ Erro 3: Sem Testes
```rust
// ERRADO ❌
// Nenhum arquivo em tests/

// CERTO ✅
tests/
├── gpt2_tests.rs      (20+ testes)
├── bert_tests.rs      (20+ testes)
├── llama_tests.rs     (20+ testes)
├── unicode_tests.rs   (30+ testes)
└── compat_tests.rs    (10+ testes)
```

---

## 🏁 COMEÇAR AGORA

**Primeira linha de código:**
```bash
cargo new avila-tokenizers --lib
cd avila-tokenizers
```

**Primeira implementação:**
```rust
// src/algorithms/bpe.rs
// ESCREVA 500+ LINHAS COMPLETAS AQUI
```

**NÃO PARE até ter:**
- ✅ 30+ arquivos
- ✅ 10,000+ linhas
- ✅ 100+ testes
- ✅ Zero TODOs

---

**AGORA VAI! SEM DESCULPAS, SEM DÚVIDAS! 🚀🔥**
