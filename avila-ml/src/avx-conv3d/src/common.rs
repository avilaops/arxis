//! Estruturas e tipos comuns para processamento 3D

use serde::{Deserialize, Serialize};

/// Ponto 3D com coordenadas inteiras
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    /// Cria novo ponto
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Origem (0, 0, 0)
    pub fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    /// Distância euclidiana para outro ponto
    pub fn distance(&self, other: &Point3D) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        let dz = (self.z - other.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Distância de Manhattan
    pub fn manhattan_distance(&self, other: &Point3D) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

/// Tamanho 3D (largura × altura × profundidade)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl Size3D {
    /// Cria novo tamanho
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    /// Retorna volume total
    pub fn volume(&self) -> u64 {
        self.width as u64 * self.height as u64 * self.depth as u64
    }

    /// Retorna área da face XY
    pub fn area_xy(&self) -> u32 {
        self.width * self.height
    }

    /// Verifica se cabe dentro de outro tamanho
    pub fn fits_in(&self, other: &Size3D) -> bool {
        self.width <= other.width && self.height <= other.height && self.depth <= other.depth
    }
}

/// Bounding box 3D
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundingBox3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl BoundingBox3D {
    /// Cria nova bounding box
    pub fn new(x: i32, y: i32, z: i32, width: u32, height: u32, depth: u32) -> Self {
        Self {
            x,
            y,
            z,
            width,
            height,
            depth,
        }
    }

    /// Cria a partir de dois pontos
    pub fn from_points(p1: Point3D, p2: Point3D) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let z = p1.z.min(p2.z);
        let width = (p1.x - p2.x).abs() as u32;
        let height = (p1.y - p2.y).abs() as u32;
        let depth = (p1.z - p2.z).abs() as u32;
        Self::new(x, y, z, width, height, depth)
    }

    /// Retorna volume
    pub fn volume(&self) -> u64 {
        self.width as u64 * self.height as u64 * self.depth as u64
    }

    /// Verifica se contém um ponto
    pub fn contains(&self, point: &Point3D) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
            && point.z >= self.z
            && point.z < self.z + self.depth as i32
    }

    /// Verifica se intersecta outra bounding box
    pub fn intersects(&self, other: &BoundingBox3D) -> bool {
        self.x < other.x + other.width as i32
            && self.x + self.width as i32 > other.x
            && self.y < other.y + other.height as i32
            && self.y + self.height as i32 > other.y
            && self.z < other.z + other.depth as i32
            && self.z + self.depth as i32 > other.z
    }

    /// Retorna centro
    pub fn center(&self) -> Point3D {
        Point3D::new(
            self.x + (self.width / 2) as i32,
            self.y + (self.height / 2) as i32,
            self.z + (self.depth / 2) as i32,
        )
    }
}

/// Eixo 3D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis3D {
    X,
    Y,
    Z,
}

/// Plano 3D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plane3D {
    XY,
    XZ,
    YZ,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_distance() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(3, 4, 0);
        assert_eq!(p1.distance(&p2), 5.0);
        assert_eq!(p1.manhattan_distance(&p2), 7);
    }

    #[test]
    fn test_size3d() {
        let size = Size3D::new(10, 20, 30);
        assert_eq!(size.volume(), 6000);
        assert_eq!(size.area_xy(), 200);
    }

    #[test]
    fn test_bbox_contains() {
        let bbox = BoundingBox3D::new(0, 0, 0, 10, 10, 10);
        assert!(bbox.contains(&Point3D::new(5, 5, 5)));
        assert!(!bbox.contains(&Point3D::new(15, 15, 15)));
    }

    #[test]
    fn test_bbox_intersects() {
        let b1 = BoundingBox3D::new(0, 0, 0, 10, 10, 10);
        let b2 = BoundingBox3D::new(5, 5, 5, 10, 10, 10);
        let b3 = BoundingBox3D::new(20, 20, 20, 10, 10, 10);

        assert!(b1.intersects(&b2));
        assert!(!b1.intersects(&b3));
    }
}
