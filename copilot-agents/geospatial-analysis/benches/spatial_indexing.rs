use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geospatial_analysis::indexing::{SpatialFeature, SpatialIndex};
use geo::Coord;

fn create_portugal_index() -> SpatialIndex {
    let features = vec![
        SpatialFeature::new("lisbon".to_string(), Coord { x: -9.1393, y: 38.7223 }),
        SpatialFeature::new("porto".to_string(), Coord { x: -8.6291, y: 41.1579 }),
        SpatialFeature::new("faro".to_string(), Coord { x: -7.9304, y: 37.0194 }),
        SpatialFeature::new("coimbra".to_string(), Coord { x: -8.4103, y: 40.2033 }),
        SpatialFeature::new("braga".to_string(), Coord { x: -8.4261, y: 41.5518 }),
    ];

    SpatialIndex::from_features(features)
}

fn bench_nearest_neighbor(c: &mut Criterion) {
    let index = create_portugal_index();
    let query = Coord { x: -9.0, y: 39.0 };

    c.bench_function("nearest_neighbor", |b| {
        b.iter(|| index.nearest_neighbor(black_box(&query)))
    });
}

fn bench_k_nearest(c: &mut Criterion) {
    let index = create_portugal_index();
    let query = Coord { x: -9.0, y: 39.0 };

    c.bench_function("k_nearest_3", |b| {
        b.iter(|| index.k_nearest_neighbors(black_box(&query), 3))
    });
}

criterion_group!(benches, bench_nearest_neighbor, bench_k_nearest);
criterion_main!(benches);
