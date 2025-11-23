//! Estruturas e tipos comuns para processamento 2D

use serde::{Deserialize, Serialize};

/// Ponto 2D com coordenadas inteiras
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    /// Cria novo ponto
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Distância euclidiana para outro ponto
    pub fn distance(&self, other: &Point2D) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Distância de Manhattan
    pub fn manhattan_distance(&self, other: &Point2D) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Tamanho 2D (largura x altura)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size2D {
    pub width: u32,
    pub height: u32,
}

impl Size2D {
    /// Cria novo tamanho
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Retorna área total
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Retorna razão de aspecto
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    /// Verifica se cabe dentro de outro tamanho
    pub fn fits_in(&self, other: &Size2D) -> bool {
        self.width <= other.width && self.height <= other.height
    }
}

/// Retângulo delimitador
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    /// Cria novo retângulo
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Cria retângulo a partir de dois pontos
    pub fn from_points(p1: Point2D, p2: Point2D) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).abs() as u32;
        let height = (p1.y - p2.y).abs() as u32;
        Self::new(x, y, width, height)
    }

    /// Retorna área do retângulo
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Verifica se contém um ponto
    pub fn contains(&self, point: &Point2D) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }

    /// Verifica se intersecta outro retângulo
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width as i32
            && self.x + self.width as i32 > other.x
            && self.y < other.y + other.height as i32
            && self.y + self.height as i32 > other.y
    }

    /// Retorna centro do retângulo
    pub fn center(&self) -> Point2D {
        Point2D::new(
            self.x + (self.width / 2) as i32,
            self.y + (self.height / 2) as i32,
        )
    }
}

/// Padding para operações 2D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Padding {
    /// Sem padding
    None,
    /// Padding constante (replicate border)
    Constant(u8),
    /// Padding simétrico
    Replicate,
    /// Padding reflexivo
    Reflect,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(3, 4);
        assert_eq!(p1.distance(&p2), 5.0);
        assert_eq!(p1.manhattan_distance(&p2), 7);
    }

    #[test]
    fn test_size() {
        let size = Size2D::new(100, 50);
        assert_eq!(size.area(), 5000);
        assert_eq!(size.aspect_ratio(), 2.0);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10, 10, 20, 20);
        assert!(rect.contains(&Point2D::new(15, 15)));
        assert!(!rect.contains(&Point2D::new(5, 5)));
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0, 0, 10, 10);
        let r2 = Rect::new(5, 5, 10, 10);
        let r3 = Rect::new(20, 20, 10, 10);

        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }
}
