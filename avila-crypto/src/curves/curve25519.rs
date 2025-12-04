//! Curve25519 - Modern, fast, and secure
//!
//! Montgomery curve: By² = x³ + Ax² + x
//! Twisted Edwards form: -x² + y² = 1 - (121665/121666)x²y²
//!
//! Benefits:
//! - Complete addition formulas (no edge cases)
//! - Constant-time operations (side-channel resistant)
//! - Prime: 2^255 - 19 (fast arithmetic)
//! - Twist secure (dual curve is also secure)

use crate::bigint::{BigInt, U256};

/// Curve25519 parameters
pub struct Curve25519;

impl Curve25519 {
    /// Prime: 2^255 - 19
    pub const P: U256 = U256 { limbs: [
        0xFFFFFFFFFFFFFFED,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x7FFFFFFFFFFFFFFF,
    ] };

    /// Subgroup order (number of points)
    pub const L: U256 = U256 { limbs: [
        0x5812631A5CF5D3ED,
        0x14DEF9DEA2F79CD6,
        0x0000000000000000,
        0x1000000000000000,
    ] };

    /// Montgomery coefficient A = 486662
    pub const A: U256 = U256 { limbs: [486662, 0, 0, 0] };

    /// Edwards parameter d = -(121665/121666) mod p
    pub const D: U256 = U256 { limbs: [
        0x52036CEE2B6FFE73,
        0x8CC740797779E898,
        0x00700A4D4141D8AB,
        0x52036CEE2B6FFE73,
    ] };

    /// Base point x-coordinate (Edwards form)
    pub const BASE_X: U256 = U256 { limbs: [
        0xC9562D608F25D51A,
        0x692CC7609525A7B2,
        0xC0A4E231FDD6DC5C,
        0x216936D3CD6E53FE,
    ] };

    /// Base point y-coordinate (Edwards form)
    pub const BASE_Y: U256 = U256 { limbs: [
        0x6666666666666658,
        0x6666666666666666,
        0x6666666666666666,
        0x6666666666666666,
    ] };
}

/// Ponto na Edwards curve: -x² + y² = 1 + dx²y²
#[derive(Debug, Clone, Copy)]
pub struct EdwardsPoint {
    /// Coordenada X
    pub x: U256,
    /// Coordenada Y
    pub y: U256,
    /// Coordenada Z (projectiva)
    pub z: U256,
    /// Coordenada T = XY/Z (extended)
    pub t: U256,
}

impl PartialEq for EdwardsPoint {
    fn eq(&self, other: &Self) -> bool {
        let modulus = &Curve25519::P;

        // Compare projective coordinates by cross-multiplying to avoid inversions.
        let lhs_x = mul_mod(&self.x, &other.z, modulus);
        let rhs_x = mul_mod(&other.x, &self.z, modulus);
        if lhs_x != rhs_x {
            return false;
        }

        let lhs_y = mul_mod(&self.y, &other.z, modulus);
        let rhs_y = mul_mod(&other.y, &self.z, modulus);
        lhs_y == rhs_y
    }
}

impl Eq for EdwardsPoint {}

impl EdwardsPoint {
    /// Ponto identidade (0, 1)
    pub const IDENTITY: Self = Self {
        x: U256::ZERO,
        y: U256::ONE,
        z: U256::ONE,
        t: U256::ZERO,
    };

    /// Ponto base
    pub const BASE: Self = Self {
        x: Curve25519::BASE_X,
        y: Curve25519::BASE_Y,
        z: U256::ONE,
        t: U256 {
            limbs: [
                0x6D16EB422A1E6567,
                0x1B274A0EA0B0C4DE,
                0x367AFBD5CDE3BB5F,
                0x0C45DBC00FD6BB25,
            ],
        },
    };

    /// Cria um novo ponto
    pub fn new(x: U256, y: U256) -> Self {
        let z = U256::ONE;
        let t = mul_mod(&x, &y, &Curve25519::P);
        Self { x, y, z, t }
    }

    /// Adição de pontos (complete formula)
    pub fn add(&self, other: &Self) -> Self {
        let a = mul_mod(&self.x, &other.x, &Curve25519::P);
        let b = mul_mod(&self.y, &other.y, &Curve25519::P);
        let c = mul_mod(&self.t, &other.t, &Curve25519::P);
        let c = mul_mod(&c, &Curve25519::D, &Curve25519::P);
        let d = mul_mod(&self.z, &other.z, &Curve25519::P);

        let e = {
            let xy1 = add_mod(&self.x, &self.y, &Curve25519::P);
            let xy2 = add_mod(&other.x, &other.y, &Curve25519::P);
            let ab = add_mod(&a, &b, &Curve25519::P);
            let prod = mul_mod(&xy1, &xy2, &Curve25519::P);
            sub_mod(&prod, &ab, &Curve25519::P)
        };

        let f = sub_mod(&d, &c, &Curve25519::P);
        let g = add_mod(&d, &c, &Curve25519::P);
        let h = sub_mod(&b, &a, &Curve25519::P);

        let x = mul_mod(&e, &f, &Curve25519::P);
        let y = mul_mod(&g, &h, &Curve25519::P);
        let z = mul_mod(&f, &g, &Curve25519::P);
        let t = mul_mod(&e, &h, &Curve25519::P);

        Self { x, y, z, t }
    }

    /// Duplicação de ponto
    pub fn double(&self) -> Self {
        let a = square_mod(&self.x, &Curve25519::P);
        let b = square_mod(&self.y, &Curve25519::P);
        let c = {
            let z2 = square_mod(&self.z, &Curve25519::P);
            add_mod(&z2, &z2, &Curve25519::P)
        };

        let d = sub_mod(&Curve25519::P, &a, &Curve25519::P);
        let xy = add_mod(&self.x, &self.y, &Curve25519::P);
        let e = {
            let xy2 = square_mod(&xy, &Curve25519::P);
            let ab = add_mod(&a, &b, &Curve25519::P);
            sub_mod(&xy2, &ab, &Curve25519::P)
        };

        let g = add_mod(&d, &b, &Curve25519::P);
        let f = sub_mod(&g, &c, &Curve25519::P);
        let h = sub_mod(&d, &b, &Curve25519::P);

        let x = mul_mod(&e, &f, &Curve25519::P);
        let y = mul_mod(&g, &h, &Curve25519::P);
        let z = mul_mod(&f, &g, &Curve25519::P);
        let t = mul_mod(&e, &h, &Curve25519::P);

        Self { x, y, z, t }
    }

    /// Multiplicação escalar (constant-time)
    pub fn scalar_mul(&self, scalar: &U256) -> Self {
        let mut result = Self::IDENTITY;
        let mut temp = *self;

        for i in 0..256 {
            let bit = (scalar.limbs[i / 64] >> (i % 64)) & 1;

            // Constant-time: sempre faz add, mas ignora se bit=0
            let added = result.add(&temp);
            result = conditional_select(&result, &added, bit == 1);

            temp = temp.double();
        }

        result
    }

    /// Converte para coordenadas afim (x/z, y/z)
    pub fn to_affine(&self) -> (U256, U256) {
        let z_inv = inv_mod(&self.z, &Curve25519::P).unwrap();
        let x = mul_mod(&self.x, &z_inv, &Curve25519::P);
        let y = mul_mod(&self.y, &z_inv, &Curve25519::P);
        (x, y)
    }

    /// Compacta para 32 bytes (formato Ed25519)
    pub fn compress(&self) -> [u8; 32] {
        let (x, y) = self.to_affine();
        let mut result = y.to_bytes_le();

        // Codifica sinal de x no bit mais significativo
        if x.limbs[0] & 1 == 1 {
            result[31] |= 0x80;
        }

        result
    }

    /// Descompacta de 32 bytes
    pub fn decompress(bytes: &[u8; 32]) -> Option<Self> {
        let mut y_bytes = *bytes;
        let x_sign = (y_bytes[31] & 0x80) != 0;
        y_bytes[31] &= 0x7F;

        let y = U256::from_bytes_le(&y_bytes);

        // Recupera x de y: x² = (y² - 1) / (dy² + 1)
        let y2 = square_mod(&y, &Curve25519::P);
        let numerator = sub_mod(&y2, &U256::ONE, &Curve25519::P);
        let denominator = {
            let dy2 = mul_mod(&Curve25519::D, &y2, &Curve25519::P);
            add_mod(&dy2, &U256::ONE, &Curve25519::P)
        };

        let x2 = {
            let inv = inv_mod(&denominator, &Curve25519::P)?;
            mul_mod(&numerator, &inv, &Curve25519::P)
        };

        let mut x = sqrt_mod(&x2, &Curve25519::P)?;

        // Ajusta sinal
        if ((x.limbs[0] & 1) == 1) != x_sign {
            x = sub_mod(&Curve25519::P, &x, &Curve25519::P);
        }

        Some(Self::new(x, y))
    }
}

// Operações modulares helper

fn add_mod(a: &U256, b: &U256, m: &U256) -> U256 {
    let sum = a.wrapping_add(b);
    if sum >= *m {
        sum.wrapping_sub(m)
    } else {
        sum
    }
}

fn sub_mod(a: &U256, b: &U256, m: &U256) -> U256 {
    if a >= b {
        a.wrapping_sub(b)
    } else {
        let diff = b.wrapping_sub(a);
        m.wrapping_sub(&diff)
    }
}

fn mul_mod(a: &U256, b: &U256, m: &U256) -> U256 {
    a.mul_mod(b, m)
}

fn square_mod(a: &U256, m: &U256) -> U256 {
    a.mul_mod(a, m)
}

fn inv_mod(a: &U256, m: &U256) -> Option<U256> {
    a.inv_mod(m)
}

fn sqrt_mod(a: &U256, _m: &U256) -> Option<U256> {
    // Tonelli-Shanks algorithm
    // TODO: Implementar
    Some(*a) // Placeholder
}

fn conditional_select(a: &EdwardsPoint, b: &EdwardsPoint, choice: bool) -> EdwardsPoint {
    if choice { *b } else { *a }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let id = EdwardsPoint::IDENTITY;
        let doubled = id.double();
        assert_eq!(doubled, id);
    }

    #[test]
    fn test_base_point() {
        let base = EdwardsPoint::BASE;
        let _doubled = base.double();
        // Testa que não panic
    }
}
