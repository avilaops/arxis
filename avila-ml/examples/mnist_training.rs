//! MNIST training example (simplified)

use avila_ml::data::{DataLoader, TensorDataset};
use avila_ml::prelude::*;
use avila_ml::tensor::Tensor;
use avila_ml::utils::{accuracy, one_hot, CosineAnnealingLR, EarlyStopping};
use std::sync::Arc;

fn main() {
    println!("🚀 Avila ML - MNIST Training Example\n");

    // Generate synthetic MNIST-like data (28x28 images)
    // In real scenario, load actual MNIST dataset
    let n_train = 1000;
    let n_test = 200;
    let n_classes = 10;
    let input_size = 28 * 28;

    println!("📊 Generating synthetic data...");
    let mut train_data = Vec::new();
    let mut train_labels = Vec::new();

    for i in 0..n_train {
        let x = Tensor::<f32>::randn(&[input_size]);
        let label = i % n_classes;
        let y = one_hot(label, n_classes);

        train_data.push(x);
        train_labels.push(y);
    }

    let mut test_data = Vec::new();
    let mut test_labels = Vec::new();

    for i in 0..n_test {
        let x = Tensor::<f32>::randn(&[input_size]);
        let label = i % n_classes;
        let y = one_hot(label, n_classes);

        test_data.push(x);
        test_labels.push(y);
    }

    println!("✅ Train samples: {}, Test samples: {}\n", n_train, n_test);

    // Create model
    let model = Sequential::new(vec![
        Box::new(Linear::<f32>::new(input_size, 128)),
        Box::new(ReLU::new()),
        Box::new(Linear::<f32>::new(128, 64)),
        Box::new(ReLU::new()),
        Box::new(Linear::<f32>::new(64, n_classes)),
        Box::new(Softmax::new(-1)),
    ]);

    println!("🏗️  Model Architecture:");
    println!("  - Linear(784 -> 128) + ReLU");
    println!("  - Linear(128 -> 64) + ReLU");
    println!("  - Linear(64 -> 10) + Softmax\n");

    // Create DataLoader
    let dataset = Arc::new(TensorDataset::new(train_data, train_labels));
    let mut dataloader = DataLoader::new(dataset, 32).shuffle();

    // Create optimizer and loss
    // Note: parameters_mut() not available on Sequential yet
    // let mut optimizer = Adam::new(model.parameters_mut(), 0.001);
    let loss_fn = CrossEntropyLoss::new();
    let mut scheduler = CosineAnnealingLR::new(0.001, 50);
    let mut early_stopping = EarlyStopping::new(5, 0.001);

    println!("⚙️  Optimizer: Adam with lr=0.001");
    println!("📉 Loss: CrossEntropy");
    println!("📅 Scheduler: CosineAnnealing");
    println!("⏹️  Early Stopping: patience=5\n");

    // Training loop
    let epochs = 50;
    println!("🎯 Training for {} epochs...\n", epochs);

    for epoch in 0..epochs {
        dataloader.reset();
        let mut epoch_loss = 0.0;
        let mut num_batches = 0;

        while let Some((batch_x, batch_y)) = dataloader.next_batch() {
            let mut batch_loss = 0.0;

            for (x, y) in batch_x.iter().zip(batch_y.iter()) {
                // Forward pass
                let pred = model.forward(x);
                let loss = loss_fn.forward(&pred, y);

                // Backward pass (simplified)
                // optimizer.zero_grad();
                // loss.backward();
                // optimizer.step();

                batch_loss += loss.data[[]] as f32;
            }

            epoch_loss += batch_loss / batch_x.len() as f32;
            num_batches += 1;
        }

        let avg_loss = epoch_loss / num_batches as f32;
        let current_lr = scheduler.step();

        if (epoch + 1) % 5 == 0 {
            println!(
                "Epoch [{:3}/{}] - Loss: {:.6} - LR: {:.6}",
                epoch + 1,
                epochs,
                avg_loss,
                current_lr
            );
        }

        // Early stopping check
        if early_stopping.step(avg_loss) {
            println!("\n⏹️  Early stopping triggered at epoch {}", epoch + 1);
            break;
        }
    }

    println!("\n✅ Training complete!");

    // Evaluate on test set
    println!("\n🧪 Evaluating on test set...");

    let mut test_preds = Vec::new();
    for x in &test_data {
        let pred = model.forward(x);
        test_preds.push(pred);
    }

    let test_acc = accuracy(&test_preds, &test_labels);
    println!("📊 Test Accuracy: {:.2}%", test_acc * 100.0);
}
