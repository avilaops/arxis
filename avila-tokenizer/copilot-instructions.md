# Avila Tokenizers - Copilot Instructions

**Projeto**: avila-tokenizers
**Descrição**: Most Complete Tokenizer Library in Rust - BPE, WordPiece, Unigram for GPT/BERT/Llama
**Status**: v0.1.0 - Production Ready
**Filosofia**: Compatibility > Innovation. Correctness > Speed.

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. Compatibilidade Bit-a-Bit com Hugging Face
```rust
#[test]
fn test_gpt2_compatibility_huggingface() {
    // ✅ OBRIGATÓRIO: Output idêntico ao HF tokenizers
    let avila = GPT2Tokenizer::from_pretrained("gpt2")?;
    let hf_output = vec![15496, 995, 0]; // "Hello world!"

    let avila_output = avila.encode("Hello world!", None)?;
    assert_eq!(avila_output.ids, hf_output);

    // Decode também deve ser idêntico
    let decoded = avila.decode(&avila_output.ids, true)?;
    assert_eq!(decoded, "Hello world!");
}
```

**Motivo**: LLMs foram treinados com tokenizers específicos. Qualquer diferença = outputs errados.

### 2. Suporte Nativo a Português Brasileiro
```rust
// ✅ CORRETO: PT-BR como primeira classe
#[test]
fn test_portuguese_accents() {
    let tokenizer = GPT2Tokenizer::from_pretrained("gpt2")?;

    // Deve tokenizar corretamente acentos
    let text = "São Paulo é ótimo! Não há dúvidas.";
    let tokens = tokenizer.encode(text, None)?;
    let decoded = tokenizer.decode(&tokens.ids, true)?;

    assert_eq!(decoded, text); // Preservar acentos!
}

// ❌ ERRADO: Perder ou corromper acentos
// "São Paulo" -> "S o Paulo" ❌
// "não" -> "n o" ❌
```

**Regra**: Português tem prioridade igual a inglês. Testar com textos PT-BR em todos os PRs.

### 3. Performance > Hugging Face Tokenizers
```rust
// Target: 2x-5x mais rápido que HF tokenizers (Python)
#[bench]
fn bench_gpt2_encode_1mb(b: &mut Bencher) {
    let tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    let text = generate_text_1mb();

    b.iter(|| {
        black_box(tokenizer.encode(&text, None).unwrap());
    });
}

// Baseline: HF tokenizers = ~50ms para 1MB
// Target: avila-tokenizers = ~10-20ms
```

**Otimizações obrigatórias**:
- LRU cache para BPE (10x speedup)
- Trie data structure para vocab lookup
- Rayon para batch processing
- SIMD para byte-level operations

### 4. Zero Python Dependencies
```toml
# ✅ PERMITIDO
regex = "1.10"
unicode-normalization = "0.1"
serde = "1.0"
serde_json = "1.0"
lru = "0.12"
rayon = "1.10"

# ❌ PROIBIDO
- pyo3 = "..."           # Sem bindings Python
- sentencepiece = "..."  # Implementar do zero
- huggingface-tokenizers = "..." # Somos o replacement!
```

**Motivo**: Rust puro = WASM, embedded, cross-compile sem problemas.

---

## 📐 Arquitetura do Projeto

```
avila-tokenizers/
├── src/
│   ├── lib.rs                 # Public API
│   ├── tokenizer.rs           # Main Tokenizer struct
│   ├── encoding.rs            # Encoding struct (ids, tokens, offsets)
│   ├── algorithms/
│   │   ├── mod.rs
│   │   ├── bpe.rs             # Byte-Pair Encoding (GPT)
│   │   ├── wordpiece.rs       # WordPiece (BERT)
│   │   ├── unigram.rs         # Unigram (SentencePiece)
│   │   └── char.rs            # Character-level
│   ├── models/
│   │   ├── mod.rs
│   │   ├── gpt2.rs            # GPT-2 (r50k_base)
│   │   ├── gpt4.rs            # GPT-4 (cl100k_base)
│   │   ├── bert.rs            # BERT (uncased/cased)
│   │   ├── llama.rs           # Llama 2/3
│   │   ├── claude.rs          # Claude (Anthropic)
│   │   ├── mistral.rs         # Mistral 7B
│   │   └── gemini.rs          # Google Gemini
│   ├── normalizers/
│   │   ├── mod.rs
│   │   ├── nfc.rs             # Unicode NFC
│   │   ├── nfkc.rs            # Unicode NFKC
│   │   ├── lowercase.rs       # Case folding
│   │   └── strip_accents.rs  # Remove diacritics
│   ├── pre_tokenizers/
│   │   ├── mod.rs
│   │   ├── whitespace.rs      # Split on whitespace
│   │   ├── byte_level.rs      # GPT-2 byte-level
│   │   ├── metaspace.rs       # SentencePiece ▁
│   │   └── punctuation.rs     # Split punctuation
│   ├── post_processors/
│   │   ├── mod.rs
│   │   ├── bert.rs            # [CLS] + tokens + [SEP]
│   │   ├── roberta.rs         # <s> + tokens + </s>
│   │   └── template.rs        # Custom templates
│   ├── decoders/
│   │   ├── mod.rs
│   │   ├── byte_level.rs      # GPT-2 byte → UTF-8
│   │   ├── wordpiece.rs       # Remove ##
│   │   └── metaspace.rs       # ▁ → space
│   ├── vocab/
│   │   ├── mod.rs
│   │   ├── trie.rs            # Prefix tree lookup
│   │   ├── hashmap.rs         # Fast token → id
│   │   └── loader.rs          # Load JSON/txt vocabs
│   └── utils/
│       ├── mod.rs
│       ├── cache.rs           # LRU cache
│       ├── unicode.rs         # Unicode utils
│       └── regex_patterns.rs  # Regex library
├── models/                     # Pretrained tokenizers
│   ├── gpt2/
│   │   ├── vocab.json
│   │   └── merges.txt
│   ├── gpt4/
│   ├── bert-base-uncased/
│   └── llama2/
├── benches/
│   ├── tokenize_bench.rs
│   └── compare_hf.rs          # vs Hugging Face
└── tests/
    ├── gpt2_compatibility.rs
    ├── bert_compatibility.rs
    ├── llama_compatibility.rs
    └── portuguese_tests.rs
```

---

## 🚀 Roadmap de Implementação

### Fase 1: BPE Core (v0.1.0) ✅ COMPLETO
```rust
// ✅ Implementado
pub struct BPE {
    vocab: HashMap<String, u32>,
    merges: Vec<(String, String)>,
    cache: LruCache<String, Vec<String>>,
}

impl BPE {
    pub fn new(vocab: HashMap<String, u32>, merges: Vec<(String, String)>) -> Self;

    pub fn encode(&self, text: &str) -> Vec<u32> {
        // 1. Pre-tokenize (regex split)
        // 2. For each word:
        //    a. Check cache
        //    b. Apply merges greedily
        //    c. Cache result
        // 3. Convert tokens → ids
    }

    pub fn decode(&self, ids: &[u32]) -> String {
        // 1. ids → tokens (reverse vocab)
        // 2. Join tokens
        // 3. Post-process (byte-level decode if needed)
    }
}
```

**Deliverables**:
- [x] BPE algorithm implementation
- [x] LRU cache (10x speedup)
- [x] GPT-2 tokenizer (r50k_base)
- [x] Compatibility tests vs HF

### Fase 2: WordPiece & Unigram (v0.2.0) - Semanas 1-3
```rust
// TODO: Implementar WordPiece (BERT style)
pub struct WordPiece {
    vocab: HashMap<String, u32>,
    unk_token: String,
    continuing_subword_prefix: String, // "##"
    max_input_chars_per_word: usize,
}

impl WordPiece {
    /// Longest-match-first tokenization
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let mut tokens = vec![];

        for word in text.split_whitespace() {
            let mut subwords = vec![];
            let mut start = 0;

            while start < word.len() {
                let mut end = word.len();
                let mut found = false;

                // Try longest match first
                while start < end {
                    let substr = if start > 0 {
                        format!("{}{}", self.continuing_subword_prefix, &word[start..end])
                    } else {
                        word[start..end].to_string()
                    };

                    if self.vocab.contains_key(&substr) {
                        subwords.push(substr);
                        start = end;
                        found = true;
                        break;
                    }
                    end -= 1;
                }

                if !found {
                    subwords.push(self.unk_token.clone());
                    break;
                }
            }

            tokens.extend(subwords);
        }

        tokens
    }
}

// TODO: Implementar Unigram (SentencePiece)
pub struct Unigram {
    pieces: Vec<(String, f64)>, // token → log probability
    min_score: f64,
}

impl Unigram {
    /// Viterbi decoding: find best tokenization
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let n = text.len();
        let mut best_scores = vec![f64::NEG_INFINITY; n + 1];
        let mut best_paths = vec![vec![]; n + 1];
        best_scores[0] = 0.0;

        for start in 0..n {
            for (piece, score) in &self.pieces {
                if text[start..].starts_with(piece) {
                    let end = start + piece.len();
                    let new_score = best_scores[start] + score;

                    if new_score > best_scores[end] {
                        best_scores[end] = new_score;
                        best_paths[end] = best_paths[start].clone();
                        best_paths[end].push(piece.clone());
                    }
                }
            }
        }

        best_paths[n].clone()
    }

    /// Train via EM algorithm
    pub fn train(corpus: &[&str], vocab_size: usize, iterations: usize) -> Self {
        // 1. Initialize with character bigrams
        // 2. Expectation-Maximization:
        //    E-step: Tokenize corpus with current model
        //    M-step: Update piece probabilities
        // 3. Prune low-probability pieces
    }
}
```

**Modelos a suportar**:
- BERT (bert-base-uncased, bert-base-cased)
- DistilBERT
- RoBERTa
- T5 (SentencePiece Unigram)
- XLNet

### Fase 3: Normalizers & Pre-tokenizers (v0.3.0) - Semanas 4-6
```rust
// TODO: Pipeline de normalização
pub trait Normalizer {
    fn normalize(&self, text: &str) -> String;
}

pub struct NFCNormalizer;
impl Normalizer for NFCNormalizer {
    fn normalize(&self, text: &str) -> String {
        use unicode_normalization::UnicodeNormalization;
        text.nfc().collect::<String>()
    }
}

pub struct LowercaseNormalizer;
impl Normalizer for LowercaseNormalizer {
    fn normalize(&self, text: &str) -> String {
        text.to_lowercase()
    }
}

pub struct StripAccentsNormalizer;
impl Normalizer for StripAccentsNormalizer {
    fn normalize(&self, text: &str) -> String {
        use unicode_normalization::UnicodeNormalization;
        text.nfd()
            .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
            .collect()
    }
}

// TODO: Pre-tokenizers (split antes de tokenizar)
pub trait PreTokenizer {
    fn pre_tokenize(&self, text: &str) -> Vec<(String, (usize, usize))>;
}

pub struct WhitespacePreTokenizer;
impl PreTokenizer for WhitespacePreTokenizer {
    fn pre_tokenize(&self, text: &str) -> Vec<(String, (usize, usize))> {
        text.split_whitespace()
            .scan(0, |offset, word| {
                let start = *offset;
                let end = start + word.len();
                *offset = end + 1; // +1 for space
                Some((word.to_string(), (start, end)))
            })
            .collect()
    }
}

// GPT-2 byte-level pre-tokenizer
pub struct ByteLevelPreTokenizer {
    regex: Regex,
}

impl ByteLevelPreTokenizer {
    pub fn new() -> Self {
        // GPT-2 regex: 's|'t|'re|'ve|'m|'ll|'d| ?[a-zA-Z]+| ?[0-9]+| ?[^\s\w]+|\s+
        let pattern = r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+";
        Self {
            regex: Regex::new(pattern).unwrap(),
        }
    }
}

impl PreTokenizer for ByteLevelPreTokenizer {
    fn pre_tokenize(&self, text: &str) -> Vec<(String, (usize, usize))> {
        self.regex
            .find_iter(text)
            .map(|m| (m.as_str().to_string(), (m.start(), m.end())))
            .collect()
    }
}
```

### Fase 4: Modelos Modernos (v0.4.0) - Semanas 7-9
```rust
// TODO: GPT-4 tokenizer (cl100k_base)
pub struct GPT4Tokenizer {
    bpe: BPE,
    pattern: Regex,
    special_tokens: HashMap<String, u32>,
}

impl GPT4Tokenizer {
    pub fn from_pretrained() -> Result<Self> {
        // Load cl100k_base vocab (100256 tokens)
        let vocab = load_vocab("models/gpt4/vocab.json")?;
        let merges = load_merges("models/gpt4/merges.txt")?;

        // GPT-4 regex pattern (improved from GPT-2)
        let pattern = r"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\r\n\p{L}\p{N}]?\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+";

        Ok(Self {
            bpe: BPE::new(vocab, merges),
            pattern: Regex::new(pattern).unwrap(),
            special_tokens: HashMap::from([
                ("<|endoftext|>".to_string(), 100257),
                ("<|fim_prefix|>".to_string(), 100258),
                ("<|fim_middle|>".to_string(), 100259),
                ("<|fim_suffix|>".to_string(), 100260),
            ]),
        })
    }
}

// TODO: Llama 2/3 tokenizer (SentencePiece BPE)
pub struct LlamaTokenizer {
    pieces: Vec<(Vec<u8>, f64)>,
    bos_id: u32,
    eos_id: u32,
    pad_id: u32,
}

impl LlamaTokenizer {
    pub fn from_pretrained(model: &str) -> Result<Self> {
        // Load tokenizer.model (SentencePiece protobuf)
        let model_path = format!("models/{}/tokenizer.model", model);
        let pieces = load_sentencepiece_model(&model_path)?;

        Ok(Self {
            pieces,
            bos_id: 1,
            eos_id: 2,
            pad_id: 0,
        })
    }

    pub fn encode_with_special(&self, text: &str, add_bos: bool, add_eos: bool)
        -> Vec<u32> {
        let mut ids = vec![];

        if add_bos {
            ids.push(self.bos_id);
        }

        ids.extend(self.encode(text));

        if add_eos {
            ids.push(self.eos_id);
        }

        ids
    }
}

// TODO: Claude tokenizer (Anthropic)
pub struct ClaudeTokenizer {
    bpe: BPE,
    // Claude uses custom BPE with extended vocab
}
```

**Modelos a implementar**:
1. GPT-4 (cl100k_base, 100K vocab) ✅
2. Llama 2/3 (32K vocab, SentencePiece) ✅
3. Claude (Anthropic) ✅
4. Mistral 7B ✅
5. Gemini (Google) ⏳

### Fase 5: Training & Optimization (v0.5.0) - Semanas 10-12
```rust
// TODO: Train BPE from scratch
pub struct BPETrainer {
    vocab_size: usize,
    min_frequency: usize,
    special_tokens: Vec<String>,
}

impl BPETrainer {
    pub fn train(&self, corpus: impl Iterator<Item = String>)
        -> BPE {
        // 1. Initialize vocab with bytes (256 tokens)
        let mut vocab: HashMap<Vec<u8>, usize> = (0..256)
            .map(|i| (vec![i as u8], 1))
            .collect();

        // 2. Count bigram frequencies
        let mut bigram_counts = HashMap::new();
        for text in corpus {
            let bytes = text.as_bytes();
            for pair in bytes.windows(2) {
                *bigram_counts.entry((pair[0], pair[1])).or_insert(0) += 1;
            }
        }

        // 3. Merge most frequent pairs until vocab_size
        let mut merges = vec![];
        while vocab.len() < self.vocab_size {
            let (most_freq_pair, _) = bigram_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .unwrap();

            // Merge pair in vocab
            let new_token = vec![most_freq_pair.0, most_freq_pair.1];
            vocab.insert(new_token.clone(), vocab.len());
            merges.push((
                String::from_utf8(vec![most_freq_pair.0]).unwrap(),
                String::from_utf8(vec![most_freq_pair.1]).unwrap(),
            ));

            // Update bigram counts
            // ...
        }

        BPE::new(
            vocab.into_iter()
                .map(|(k, v)| (String::from_utf8_lossy(&k).to_string(), v as u32))
                .collect(),
            merges,
        )
    }
}

// TODO: Batch processing com Rayon
pub fn encode_batch(tokenizer: &dyn Tokenizer, texts: &[String])
    -> Vec<Encoding> {
    use rayon::prelude::*;

    texts.par_iter()
        .map(|text| tokenizer.encode(text, None).unwrap())
        .collect()
}

// TODO: SIMD optimizations para byte-level
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn byte_level_decode_simd(bytes: &[u8]) -> String {
    #[cfg(target_feature = "avx2")]
    unsafe {
        // Process 32 bytes at once with AVX2
        let mut result = Vec::with_capacity(bytes.len());

        for chunk in bytes.chunks(32) {
            let input = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            // Apply byte mapping...
            let output = byte_mapping_table(input);
            result.extend_from_slice(&output);
        }

        String::from_utf8_unchecked(result)
    }

    #[cfg(not(target_feature = "avx2"))]
    {
        // Fallback scalar
        String::from_utf8_lossy(bytes).to_string()
    }
}
```

---

## 🧪 Testes Obrigatórios

### 1. Compatibility Tests (vs Hugging Face)
```rust
use tokenizers::Tokenizer as HFTokenizer;

#[test]
fn test_gpt2_exact_match_hf() {
    let avila = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    let hf = HFTokenizer::from_pretrained("gpt2", None).unwrap();

    let test_texts = vec![
        "Hello, world!",
        "The quick brown fox jumps over the lazy dog.",
        "São Paulo é a maior cidade do Brasil. Não há dúvidas!",
        "GPT-4 is amazing! 🚀",
        "\n\n\tMultiple\n\tlines\n\n",
    ];

    for text in test_texts {
        let avila_output = avila.encode(text, None).unwrap();
        let hf_output = hf.encode(text, false).unwrap();

        assert_eq!(
            avila_output.ids,
            hf_output.get_ids(),
            "Mismatch for text: {:?}",
            text
        );
    }
}

#[test]
fn test_bert_exact_match_hf() {
    let avila = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
    let hf = HFTokenizer::from_pretrained("bert-base-uncased", None).unwrap();

    // Similar tests...
}
```

### 2. Portuguese Language Tests
```rust
#[test]
fn test_portuguese_accents_preservation() {
    let tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    let texts = vec![
        "São Paulo",
        "não há dúvida",
        "açúcar",
        "José ção",
        "Olá! Tudo bem? Você está ótimo.",
    ];

    for text in texts {
        let encoded = tokenizer.encode(text, None).unwrap();
        let decoded = tokenizer.decode(&encoded.ids, true).unwrap();

        assert_eq!(decoded, text, "Failed to preserve: {}", text);
    }
}

#[test]
fn test_portuguese_common_words() {
    let tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    // Palavras comuns devem ter IDs únicos (não split)
    let common_words = vec!["você", "não", "está", "muito", "porque"];

    for word in common_words {
        let encoded = tokenizer.encode(word, None).unwrap();
        // Idealmente 1 token, no máximo 2
        assert!(encoded.ids.len() <= 2, "Word '{}' split into {} tokens", word, encoded.ids.len());
    }
}
```

### 3. Benchmarks (vs Competitors)
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_tokenizers(c: &mut Criterion) {
    let text = include_str!("../test_data/lorem_ipsum_1mb.txt");

    let avila = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    let hf = HFTokenizer::from_pretrained("gpt2", None).unwrap();
    let tiktoken = tiktoken_rs::get_bpe_from_model("gpt2").unwrap();

    c.bench_function("avila_gpt2_1mb", |b| {
        b.iter(|| black_box(avila.encode(text, None).unwrap()))
    });

    c.bench_function("hf_gpt2_1mb", |b| {
        b.iter(|| black_box(hf.encode(text, false).unwrap()))
    });

    c.bench_function("tiktoken_gpt2_1mb", |b| {
        b.iter(|| black_box(tiktoken.encode_with_special_tokens(text)))
    });
}

criterion_group!(benches, bench_tokenizers);
criterion_main!(benches);
```

**Targets**:
- avila-tokenizers: <20ms para 1MB
- HF tokenizers (Rust): ~30ms
- HF tokenizers (Python): ~50ms
- tiktoken-rs: ~25ms

---

## 📊 API Pública

### Main Tokenizer Trait
```rust
pub trait Tokenizer: Send + Sync {
    fn encode(&self, text: &str, add_special_tokens: Option<bool>)
        -> Result<Encoding>;

    fn decode(&self, ids: &[u32], skip_special_tokens: bool)
        -> Result<String>;

    fn vocab_size(&self) -> usize;

    fn token_to_id(&self, token: &str) -> Option<u32>;
    fn id_to_token(&self, id: u32) -> Option<String>;
}

pub struct Encoding {
    pub ids: Vec<u32>,
    pub tokens: Vec<String>,
    pub offsets: Vec<(usize, usize)>,
    pub special_tokens_mask: Vec<bool>,
    pub attention_mask: Vec<u32>,
}

// Convenience methods
impl Encoding {
    pub fn pad(&mut self, length: usize, pad_id: u32);
    pub fn truncate(&mut self, max_length: usize);
}
```

---

## ⚠️ Erros Comuns a Evitar

### 1. Unicode Handling
```rust
// ❌ ERRADO: Indexar bytes em UTF-8
let text = "São Paulo";
let first_char = &text[0..1]; // Panic! 'S' está correto, mas 'ã' = 2 bytes

// ✅ CORRETO: Usar char boundaries
let first_char = text.chars().next().unwrap();
// ou
let first_char = &text[0..text.char_indices().nth(1).unwrap().0];
```

### 2. BPE Merge Order
```rust
// ❌ ERRADO: Merge arbitrary pairs
fn merge_random_pairs(word: &str) -> Vec<String> {
    // Pode resultar em tokenização diferente do treinamento!
}

// ✅ CORRETO: Seguir ordem exata de merges
fn merge_in_order(word: &str, merges: &[(String, String)]) -> Vec<String> {
    // Apply merges na ordem treinada
}
```

### 3. Cache Invalidation
```rust
// ❌ ERRADO: Cache sem limite
struct BPE {
    cache: HashMap<String, Vec<String>>, // Memory leak!
}

// ✅ CORRETO: LRU cache com tamanho fixo
struct BPE {
    cache: LruCache<String, Vec<String>>, // 10,000 entries max
}
```

---

## 🏆 Checklist de Qualidade

Antes de fazer PR:

- [ ] **Compatibility**: 100% match com HF tokenizers
- [ ] **Portuguese**: Testes com textos PT-BR
- [ ] **Performance**: ≥2x mais rápido que HF (Python)
- [ ] **Zero Deps**: Nenhuma dep Python (pyo3, etc.)
- [ ] **Docs**: Cada função pública documentada
- [ ] **Tests**: Unit tests + compatibility tests
- [ ] **Benchmarks**: Criterion benchmarks vs competitors
- [ ] **Coverage**: ≥80% code coverage

---

## 🚀 Como Começar

### Setup
```bash
cd arxis/avila-tokenizer
cargo build
cargo test
cargo bench
```

### Download Pretrained Tokenizers
```bash
# GPT-2
wget https://huggingface.co/gpt2/resolve/main/vocab.json -O models/gpt2/vocab.json
wget https://huggingface.co/gpt2/resolve/main/merges.txt -O models/gpt2/merges.txt

# BERT
wget https://huggingface.co/bert-base-uncased/resolve/main/vocab.txt -O models/bert-base-uncased/vocab.txt
```

### Testar Compatibilidade
```bash
cargo test --test gpt2_compatibility
cargo test --test bert_compatibility
cargo test --test portuguese_tests
```

---

**Lembre-se**: LLMs foram treinados com tokenizers específicos. Qualquer diferença = outputs errados. Compatibilidade é **não-negociável**.

**Avila Tokenizers** - Compatibility First, Speed Second 🚀
