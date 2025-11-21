//! Processamento espaço-temporal e análise de movimento 4D

use crate::sequence::VolumeSequence;
use crate::tensor::Tensor4D;

/// Processador espaço-temporal
pub struct SpatioTemporalProcessor;

impl SpatioTemporalProcessor {
    /// Calcula correlação temporal
    pub fn temporal_correlation(tensor: &Tensor4D, lag: usize, channel: usize) -> f32 {
        let (t, d, h, w, _) = tensor.shape();
        if lag >= t {
            return 0.0;
        }

        let mut correlation = 0.0;
        let mut count = 0;

        for time in 0..(t - lag) {
            for depth in 0..d {
                for height in 0..h {
                    for width in 0..w {
                        let v1 = tensor.data[[time, depth, height, width, channel]];
                        let v2 = tensor.data[[time + lag, depth, height, width, channel]];
                        correlation += v1 * v2;
                        count += 1;
                    }
                }
            }
        }

        if count > 0 {
            correlation / count as f32
        } else {
            0.0
        }
    }

    /// Deriva temporal (gradiente no tempo)
    pub fn temporal_derivative(tensor: &Tensor4D) -> Tensor4D {
        let (t, d, h, w, c) = tensor.shape();
        let mut result = Tensor4D::zeros(t, d, h, w, c);

        for time in 1..t {
            for depth in 0..d {
                for height in 0..h {
                    for width in 0..w {
                        for channel in 0..c {
                            let v1 = tensor.data[[time - 1, depth, height, width, channel]];
                            let v2 = tensor.data[[time, depth, height, width, channel]];
                            result.data[[time, depth, height, width, channel]] = v2 - v1;
                        }
                    }
                }
            }
        }

        result
    }

    /// Deriva espacial (gradiente espacial em cada frame temporal)
    pub fn spatial_derivative(tensor: &Tensor4D, axis: usize) -> Tensor4D {
        let (t, d, h, w, c) = tensor.shape();
        let mut result = Tensor4D::zeros(t, d, h, w, c);

        for time in 0..t {
            for depth in 1..d - 1 {
                for height in 1..h - 1 {
                    for width in 1..w - 1 {
                        for channel in 0..c {
                            let grad = match axis {
                                0 => {
                                    // Z
                                    tensor.data[[time, depth + 1, height, width, channel]]
                                        - tensor.data[[time, depth - 1, height, width, channel]]
                                }
                                1 => {
                                    // Y
                                    tensor.data[[time, depth, height + 1, width, channel]]
                                        - tensor.data[[time, depth, height - 1, width, channel]]
                                }
                                2 => {
                                    // X
                                    tensor.data[[time, depth, height, width + 1, channel]]
                                        - tensor.data[[time, depth, height, width - 1, channel]]
                                }
                                _ => 0.0,
                            };

                            result.data[[time, depth, height, width, channel]] = grad / 2.0;
                        }
                    }
                }
            }
        }

        result
    }

    /// Magnitude do gradiente 4D
    pub fn gradient_magnitude_4d(tensor: &Tensor4D) -> Tensor4D {
        let dt = Self::temporal_derivative(tensor);
        let dx = Self::spatial_derivative(tensor, 2);
        let dy = Self::spatial_derivative(tensor, 1);
        let dz = Self::spatial_derivative(tensor, 0);

        let (t, d, h, w, c) = tensor.shape();
        let mut result = Tensor4D::zeros(t, d, h, w, c);

        for time in 0..t {
            for depth in 0..d {
                for height in 0..h {
                    for width in 0..w {
                        for channel in 0..c {
                            let gdt = dt.data[[time, depth, height, width, channel]];
                            let gdx = dx.data[[time, depth, height, width, channel]];
                            let gdy = dy.data[[time, depth, height, width, channel]];
                            let gdz = dz.data[[time, depth, height, width, channel]];

                            let mag = (gdt * gdt + gdx * gdx + gdy * gdy + gdz * gdz).sqrt();
                            result.data[[time, depth, height, width, channel]] = mag;
                        }
                    }
                }
            }
        }

        result
    }
}

/// Analisador de movimento volumétrico
pub struct MotionAnalyzer;

impl MotionAnalyzer {
    /// Detecta movimento volumétrico ao longo do tempo
    pub fn detect_volumetric_motion(sequence: &VolumeSequence, threshold: f32) -> Vec<bool> {
        let mut motion_frames = vec![false; sequence.num_frames()];

        for i in 1..sequence.num_frames() {
            if let (Some(f1), Some(f2)) = (sequence.get_frame(i - 1), sequence.get_frame(i)) {
                let mut total_diff = 0.0;
                let mut count = 0;

                let (d, h, w, c) = f1.shape();
                for z in 0..d {
                    for y in 0..h {
                        for x in 0..w {
                            for ch in 0..c {
                                let diff = (f2.data[[z, y, x, ch]] - f1.data[[z, y, x, ch]]).abs();
                                total_diff += diff;
                                count += 1;
                            }
                        }
                    }
                }

                let avg_diff = total_diff / count as f32;
                if avg_diff > threshold {
                    motion_frames[i] = true;
                }
            }
        }

        motion_frames
    }

    /// Calcula taxa de movimento (% de frames com movimento)
    pub fn motion_rate(motion_frames: &[bool]) -> f32 {
        let motion_count = motion_frames.iter().filter(|&&x| x).count();
        motion_count as f32 / motion_frames.len() as f32
    }

    /// Detecta mudanças abruptas (picos de movimento)
    pub fn detect_transitions(sequence: &VolumeSequence, threshold: f32) -> Vec<usize> {
        let mut transitions = Vec::new();

        for i in 1..sequence.num_frames() {
            if let (Some(f1), Some(f2)) = (sequence.get_frame(i - 1), sequence.get_frame(i)) {
                let diff: f32 = f1
                    .data
                    .iter()
                    .zip(f2.data.iter())
                    .map(|(a, b)| (a - b).abs())
                    .sum();

                let avg_diff = diff / f1.data.len() as f32;

                if avg_diff > threshold {
                    transitions.push(i);
                }
            }
        }

        transitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::VolumeSequenceProcessor;
    use crate::tensor::TensorOps;

    #[test]
    fn test_temporal_correlation() {
        let tensor = TensorOps::create_test_tensor(10, 5, 5, 5);
        let corr = SpatioTemporalProcessor::temporal_correlation(&tensor, 1, 0);
        assert!(corr >= 0.0);
    }

    #[test]
    fn test_temporal_derivative() {
        let tensor = TensorOps::create_test_tensor(5, 5, 5, 5);
        let deriv = SpatioTemporalProcessor::temporal_derivative(&tensor);
        assert_eq!(deriv.shape(), tensor.shape());
    }

    #[test]
    fn test_spatial_derivative() {
        let tensor = TensorOps::create_test_tensor(5, 10, 10, 10);
        let deriv = SpatioTemporalProcessor::spatial_derivative(&tensor, 0);
        assert_eq!(deriv.shape(), tensor.shape());
    }

    #[test]
    fn test_gradient_magnitude_4d() {
        let tensor = TensorOps::create_test_tensor(5, 10, 10, 10);
        let grad = SpatioTemporalProcessor::gradient_magnitude_4d(&tensor);
        assert_eq!(grad.shape(), tensor.shape());
    }

    #[test]
    fn test_detect_volumetric_motion() {
        let seq = VolumeSequenceProcessor::create_test_sequence(5, 10, 10, 10);
        let motion = MotionAnalyzer::detect_volumetric_motion(&seq, 10.0);
        assert_eq!(motion.len(), 5);
    }

    #[test]
    fn test_motion_rate() {
        let motion_frames = vec![false, true, true, false, true];
        let rate = MotionAnalyzer::motion_rate(&motion_frames);
        assert!((rate - 0.6).abs() < 1e-5);
    }

    #[test]
    fn test_detect_transitions() {
        let seq = VolumeSequenceProcessor::create_test_sequence(5, 10, 10, 10);
        let transitions = MotionAnalyzer::detect_transitions(&seq, 100.0);
        assert!(transitions.len() <= seq.num_frames());
    }
}
