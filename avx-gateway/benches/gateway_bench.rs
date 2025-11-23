//! Benchmarks for the gateway

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avx_gateway::{
    cache::ResponseCache,
    circuit_breaker::CircuitBreaker,
    load_balancer::{LoadBalancer, Strategy},
    routing::Router,
};

fn benchmark_routing(c: &mut Criterion) {
    use avx_gateway::routing::Route;

    let routes = vec![
        Route::new("/api/users/*", "http://localhost:8001"),
        Route::new("/api/products/*", "http://localhost:8002"),
        Route::new("/api/orders/*", "http://localhost:8003"),
    ];

    let router = Router::new(routes);

    c.bench_function("route_matching", |b| {
        b.iter(|| {
            let req = axum::extract::Request::builder()
                .uri("/api/users/123")
                .body(())
                .unwrap();
            black_box(router.match_route(&req))
        });
    });
}

fn benchmark_load_balancer(c: &mut Criterion) {
    let lb = LoadBalancer::new()
        .upstream("http://localhost:8001")
        .upstream("http://localhost:8002")
        .upstream("http://localhost:8003")
        .strategy(Strategy::RoundRobin)
        .build()
        .unwrap();

    c.bench_function("load_balancer_next", |b| {
        b.iter(|| {
            black_box(lb.next_upstream())
        });
    });
}

fn benchmark_cache_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = ResponseCache::new();

    c.bench_function("cache_put_get", |b| {
        b.to_async(&rt).iter(|| async {
            cache.put(
                "GET",
                "/api/test",
                None,
                200,
                vec![],
                b"test response".to_vec(),
                None,
            ).await;
            black_box(cache.get("GET", "/api/test", None).await)
        });
    });
}

criterion_group!(
    benches,
    benchmark_routing,
    benchmark_load_balancer,
    benchmark_cache_operations
);
criterion_main!(benches);
