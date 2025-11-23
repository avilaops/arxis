//! Neural network modules
//!
//! This module provides building blocks for neural networks including
//! layers, activations, and containers.

pub mod activation;
pub mod attention;
pub mod normalization;

use crate::tensor::Tensor;
use num_traits::{Float, NumAssign};

/// Base trait for all neural network modules
pub trait Module<T = f32>: Send + Sync
where
    T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static,
{
    /// Forward pass through the module
    fn forward(&self, input: &Tensor<T>) -> Tensor<T>;

    /// Get trainable parameters
    fn parameters(&self) -> Vec<&Tensor<T>> {
        vec![]
    }

    /// Get mutable trainable parameters
    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        vec![]
    }

    /// Zero out all gradients
    fn zero_grad(&mut self) {
        for param in self.parameters_mut() {
            param.zero_grad();
        }
    }
}

/// Linear (fully connected) layer: y = xW^T + b
pub struct Linear<T = f32> {
    pub weight: Tensor<T>,
    pub bias: Option<Tensor<T>>,
    in_features: usize,
    out_features: usize,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Linear<T> {
    /// Create a new linear layer
    pub fn new(in_features: usize, out_features: usize) -> Self {
        // Xavier initialization
        let k = T::from(1.0 / (in_features as f64).sqrt()).unwrap();
        let mut weight = Tensor::randn(&[out_features, in_features]);

        // Scale by k
        weight.data_mut().mapv_inplace(|x| x * k);
        weight = weight.requires_grad_();

        let bias = Some(Tensor::zeros(&[out_features]).requires_grad_());

        Self {
            weight,
            bias,
            in_features,
            out_features,
        }
    }

    /// Create a linear layer without bias
    pub fn new_no_bias(in_features: usize, out_features: usize) -> Self {
        let k = T::from(1.0 / (in_features as f64).sqrt()).unwrap();
        let mut weight = Tensor::randn(&[out_features, in_features]);
        weight.data_mut().mapv_inplace(|x| x * k);
        weight = weight.requires_grad_();

        Self {
            weight,
            bias: None,
            in_features,
            out_features,
        }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for Linear<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        use crate::tensor::TensorLike;

        // y = xW^T + b
        let output = input.matmul(&self.weight.reshape(&[self.in_features, self.out_features]));

        if let Some(ref bias) = self.bias {
            output.add(bias)
        } else {
            output
        }
    }

    fn parameters(&self) -> Vec<&Tensor<T>> {
        let mut params = vec![&self.weight];
        if let Some(ref bias) = self.bias {
            params.push(bias);
        }
        params
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        let mut params = vec![&mut self.weight];
        if let Some(ref mut bias) = self.bias {
            params.push(bias);
        }
        params
    }
}

/// 2D Convolutional layer
pub struct Conv2d<T = f32> {
    pub weight: Tensor<T>,
    pub bias: Option<Tensor<T>>,
    _in_channels: usize,
    _out_channels: usize,
    _kernel_size: (usize, usize),
    stride: (usize, usize),
    padding: (usize, usize),
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Conv2d<T> {
    pub fn new(in_channels: usize, out_channels: usize, kernel_size: (usize, usize)) -> Self {
        let k = T::from(
            1.0 / (in_channels as f64 * kernel_size.0 as f64 * kernel_size.1 as f64).sqrt(),
        )
        .unwrap();

        let mut weight = Tensor::randn(&[out_channels, in_channels, kernel_size.0, kernel_size.1]);
        weight.data_mut().mapv_inplace(|x| x * k);
        weight = weight.requires_grad_();

        let bias = Some(Tensor::zeros(&[out_channels]).requires_grad_());

        Self {
            weight,
            bias,
            _in_channels: in_channels,
            _out_channels: out_channels,
            _kernel_size: kernel_size,
            stride: (1, 1),
            padding: (0, 0),
        }
    }

    pub fn with_stride(mut self, stride: (usize, usize)) -> Self {
        self.stride = stride;
        self
    }

    pub fn with_padding(mut self, padding: (usize, usize)) -> Self {
        self.padding = padding;
        self
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for Conv2d<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        use rayon::prelude::*;

        // Input shape: (batch, in_channels, height, width)
        let input_shape = input.shape();
        let weight_shape = self.weight.shape();

        let batch = input_shape[0];
        let in_channels = input_shape[1];
        let height = input_shape[2];
        let width = input_shape[3];

        let out_channels = weight_shape[0];
        let kernel_h = weight_shape[2];
        let kernel_w = weight_shape[3];

        // Calculate output dimensions
        let out_height = (height + 2 * self.padding.0 - kernel_h) / self.stride.0 + 1;
        let out_width = (width + 2 * self.padding.1 - kernel_w) / self.stride.1 + 1;

        let output_size = batch * out_channels * out_height * out_width;
        let mut output_data = vec![T::zero(); output_size];

        // Parallelize over batch and output channels using Rayon
        let input_slice = input.data.as_slice().unwrap();
        let weight_slice = self.weight.data.as_slice().unwrap();
        let bias_slice = self.bias.as_ref().map(|b| b.data.as_slice().unwrap());

        output_data.par_chunks_mut(out_height * out_width)
            .enumerate()
            .for_each(|(idx, chunk)| {
                let b = idx / out_channels;
                let oc = idx % out_channels;

                if b >= batch { return; }

                for oh in 0..out_height {
                    for ow in 0..out_width {
                        let mut sum = T::zero();

                        // Convolve kernel over input
                        for ic in 0..in_channels {
                            for kh in 0..kernel_h {
                                for kw in 0..kernel_w {
                                    let ih = oh * self.stride.0 + kh;
                                    let iw = ow * self.stride.1 + kw;

                                    if ih >= self.padding.0 && ih < height + self.padding.0 &&
                                       iw >= self.padding.1 && iw < width + self.padding.1 {
                                        let ih_actual = ih - self.padding.0;
                                        let iw_actual = iw - self.padding.1;

                                        let input_idx = ((b * in_channels + ic) * height + ih_actual) * width + iw_actual;
                                        let weight_idx = ((oc * in_channels + ic) * kernel_h + kh) * kernel_w + kw;

                                        sum += input_slice[input_idx] * weight_slice[weight_idx];
                                    }
                                }
                            }
                        }

                        // Add bias if present
                        if let Some(bias) = bias_slice {
                            sum += bias[oc];
                        }

                        chunk[oh * out_width + ow] = sum;
                    }
                }
            });

        let output_array = ndarray::ArrayD::from_shape_vec(
            ndarray::IxDyn(&[batch, out_channels, out_height, out_width]),
            output_data
        ).unwrap();

        Tensor::new(output_array)
    }    fn parameters(&self) -> Vec<&Tensor<T>> {
        let mut params = vec![&self.weight];
        if let Some(ref bias) = self.bias {
            params.push(bias);
        }
        params
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        let mut params = vec![&mut self.weight];
        if let Some(ref mut bias) = self.bias {
            params.push(bias);
        }
        params
    }
}

/// 4D Convolutional layer for spatio-temporal data (e.g., astrophysical data)
pub struct Conv4d<T = f32> {
    pub weight: Tensor<T>,
    pub bias: Option<Tensor<T>>,
    _in_channels: usize,
    _out_channels: usize,
    _kernel_size: (usize, usize, usize, usize),
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Conv4d<T> {
    pub fn new(
        in_channels: usize,
        out_channels: usize,
        kernel_size: (usize, usize, usize, usize),
    ) -> Self {
        let kernel_vol = (kernel_size.0 * kernel_size.1 * kernel_size.2 * kernel_size.3) as f64;
        let k = T::from(1.0 / (in_channels as f64 * kernel_vol).sqrt()).unwrap();

        let mut weight = Tensor::randn(&[
            out_channels,
            in_channels,
            kernel_size.0,
            kernel_size.1,
            kernel_size.2,
            kernel_size.3,
        ]);
        weight.data_mut().mapv_inplace(|x| x * k);
        weight = weight.requires_grad_();

        let bias = Some(Tensor::zeros(&[out_channels]).requires_grad_());

        Self {
            weight,
            bias,
            _in_channels: in_channels,
            _out_channels: out_channels,
            _kernel_size: kernel_size,
        }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for Conv4d<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        use rayon::prelude::*;

        // Input shape: (batch, in_channels, t, x, y, z)
        let input_shape = input.shape();
        let weight_shape = self.weight.shape();

        let batch = input_shape[0];
        let in_channels = input_shape[1];
        let t = input_shape[2];
        let x = input_shape[3];
        let y = input_shape[4];
        let z = input_shape[5];

        let out_channels = weight_shape[0];
        let kt = weight_shape[2];
        let kx = weight_shape[3];
        let ky = weight_shape[4];
        let kz = weight_shape[5];

        let out_t = t - kt + 1;
        let out_x = x - kx + 1;
        let out_y = y - ky + 1;
        let out_z = z - kz + 1;

        let output_size = batch * out_channels * out_t * out_x * out_y * out_z;
        let mut output_data = vec![T::zero(); output_size];

        let input_slice = input.data.as_slice().unwrap();
        let weight_slice = self.weight.data.as_slice().unwrap();
        let bias_slice = self.bias.as_ref().map(|b| b.data.as_slice().unwrap());

        // Parallelize 4D convolution over batch and output channels
        let spatial_size = out_t * out_x * out_y * out_z;

        output_data.par_chunks_mut(spatial_size)
            .enumerate()
            .for_each(|(idx, chunk)| {
                let b = idx / out_channels;
                let oc = idx % out_channels;

                if b >= batch { return; }

                let mut chunk_idx = 0;
                for ot in 0..out_t {
                    for ox in 0..out_x {
                        for oy in 0..out_y {
                            for oz in 0..out_z {
                                let mut sum = T::zero();

                                // 4D convolution kernel
                                for ic in 0..in_channels {
                                    for ikt in 0..kt {
                                        for ikx in 0..kx {
                                            for iky in 0..ky {
                                                for ikz in 0..kz {
                                                    let it = ot + ikt;
                                                    let ix = ox + ikx;
                                                    let iy = oy + iky;
                                                    let iz = oz + ikz;

                                                    let input_idx = ((((b * in_channels + ic) * t + it) * x + ix) * y + iy) * z + iz;
                                                    let weight_idx = ((((oc * in_channels + ic) * kt + ikt) * kx + ikx) * ky + iky) * kz + ikz;

                                                    sum += input_slice[input_idx] * weight_slice[weight_idx];
                                                }
                                            }
                                        }
                                    }
                                }

                                // Add bias
                                if let Some(bias) = bias_slice {
                                    sum += bias[oc];
                                }

                                chunk[chunk_idx] = sum;
                                chunk_idx += 1;
                            }
                        }
                    }
                }
            });

        let output_array = ndarray::ArrayD::from_shape_vec(
            ndarray::IxDyn(&[batch, out_channels, out_t, out_x, out_y, out_z]),
            output_data
        ).unwrap();

        Tensor::new(output_array)
    }    fn parameters(&self) -> Vec<&Tensor<T>> {
        let mut params = vec![&self.weight];
        if let Some(ref bias) = self.bias {
            params.push(bias);
        }
        params
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        let mut params = vec![&mut self.weight];
        if let Some(ref mut bias) = self.bias {
            params.push(bias);
        }
        params
    }
}

/// Sequential container for stacking modules
pub struct Sequential<T = f32> {
    modules: Vec<Box<dyn Module<T>>>,
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Sequential<T> {
    pub fn new(modules: Vec<Box<dyn Module<T>>>) -> Self {
        Self { modules }
    }
}

impl<T: Float + NumAssign + ndarray::ScalarOperand + Send + Sync + 'static> Module<T>
    for Sequential<T>
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        let mut output = input.clone();
        for module in &self.modules {
            output = module.forward(&output);
        }
        output
    }

    fn parameters(&self) -> Vec<&Tensor<T>> {
        self.modules.iter().flat_map(|m| m.parameters()).collect()
    }

    fn parameters_mut(&mut self) -> Vec<&mut Tensor<T>> {
        self.modules
            .iter_mut()
            .flat_map(|m| m.parameters_mut())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_layer() {
        let layer = Linear::<f32>::new(10, 5);
        let input = Tensor::randn(&[32, 10]); // batch_size=32

        let output = layer.forward(&input);
        assert_eq!(output.shape(), &[32, 5]);
    }

    #[test]
    fn test_linear_parameters() {
        let mut layer = Linear::<f32>::new(10, 5);
        let params = layer.parameters();

        assert_eq!(params.len(), 2); // weight + bias

        layer.zero_grad();
    }

    #[test]
    fn test_sequential() {
        let model = Sequential::new(vec![
            Box::new(Linear::<f32>::new(10, 20)),
            Box::new(Linear::<f32>::new(20, 5)),
        ]);

        let input = Tensor::randn(&[32, 10]);
        let output = model.forward(&input);

        assert_eq!(output.shape(), &[32, 5]);
    }
}
