//! ECDSA - Elliptic Curve Digital Signature Algorithm
//!
//! Used by Bitcoin and Ethereum with secp256k1

use crate::bigint::U256;
use super::VerifyResult;

/// ECDSA signature (r, s)
#[derive(Debug, Clone, Copy)]
pub struct Signature {
    /// R component
    pub r: U256,
    /// S component
    pub s: U256,
}

/// ECDSA public key
#[derive(Debug, Clone, Copy)]
pub struct PublicKey {
    /// X coordinate
    pub x: U256,
    /// Y coordinate
    pub y: U256,
}

impl Signature {
    /// Verify ECDSA signature
    ///
    /// Algorithm:
    /// 1. Verify r, s in [1, n-1]
    /// 2. u1 = z * s^-1 mod n
    /// 3. u2 = r * s^-1 mod n
    /// 4. P = u1*G + u2*Q
    /// 5. Verify r == P.x mod n
    pub fn verify(&self, message_hash: &U256, public_key: &PublicKey) -> VerifyResult {
        use crate::bigint::BigInt;
        use crate::curves::{Point, EllipticCurve, secp256k1::Secp256k1};
        use super::SignatureError;
        use core::cmp::Ordering;

        // Verify r and s are in valid range [1, n-1]
        if self.r.is_zero() || self.s.is_zero() {
            return Err(SignatureError::InvalidFormat);
        }

        let n = Secp256k1::N;

        if matches!(self.r.cmp(&n), Ordering::Greater | Ordering::Equal) {
            return Err(SignatureError::InvalidFormat);
        }
        if matches!(self.s.cmp(&n), Ordering::Greater | Ordering::Equal) {
            return Err(SignatureError::InvalidFormat);
        }

        // Calculate s^-1 mod n
        let s_inv = match self.s.inv_mod(&n) {
            Some(inv) => inv,
            None => return Err(SignatureError::VerificationFailed),
        };

        // u1 = z * s^-1 mod n
        let u1 = message_hash.mul_mod(&s_inv, &n);

        // u2 = r * s^-1 mod n
        let u2 = self.r.mul_mod(&s_inv, &n);

        // P = u1*G + u2*Q
        let g = Secp256k1::generator();
        let q = Point {
            x: public_key.x,
            y: public_key.y,
            infinity: false,
        };

        let u1g = Secp256k1::scalar_mul(&u1, &g);
        let u2q = Secp256k1::scalar_mul(&u2, &q);
        let p = Secp256k1::add(&u1g, &u2q);

        if p.infinity {
            return Err(SignatureError::VerificationFailed);
        }

        // Verify r == p.x mod n
        let p_x_mod_n = if matches!(p.x.cmp(&n), Ordering::Less) {
            p.x
        } else {
            // Reduce p.x modulo n (simplified)
            let mut result = p.x;
            while matches!(result.cmp(&n), Ordering::Greater | Ordering::Equal) {
                result = result.sub(&n);
            }
            result
        };

        if p_x_mod_n == self.r {
            Ok(())
        } else {
            Err(SignatureError::VerificationFailed)
        }
    }
}
