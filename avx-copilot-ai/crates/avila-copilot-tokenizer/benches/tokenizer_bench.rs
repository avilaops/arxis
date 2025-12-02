use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_copilot_tokenizer::CopilotTokenizer;

fn bench_encode(c: &mut Criterion) {
    let tokenizer = CopilotTokenizer::new().unwrap();

    c.bench_function("tokenizer_encode", |b| {
        b.iter(|| {
            let code = "fn example() { let x = 42; }";
            tokenizer.encode(black_box(code))
        });
    });
}

fn bench_decode(c: &mut Criterion) {
    let tokenizer = CopilotTokenizer::new().unwrap();
    let tokens = vec![1, 2, 3, 4, 5];

    c.bench_function("tokenizer_decode", |b| {
        b.iter(|| {
            tokenizer.decode(black_box(&tokens))
        });
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
