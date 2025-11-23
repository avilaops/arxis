//! Tensores 4D (tempo × profundidade × altura × largura)

use ndarray::Array5;

use crate::common::Size4D;

/// Tensor 4D com dados espaço-temporais
#[derive(Debug, Clone)]
pub struct Tensor4D {
    /// Dados (time × depth × height × width × channels)
    pub data: Array5<f32>,
}

impl Tensor4D {
    /// Cria novo tensor 4D
    pub fn new(data: Array5<f32>) -> Self {
        Self { data }
    }

    /// Cria tensor preenchido com zeros
    pub fn zeros(time: usize, depth: usize, height: usize, width: usize, channels: usize) -> Self {
        Self {
            data: Array5::zeros((time, depth, height, width, channels)),
        }
    }

    /// Cria tensor preenchido com valor
    pub fn filled(
        time: usize,
        depth: usize,
        height: usize,
        width: usize,
        channels: usize,
        value: f32,
    ) -> Self {
        Self {
            data: Array5::from_elem((time, depth, height, width, channels), value),
        }
    }

    /// Retorna tamanho
    pub fn size(&self) -> Size4D {
        Size4D::new(
            self.data.shape()[0] as u32,
            self.data.shape()[1] as u32,
            self.data.shape()[2] as u32,
            self.data.shape()[3] as u32,
        )
    }

    /// Retorna dimensões (t, d, h, w, c)
    pub fn shape(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.data.shape()[0],
            self.data.shape()[1],
            self.data.shape()[2],
            self.data.shape()[3],
            self.data.shape()[4],
        )
    }

    /// Retorna valor em posição
    pub fn get(&self, t: usize, z: usize, y: usize, x: usize, c: usize) -> f32 {
        self.data[[t, z, y, x, c]]
    }

    /// Define valor em posição
    pub fn set(&mut self, t: usize, z: usize, y: usize, x: usize, c: usize, value: f32) {
        self.data[[t, z, y, x, c]] = value;
    }

    /// Normaliza entre 0 e 1
    pub fn normalize(&mut self) {
        let min = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self
            .data
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);

        if (max - min).abs() > 1e-6 {
            self.data.mapv_inplace(|v| (v - min) / (max - min));
        }
    }
}

/// Operações em tensores 4D
pub struct TensorOps;

impl TensorOps {
    /// Cria tensor de teste (gradiente 4D)
    pub fn create_test_tensor(
        time: usize,
        depth: usize,
        height: usize,
        width: usize,
    ) -> Tensor4D {
        let mut data = Array5::zeros((time, depth, height, width, 1));

        for t in 0..time {
            for z in 0..depth {
                for y in 0..height {
                    for x in 0..width {
                        let value = (t + z + y + x) as f32 / (time + depth + height + width) as f32;
                        data[[t, z, y, x, 0]] = value;
                    }
                }
            }
        }

        Tensor4D::new(data)
    }

    /// Calcula média espaço-temporal
    pub fn mean(tensor: &Tensor4D, channel: usize) -> f32 {
        let (t, d, h, w, _) = tensor.shape();
        let mut sum = 0.0;

        for time in 0..t {
            for depth in 0..d {
                for height in 0..h {
                    for width in 0..w {
                        sum += tensor.data[[time, depth, height, width, channel]];
                    }
                }
            }
        }

        sum / (t * d * h * w) as f32
    }

    /// Calcula desvio padrão
    pub fn std(tensor: &Tensor4D, channel: usize, mean: f32) -> f32 {
        let (t, d, h, w, _) = tensor.shape();
        let mut variance = 0.0;

        for time in 0..t {
            for depth in 0..d {
                for height in 0..h {
                    for width in 0..w {
                        let diff = tensor.data[[time, depth, height, width, channel]] - mean;
                        variance += diff * diff;
                    }
                }
            }
        }

        (variance / (t * d * h * w) as f32).sqrt()
    }

    /// Média temporal (colapsa dimensão tempo)
    pub fn temporal_mean(tensor: &Tensor4D) -> Array5<f32> {
        let (t, d, h, w, c) = tensor.shape();
        let mut result = Array5::zeros((1, d, h, w, c));

        for depth in 0..d {
            for height in 0..h {
                for width in 0..w {
                    for channel in 0..c {
                        let mut sum = 0.0;
                        for time in 0..t {
                            sum += tensor.data[[time, depth, height, width, channel]];
                        }
                        result[[0, depth, height, width, channel]] = sum / t as f32;
                    }
                }
            }
        }

        result
    }

    /// Média espacial (colapsa dimensões espaciais)
    pub fn spatial_mean(tensor: &Tensor4D) -> Array5<f32> {
        let (t, d, h, w, c) = tensor.shape();
        let mut result = Array5::zeros((t, 1, 1, 1, c));

        for time in 0..t {
            for channel in 0..c {
                let mut sum = 0.0;
                for depth in 0..d {
                    for height in 0..h {
                        for width in 0..w {
                            sum += tensor.data[[time, depth, height, width, channel]];
                        }
                    }
                }
                result[[time, 0, 0, 0, channel]] = sum / (d * h * w) as f32;
            }
        }

        result
    }

    /// Threshold 4D
    pub fn threshold(tensor: &Tensor4D, threshold: f32) -> Tensor4D {
        let mut result = tensor.clone();
        result
            .data
            .mapv_inplace(|v| if v >= threshold { 1.0 } else { 0.0 });
        result
    }

    /// Downsampling 4D
    pub fn downsample(tensor: &Tensor4D, factor: usize) -> Tensor4D {
        let (t, d, h, w, c) = tensor.shape();
        let new_t = t / factor;
        let new_d = d / factor;
        let new_h = h / factor;
        let new_w = w / factor;

        let mut result = Tensor4D::zeros(new_t, new_d, new_h, new_w, c);

        for channel in 0..c {
            for time in 0..new_t {
                for depth in 0..new_d {
                    for height in 0..new_h {
                        for width in 0..new_w {
                            let mut sum = 0.0;
                            let mut count = 0;

                            for dt in 0..factor {
                                for dz in 0..factor {
                                    for dy in 0..factor {
                                        for dx in 0..factor {
                                            let ot = time * factor + dt;
                                            let oz = depth * factor + dz;
                                            let oy = height * factor + dy;
                                            let ox = width * factor + dx;

                                            if ot < t && oz < d && oy < h && ox < w {
                                                sum += tensor.data[[ot, oz, oy, ox, channel]];
                                                count += 1;
                                            }
                                        }
                                    }
                                }
                            }

                            result.data[[time, depth, height, width, channel]] =
                                sum / count as f32;
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor4D::zeros(5, 10, 20, 30, 1);
        assert_eq!(tensor.shape(), (5, 10, 20, 30, 1));
    }

    #[test]
    fn test_tensor_get_set() {
        let mut tensor = Tensor4D::zeros(5, 5, 5, 5, 1);
        tensor.set(2, 2, 2, 2, 0, 42.0);
        assert_eq!(tensor.get(2, 2, 2, 2, 0), 42.0);
    }

    #[test]
    fn test_create_test_tensor() {
        let tensor = TensorOps::create_test_tensor(5, 5, 5, 5);
        assert!(tensor.get(4, 4, 4, 4, 0) > tensor.get(0, 0, 0, 0, 0));
    }

    #[test]
    fn test_mean_std() {
        let tensor = Tensor4D::filled(5, 5, 5, 5, 1, 10.0);
        let mean = TensorOps::mean(&tensor, 0);
        assert!((mean - 10.0).abs() < 1e-5);

        let std = TensorOps::std(&tensor, 0, mean);
        assert!(std.abs() < 1e-5);
    }

    #[test]
    fn test_threshold() {
        let tensor = TensorOps::create_test_tensor(5, 5, 5, 5);
        let binary = TensorOps::threshold(&tensor, 0.5);
        let val = binary.get(0, 0, 0, 0, 0);
        assert!(val == 0.0 || val == 1.0);
    }

    #[test]
    fn test_downsample() {
        let tensor = Tensor4D::filled(10, 10, 10, 10, 1, 1.0);
        let down = TensorOps::downsample(&tensor, 2);
        assert_eq!(down.shape(), (5, 5, 5, 5, 1));
    }
}
