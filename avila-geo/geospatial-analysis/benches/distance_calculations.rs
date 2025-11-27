use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geospatial_analysis::distance::{haversine_distance, vincenty_distance};
use geo::Coord;

fn bench_haversine(c: &mut Criterion) {
    let lisbon = Coord { x: -9.1393, y: 38.7223 };
    let porto = Coord { x: -8.6291, y: 41.1579 };

    c.bench_function("haversine_lisbon_porto", |b| {
        b.iter(|| haversine_distance(black_box(&lisbon), black_box(&porto)))
    });
}

fn bench_vincenty(c: &mut Criterion) {
    let lisbon = Coord { x: -9.1393, y: 38.7223 };
    let porto = Coord { x: -8.6291, y: 41.1579 };

    c.bench_function("vincenty_lisbon_porto", |b| {
        b.iter(|| vincenty_distance(black_box(&lisbon), black_box(&porto)))
    });
}

criterion_group!(benches, bench_haversine, bench_vincenty);
criterion_main!(benches);
