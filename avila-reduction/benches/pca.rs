use avila_reduction::linear::pca::PCA;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::Array2;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn pca_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pca");

    // Small dataset: 1000 samples, 50 features
    let data_small = Array2::random((1000, 50), Uniform::new(0.0, 1.0));
    group.bench_function("pca_1000x50", |b| {
        b.iter(|| {
            let pca = PCA::new(black_box(10));
            pca.fit_transform(&data_small).unwrap()
        })
    });

    // Medium dataset: 5000 samples, 100 features
    let data_medium = Array2::random((5000, 100), Uniform::new(0.0, 1.0));
    group.bench_function("pca_5000x100", |b| {
        b.iter(|| {
            let pca = PCA::new(black_box(20));
            pca.fit_transform(&data_medium).unwrap()
        })
    });

    group.finish();
}

criterion_group!(benches, pca_benchmark);
criterion_main!(benches);
