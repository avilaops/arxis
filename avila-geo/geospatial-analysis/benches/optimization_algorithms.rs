use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geospatial_analysis::optimization::{weber_location, DemandPoint};
use geo::Coord;

fn create_demand_points() -> Vec<DemandPoint> {
    vec![
        DemandPoint::new(Coord { x: -9.1393, y: 38.7223 }, 1000.0), // Lisbon
        DemandPoint::new(Coord { x: -8.6291, y: 41.1579 }, 800.0),  // Porto
        DemandPoint::new(Coord { x: -7.9304, y: 37.0194 }, 300.0),  // Faro
    ]
}

fn bench_weber(c: &mut Criterion) {
    let demand = create_demand_points();

    c.bench_function("weber_location_portugal", |b| {
        b.iter(|| weber_location(black_box(&demand), 100))
    });
}

criterion_group!(benches, bench_weber);
criterion_main!(benches);
