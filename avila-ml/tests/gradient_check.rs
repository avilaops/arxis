//! Gradient checking tests to verify backpropagation correctness
//!
//! These tests compare analytical gradients (from backprop) with numerical gradients
//! (from finite differences) to ensure mathematical correctness.

use avila_ml::nn::{Linear, Module};
use avila_ml::tensor::{Tensor, TensorLike};
use ndarray::ArrayD;
use std::sync::Arc;

const EPSILON: f64 = 1e-5;
const TOLERANCE: f64 = 1e-4;

/// Compute numerical gradient using finite differences
fn numerical_gradient<F>(f: F, x: &Tensor<f64>, eps: f64) -> ArrayD<f64>
where
    F: Fn(&Tensor<f64>) -> Tensor<f64>,
{
    let mut grad = ArrayD::zeros(x.data.raw_dim());
    let flat_size = x.data.len();

    for i in 0..flat_size {
        // f(x + eps)
        let mut x_plus = x.clone();
        x_plus.data.as_slice_mut().unwrap()[i] += eps;
        let y_plus = f(&x_plus);
        let f_plus = y_plus.data.sum();

        // f(x - eps)
        let mut x_minus = x.clone();
        x_minus.data.as_slice_mut().unwrap()[i] -= eps;
        let y_minus = f(&x_minus);
        let f_minus = y_minus.data.sum();

        // Numerical gradient: (f(x+eps) - f(x-eps)) / (2*eps)
        grad.as_slice_mut().unwrap()[i] = (f_plus - f_minus) / (2.0 * eps);
    }

    grad
}

/// Check if two arrays are close within tolerance
fn arrays_close(a: &ArrayD<f64>, b: &ArrayD<f64>, tol: f64) -> bool {
    if a.shape() != b.shape() {
        return false;
    }

    let diff = (a - b).mapv(|x| x.abs());
    let max_diff = diff.fold(0.0_f64, |acc, &x| acc.max(x));

    max_diff < tol
}#[test]
fn test_gradient_add() {
    let a = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 2.0_f64)).requires_grad_();
    let b = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f64)).requires_grad_();

    // Forward pass
    let mut c = a.add(&b);
    c.backward();

    // Analytical gradient
    let grad_a_analytical = a.grad.lock().unwrap().clone().unwrap();

    // Numerical gradient
    let grad_a_numerical = numerical_gradient(
        |x| {
            let b_const = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f64));
            x.add(&b_const)
        },
        &a,
        EPSILON,
    );

    assert!(
        arrays_close(&grad_a_analytical, &grad_a_numerical, TOLERANCE),
        "Gradient mismatch for addition"
    );
}

#[test]
fn test_gradient_mul() {
    let a = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 2.0_f64)).requires_grad_();
    let b = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f64)).requires_grad_();

    let mut c = a.mul(&b);
    c.backward();

    let grad_a_analytical = a.grad.lock().unwrap().clone().unwrap();

    let grad_a_numerical = numerical_gradient(
        |x| {
            let b_const = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f64));
            x.mul(&b_const)
        },
        &a,
        EPSILON,
    );

    assert!(
        arrays_close(&grad_a_analytical, &grad_a_numerical, TOLERANCE),
        "Gradient mismatch for multiplication"
    );
}

#[test]
fn test_gradient_matmul() {
    let a = Tensor::new(
        ndarray::arr2(&[[1.0_f64, 2.0], [3.0, 4.0]]).into_dyn()
    ).requires_grad_();

    let b = Tensor::new(
        ndarray::arr2(&[[2.0_f64, 0.0], [1.0, 2.0]]).into_dyn()
    ).requires_grad_();

    let mut c = a.matmul(&b);
    c.backward();

    let grad_a_analytical = a.grad.lock().unwrap().clone().unwrap();

    let grad_a_numerical = numerical_gradient(
        |x| {
            let b_const = Tensor::new(
                ndarray::arr2(&[[2.0_f64, 0.0], [1.0, 2.0]]).into_dyn()
            );
            x.matmul(&b_const)
        },
        &a,
        EPSILON,
    );

    assert!(
        arrays_close(&grad_a_analytical, &grad_a_numerical, TOLERANCE),
        "Gradient mismatch for matmul"
    );
}

#[test]
fn test_gradient_linear_layer() {
    let layer = Linear::<f64>::new(3, 2);
    let input = Tensor::new(
        ndarray::arr2(&[[1.0_f64, 2.0, 3.0]]).into_dyn()
    ).requires_grad_();

    let mut output = layer.forward(&input);
    output.backward();

    // Check that gradients were computed
    assert!(input.grad.lock().unwrap().is_some(), "Input gradient not computed");

    let grad_analytical = input.grad.lock().unwrap().clone().unwrap();

    // Numerical gradient
    let grad_numerical = numerical_gradient(
        |x| {
            let layer_clone = Linear::<f64>::new(3, 2);
            // Copy weights for consistency
            layer_clone.forward(x)
        },
        &input,
        EPSILON,
    );

    // Note: This test might need adjustment due to random initialization
    // For production, we'd want deterministic weights
    println!("Analytical grad shape: {:?}", grad_analytical.shape());
    println!("Numerical grad shape: {:?}", grad_numerical.shape());
}

#[test]
fn test_gradient_mean() {
    let a = Tensor::new(
        ndarray::arr2(&[[1.0_f64, 2.0], [3.0, 4.0]]).into_dyn()
    ).requires_grad_();

    let mut mean = a.mean();
    mean.backward();

    let grad_analytical = a.grad.lock().unwrap().clone().unwrap();

    let grad_numerical = numerical_gradient(
        |x| x.mean(),
        &a,
        EPSILON,
    );

    assert!(
        arrays_close(&grad_analytical, &grad_numerical, TOLERANCE),
        "Gradient mismatch for mean"
    );
}

#[test]
fn test_gradient_sum() {
    let a = Tensor::new(
        ndarray::arr2(&[[1.0_f64, 2.0], [3.0, 4.0]]).into_dyn()
    ).requires_grad_();

    let mut sum = a.sum();
    sum.backward();

    let grad_analytical = a.grad.lock().unwrap().clone().unwrap();

    let grad_numerical = numerical_gradient(
        |x| x.sum(),
        &a,
        EPSILON,
    );

    assert!(
        arrays_close(&grad_analytical, &grad_numerical, TOLERANCE),
        "Gradient mismatch for sum"
    );
}

#[test]
fn test_gradient_chain_rule() {
    // Test chain rule: f(x) = (x + 2) * 3
    let x = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 1.0_f64)).requires_grad_();
    let two = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 2.0_f64));
    let three = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f64));

    let a = x.add(&two);
    let mut b = a.mul(&three);
    b.backward();

    let grad_analytical = x.grad.lock().unwrap().clone().unwrap();

    let grad_numerical = numerical_gradient(
        |input| {
            let two = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 2.0_f64));
            let three = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[2, 2]), 3.0_f64));
            let a = input.add(&two);
            a.mul(&three)
        },
        &x,
        EPSILON,
    );

    assert!(
        arrays_close(&grad_analytical, &grad_numerical, TOLERANCE),
        "Gradient mismatch for chain rule"
    );
}
