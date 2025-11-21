//! Estruturas e tipos comuns para processamento 4D

use serde::{Deserialize, Serialize};

/// Ponto 4D (t, z, y, x)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point4D {
    pub t: i32,  // tempo
    pub z: i32,  // profundidade
    pub y: i32,  // altura
    pub x: i32,  // largura
}

impl Point4D {
    /// Cria novo ponto 4D
    pub fn new(t: i32, z: i32, y: i32, x: i32) -> Self {
        Self { t, z, y, x }
    }

    /// Origem (0, 0, 0, 0)
    pub fn origin() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Distância euclidiana 4D
    pub fn distance(&self, other: &Point4D) -> f32 {
        let dt = (self.t - other.t) as f32;
        let dz = (self.z - other.z) as f32;
        let dy = (self.y - other.y) as f32;
        let dx = (self.x - other.x) as f32;
        (dt * dt + dz * dz + dy * dy + dx * dx).sqrt()
    }

    /// Distância de Manhattan 4D
    pub fn manhattan_distance(&self, other: &Point4D) -> i32 {
        (self.t - other.t).abs()
            + (self.z - other.z).abs()
            + (self.y - other.y).abs()
            + (self.x - other.x).abs()
    }
}

/// Tamanho 4D (tempo × profundidade × altura × largura)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size4D {
    pub time: u32,
    pub depth: u32,
    pub height: u32,
    pub width: u32,
}

impl Size4D {
    /// Cria novo tamanho 4D
    pub fn new(time: u32, depth: u32, height: u32, width: u32) -> Self {
        Self {
            time,
            depth,
            height,
            width,
        }
    }

    /// Retorna hipervolume total (t × d × h × w)
    pub fn hypervolume(&self) -> u64 {
        self.time as u64 * self.depth as u64 * self.height as u64 * self.width as u64
    }

    /// Retorna volume espacial (d × h × w)
    pub fn spatial_volume(&self) -> u64 {
        self.depth as u64 * self.height as u64 * self.width as u64
    }
}

/// Bounding box 4D
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundingBox4D {
    pub t: i32,
    pub z: i32,
    pub y: i32,
    pub x: i32,
    pub time: u32,
    pub depth: u32,
    pub height: u32,
    pub width: u32,
}

impl BoundingBox4D {
    /// Cria nova bounding box 4D
    pub fn new(
        t: i32,
        z: i32,
        y: i32,
        x: i32,
        time: u32,
        depth: u32,
        height: u32,
        width: u32,
    ) -> Self {
        Self {
            t,
            z,
            y,
            x,
            time,
            depth,
            height,
            width,
        }
    }

    /// Retorna hipervolume
    pub fn hypervolume(&self) -> u64 {
        self.time as u64 * self.depth as u64 * self.height as u64 * self.width as u64
    }

    /// Verifica se contém um ponto
    pub fn contains(&self, point: &Point4D) -> bool {
        point.t >= self.t
            && point.t < self.t + self.time as i32
            && point.z >= self.z
            && point.z < self.z + self.depth as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
            && point.x >= self.x
            && point.x < self.x + self.width as i32
    }

    /// Verifica se intersecta outra bounding box
    pub fn intersects(&self, other: &BoundingBox4D) -> bool {
        self.t < other.t + other.time as i32
            && self.t + self.time as i32 > other.t
            && self.z < other.z + other.depth as i32
            && self.z + self.depth as i32 > other.z
            && self.y < other.y + other.height as i32
            && self.y + self.height as i32 > other.y
            && self.x < other.x + other.width as i32
            && self.x + self.width as i32 > other.x
    }

    /// Retorna centro
    pub fn center(&self) -> Point4D {
        Point4D::new(
            self.t + (self.time / 2) as i32,
            self.z + (self.depth / 2) as i32,
            self.y + (self.height / 2) as i32,
            self.x + (self.width / 2) as i32,
        )
    }
}

/// Eixo 4D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis4D {
    T, // tempo
    Z, // profundidade
    Y, // altura
    X, // largura
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point4d_distance() {
        let p1 = Point4D::new(0, 0, 0, 0);
        let p2 = Point4D::new(1, 1, 1, 1);
        assert_eq!(p1.distance(&p2), 2.0);
        assert_eq!(p1.manhattan_distance(&p2), 4);
    }

    #[test]
    fn test_size4d() {
        let size = Size4D::new(10, 20, 30, 40);
        assert_eq!(size.hypervolume(), 240000);
        assert_eq!(size.spatial_volume(), 24000);
    }

    #[test]
    fn test_bbox_contains() {
        let bbox = BoundingBox4D::new(0, 0, 0, 0, 10, 10, 10, 10);
        assert!(bbox.contains(&Point4D::new(5, 5, 5, 5)));
        assert!(!bbox.contains(&Point4D::new(15, 15, 15, 15)));
    }

    #[test]
    fn test_bbox_intersects() {
        let b1 = BoundingBox4D::new(0, 0, 0, 0, 10, 10, 10, 10);
        let b2 = BoundingBox4D::new(5, 5, 5, 5, 10, 10, 10, 10);
        let b3 = BoundingBox4D::new(20, 20, 20, 20, 10, 10, 10, 10);

        assert!(b1.intersects(&b2));
        assert!(!b1.intersects(&b3));
    }
}
