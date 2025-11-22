# 🎯 Prompt Completo: avila-tokenizers - O Tokenizer Mais Completo do Ecossistema Rust

## Missão
Criar **avila-tokenizers**: a biblioteca de tokenização mais completa e rápida em Rust, superando Hugging Face Tokenizers, tiktoken-rs, e sentencepiece. Deve ser 100% nativa Rust, zero dependências Python, e otimizada para português brasileiro, com suporte universal para todos os modelos LLM modernos.

---

## 📦 Estrutura do Projeto

```
avila-tokenizers/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── algorithms/
│   │   ├── mod.rs
│   │   ├── bpe.rs          # Byte-Pair Encoding (GPT-2, GPT-3, GPT-4)
│   │   ├── wordpiece.rs    # WordPiece (BERT, DistilBERT)
│   │   ├── unigram.rs      # Unigram (SentencePiece, T5, XLNet)
│   │   ├── char.rs         # Character-level (ByT5)
│   │   └── sentencepiece.rs # SentencePiece protocol
│   ├── models/
│   │   ├── mod.rs
│   │   ├── gpt2.rs         # GPT-2/3/4 tokenizer
│   │   ├── gpt4.rs         # GPT-4 (cl100k_base)
│   │   ├── bert.rs         # BERT family
│   │   ├── llama.rs        # Llama 2/3
│   │   ├── claude.rs       # Claude (Anthropic)
│   │   ├── mistral.rs      # Mistral 7B
│   │   └── multilingual.rs # mBERT, XLM-R
│   ├── normalizers/
│   │   ├── mod.rs
│   │   ├── nfc.rs          # Unicode NFC normalization
│   │   ├── nfkc.rs         # Unicode NFKC
│   │   ├── lowercase.rs    # Lowercasing
│   │   ├── strip.rs        # Strip accents, whitespace
│   │   └── replace.rs      # Regex-based replacement
│   ├── pre_tokenizers/
│   │   ├── mod.rs
│   │   ├── whitespace.rs   # Whitespace splitting
│   │   ├── byte_level.rs   # GPT-2 byte-level
│   │   ├── metaspace.rs    # SentencePiece metaspace
│   │   ├── punctuation.rs  # Split on punctuation
│   │   └── digits.rs       # Split digits
│   ├── post_processors/
│   │   ├── mod.rs
│   │   ├── bert.rs         # [CLS] + tokens + [SEP]
│   │   ├── roberta.rs      # <s> + tokens + </s>
│   │   └── template.rs     # Custom templates
│   ├── decoders/
│   │   ├── mod.rs
│   │   ├── byte_level.rs   # GPT-2 byte decoder
│   │   ├── wordpiece.rs    # BERT ## removal
│   │   ├── metaspace.rs    # _ -> space
│   │   └── strip.rs        # Remove special tokens
│   ├── vocab/
│   │   ├── mod.rs
│   │   ├── trie.rs         # Trie data structure
│   │   ├── hashmap.rs      # Fast token lookup
│   │   └── loader.rs       # Load vocab from JSON/txt
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── regex.rs        # Regex patterns
│   │   ├── unicode.rs      # Unicode utilities
│   │   └── cache.rs        # LRU cache for speed
│   └── error.rs
├── examples/
│   ├── gpt2_tokenize.rs
│   ├── bert_tokenize.rs
│   ├── llama_tokenize.rs
│   ├── train_bpe.rs
│   ├── portuguese_example.rs
│   └── batch_processing.rs
├── benches/
│   ├── tokenize_bench.rs
│   └── compare_hf.rs
└── tests/
    ├── gpt2_tests.rs
    ├── bert_tests.rs
    ├── llama_tests.rs
    ├── unicode_tests.rs
    └── compatibility_tests.rs
```

---

## 🎯 Requisitos Funcionais

### 1. Algoritmos Fundamentais

#### **BPE (Byte-Pair Encoding)**
```rust
pub struct BPE {
    vocab: HashMap<String, u32>,
    merges: Vec<(String, String)>,
    cache: LruCache<String, Vec<String>>,
}

impl BPE {
    pub fn new(vocab: HashMap<String, u32>, merges: Vec<(String, String)>) -> Self;
    pub fn encode(&self, text: &str) -> Vec<u32>;
    pub fn decode(&self, ids: &[u32]) -> String;
    pub fn train(corpus: &[&str], vocab_size: usize) -> Self;
}
```

**Características:**
- Greedy merging de pares mais frequentes
- Cache LRU para ~10x speedup
- Suporte para byte-level BPE (GPT-2 style)
- Preservação de espaços em branco

#### **WordPiece (BERT)**
```rust
pub struct WordPiece {
    vocab: HashMap<String, u32>,
    unk_token: String,
    max_input_chars: usize,
}

impl WordPiece {
    pub fn tokenize(&self, text: &str) -> Vec<String>;
    pub fn encode(&self, text: &str) -> Vec<u32>;
    // Longest-match-first strategy
    // ## prefix for subwords
}
```

#### **Unigram (SentencePiece)**
```rust
pub struct Unigram {
    pieces: Vec<(String, f64)>, // token -> log probability
}

impl Unigram {
    pub fn tokenize_with_scores(&self, text: &str) -> Vec<(String, f64)>;
    pub fn train_em(corpus: &[&str], vocab_size: usize, iterations: usize) -> Self;
    // EM algorithm for training
    // Viterbi decoding for tokenization
}
```

---

### 2. Modelos Pré-configurados

#### **GPT-2/3/4**
```rust
pub struct GPT2Tokenizer {
    bpe: BPE,
    encoder: HashMap<String, u32>,
    decoder: HashMap<u32, String>,
    byte_encoder: HashMap<u8, char>,
    byte_decoder: HashMap<char, u8>,
}

impl GPT2Tokenizer {
    pub fn from_pretrained(model: &str) -> Result<Self>; // "gpt2", "gpt2-medium", etc.
    pub fn encode(&self, text: &str) -> Vec<u32>;
    pub fn decode(&self, ids: &[u32]) -> String;
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<u32>>;
}
```

**Vocabulários:**
- GPT-2: 50,257 tokens (r50k_base)
- GPT-3: 50,257 tokens
- GPT-4: 100,256 tokens (cl100k_base)

#### **BERT**
```rust
pub struct BertTokenizer {
    wordpiece: WordPiece,
    normalizer: Box<dyn Normalizer>,
    pre_tokenizer: WhitespaceSplit,
}

impl BertTokenizer {
    pub fn from_pretrained(model: &str) -> Result<Self>; // "bert-base-uncased", etc.
    pub fn encode_with_special(&self, text: &str) -> Vec<u32>; // [CLS] + tokens + [SEP]
    pub fn encode_pair(&self, text_a: &str, text_b: &str) -> Vec<u32>;
}
```

**Special tokens:**
- `[CLS]`: 101
- `[SEP]`: 102
- `[PAD]`: 0
- `[UNK]`: 100
- `[MASK]`: 103

#### **Llama 2/3**
```rust
pub struct LlamaTokenizer {
    sentencepiece: Unigram,
    vocab_size: usize,
}

impl LlamaTokenizer {
    pub fn from_pretrained(model: &str) -> Result<Self>; // "llama-2-7b", "llama-3-70b"
    pub fn encode(&self, text: &str, add_bos: bool, add_eos: bool) -> Vec<u32>;
    // BOS: <s> (1), EOS: </s> (2)
}
```

**Vocabulários:**
- Llama 2: 32,000 tokens
- Llama 3: 128,256 tokens (expandido!)

#### **Claude (Anthropic)**
```rust
pub struct ClaudeTokenizer {
    bpe: BPE,
    vocab_size: usize,
}

impl ClaudeTokenizer {
    pub fn from_pretrained(model: &str) -> Result<Self>; // "claude-2", "claude-3"
    // Similar to GPT-4 cl100k_base but with custom vocab
}
```

---

### 3. Normalização

```rust
pub trait Normalizer: Send + Sync {
    fn normalize(&self, text: &str) -> String;
}

// NFC: Canonical composition (é -> é)
pub struct NFC;

// NFKC: Compatibility composition (ﬁ -> fi)
pub struct NFKC;

// Lowercase
pub struct Lowercase;

// Strip accents: é -> e, ç -> c
pub struct StripAccents;

// Chain normalizers
pub struct Sequence {
    normalizers: Vec<Box<dyn Normalizer>>,
}
```

**Exemplo:**
```rust
let normalizer = Sequence::new(vec![
    Box::new(NFKC),
    Box::new(StripAccents),
    Box::new(Lowercase),
]);
let normalized = normalizer.normalize("Olá, José!");
// Output: "ola, jose!"
```

---

### 4. Pre-Tokenização

```rust
pub trait PreTokenizer: Send + Sync {
    fn pre_tokenize(&self, text: &str) -> Vec<String>;
}

// Whitespace splitting
pub struct WhitespaceSplit;

// Byte-level (GPT-2): maps bytes to Unicode
pub struct ByteLevel;

// Metaspace (SentencePiece): space -> _
pub struct Metaspace {
    replacement: char,
    add_prefix_space: bool,
}

// Punctuation splitting
pub struct Punctuation;
```

---

### 5. Post-Processamento

```rust
pub trait PostProcessor: Send + Sync {
    fn process(&self, tokens: Vec<u32>, pair: Option<Vec<u32>>) -> Vec<u32>;
}

// BERT: [CLS] + A + [SEP] + B + [SEP]
pub struct BertProcessing {
    cls: u32,
    sep: u32,
}

// RoBERTa: <s> + A + </s> + </s> + B + </s>
pub struct RobertaProcessing {
    bos: u32,
    eos: u32,
}

// Template: custom formatting
pub struct TemplateProcessing {
    template: String, // e.g., "[CLS] $A [SEP] $B [SEP]"
}
```

---

### 6. Decodificação

```rust
pub trait Decoder: Send + Sync {
    fn decode(&self, tokens: &[String]) -> String;
}

// GPT-2 byte-level decoder
pub struct ByteLevelDecoder;

// BERT WordPiece: remove ##
pub struct WordPieceDecoder;

// Metaspace: _ -> space
pub struct MetaspaceDecoder;

// Strip special tokens
pub struct StripDecoder {
    special_tokens: HashSet<String>,
}
```

---

### 7. Treinamento de Vocabulário

```rust
pub struct Trainer {
    vocab_size: usize,
    min_frequency: usize,
    special_tokens: Vec<String>,
}

impl Trainer {
    pub fn train_bpe(&self, corpus: &[&str]) -> BPE;
    pub fn train_wordpiece(&self, corpus: &[&str]) -> WordPiece;
    pub fn train_unigram(&self, corpus: &[&str]) -> Unigram;
}
```

**Exemplo:**
```rust
let corpus = vec![
    "Olá, como você está?",
    "Eu estou bem, obrigado!",
    // ... 1M+ frases
];

let trainer = Trainer::new()
    .vocab_size(50000)
    .min_frequency(2)
    .special_tokens(vec!["[PAD]", "[UNK]", "[CLS]", "[SEP]"]);

let bpe = trainer.train_bpe(&corpus);
bpe.save("vocab.json")?;
```

---

### 8. Otimização para Português Brasileiro

```rust
pub struct PortugueseTokenizer {
    bpe: BPE,
    normalizer: Sequence,
}

impl PortugueseTokenizer {
    pub fn new() -> Self {
        // Pre-trained no corpus brasileiro (CC-100, Oscar)
        // Vocab otimizado para acentos (á, é, í, ó, ú, ã, õ, ç)
        // Preserva contrações (d', l', pr', pra, né, tá)
    }
}
```

**Corpus sugerido:**
- BrWaC (2.68B tokens)
- CC-100 Portuguese (71GB)
- Oscar Portuguese (84GB)

---

## 🚀 API de Alto Nível

```rust
// Simple API
let tokenizer = Tokenizer::from_pretrained("gpt2")?;
let ids = tokenizer.encode("Hello, world!")?;
let text = tokenizer.decode(&ids)?;

// Builder API
let tokenizer = Tokenizer::builder()
    .algorithm(Algorithm::BPE)
    .normalizer(NFKC)
    .pre_tokenizer(ByteLevel::default())
    .decoder(ByteLevelDecoder)
    .build()?;

// Batch processing
let texts = vec!["Text 1", "Text 2", "Text 3"];
let encodings = tokenizer.encode_batch(&texts)?;

// With padding/truncation
let encodings = tokenizer.encode_batch(&texts)
    .padding(Padding::MaxLength(512))
    .truncation(Truncation::MaxLength(512))
    .execute()?;
```

---

## 📊 Performance Targets

### Benchmarks vs Hugging Face Tokenizers

**GPT-2 Tokenization:**
- **HF Tokenizers (Python)**: 100k tokens/sec
- **HF Tokenizers (Rust)**: 1M tokens/sec
- **avila-tokenizers (target)**: **3M tokens/sec** (3x faster)

**BERT Tokenization:**
- **HF**: 500k tokens/sec
- **avila-tokenizers**: **2M tokens/sec** (4x faster)

**Memory:**
- **HF**: ~500MB (loaded model)
- **avila-tokenizers**: **< 100MB** (optimized vocab storage)

---

## 🧪 Testes de Compatibilidade

### Deve passar 100% dos testes:

```rust
#[test]
fn test_gpt2_compatibility() {
    let tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    // Test against known encodings from OpenAI
    assert_eq!(
        tokenizer.encode("Hello, world!"),
        vec![15496, 11, 995, 0] // verified with tiktoken
    );
}

#[test]
fn test_bert_compatibility() {
    let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();

    assert_eq!(
        tokenizer.encode("Hello, world!"),
        vec![101, 7592, 1010, 2088, 999, 102] // [CLS] hello , world ! [SEP]
    );
}

#[test]
fn test_unicode_normalization() {
    let tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    // NFC vs NFD
    let nfc = tokenizer.encode("é"); // U+00E9
    let nfd = tokenizer.encode("é"); // U+0065 U+0301
    assert_eq!(nfc, nfd); // should normalize to same tokens
}

#[test]
fn test_portuguese_accents() {
    let tokenizer = PortugueseTokenizer::new();

    let text = "São Paulo, você está aí?";
    let ids = tokenizer.encode(text);
    let decoded = tokenizer.decode(&ids);
    assert_eq!(text, decoded); // preserve accents
}
```

---

## 🔬 Casos de Uso Avançados

### 1. Streaming Tokenization
```rust
let mut tokenizer_stream = tokenizer.stream();
for chunk in text_stream {
    let tokens = tokenizer_stream.feed(chunk)?;
    process(tokens);
}
let remaining = tokenizer_stream.flush()?;
```

### 2. Custom Vocabulary
```rust
let custom_vocab = vec![
    ("aviladb", 50000),
    ("arxis", 50001),
    ("quaternion", 50002),
];
let tokenizer = tokenizer.extend_vocab(custom_vocab)?;
```

### 3. Token Alignment
```rust
let encoding = tokenizer.encode_with_offsets("Hello, world!")?;
// encoding.tokens: ["Hello", ",", " world", "!"]
// encoding.offsets: [(0, 5), (5, 6), (6, 12), (12, 13)]
```

---

## 📦 Cargo.toml

```toml
[package]
name = "avila-tokenizers"
version = "0.1.0"
edition = "2021"
authors = ["Nícolas Ávila <nicolas@avila.inc>"]
license = "MIT OR Apache-2.0"
description = "The most complete tokenizer library in Rust - BPE, WordPiece, Unigram, with native support for GPT, BERT, Llama, Claude"
repository = "https://github.com/avilaops/arxis"
keywords = ["tokenizer", "nlp", "llm", "gpt", "bert"]
categories = ["text-processing", "algorithms"]

[dependencies]
# Zero heavy dependencies - 100% Rust
regex = "1.10"
unicode-normalization = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
lru = "0.12"  # LRU cache
rayon = "1.10"  # Parallel processing

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
tokio = { version = "1", features = ["rt-multi-thread"] }

[features]
default = []
python = []  # Future: PyO3 bindings
wasm = []    # Future: WASM support

[[bench]]
name = "tokenize_bench"
harness = false
```

---

## 🎯 Deliverables

Ao final, deve entregar:

1. ✅ **Código completo** (src/, examples/, tests/, benches/)
2. ✅ **README.md** com exemplos para cada modelo
3. ✅ **Benchmarks** comparando com HF Tokenizers
4. ✅ **Testes** de compatibilidade 100% (GPT-2, BERT, Llama)
5. ✅ **Vocabulários** pré-treinados (JSON files)
6. ✅ **Documentação** (docs.rs quality)
7. ✅ **Performance report** (speedup vs HF)

---

## 🏆 Critérios de Sucesso

- [ ] Tokeniza GPT-2 3x mais rápido que HF Tokenizers
- [ ] 100% compatível com OpenAI tiktoken (GPT-2/3/4)
- [ ] 100% compatível com BERT WordPiece
- [ ] Suporta Llama 2/3 SentencePiece
- [ ] Zero dependências Python
- [ ] < 100MB memory footprint
- [ ] Vocabulário português otimizado
- [ ] Testes passam em Windows, Linux, macOS

---

**GO! Construa o tokenizer mais completo do ecossistema Rust! 🚀**
