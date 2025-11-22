use avila_ml::tensor::{Tensor, TensorLike};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_tensor_creation(c: &mut Criterion) {
    c.bench_function("tensor_zeros_100x100", |b| {
        b.iter(|| {
            let _t = Tensor::<f32>::zeros(black_box(vec![100, 100]));
        })
    });

    c.bench_function("tensor_randn_100x100", |b| {
        b.iter(|| {
            let _t = Tensor::<f32>::randn(black_box(vec![100, 100]));
        })
    });
}

fn bench_tensor_operations(c: &mut Criterion) {
    let a = Tensor::<f32>::randn(vec![100, 100]);
    let b = Tensor::<f32>::randn(vec![100, 100]);

    c.bench_function("tensor_add_100x100", |b| {
        b.iter(|| {
            let _result = black_box(&a).add(black_box(&b));
        })
    });

    c.bench_function("tensor_mul_100x100", |b| {
        b.iter(|| {
            let _result = black_box(&a).mul(black_box(&b));
        })
    });

    c.bench_function("tensor_matmul_100x100", |b| {
        b.iter(|| {
            let _result = black_box(&a).matmul(black_box(&b));
        })
    });
}

fn bench_autograd(c: &mut Criterion) {
    c.bench_function("backward_add", |b| {
        b.iter(|| {
            let a = Tensor::<f32>::randn(vec![10, 10]).requires_grad_();
            let b = Tensor::<f32>::randn(vec![10, 10]).requires_grad_();
            let mut c = a.add(&b);
            c.backward();
        })
    });

    c.bench_function("backward_matmul", |b| {
        b.iter(|| {
            let a = Tensor::<f32>::randn(vec![10, 10]).requires_grad_();
            let b = Tensor::<f32>::randn(vec![10, 10]).requires_grad_();
            let mut c = a.matmul(&b);
            c.backward();
        })
    });
}

criterion_group!(
    benches,
    bench_tensor_creation,
    bench_tensor_operations,
    bench_autograd
);
criterion_main!(benches);
