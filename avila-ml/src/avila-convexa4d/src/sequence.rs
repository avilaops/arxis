//! Sequências de volumes 3D ao longo do tempo

use ndarray::Array4;

use crate::tensor::Tensor4D;

/// Volume temporal (t, d, h, w, c) - um frame de volume
#[derive(Debug, Clone)]
pub struct VolumeFrame {
    /// Dados do volume (depth × height × width × channels)
    pub data: Array4<f32>,
    /// Timestamp em segundos
    pub timestamp: f32,
    /// Número do frame
    pub frame_number: usize,
}

impl VolumeFrame {
    /// Cria novo frame de volume
    pub fn new(data: Array4<f32>, timestamp: f32, frame_number: usize) -> Self {
        Self {
            data,
            timestamp,
            frame_number,
        }
    }

    /// Cria frame vazio (zeros)
    pub fn zeros(depth: usize, height: usize, width: usize, channels: usize) -> Self {
        Self {
            data: Array4::zeros((depth, height, width, channels)),
            timestamp: 0.0,
            frame_number: 0,
        }
    }

    /// Retorna dimensões (d, h, w, c)
    pub fn shape(&self) -> (usize, usize, usize, usize) {
        (
            self.data.shape()[0],
            self.data.shape()[1],
            self.data.shape()[2],
            self.data.shape()[3],
        )
    }
}

/// Sequência de volumes ao longo do tempo
#[derive(Debug, Clone)]
pub struct VolumeSequence {
    /// Frames volumétricos
    pub frames: Vec<VolumeFrame>,
    /// Frames por segundo
    pub fps: f32,
    /// Dimensões espaciais
    pub depth: usize,
    pub height: usize,
    pub width: usize,
    pub channels: usize,
}

impl VolumeSequence {
    /// Cria nova sequência vazia
    pub fn new(
        depth: usize,
        height: usize,
        width: usize,
        channels: usize,
        fps: f32,
    ) -> Self {
        Self {
            frames: Vec::new(),
            fps,
            depth,
            height,
            width,
            channels,
        }
    }

    /// Adiciona frame
    pub fn add_frame(&mut self, data: Array4<f32>) {
        let frame_number = self.frames.len();
        let timestamp = frame_number as f32 / self.fps;
        self.frames
            .push(VolumeFrame::new(data, timestamp, frame_number));
    }

    /// Retorna número de frames
    pub fn num_frames(&self) -> usize {
        self.frames.len()
    }

    /// Retorna duração em segundos
    pub fn duration(&self) -> f32 {
        self.num_frames() as f32 / self.fps
    }

    /// Retorna frame por índice
    pub fn get_frame(&self, index: usize) -> Option<&VolumeFrame> {
        self.frames.get(index)
    }

    /// Converte para tensor 4D
    pub fn to_tensor(&self) -> Tensor4D {
        let t = self.num_frames();
        let mut tensor = Tensor4D::zeros(t, self.depth, self.height, self.width, self.channels);

        for (i, frame) in self.frames.iter().enumerate() {
            for z in 0..self.depth {
                for y in 0..self.height {
                    for x in 0..self.width {
                        for c in 0..self.channels {
                            tensor.data[[i, z, y, x, c]] = frame.data[[z, y, x, c]];
                        }
                    }
                }
            }
        }

        tensor
    }
}

/// Processador de sequências volumétricas
pub struct VolumeSequenceProcessor;

impl VolumeSequenceProcessor {
    /// Cria sequência de teste
    pub fn create_test_sequence(
        num_frames: usize,
        depth: usize,
        height: usize,
        width: usize,
    ) -> VolumeSequence {
        let mut sequence = VolumeSequence::new(depth, height, width, 1, 30.0);

        for t in 0..num_frames {
            let mut frame_data = Array4::zeros((depth, height, width, 1));
            let intensity = (t as f32 / num_frames as f32 * 255.0) as u8 as f32;

            for z in 0..depth {
                for y in 0..height {
                    for x in 0..width {
                        frame_data[[z, y, x, 0]] = intensity + (z + y + x) as f32 / 10.0;
                    }
                }
            }

            sequence.add_frame(frame_data);
        }

        sequence
    }

    /// Diferença temporal entre frames consecutivos
    pub fn temporal_difference(frame1: &VolumeFrame, frame2: &VolumeFrame) -> Array4<f32> {
        let (d, h, w, c) = frame1.shape();
        let mut diff = Array4::zeros((d, h, w, c));

        for z in 0..d {
            for y in 0..h {
                for x in 0..w {
                    for ch in 0..c {
                        diff[[z, y, x, ch]] =
                            frame2.data[[z, y, x, ch]] - frame1.data[[z, y, x, ch]];
                    }
                }
            }
        }

        diff
    }

    /// Calcula média temporal da sequência
    pub fn temporal_mean(sequence: &VolumeSequence) -> Array4<f32> {
        let (d, h, w, c) = (
            sequence.depth,
            sequence.height,
            sequence.width,
            sequence.channels,
        );
        let mut mean = Array4::zeros((d, h, w, c));
        let n = sequence.num_frames() as f32;

        for frame in &sequence.frames {
            for z in 0..d {
                for y in 0..h {
                    for x in 0..w {
                        for ch in 0..c {
                            mean[[z, y, x, ch]] += frame.data[[z, y, x, ch]] / n;
                        }
                    }
                }
            }
        }

        mean
    }

    /// Energia espaço-temporal
    pub fn spatiotemporal_energy(sequence: &VolumeSequence) -> f32 {
        let mut energy = 0.0;

        for i in 1..sequence.num_frames() {
            let diff = Self::temporal_difference(&sequence.frames[i - 1], &sequence.frames[i]);
            energy += diff.iter().map(|&v| v.abs()).sum::<f32>();
        }

        energy / (sequence.num_frames() - 1) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_frame() {
        let frame = VolumeFrame::zeros(10, 20, 30, 1);
        assert_eq!(frame.shape(), (10, 20, 30, 1));
    }

    #[test]
    fn test_volume_sequence() {
        let mut seq = VolumeSequence::new(10, 10, 10, 1, 30.0);
        seq.add_frame(Array4::zeros((10, 10, 10, 1)));
        seq.add_frame(Array4::zeros((10, 10, 10, 1)));

        assert_eq!(seq.num_frames(), 2);
        assert!((seq.duration() - 2.0 / 30.0).abs() < 1e-5);
    }

    #[test]
    fn test_to_tensor() {
        let seq = VolumeSequenceProcessor::create_test_sequence(5, 10, 10, 10);
        let tensor = seq.to_tensor();
        assert_eq!(tensor.shape(), (5, 10, 10, 10, 1));
    }

    #[test]
    fn test_temporal_mean() {
        let seq = VolumeSequenceProcessor::create_test_sequence(5, 10, 10, 10);
        let mean = VolumeSequenceProcessor::temporal_mean(&seq);
        assert_eq!(mean.shape(), [10, 10, 10, 1]);
    }

    #[test]
    fn test_spatiotemporal_energy() {
        let seq = VolumeSequenceProcessor::create_test_sequence(5, 10, 10, 10);
        let energy = VolumeSequenceProcessor::spatiotemporal_energy(&seq);
        assert!(energy > 0.0);
    }
}
