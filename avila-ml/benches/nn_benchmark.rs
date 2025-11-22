use avila_ml::nn::activation::ReLU;
use avila_ml::nn::{Linear, Module};
use avila_ml::tensor::Tensor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_linear_layer(c: &mut Criterion) {
    let layer = Linear::<f32>::new(784, 128);
    let input = Tensor::randn(&[32, 784]); // batch_size=32

    c.bench_function("linear_forward_32x784", |b| {
        b.iter(|| {
            let _output = layer.forward(black_box(&input));
        })
    });
}

fn bench_activation(c: &mut Criterion) {
    let relu = ReLU::new();
    let input = Tensor::<f32>::randn(&[32, 128]);

    c.bench_function("relu_forward_32x128", |b| {
        b.iter(|| {
            let _output = relu.forward(black_box(&input));
        })
    });
}

fn bench_multilayer_network(c: &mut Criterion) {
    let layer1 = Linear::<f32>::new(784, 256);
    let relu1 = ReLU::new();
    let layer2 = Linear::<f32>::new(256, 128);
    let relu2 = ReLU::new();
    let layer3 = Linear::<f32>::new(128, 10);

    let input = Tensor::randn(vec![32, 784]);

    c.bench_function("mlp_3layers_forward", |b| {
        b.iter(|| {
            let x = layer1.forward(black_box(&input));
            let x = relu1.forward(&x);
            let x = layer2.forward(&x);
            let x = relu2.forward(&x);
            let _output = layer3.forward(&x);
        })
    });
}

criterion_group!(
    benches,
    bench_linear_layer,
    bench_activation,
    bench_multilayer_network
);
criterion_main!(benches);
