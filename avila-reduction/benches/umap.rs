use avila_reduction::manifold::umap::UMAP;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::Array2;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn umap_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("umap");
    group.sample_size(10); // UMAP can be slow, reduce sample size

    // Small dataset: 1000 samples, 50 features
    let data_small = Array2::random((1000, 50), Uniform::new(0.0, 1.0));
    group.bench_function("umap_1000x50", |b| {
        b.iter(|| {
            let umap = UMAP::builder()
                .n_neighbors(black_box(15))
                .n_components(2)
                .min_dist(0.1)
                .build();
            umap.fit_transform(&data_small).unwrap()
        })
    });

    // Medium dataset: 2000 samples, 100 features
    let data_medium = Array2::random((2000, 100), Uniform::new(0.0, 1.0));
    group.bench_function("umap_2000x100", |b| {
        b.iter(|| {
            let umap = UMAP::builder()
                .n_neighbors(black_box(15))
                .n_components(2)
                .min_dist(0.1)
                .build();
            umap.fit_transform(&data_medium).unwrap()
        })
    });

    group.finish();
}

criterion_group!(benches, umap_benchmark);
criterion_main!(benches);
