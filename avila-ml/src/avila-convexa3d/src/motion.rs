//! Detecção de movimento e optical flow

use ndarray::Array2;
use serde::{Deserialize, Serialize};

use crate::video::{Video, VideoFrame};

/// Vetor de movimento (2D)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MotionVector {
    pub x: f32,
    pub y: f32,
}

impl MotionVector {
    /// Cria novo vetor
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Magnitude do vetor
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Ângulo do vetor em radianos
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

/// Campo de optical flow (grid de vetores)
#[derive(Debug, Clone)]
pub struct FlowField {
    /// Vetores de movimento (height × width)
    pub vectors: Vec<Vec<MotionVector>>,
    pub width: usize,
    pub height: usize,
}

impl FlowField {
    /// Cria novo campo vazio
    pub fn new(width: usize, height: usize) -> Self {
        let vectors = vec![vec![MotionVector::new(0.0, 0.0); width]; height];
        Self {
            vectors,
            width,
            height,
        }
    }

    /// Retorna vetor em posição
    pub fn get(&self, x: usize, y: usize) -> MotionVector {
        self.vectors[y][x]
    }

    /// Define vetor em posição
    pub fn set(&mut self, x: usize, y: usize, vector: MotionVector) {
        self.vectors[y][x] = vector;
    }

    /// Calcula magnitude média do fluxo
    pub fn average_magnitude(&self) -> f32 {
        let mut sum = 0.0;
        for row in &self.vectors {
            for v in row {
                sum += v.magnitude();
            }
        }
        sum / (self.width * self.height) as f32
    }
}

/// Optical flow
pub struct OpticalFlow;

impl OpticalFlow {
    /// Calcula optical flow simples (block matching)
    pub fn block_matching(
        frame1: &VideoFrame,
        frame2: &VideoFrame,
        block_size: usize,
        search_range: i32,
    ) -> FlowField {
        let (h, w, _) = frame1.shape();
        let grid_h = h / block_size;
        let grid_w = w / block_size;

        let mut flow = FlowField::new(grid_w, grid_h);

        // Convert to grayscale for simpler matching
        let gray1 = frame1.to_grayscale();
        let gray2 = frame2.to_grayscale();

        for by in 0..grid_h {
            for bx in 0..grid_w {
                let base_y = by * block_size;
                let base_x = bx * block_size;

                let mut best_dx = 0;
                let mut best_dy = 0;
                let mut best_sad = f32::MAX;

                // Search in neighborhood
                for dy in -search_range..=search_range {
                    for dx in -search_range..=search_range {
                        let mut sad = 0.0;
                        let mut count = 0;

                        for i in 0..block_size {
                            for j in 0..block_size {
                                let y1 = base_y + i;
                                let x1 = base_x + j;

                                let y2 = (base_y as i32 + i as i32 + dy) as usize;
                                let x2 = (base_x as i32 + j as i32 + dx) as usize;

                                if y1 < h && x1 < w && y2 < h && x2 < w {
                                    let diff =
                                        gray1[[y1, x1, 0]] as f32 - gray2[[y2, x2, 0]] as f32;
                                    sad += diff.abs();
                                    count += 1;
                                }
                            }
                        }

                        if count > 0 {
                            sad /= count as f32;
                            if sad < best_sad {
                                best_sad = sad;
                                best_dx = dx;
                                best_dy = dy;
                            }
                        }
                    }
                }

                flow.set(bx, by, MotionVector::new(best_dx as f32, best_dy as f32));
            }
        }

        flow
    }

    /// Calcula optical flow por gradientes (Lucas-Kanade simplificado)
    pub fn gradient_based(frame1: &VideoFrame, frame2: &VideoFrame) -> FlowField {
        let (h, w, _) = frame1.shape();
        let mut flow = FlowField::new(w, h);

        let gray1 = frame1.to_grayscale();
        let gray2 = frame2.to_grayscale();

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                // Spatial gradients
                let ix = (gray1[[y, x + 1, 0]] as f32 - gray1[[y, x - 1, 0]] as f32) / 2.0;
                let iy = (gray1[[y + 1, x, 0]] as f32 - gray1[[y - 1, x, 0]] as f32) / 2.0;

                // Temporal gradient
                let it = gray2[[y, x, 0]] as f32 - gray1[[y, x, 0]] as f32;

                // Solve for motion (simplified)
                let denom = ix * ix + iy * iy + 1e-6;
                let u = -ix * it / denom;
                let v = -iy * it / denom;

                flow.set(x, y, MotionVector::new(u, v));
            }
        }

        flow
    }
}

/// Detector de movimento
pub struct MotionDetector;

impl MotionDetector {
    /// Detecta movimento simples por diferença de frames
    pub fn frame_difference_threshold(
        frame1: &VideoFrame,
        frame2: &VideoFrame,
        threshold: f32,
    ) -> Array2<bool> {
        let (h, w, _) = frame1.shape();
        let mut motion_mask = Array2::from_elem((h, w), false);

        let gray1 = frame1.to_grayscale();
        let gray2 = frame2.to_grayscale();

        for y in 0..h {
            for x in 0..w {
                let diff = (gray1[[y, x, 0]] as f32 - gray2[[y, x, 0]] as f32).abs();
                if diff > threshold {
                    motion_mask[[y, x]] = true;
                }
            }
        }

        motion_mask
    }

    /// Calcula energia de movimento temporal em um vídeo
    pub fn temporal_motion_energy(video: &Video, start: usize, end: usize) -> f32 {
        let mut total_energy = 0.0;

        for i in start..end - 1 {
            if let (Some(f1), Some(f2)) = (video.get_frame(i), video.get_frame(i + 1)) {
                let diff = crate::video::VideoProcessor::frame_difference(f1, f2);
                let energy: f32 = diff.iter().map(|&v| (v as f32).abs()).sum();
                total_energy += energy;
            }
        }

        total_energy / (end - start - 1) as f32
    }

    /// Detecta frames com movimento significativo
    pub fn detect_motion_frames(video: &Video, threshold: f32) -> Vec<usize> {
        let mut motion_frames = Vec::new();

        for i in 0..video.num_frames() - 1 {
            if let (Some(f1), Some(f2)) = (video.get_frame(i), video.get_frame(i + 1)) {
                let diff = crate::video::VideoProcessor::frame_difference(f1, f2);
                let energy: f32 = diff.iter().map(|&v| (v as f32).abs()).sum();
                let avg_energy = energy / (f1.height() * f1.width() * f1.channels()) as f32;

                if avg_energy > threshold {
                    motion_frames.push(i);
                }
            }
        }

        motion_frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::video::VideoProcessor;
    use ndarray::Array3;

    #[test]
    fn test_motion_vector() {
        let v = MotionVector::new(3.0, 4.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_flow_field() {
        let mut flow = FlowField::new(10, 10);
        flow.set(5, 5, MotionVector::new(1.0, 2.0));
        assert_eq!(flow.get(5, 5).x, 1.0);
        assert_eq!(flow.get(5, 5).y, 2.0);
    }

    #[test]
    fn test_block_matching() {
        let f1 = VideoFrame::zeros(50, 50, 1);
        let f2 = VideoFrame::zeros(50, 50, 1);

        let flow = OpticalFlow::block_matching(&f1, &f2, 10, 5);
        assert!(flow.average_magnitude() >= 0.0);
    }

    #[test]
    fn test_gradient_based() {
        let f1 = VideoFrame::zeros(50, 50, 1);
        let f2 = VideoFrame::zeros(50, 50, 1);

        let flow = OpticalFlow::gradient_based(&f1, &f2);
        assert_eq!(flow.width, 50);
        assert_eq!(flow.height, 50);
    }

    #[test]
    fn test_frame_difference_threshold() {
        let data1 = Array3::zeros((10, 10, 1));
        let mut data2 = Array3::zeros((10, 10, 1));
        data2[[5, 5, 0]] = 100; // movimento em (5,5)

        let f1 = VideoFrame::new(data1, 0.0, 0);
        let f2 = VideoFrame::new(data2, 0.033, 1);

        let mask = MotionDetector::frame_difference_threshold(&f1, &f2, 50.0);
        assert!(mask[[5, 5]]);
    }

    #[test]
    fn test_temporal_motion_energy() {
        let video = VideoProcessor::create_test_video(20, 20, 10, 30.0);
        let energy = MotionDetector::temporal_motion_energy(&video, 0, video.num_frames());
        assert!(energy > 0.0);
    }
}
