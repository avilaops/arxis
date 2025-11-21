//! Processamento de volumes 3D (dados espaciais)

use ndarray::{Array3, Array4};

use crate::common::{Axis3D, Size3D};

/// Volume 3D (dados espaciais: profundidade × altura × largura)
#[derive(Debug, Clone)]
pub struct Volume3D {
    /// Dados do volume (depth × height × width × channels)
    pub data: Array4<f32>,
}

impl Volume3D {
    /// Cria novo volume
    pub fn new(data: Array4<f32>) -> Self {
        Self { data }
    }

    /// Cria volume vazio (zeros)
    pub fn zeros(depth: usize, height: usize, width: usize, channels: usize) -> Self {
        Self {
            data: Array4::zeros((depth, height, width, channels)),
        }
    }

    /// Cria volume preenchido com valor
    pub fn filled(
        depth: usize,
        height: usize,
        width: usize,
        channels: usize,
        value: f32,
    ) -> Self {
        Self {
            data: Array4::from_elem((depth, height, width, channels), value),
        }
    }

    /// Retorna tamanho
    pub fn size(&self) -> Size3D {
        Size3D::new(
            self.data.shape()[2] as u32,
            self.data.shape()[1] as u32,
            self.data.shape()[0] as u32,
        )
    }

    /// Retorna dimensões (depth, height, width, channels)
    pub fn shape(&self) -> (usize, usize, usize, usize) {
        (
            self.data.shape()[0],
            self.data.shape()[1],
            self.data.shape()[2],
            self.data.shape()[3],
        )
    }

    /// Retorna valor em posição
    pub fn get(&self, x: usize, y: usize, z: usize, channel: usize) -> f32 {
        self.data[[z, y, x, channel]]
    }

    /// Define valor em posição
    pub fn set(&mut self, x: usize, y: usize, z: usize, channel: usize, value: f32) {
        self.data[[z, y, x, channel]] = value;
    }

    /// Extrai slice 2D ao longo de um eixo
    pub fn slice_at(&self, axis: Axis3D, index: usize, channel: usize) -> Array3<f32> {
        let (d, h, w, _) = self.shape();

        match axis {
            Axis3D::X => {
                // YZ plane at x=index
                let mut slice = Array3::zeros((d, h, 1));
                for z in 0..d {
                    for y in 0..h {
                        slice[[z, y, 0]] = self.data[[z, y, index, channel]];
                    }
                }
                slice
            }
            Axis3D::Y => {
                // XZ plane at y=index
                let mut slice = Array3::zeros((d, w, 1));
                for z in 0..d {
                    for x in 0..w {
                        slice[[z, x, 0]] = self.data[[z, index, x, channel]];
                    }
                }
                slice
            }
            Axis3D::Z => {
                // XY plane at z=index
                let mut slice = Array3::zeros((h, w, 1));
                for y in 0..h {
                    for x in 0..w {
                        slice[[y, x, 0]] = self.data[[index, y, x, channel]];
                    }
                }
                slice
            }
        }
    }

    /// Normaliza valores entre 0 e 1
    pub fn normalize(&mut self) {
        let min = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        if (max - min).abs() > 1e-6 {
            self.data.mapv_inplace(|v| (v - min) / (max - min));
        }
    }
}

/// Processador de volumes
pub struct VolumeProcessor;

impl VolumeProcessor {
    /// Cria volume de teste (gradiente 3D)
    pub fn create_test_volume(depth: usize, height: usize, width: usize) -> Volume3D {
        let mut data = Array4::zeros((depth, height, width, 1));

        for z in 0..depth {
            for y in 0..height {
                for x in 0..width {
                    let value = (x + y + z) as f32 / (width + height + depth) as f32;
                    data[[z, y, x, 0]] = value;
                }
            }
        }

        Volume3D::new(data)
    }

    /// Cria volume esférico (1 dentro, 0 fora)
    pub fn create_sphere(radius: usize) -> Volume3D {
        let size = radius * 2 + 1;
        let mut volume = Volume3D::zeros(size, size, size, 1);
        let center = radius as f32;

        for z in 0..size {
            for y in 0..size {
                for x in 0..size {
                    let dist = ((x as f32 - center).powi(2)
                        + (y as f32 - center).powi(2)
                        + (z as f32 - center).powi(2))
                    .sqrt();

                    if dist <= radius as f32 {
                        volume.data[[z, y, x, 0]] = 1.0;
                    }
                }
            }
        }

        volume
    }

    /// Calcula média espacial
    pub fn spatial_mean(volume: &Volume3D, channel: usize) -> f32 {
        let (d, h, w, _) = volume.shape();
        let mut sum = 0.0;

        for z in 0..d {
            for y in 0..h {
                for x in 0..w {
                    sum += volume.data[[z, y, x, channel]];
                }
            }
        }

        sum / (d * h * w) as f32
    }

    /// Calcula desvio padrão espacial
    pub fn spatial_std(volume: &Volume3D, channel: usize, mean: f32) -> f32 {
        let (d, h, w, _) = volume.shape();
        let mut variance = 0.0;

        for z in 0..d {
            for y in 0..h {
                for x in 0..w {
                    let diff = volume.data[[z, y, x, channel]] - mean;
                    variance += diff * diff;
                }
            }
        }

        (variance / (d * h * w) as f32).sqrt()
    }

    /// Aplica threshold no volume
    pub fn threshold(volume: &Volume3D, threshold: f32) -> Volume3D {
        let mut result = volume.clone();
        result.data.mapv_inplace(|v| if v >= threshold { 1.0 } else { 0.0 });
        result
    }

    /// Downsampling 3D (reduz resolução)
    pub fn downsample(volume: &Volume3D, factor: usize) -> Volume3D {
        let (d, h, w, c) = volume.shape();
        let new_d = d / factor;
        let new_h = h / factor;
        let new_w = w / factor;

        let mut result = Volume3D::zeros(new_d, new_h, new_w, c);

        for channel in 0..c {
            for z in 0..new_d {
                for y in 0..new_h {
                    for x in 0..new_w {
                        // Average values in the cube
                        let mut sum = 0.0;
                        let mut count = 0;

                        for dz in 0..factor {
                            for dy in 0..factor {
                                for dx in 0..factor {
                                    let oz = z * factor + dz;
                                    let oy = y * factor + dy;
                                    let ox = x * factor + dx;

                                    if oz < d && oy < h && ox < w {
                                        sum += volume.data[[oz, oy, ox, channel]];
                                        count += 1;
                                    }
                                }
                            }
                        }

                        result.data[[z, y, x, channel]] = sum / count as f32;
                    }
                }
            }
        }

        result
    }

    /// Calcula gradiente 3D
    pub fn gradient_magnitude(volume: &Volume3D, channel: usize) -> Volume3D {
        let (d, h, w, _) = volume.shape();
        let mut result = Volume3D::zeros(d, h, w, 1);

        for z in 1..d - 1 {
            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    let gx = volume.data[[z, y, x + 1, channel]]
                        - volume.data[[z, y, x - 1, channel]];
                    let gy = volume.data[[z, y + 1, x, channel]]
                        - volume.data[[z, y - 1, x, channel]];
                    let gz = volume.data[[z + 1, y, x, channel]]
                        - volume.data[[z - 1, y, x, channel]];

                    let magnitude = (gx * gx + gy * gy + gz * gz).sqrt();
                    result.data[[z, y, x, 0]] = magnitude;
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
    fn test_volume_creation() {
        let volume = Volume3D::zeros(10, 20, 30, 1);
        assert_eq!(volume.shape(), (10, 20, 30, 1));
    }

    #[test]
    fn test_volume_get_set() {
        let mut volume = Volume3D::zeros(5, 5, 5, 1);
        volume.set(2, 2, 2, 0, 42.0);
        assert_eq!(volume.get(2, 2, 2, 0), 42.0);
    }

    #[test]
    fn test_create_test_volume() {
        let volume = VolumeProcessor::create_test_volume(10, 10, 10);
        assert!(volume.get(9, 9, 9, 0) > volume.get(0, 0, 0, 0));
    }

    #[test]
    fn test_create_sphere() {
        let sphere = VolumeProcessor::create_sphere(5);
        assert_eq!(sphere.get(5, 5, 5, 0), 1.0); // center
        assert_eq!(sphere.get(0, 0, 0, 0), 0.0); // corner
    }

    #[test]
    fn test_spatial_mean() {
        let volume = Volume3D::filled(10, 10, 10, 1, 5.0);
        let mean = VolumeProcessor::spatial_mean(&volume, 0);
        assert!((mean - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_threshold() {
        let volume = VolumeProcessor::create_test_volume(10, 10, 10);
        let binary = VolumeProcessor::threshold(&volume, 0.5);
        assert!(binary.get(0, 0, 0, 0) == 0.0 || binary.get(0, 0, 0, 0) == 1.0);
    }

    #[test]
    fn test_downsample() {
        let volume = Volume3D::filled(20, 20, 20, 1, 1.0);
        let downsampled = VolumeProcessor::downsample(&volume, 2);
        assert_eq!(downsampled.shape(), (10, 10, 10, 1));
    }
}
