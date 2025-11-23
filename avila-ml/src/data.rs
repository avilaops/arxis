//! Data loading and preprocessing utilities

use crate::tensor::Tensor;
use num_traits::{Float, NumAssign};
use std::sync::Arc;

/// Trait for datasets
pub trait Dataset<T = f32>: Send + Sync
where
    T: Float + NumAssign + Send + Sync,
{
    /// Get a single sample from the dataset
    fn get(&self, index: usize) -> (Tensor<T>, Tensor<T>);

    /// Get the total number of samples
    fn len(&self) -> usize;

    /// Check if the dataset is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Simple in-memory dataset
pub struct TensorDataset<T = f32> {
    data: Vec<Tensor<T>>,
    targets: Vec<Tensor<T>>,
}

impl<T: Float + NumAssign> TensorDataset<T> {
    pub fn new(data: Vec<Tensor<T>>, targets: Vec<Tensor<T>>) -> Self {
        assert_eq!(
            data.len(),
            targets.len(),
            "Data and targets must have same length"
        );
        Self { data, targets }
    }
}

impl<T: Float + NumAssign + Send + Sync> Dataset<T> for TensorDataset<T> {
    fn get(&self, index: usize) -> (Tensor<T>, Tensor<T>) {
        (self.data[index].clone(), self.targets[index].clone())
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

/// DataLoader for batching and iterating over datasets
pub struct DataLoader<T = f32> {
    dataset: Arc<dyn Dataset<T>>,
    batch_size: usize,
    shuffle: bool,
    drop_last: bool,
    indices: Vec<usize>,
    current_idx: usize,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> DataLoader<T> {
    /// Create a new DataLoader
    pub fn new(dataset: Arc<dyn Dataset<T>>, batch_size: usize) -> Self {
        let len = dataset.len();
        let indices: Vec<usize> = (0..len).collect();

        Self {
            dataset,
            batch_size,
            shuffle: false,
            drop_last: false,
            indices,
            current_idx: 0,
        }
    }

    /// Enable shuffling
    pub fn shuffle(mut self) -> Self {
        self.shuffle = true;
        self
    }

    /// Drop last incomplete batch
    pub fn drop_last(mut self) -> Self {
        self.drop_last = true;
        self
    }

    /// Reset the iterator
    pub fn reset(&mut self) {
        self.current_idx = 0;

        if self.shuffle {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            self.indices.shuffle(&mut rng);
        }
    }

    /// Get the next batch
    pub fn next_batch(&mut self) -> Option<(Vec<Tensor<T>>, Vec<Tensor<T>>)> {
        if self.current_idx >= self.dataset.len() {
            return None;
        }

        let end_idx = (self.current_idx + self.batch_size).min(self.dataset.len());

        // Drop last incomplete batch if requested
        if self.drop_last && end_idx - self.current_idx < self.batch_size {
            return None;
        }

        let batch_indices = &self.indices[self.current_idx..end_idx];

        let mut data_batch = Vec::new();
        let mut target_batch = Vec::new();

        for &idx in batch_indices {
            let (data, target) = self.dataset.get(idx);
            data_batch.push(data);
            target_batch.push(target);
        }

        self.current_idx = end_idx;

        Some((data_batch, target_batch))
    }

    /// Get the number of batches
    pub fn num_batches(&self) -> usize {
        let total = self.dataset.len();
        if self.drop_last {
            total / self.batch_size
        } else {
            total.div_ceil(self.batch_size)
        }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Iterator
    for DataLoader<T>
{
    type Item = (Vec<Tensor<T>>, Vec<Tensor<T>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_batch()
    }
}

/// CSV Dataset loader (requires external dependencies - DISABLED)
/*
#[cfg(feature = "data-loading")]
pub struct CSVDataset<T = f32> {
    data: Vec<Tensor<T>>,
    targets: Vec<Tensor<T>>,
}

#[cfg(feature = "data-loading")]
impl<T: Float + NumAssign> CSVDataset<T> {
    /// Load a dataset from CSV file
    pub fn from_csv(path: &str, target_column: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use polars::prelude::*;

        let df = CsvReader::from_path(path)?.has_header(true).finish()?;

        // Extract features (all columns except target)
        let feature_columns: Vec<String> = df
            .get_column_names()
            .iter()
            .filter(|&&name| name != target_column)
            .map(|&s| s.to_string())
            .collect();

        let features = df.select(&feature_columns)?;
        let targets = df.column(target_column)?;

        // Convert to tensors
        // TODO: Implement proper conversion from Polars to Tensor
        let data = vec![];
        let targets_vec = vec![];

        Ok(Self {
            data,
            targets: targets_vec,
        })
    }
}

#[cfg(feature = "data-loading")]
impl<T: Float + NumAssign> Dataset<T> for CSVDataset<T> {
    fn get(&self, index: usize) -> (Tensor<T>, Tensor<T>) {
        (self.data[index].clone(), self.targets[index].clone())
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
*/

/// Utility to stack tensors into a batch
pub fn stack_tensors<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static>(
    tensors: &[Tensor<T>],
) -> Tensor<T> {
    if tensors.is_empty() {
        return Tensor::new(ndarray::ArrayD::zeros(ndarray::IxDyn(&[0])));
    }

    let first_shape = tensors[0].shape();
    let batch_size = tensors.len();

    // Create new shape with batch dimension
    let mut new_shape = vec![batch_size];
    new_shape.extend_from_slice(first_shape);

    // Stack tensors
    // TODO: Implement proper stacking
    tensors[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_dataset() {
        let data = vec![
            Tensor::<f32>::randn(&[10]),
            Tensor::<f32>::randn(&[10]),
            Tensor::<f32>::randn(&[10]),
        ];
        let targets = vec![
            Tensor::<f32>::scalar(0.0),
            Tensor::<f32>::scalar(1.0),
            Tensor::<f32>::scalar(2.0),
        ];

        let dataset = TensorDataset::new(data, targets);
        assert_eq!(dataset.len(), 3);

        let (x, _y) = dataset.get(0);
        assert_eq!(x.shape(), &[10]);
    }

    #[test]
    fn test_dataloader() {
        let data = vec![
            Tensor::<f32>::randn(&[10]),
            Tensor::<f32>::randn(&[10]),
            Tensor::<f32>::randn(&[10]),
            Tensor::<f32>::randn(&[10]),
        ];
        let targets = vec![
            Tensor::<f32>::scalar(0.0),
            Tensor::<f32>::scalar(1.0),
            Tensor::<f32>::scalar(2.0),
            Tensor::<f32>::scalar(3.0),
        ];

        let dataset = Arc::new(TensorDataset::new(data, targets));
        let mut loader = DataLoader::new(dataset, 2);

        assert_eq!(loader.num_batches(), 2);

        let batch1 = loader.next_batch();
        assert!(batch1.is_some());

        let (data_batch, target_batch) = batch1.unwrap();
        assert_eq!(data_batch.len(), 2);
        assert_eq!(target_batch.len(), 2);
    }
}
