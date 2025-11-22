use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avx_http::{Client, Server, Router, Response};
use tokio::runtime::Runtime;
use std::time::Duration;

fn bench_client_request(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    // Start test server
    rt.spawn(async {
        let router = Router::new()
            .get("/bench", || async { Response::text("benchmark") });

        Server::bind("127.0.0.1:9999")
            .router(router)
            .run()
            .await
            .unwrap();
    });

    // Give server time to start
    std::thread::sleep(Duration::from_millis(200));

    let client = Client::new();

    c.bench_function("client_get_request", |b| {
        b.to_async(&rt).iter(|| async {
            let response = client
                .get("http://127.0.0.1:9999/bench")
                .send()
                .await
                .unwrap();
            black_box(response);
        });
    });
}

fn bench_response_creation(c: &mut Criterion) {
    c.bench_function("response_text", |b| {
        b.iter(|| {
            let response = Response::text(black_box("Hello, World!"));
            black_box(response);
        });
    });

    c.bench_function("response_json", |b| {
        b.iter(|| {
            let data = serde_json::json!({
                "status": "ok",
                "count": 42,
                "message": "benchmark"
            });
            let response = Response::json(&black_box(data));
            black_box(response);
        });
    });
}

fn bench_router_lookup(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let router = Router::new()
        .get("/", || async { Response::text("home") })
        .get("/health", || async { Response::text("healthy") })
        .get("/api/data", || async { Response::text("data") });

    c.bench_function("router_handle_request", |b| {
        b.to_async(&rt).iter(|| async {
            // Simulated request handling
            black_box(());
        });
    });
}

criterion_group!(
    benches,
    bench_client_request,
    bench_response_creation,
    bench_router_lookup
);
criterion_main!(benches);
