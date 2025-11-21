//! Simple linear regression example

use avila_ml::prelude::*;
use avila_ml::tensor::Tensor;

fn main() {
    println!("🚀 Avila ML - Linear Regression Example\n");

    // Generate synthetic data: y = 2x + 1 + noise
    let n_samples = 100;
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();

    for i in 0..n_samples {
        let x = i as f32 / n_samples as f32;
        let y = 2.0 * x + 1.0 + (rand::random::<f32>() - 0.5) * 0.1;

        // Create 2D tensors [1, 1] for batch processing
        x_data.push(Tensor::new(ndarray::arr2(&[[x]]).into_dyn()));
        y_data.push(Tensor::new(ndarray::arr2(&[[y]]).into_dyn()));
    }

    println!("📊 Generated {} samples", n_samples);

    // Create model: y = wx + b
    let mut model = Linear::<f32>::new(1, 1);
    println!("🏗️  Created linear model");

    // Create optimizer and loss function
    let mut optimizer = SGD::new(model.parameters_mut(), 0.1);
    let loss_fn = MSELoss::new();

    println!("⚙️  Optimizer: SGD with lr=0.1");
    println!("📉 Loss: MSE\n");

    // Training loop
    let epochs = 100;
    println!("🎯 Training for {} epochs...\n", epochs);
    println!("ℹ️  Note: Backpropagation not yet implemented, so weights won't update\n");

    for epoch in 0..epochs {
        let mut total_loss = 0.0;

        for (x, y) in x_data.iter().zip(y_data.iter()) {
            // Forward pass
            let pred = model.forward(x);
            let loss = loss_fn.forward(&pred, y);

            // Backward pass
            optimizer.zero_grad();
            // loss.backward(); // Note: backward not fully implemented yet

            // Update weights
            optimizer.step();

            // Loss is a scalar (0-dimensional tensor)
            total_loss += *loss.data.first().unwrap();
        }

        let avg_loss = total_loss / n_samples as f32;

        if (epoch + 1) % 10 == 0 {
            println!("Epoch [{:3}/{}] - Loss: {:.6}", epoch + 1, epochs, avg_loss);
        }
    }

    // Test the model
    println!("\n✅ Training complete!");
    println!("\n🧪 Testing predictions:");

    let test_inputs = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    for x in test_inputs {
        let input = Tensor::new(ndarray::arr2(&[[x]]).into_dyn());
        let pred = model.forward(&input);
        let expected = 2.0 * x + 1.0;

        println!(
            "  x={:.2} -> pred={:.4}, expected={:.4}",
            x,
            pred.data[[0, 0]],
            expected
        );
    }
}
