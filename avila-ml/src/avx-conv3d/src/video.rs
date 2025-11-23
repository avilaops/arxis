//! Processamento de vídeo (sequências temporais de frames)

use ndarray::Array3;

/// Frame de vídeo (wrapper para imagem 2D)
#[derive(Debug, Clone)]
pub struct VideoFrame {
    /// Dados do frame (altura × largura × canais)
    pub data: Array3<u8>,
    /// Timestamp em segundos
    pub timestamp: f32,
    /// Número do frame
    pub frame_number: usize,
}

impl VideoFrame {
    /// Cria novo frame
    pub fn new(data: Array3<u8>, timestamp: f32, frame_number: usize) -> Self {
        Self {
            data,
            timestamp,
            frame_number,
        }
    }

    /// Cria frame vazio (preto)
    pub fn zeros(height: usize, width: usize, channels: usize) -> Self {
        Self {
            data: Array3::zeros((height, width, channels)),
            timestamp: 0.0,
            frame_number: 0,
        }
    }

    /// Retorna dimensões (altura, largura, canais)
    pub fn shape(&self) -> (usize, usize, usize) {
        (self.data.shape()[0], self.data.shape()[1], self.data.shape()[2])
    }

    /// Retorna altura
    pub fn height(&self) -> usize {
        self.data.shape()[0]
    }

    /// Retorna largura
    pub fn width(&self) -> usize {
        self.data.shape()[1]
    }

    /// Retorna número de canais
    pub fn channels(&self) -> usize {
        self.data.shape()[2]
    }

    /// Converte para escala de cinza
    pub fn to_grayscale(&self) -> Array3<u8> {
        let (h, w, c) = self.shape();
        if c == 1 {
            return self.data.clone();
        }

        let mut gray = Array3::zeros((h, w, 1));
        for i in 0..h {
            for j in 0..w {
                let r = self.data[[i, j, 0]] as f32;
                let g = self.data[[i, j, 1]] as f32;
                let b = self.data[[i, j, 2]] as f32;
                gray[[i, j, 0]] = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            }
        }
        gray
    }
}

/// Vídeo (sequência de frames)
#[derive(Debug, Clone)]
pub struct Video {
    /// Frames do vídeo
    pub frames: Vec<VideoFrame>,
    /// Frames por segundo
    pub fps: f32,
    /// Largura
    pub width: usize,
    /// Altura
    pub height: usize,
    /// Canais
    pub channels: usize,
}

impl Video {
    /// Cria novo vídeo vazio
    pub fn new(width: usize, height: usize, channels: usize, fps: f32) -> Self {
        Self {
            frames: Vec::new(),
            fps,
            width,
            height,
            channels,
        }
    }

    /// Adiciona frame
    pub fn add_frame(&mut self, data: Array3<u8>) {
        let frame_number = self.frames.len();
        let timestamp = frame_number as f32 / self.fps;
        self.frames.push(VideoFrame::new(data, timestamp, frame_number));
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
    pub fn get_frame(&self, index: usize) -> Option<&VideoFrame> {
        self.frames.get(index)
    }

    /// Retorna frame por timestamp
    pub fn get_frame_at_time(&self, time: f32) -> Option<&VideoFrame> {
        let frame_index = (time * self.fps) as usize;
        self.get_frame(frame_index)
    }

    /// Extrai sub-vídeo
    pub fn subvideo(&self, start_frame: usize, end_frame: usize) -> Video {
        let frames: Vec<VideoFrame> = self.frames[start_frame..end_frame]
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let mut frame = f.clone();
                frame.frame_number = i;
                frame.timestamp = i as f32 / self.fps;
                frame
            })
            .collect();

        Video {
            frames,
            fps: self.fps,
            width: self.width,
            height: self.height,
            channels: self.channels,
        }
    }
}

/// Processador de vídeo
pub struct VideoProcessor;

impl VideoProcessor {
    /// Cria vídeo de teste (gradiente temporal)
    pub fn create_test_video(
        width: usize,
        height: usize,
        num_frames: usize,
        fps: f32,
    ) -> Video {
        let mut video = Video::new(width, height, 3, fps);

        for t in 0..num_frames {
            let mut frame = Array3::zeros((height, width, 3));
            let intensity = (t as f32 / num_frames as f32 * 255.0) as u8;

            for i in 0..height {
                for j in 0..width {
                    frame[[i, j, 0]] = intensity;
                    frame[[i, j, 1]] = 255 - intensity;
                    frame[[i, j, 2]] = 128;
                }
            }
            video.add_frame(frame);
        }

        video
    }

    /// Diferença entre frames consecutivos
    pub fn frame_difference(frame1: &VideoFrame, frame2: &VideoFrame) -> Array3<i16> {
        let (h, w, c) = frame1.shape();
        let mut diff = Array3::zeros((h, w, c));

        for i in 0..h {
            for j in 0..w {
                for k in 0..c {
                    let v1 = frame1.data[[i, j, k]] as i16;
                    let v2 = frame2.data[[i, j, k]] as i16;
                    diff[[i, j, k]] = v1 - v2;
                }
            }
        }
        diff
    }

    /// Calcula média temporal
    pub fn temporal_mean(video: &Video) -> Array3<f32> {
        let (h, w, c) = (video.height, video.width, video.channels);
        let mut mean = Array3::zeros((h, w, c));
        let n = video.num_frames() as f32;

        for frame in &video.frames {
            for i in 0..h {
                for j in 0..w {
                    for k in 0..c {
                        mean[[i, j, k]] += frame.data[[i, j, k]] as f32 / n;
                    }
                }
            }
        }
        mean
    }

    /// Calcula desvio padrão temporal
    pub fn temporal_std(video: &Video, mean: &Array3<f32>) -> Array3<f32> {
        let (h, w, c) = (video.height, video.width, video.channels);
        let mut variance = Array3::zeros((h, w, c));
        let n = video.num_frames() as f32;

        for frame in &video.frames {
            for i in 0..h {
                for j in 0..w {
                    for k in 0..c {
                        let diff = frame.data[[i, j, k]] as f32 - mean[[i, j, k]];
                        variance[[i, j, k]] += diff * diff / n;
                    }
                }
            }
        }

        variance.mapv(|v: f32| v.sqrt())
    }

    /// Extrai energia temporal (soma das diferenças)
    pub fn temporal_energy(video: &Video) -> f32 {
        let mut energy = 0.0;
        for i in 1..video.num_frames() {
            let diff = Self::frame_difference(&video.frames[i - 1], &video.frames[i]);
            energy += diff.iter().map(|&v| (v as f32).abs()).sum::<f32>();
        }
        energy / (video.num_frames() - 1) as f32
    }

    /// Converte vídeo para escala de cinza
    pub fn to_grayscale(video: &Video) -> Video {
        let mut gray_video = Video::new(video.width, video.height, 1, video.fps);

        for frame in &video.frames {
            gray_video.add_frame(frame.to_grayscale());
        }

        gray_video
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_frame() {
        let frame = VideoFrame::zeros(10, 20, 3);
        assert_eq!(frame.height(), 10);
        assert_eq!(frame.width(), 20);
        assert_eq!(frame.channels(), 3);
    }

    #[test]
    fn test_video_creation() {
        let mut video = Video::new(100, 100, 3, 30.0);
        video.add_frame(Array3::zeros((100, 100, 3)));
        video.add_frame(Array3::zeros((100, 100, 3)));

        assert_eq!(video.num_frames(), 2);
        assert_eq!(video.duration(), 2.0 / 30.0);
    }

    #[test]
    fn test_video_processor_test_video() {
        let video = VideoProcessor::create_test_video(50, 50, 10, 30.0);
        assert_eq!(video.num_frames(), 10);
        assert_eq!(video.width, 50);
        assert_eq!(video.height, 50);
    }

    #[test]
    fn test_frame_difference() {
        let f1 = VideoFrame::new(Array3::from_elem((10, 10, 1), 100), 0.0, 0);
        let f2 = VideoFrame::new(Array3::from_elem((10, 10, 1), 150), 0.0, 1);

        let diff = VideoProcessor::frame_difference(&f1, &f2);
        assert_eq!(diff[[0, 0, 0]], -50);
    }

    #[test]
    fn test_temporal_mean() {
        let video = VideoProcessor::create_test_video(10, 10, 5, 30.0);
        let mean = VideoProcessor::temporal_mean(&video);

        // Mean should be between min and max intensities
        assert!(mean[[0, 0, 0]] > 0.0);
        assert!(mean[[0, 0, 0]] < 255.0);
    }

    #[test]
    fn test_to_grayscale() {
        let video = VideoProcessor::create_test_video(10, 10, 3, 30.0);
        let gray = VideoProcessor::to_grayscale(&video);

        assert_eq!(gray.channels, 1);
        assert_eq!(gray.num_frames(), 3);
    }
}
