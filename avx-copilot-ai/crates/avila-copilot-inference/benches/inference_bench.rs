use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_copilot_inference::{InferenceEngine, InferenceStats};
use avila_copilot_model_storage::ModelStorage;
use avila_copilot_tokenizer::CopilotTokenizer;
use std::sync::Arc;

async fn setup_engine() -> InferenceEngine {
    let model_storage = Arc::new(ModelStorage::new("./models", 1024).await.unwrap());
    let tokenizer = Arc::new(CopilotTokenizer::new().unwrap());
    InferenceEngine::new(model_storage, tokenizer).await.unwrap()
}

fn bench_inference_latency(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let engine = rt.block_on(setup_engine());

    let mut group = c.benchmark_group("inference_latency");
    group.sample_size(100);

    // Test different input sizes
    for size in [10, 50, 100, 500].iter() {
        let input: Vec<u32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.to_async(&rt).iter(|| async {
                let result = engine.infer(black_box(input)).await;
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_tokenization(c: &mut Criterion) {
    let tokenizer = CopilotTokenizer::new().unwrap();

    let mut group = c.benchmark_group("tokenization");

    let code_samples = vec![
        "fn main() { println!(\"Hello\"); }",
        "pub struct Example { field: i32 }",
        "impl Example { pub fn new() -> Self { Self { field: 0 } } }",
    ];

    for (idx, code) in code_samples.iter().enumerate() {
        group.bench_with_input(BenchmarkId::from_parameter(idx), code, |b, code| {
            b.iter(|| {
                let tokens = tokenizer.encode(black_box(code));
                black_box(tokens)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_inference_latency, bench_tokenization);
criterion_main!(benches);
