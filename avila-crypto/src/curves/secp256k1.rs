//! secp256k1 - Bitcoin's battle-tested curve
//!
//! Equation: y² = x³ + 7 (mod p)
//!
//! Chosen by Satoshi Nakamoto for mathematical transparency.
//! Constants are verifiable and simple (a=0, b=7).
//!
//! Security: 128-bit (equivalent to AES-128)
//! Used by: Bitcoin, Ethereum, most cryptocurrencies

use crate::bigint::U256;
use super::{Point, EllipticCurve};

/// secp256k1 curve parameters
pub struct Secp256k1;

impl Secp256k1 {
    /// Prime field modulus: 2^256 - 2^32 - 977
    pub const P: U256 = U256 {
        limbs: [
            0xFFFFFFFFFFFFFC2F,
            0xFFFFFFFFFFFFFFFE,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
        ],
    };

    /// Curve order (number of points)
    pub const N: U256 = U256 {
        limbs: [
            0xBFD25E8CD0364141,
            0xBAAEDCE6AF48A03B,
            0xFFFFFFFFFFFFFFFE,
            0xFFFFFFFFFFFFFFFF,
        ],
    };

    /// Generator point G
    pub const GX: U256 = U256 {
        limbs: [
            0x59F2815B16F81798,
            0x029BFCDB2DCE28D9,
            0x55A06295CE870B07,
            0x79BE667EF9DCBBAC,
        ],
    };

    pub const GY: U256 = U256 {
        limbs: [
            0x9C47D08FFB10D4B8,
            0xFD17B448A6855419,
            0x5DA4FBFC0E1108A8,
            0x483ADA7726A3C465,
        ],
    };

    /// Coefficient b in y² = x³ + b
    pub const B: U256 = U256 {
        limbs: [7, 0, 0, 0],
    };
    
    /// Modular subtraction: (a - b) mod p
    fn sub_mod(a: &U256, b: &U256, p: &U256) -> U256 {
        use crate::bigint::BigInt;
        use core::cmp::Ordering;
        
        match a.cmp(b) {
            Ordering::Greater | Ordering::Equal => a.sub(b),
            Ordering::Less => {
                // a < b, so compute p - (b - a)
                let diff = b.sub(a);
                p.sub(&diff)
            }
        }
    }
}

impl EllipticCurve for Secp256k1 {
    type Field = U256;
    type Scalar = U256;

    fn generator() -> Point<U256> {
        Point {
            x: Self::GX,
            y: Self::GY,
            infinity: false,
        }
    }

    fn add(p: &Point<U256>, q: &Point<U256>) -> Point<U256> {
        use crate::bigint::BigInt;

        if p.infinity {
            return *q;
        }
        if q.infinity {
            return *p;
        }

        // Point at infinity if P = -Q
        if p.x == q.x {
            if p.y == q.y {
                return Self::double(p);
            } else {
                return Point {
                    x: U256::ZERO,
                    y: U256::ZERO,
                    infinity: true,
                };
            }
        }

        // λ = (y2 - y1) / (x2 - x1) mod p
        // x3 = λ² - x1 - x2 mod p
        // y3 = λ(x1 - x3) - y1 mod p
        
        let y_diff = Self::sub_mod(&q.y, &p.y, &Self::P);
        let x_diff = Self::sub_mod(&q.x, &p.x, &Self::P);

        let x_diff_inv = match x_diff.inv_mod(&Self::P) {
            Some(inv) => inv,
            None => return Point { x: U256::ZERO, y: U256::ZERO, infinity: true },
        };

        let lambda = y_diff.mul_mod(&x_diff_inv, &Self::P);
        let lambda_sq = lambda.mul_mod(&lambda, &Self::P);
        
        let x3_temp = Self::sub_mod(&lambda_sq, &p.x, &Self::P);
        let x3 = Self::sub_mod(&x3_temp, &q.x, &Self::P);
        
        let x1_minus_x3 = Self::sub_mod(&p.x, &x3, &Self::P);
        let y3_temp = lambda.mul_mod(&x1_minus_x3, &Self::P);
        let y3 = Self::sub_mod(&y3_temp, &p.y, &Self::P);

        Point { x: x3, y: y3, infinity: false }
    }

    fn double(p: &Point<U256>) -> Point<U256> {
        use crate::bigint::BigInt;

        if p.infinity {
            return *p;
        }

        // λ = (3x² + a) / (2y) mod p
        // For secp256k1, a = 0, so λ = 3x² / 2y
        // x3 = λ² - 2x mod p
        // y3 = λ(x - x3) - y mod p

        let x_sq = p.x.mul_mod(&p.x, &Self::P);
        let three_x_sq = x_sq.add_mod(&x_sq, &Self::P).add_mod(&x_sq, &Self::P);
        
        let two_y = p.y.add_mod(&p.y, &Self::P);
        let two_y_inv = match two_y.inv_mod(&Self::P) {
            Some(inv) => inv,
            None => return Point { x: U256::ZERO, y: U256::ZERO, infinity: true },
        };

        let lambda = three_x_sq.mul_mod(&two_y_inv, &Self::P);
        let lambda_sq = lambda.mul_mod(&lambda, &Self::P);
        
        let two_x = p.x.add_mod(&p.x, &Self::P);
        let x3 = Self::sub_mod(&lambda_sq, &two_x, &Self::P);
        
        let x_minus_x3 = Self::sub_mod(&p.x, &x3, &Self::P);
        let y3_temp = lambda.mul_mod(&x_minus_x3, &Self::P);
        let y3 = Self::sub_mod(&y3_temp, &p.y, &Self::P);

        Point { x: x3, y: y3, infinity: false }
    }

    fn scalar_mul(k: &U256, p: &Point<U256>) -> Point<U256> {
        let mut result = Point {
            x: <U256 as crate::bigint::BigInt>::ZERO,
            y: <U256 as crate::bigint::BigInt>::ZERO,
            infinity: true,
        };
        let mut base = *p;
        let mut scalar = *k;

        // Double-and-add algorithm
        while !scalar.is_zero() {
            if scalar.limbs[0] & 1 == 1 {
                result = Self::add(&result, &base);
            }
            base = Self::double(&base);
            scalar = scalar.shr(1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        // Verify curve parameters are correctly defined
        assert!(!Secp256k1::P.is_zero());
        assert!(!Secp256k1::N.is_zero());
    }

    #[test]
    fn test_generator() {
        let g = Secp256k1::generator();
        assert!(!g.infinity);
        assert!(!g.x.is_zero());
        assert!(!g.y.is_zero());
    }
}
