use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_geo::{
    coords::GeoCoord,
    projection::{Equirectangular, Mercator, WebMercator, AlbersEqualArea, Projection},
};

fn benchmark_projections(c: &mut Criterion) {
    let coords = vec![
        GeoCoord::new(-23.55, -46.63),
        GeoCoord::new(40.71, -74.01),
        GeoCoord::new(51.51, -0.13),
        GeoCoord::new(35.68, 139.65),
    ];

    let width = 1920.0;
    let height = 1080.0;

    c.bench_function("equirectangular_projection", |b| {
        let proj = Equirectangular::new();
        b.iter(|| {
            for coord in &coords {
                black_box(proj.project(black_box(coord), width, height));
            }
        });
    });

    c.bench_function("mercator_projection", |b| {
        let proj = Mercator::new();
        b.iter(|| {
            for coord in &coords {
                black_box(proj.project(black_box(coord), width, height));
            }
        });
    });

    c.bench_function("web_mercator_projection", |b| {
        let proj = WebMercator::new();
        b.iter(|| {
            for coord in &coords {
                black_box(proj.project(black_box(coord), width, height));
            }
        });
    });

    c.bench_function("albers_projection", |b| {
        let proj = AlbersEqualArea::brazil();
        b.iter(|| {
            for coord in &coords {
                black_box(proj.project(black_box(coord), width, height));
            }
        });
    });
}

criterion_group!(benches, benchmark_projections);
criterion_main!(benches);
