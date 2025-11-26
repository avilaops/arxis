use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_geo::{
    coords::{CartesianCoord, GeoCoord},
    geometry::shapes,
    render::{Framebuffer, Color, draw_line_bresenham, fill_polygon},
    projection::{Mercator, Projection},
};

fn benchmark_rendering(c: &mut Criterion) {
    c.bench_function("draw_line_1000px", |b| {
        let mut fb = Framebuffer::new(1000, 1000, Color::WHITE);
        b.iter(|| {
            draw_line_bresenham(
                black_box(&mut fb),
                black_box(0),
                black_box(0),
                black_box(999),
                black_box(999),
                black_box(Color::BLACK),
            );
        });
    });

    c.bench_function("fill_polygon_complex", |b| {
        let mut fb = Framebuffer::new(1000, 1000, Color::WHITE);
        let polygon = vec![
            CartesianCoord::new(100.0, 100.0),
            CartesianCoord::new(900.0, 150.0),
            CartesianCoord::new(850.0, 800.0),
            CartesianCoord::new(200.0, 750.0),
            CartesianCoord::new(150.0, 400.0),
        ];

        b.iter(|| {
            fill_polygon(black_box(&mut fb), black_box(&polygon), black_box(Color::RED));
        });
    });

    c.bench_function("project_and_render_100_points", |b| {
        let mut fb = Framebuffer::new(800, 600, Color::WHITE);
        let projection = Mercator::new();
        let points: Vec<_> = (0..100)
            .map(|i| GeoCoord::new(i as f64 - 50.0, i as f64 * 3.6 - 180.0))
            .collect();

        b.iter(|| {
            for point in &points {
                let cart = projection.project(black_box(point), 800.0, 600.0);
                if let Some((x, y)) = cart.to_u32() {
                    fb.set_pixel(x, y, Color::RED);
                }
            }
        });
    });
}

criterion_group!(benches, benchmark_rendering);
criterion_main!(benches);
