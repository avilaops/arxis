use avila_reduction::manifold::tsne::TSNE;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::Array2;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn tsne_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tsne");
    group.sample_size(10); // t-SNE is slow, reduce sample size

    // Small dataset: 500 samples, 50 features
    let data_small = Array2::random((500, 50), Uniform::new(0.0, 1.0));
    group.bench_function("tsne_500x50", |b| {
        b.iter(|| {
            let tsne = TSNE::builder()
                .perplexity(black_box(30.0))
                .n_components(2)
                .max_iter(100) // Reduced for benchmarking
                .build();
            tsne.fit_transform(&data_small).unwrap()
        })
    });

    group.finish();
}

criterion_group!(benches, tsne_benchmark);
criterion_main!(benches);
