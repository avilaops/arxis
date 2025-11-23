use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use avl_console::{
    ai_assistant::process_natural_language,
    embeddings::{VectorStore, generate_embedding},
    query_safety::validate_query,
    rate_limiter::RateLimiter,
};

/// Benchmark AI Assistant query processing
fn bench_ai_assistant(c: &mut Criterion) {
    let mut group = c.benchmark_group("ai_assistant");

    let queries = vec![
        "usuários mais ativos",
        "vendas por categoria",
        "pedidos pendentes acima de 1000",
        "otimização de query",
    ];

    for query in queries {
        group.bench_with_input(
            BenchmarkId::from_parameter(query),
            &query,
            |b, &query| {
                b.iter(|| {
                    let (response, sql, explanation, tips) =
                        process_natural_language(black_box(query));
                    black_box((response, sql, explanation, tips))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark vector similarity search
fn bench_vector_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_search");

    // Setup: Create vector store with sample data
    let mut store = VectorStore::new();
    for i in 0..1000 {
        let content = format!("Sample document {}", i);
        let embedding = generate_embedding(&content);
        store.add(
            format!("doc_{}", i),
            content,
            vec![("index".to_string(), i.to_string())],
        );
    }

    let query = "sample query for search";

    group.throughput(Throughput::Elements(1));
    group.bench_function("search_1000_docs", |b| {
        b.iter(|| {
            let results = store.search(black_box(query), black_box(5));
            black_box(results)
        });
    });

    group.finish();
}

/// Benchmark query safety validation
fn bench_query_safety(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_safety");

    let queries = vec![
        ("safe_select", "SELECT * FROM users WHERE id = 1"),
        ("safe_join", "SELECT u.*, o.* FROM users u JOIN orders o ON u.id = o.user_id"),
        ("injection_attempt", "SELECT * FROM users WHERE id = 1 OR 1=1--"),
        ("drop_attempt", "DROP TABLE users;"),
    ];

    for (name, query) in queries {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &query,
            |b, &query| {
                b.iter(|| {
                    let result = validate_query(black_box(query), "admin");
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark rate limiter performance
fn bench_rate_limiter(c: &mut Criterion) {
    let mut group = c.benchmark_group("rate_limiter");

    let limiter = RateLimiter::new(1000, 10000);

    group.throughput(Throughput::Elements(1));
    group.bench_function("check_request", |b| {
        b.iter(|| {
            let result = limiter.check_request(black_box("user_123"));
            black_box(result)
        });
    });

    group.throughput(Throughput::Elements(1));
    group.bench_function("check_tokens", |b| {
        b.iter(|| {
            let result = limiter.check_tokens(black_box("user_123"), black_box(100));
            black_box(result)
        });
    });

    group.finish();
}

/// Benchmark embedding generation
fn bench_embeddings(c: &mut Criterion) {
    let mut group = c.benchmark_group("embeddings");

    let texts = vec![
        "Short text",
        "Medium length text with some more words in it",
        "A much longer text that contains multiple sentences and provides more context for embedding generation. This should be representative of real-world usage.",
    ];

    for (i, text) in texts.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("text_{}", i)),
            text,
            |b, &text| {
                b.iter(|| {
                    let embedding = generate_embedding(black_box(text));
                    black_box(embedding)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_ai_assistant,
    bench_vector_search,
    bench_query_safety,
    bench_rate_limiter,
    bench_embeddings
);
criterion_main!(benches);
