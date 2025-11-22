use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_tokenizers::models::{GPT2Tokenizer, BertTokenizer, LlamaTokenizer};

// Sample texts for benchmarking
const SHORT_TEXT: &str = "Hello world";
const MEDIUM_TEXT: &str = "The quick brown fox jumps over the lazy dog. This is a test sentence.";
const LONG_TEXT: &str = "Natural language processing (NLP) is a subfield of linguistics, computer science, \
                         and artificial intelligence concerned with the interactions between computers and \
                         human language, in particular how to program computers to process and analyze large \
                         amounts of natural language data. The result is a computer capable of understanding \
                         the contents of documents, including the contextual nuances of the language within them.";
const PORTUGUESE_TEXT: &str = "O Brasil é o maior país da América do Sul. A língua portuguesa é falada por \
                               mais de 200 milhões de pessoas. São Paulo é a maior cidade do país.";

// Benchmark GPT-2 encoding
fn bench_gpt2_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("gpt2_encode");
    let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();

    group.bench_function(BenchmarkId::new("short", SHORT_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(SHORT_TEXT)))
    });

    group.bench_function(BenchmarkId::new("medium", MEDIUM_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(MEDIUM_TEXT)))
    });

    group.bench_function(BenchmarkId::new("long", LONG_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(LONG_TEXT)))
    });

    group.bench_function(BenchmarkId::new("portuguese", PORTUGUESE_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(PORTUGUESE_TEXT)))
    });

    group.finish();
}

// Benchmark BERT encoding
fn bench_bert_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("bert_encode");
    let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();

    group.bench_function(BenchmarkId::new("short", SHORT_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(SHORT_TEXT)))
    });

    group.bench_function(BenchmarkId::new("medium", MEDIUM_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(MEDIUM_TEXT)))
    });

    group.bench_function(BenchmarkId::new("long", LONG_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(LONG_TEXT)))
    });

    group.bench_function(BenchmarkId::new("portuguese", PORTUGUESE_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(PORTUGUESE_TEXT)))
    });

    group.finish();
}

// Benchmark Llama encoding
fn bench_llama_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("llama_encode");
    let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();

    group.bench_function(BenchmarkId::new("short", SHORT_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(SHORT_TEXT)))
    });

    group.bench_function(BenchmarkId::new("medium", MEDIUM_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(MEDIUM_TEXT)))
    });

    group.bench_function(BenchmarkId::new("long", LONG_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(LONG_TEXT)))
    });

    group.bench_function(BenchmarkId::new("portuguese", PORTUGUESE_TEXT.len()), |b| {
        b.iter(|| tokenizer.encode(black_box(PORTUGUESE_TEXT)))
    });

    group.finish();
}

// Benchmark decoding
fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    // GPT-2 decode
    let mut gpt2 = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    let gpt2_ids = gpt2.encode(MEDIUM_TEXT);
    group.bench_function("gpt2", |b| {
        b.iter(|| gpt2.decode(black_box(&gpt2_ids)))
    });

    // BERT decode
    let bert = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
    let bert_ids = bert.encode(MEDIUM_TEXT);
    group.bench_function("bert", |b| {
        b.iter(|| bert.decode(black_box(&bert_ids)))
    });

    // Llama decode
    let llama = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
    let llama_ids = llama.encode(MEDIUM_TEXT);
    group.bench_function("llama", |b| {
        b.iter(|| llama.decode(black_box(&llama_ids)))
    });

    group.finish();
}

// Benchmark batch encoding
fn bench_batch_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_encode");

    let texts = vec![
        "First sentence.",
        "Second sentence with more words.",
        "Third sentence is longer and has more complexity.",
        "Fourth!",
        "Fifth sentence here.",
    ];

    // GPT-2 batch
    let mut gpt2 = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    group.bench_function("gpt2_batch_5", |b| {
        b.iter(|| gpt2.encode_batch(black_box(&texts)))
    });

    // BERT batch
    let bert = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
    group.bench_function("bert_batch_5", |b| {
        b.iter(|| bert.encode_batch(black_box(&texts)))
    });

    // Llama batch
    let llama = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
    group.bench_function("llama_batch_5", |b| {
        b.iter(|| llama.encode_batch(black_box(&texts)))
    });

    group.finish();
}

// Benchmark tokenizer initialization
fn bench_tokenizer_init(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenizer_init");

    group.bench_function("gpt2", |b| {
        b.iter(|| GPT2Tokenizer::from_pretrained(black_box("gpt2")))
    });

    group.bench_function("bert", |b| {
        b.iter(|| BertTokenizer::from_pretrained(black_box("bert-base-uncased")))
    });

    group.bench_function("llama", |b| {
        b.iter(|| LlamaTokenizer::from_pretrained(black_box("llama-2-7b")))
    });

    group.finish();
}

// Benchmark Portuguese-specific operations
fn bench_portuguese(c: &mut Criterion) {
    let mut group = c.benchmark_group("portuguese");

    let pt_texts = vec![
        "Olá, como você está?",
        "O Brasil é um país grande.",
        "São Paulo tem muitos habitantes.",
        "A língua portuguesa é bonita.",
    ];

    // GPT-2
    let mut gpt2 = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    group.bench_function("gpt2_pt", |b| {
        b.iter(|| {
            for text in &pt_texts {
                gpt2.encode(black_box(text));
            }
        })
    });

    // BERT
    let bert = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
    group.bench_function("bert_pt", |b| {
        b.iter(|| {
            for text in &pt_texts {
                bert.encode(black_box(text));
            }
        })
    });

    // Llama
    let llama = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
    group.bench_function("llama_pt", |b| {
        b.iter(|| {
            for text in &pt_texts {
                llama.encode(black_box(text));
            }
        })
    });

    group.finish();
}

// Throughput benchmark (tokens per second)
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(criterion::Throughput::Elements(LONG_TEXT.len() as u64));

    let mut gpt2 = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    group.bench_function("gpt2_throughput", |b| {
        b.iter(|| gpt2.encode(black_box(LONG_TEXT)))
    });

    let bert = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
    group.bench_function("bert_throughput", |b| {
        b.iter(|| bert.encode(black_box(LONG_TEXT)))
    });

    let llama = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
    group.bench_function("llama_throughput", |b| {
        b.iter(|| llama.encode(black_box(LONG_TEXT)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_gpt2_encode,
    bench_bert_encode,
    bench_llama_encode,
    bench_decode,
    bench_batch_encode,
    bench_tokenizer_init,
    bench_portuguese,
    bench_throughput,
);
criterion_main!(benches);
