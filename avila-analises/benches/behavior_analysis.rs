use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_analises::*;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

// Benchmark de inserção de eventos
fn bench_event_insertion(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_insertion");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async {
                    let mut tracker = tracker::BehaviorTracker::new(30);

                    for i in 0..size {
                        let event = create_test_event(&format!("user_{}", i % 100));
                        tracker.track_event(event).await.unwrap();
                    }
                });
        });
    }

    group.finish();
}

// Benchmark de análise de funil
fn bench_funnel_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("funnel_analysis");

    for size in [1000, 5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let analyzer = funnel::FunnelAnalyzer::new();
            let funnel = funnel::FunnelAnalyzer::create_ecommerce_funnel();

            let events: Vec<_> = (0..size)
                .map(|i| create_test_event(&format!("user_{}", i % 100)))
                .collect();

            b.iter(|| {
                analyzer.analyze_funnel(black_box(&funnel), black_box(&events))
            });
        });
    }

    group.finish();
}

// Benchmark de segmentação
fn bench_user_segmentation(c: &mut Criterion) {
    let mut group = c.benchmark_group("user_segmentation");

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let segmentation = segmentation::UserSegmentation::with_default_segments();

            let profiles: Vec<_> = (0..size)
                .map(|i| create_test_profile(&format!("user_{}", i)))
                .collect();

            b.iter(|| {
                for profile in &profiles {
                    segmentation.classify_user(black_box(profile));
                }
            });
        });
    }

    group.finish();
}

// Benchmark de predição de churn
fn bench_churn_prediction(c: &mut Criterion) {
    let mut group = c.benchmark_group("churn_prediction");

    let model = prediction::ChurnModel::new();
    let profiles: Vec<_> = (0..1000)
        .map(|i| create_test_profile(&format!("user_{}", i)))
        .collect();

    group.bench_function("predict_1000_users", |b| {
        b.iter(|| {
            for profile in &profiles {
                model.predict(black_box(profile));
            }
        });
    });

    group.finish();
}

// Benchmark de sistema de recomendação
fn bench_recommendations(c: &mut Criterion) {
    let mut group = c.benchmark_group("recommendations");

    let mut model = prediction::RecommendationModel::new();

    // Gerar dados de treino
    let events: Vec<_> = (0..10000)
        .map(|i| create_test_event(&format!("user_{}", i % 100)))
        .collect();

    model.train(&events);

    group.bench_function("recommend_10_items", |b| {
        b.iter(|| {
            model.recommend(black_box("user_1"), black_box(10))
        });
    });

    group.finish();
}

// Funções auxiliares
fn create_test_event(user_id: &str) -> models::BehaviorEvent {
    use models::*;

    BehaviorEvent {
        event_id: Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        session_id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        event_type: EventType::PageView {
            url: "/products".to_string(),
            title: "Products".to_string(),
            duration_ms: 5000,
        },
        metadata: HashMap::new(),
        context: EventContext {
            device: DeviceInfo {
                device_type: DeviceType::Desktop,
                os: "Windows".to_string(),
                browser: "Chrome".to_string(),
                screen_resolution: (1920, 1080),
            },
            location: LocationInfo {
                country: "BR".to_string(),
                city: Some("São Paulo".to_string()),
                timezone: "America/Sao_Paulo".to_string(),
                ip_address: "192.168.1.1".to_string(),
            },
            referrer: None,
            user_agent: "Mozilla/5.0".to_string(),
            viewport: Viewport {
                width: 1920,
                height: 1080,
            },
        },
    }
}

fn create_test_profile(user_id: &str) -> models::UserProfile {
    use models::*;

    UserProfile {
        user_id: user_id.to_string(),
        first_seen: Utc::now(),
        last_seen: Utc::now(),
        total_sessions: 10,
        total_events: 100,
        behaviors: UserBehaviors {
            avg_session_duration_seconds: 300.0,
            avg_pages_per_session: 5.0,
            bounce_rate: 0.3,
            conversion_rate: 0.1,
            most_active_hours: vec![],
            most_active_days: vec![],
            total_purchases: 5,
            total_spent: 1000.0,
            avg_order_value: 200.0,
            days_since_last_purchase: Some(30),
            pages_viewed: std::collections::HashSet::new(),
            search_queries: vec!["laptop".to_string()],
            clicked_products: vec!["prod1".to_string()],
        },
        segments: vec![],
        engagement_score: 0.7,
        loyalty_score: 0.6,
        conversion_probability: 0.5,
        churn_risk: 0.4,
        interests: vec![],
        preferred_categories: HashMap::new(),
        browsing_patterns: BrowsingPatterns::default(),
    }
}

criterion_group!(
    benches,
    bench_event_insertion,
    bench_funnel_analysis,
    bench_user_segmentation,
    bench_churn_prediction,
    bench_recommendations
);

criterion_main!(benches);
