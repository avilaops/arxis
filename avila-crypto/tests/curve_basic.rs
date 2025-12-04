use avila_crypto::bigint::{BigInt, U256, U512};
use avila_crypto::curves::{Point, EllipticCurve, secp256k1::Secp256k1};

#[test]
fn test_add_basic() {
    let g = Secp256k1::generator();
    let two_g = Secp256k1::double(&g);
    let g_plus_g = Secp256k1::add(&g, &g);

    println!("G:");
    println!("  x: {:016x} {:016x} {:016x} {:016x}", g.x.limbs[3], g.x.limbs[2], g.x.limbs[1], g.x.limbs[0]);
    println!("  y: {:016x} {:016x} {:016x} {:016x}", g.y.limbs[3], g.y.limbs[2], g.y.limbs[1], g.y.limbs[0]);

    println!("\n2G (via double):");
    println!("  x: {:016x} {:016x} {:016x} {:016x}", two_g.x.limbs[3], two_g.x.limbs[2], two_g.x.limbs[1], two_g.x.limbs[0]);
    println!("  y: {:016x} {:016x} {:016x} {:016x}", two_g.y.limbs[3], two_g.y.limbs[2], two_g.y.limbs[1], two_g.y.limbs[0]);

    println!("\nG+G (via add):");
    println!("  x: {:016x} {:016x} {:016x} {:016x}", g_plus_g.x.limbs[3], g_plus_g.x.limbs[2], g_plus_g.x.limbs[1], g_plus_g.x.limbs[0]);
    println!("  y: {:016x} {:016x} {:016x} {:016x}", g_plus_g.y.limbs[3], g_plus_g.y.limbs[2], g_plus_g.y.limbs[1], g_plus_g.y.limbs[0]);

    assert_eq!(two_g.x, g_plus_g.x, "G+G should equal 2G (x coordinate)");
    assert_eq!(two_g.y, g_plus_g.y, "G+G should equal 2G (y coordinate)");
}

#[test]
fn test_known_values() {
    // Test vectors from Bitcoin
    // 2G should be:
    // x = 0xc6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5
    // y = 0x1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a

    let g = Secp256k1::generator();
    let two_g = Secp256k1::double(&g);

    let expected_x = U256::from_bytes_be(&[
        0xc6, 0x04, 0x7f, 0x94, 0x41, 0xed, 0x7d, 0x6d,
        0x30, 0x45, 0x40, 0x6e, 0x95, 0xc0, 0x7c, 0xd8,
        0x5c, 0x77, 0x8e, 0x4b, 0x8c, 0xef, 0x3c, 0xa7,
        0xab, 0xac, 0x09, 0xb9, 0x5c, 0x70, 0x9e, 0xe5,
    ]);

    let expected_y = U256::from_bytes_be(&[
        0x1a, 0xe1, 0x68, 0xfe, 0xa6, 0x3d, 0xc3, 0x39,
        0xa3, 0xc5, 0x84, 0x19, 0x46, 0x6c, 0xea, 0xee,
        0xf7, 0xf6, 0x32, 0x65, 0x32, 0x66, 0xd0, 0xe1,
        0x23, 0x64, 0x31, 0xa9, 0x50, 0xcf, 0xe5, 0x2a,
    ]);

    println!("Expected 2G:");
    println!("  x: {:016x} {:016x} {:016x} {:016x}", expected_x.limbs[3], expected_x.limbs[2], expected_x.limbs[1], expected_x.limbs[0]);
    println!("  y: {:016x} {:016x} {:016x} {:016x}", expected_y.limbs[3], expected_y.limbs[2], expected_y.limbs[1], expected_y.limbs[0]);

    println!("\nGot 2G:");
    println!("  x: {:016x} {:016x} {:016x} {:016x}", two_g.x.limbs[3], two_g.x.limbs[2], two_g.x.limbs[1], two_g.x.limbs[0]);
    println!("  y: {:016x} {:016x} {:016x} {:016x}", two_g.y.limbs[3], two_g.y.limbs[2], two_g.y.limbs[1], two_g.y.limbs[0]);

    let p = Secp256k1::P;
    let two_y = g.y.add_mod(&g.y, &p);
    let two_y_mod = two_y.mul_mod(&U256::ONE, &p);
    let fermat_exp = p.sub(&U256 { limbs: [2, 0, 0, 0] });
    let two_y_inv = two_y.inv_mod(&p).unwrap();
    let check = two_y.mul_mod(&two_y_inv, &p);
    println!("2y: {:016x} {:016x} {:016x} {:016x}", two_y.limbs[3], two_y.limbs[2], two_y.limbs[1], two_y.limbs[0]);
    println!("2y mod p via mul: {:016x} {:016x} {:016x} {:016x}", two_y_mod.limbs[3], two_y_mod.limbs[2], two_y_mod.limbs[1], two_y_mod.limbs[0]);
    println!("Fermat exponent p-2: {:016x} {:016x} {:016x} {:016x}", fermat_exp.limbs[3], fermat_exp.limbs[2], fermat_exp.limbs[1], fermat_exp.limbs[0]);
    let two_y_sq = two_y.mul_mod(&two_y, &p);
    println!("(2y)^2 mod p: {:016x} {:016x} {:016x} {:016x}", two_y_sq.limbs[3], two_y_sq.limbs[2], two_y_sq.limbs[1], two_y_sq.limbs[0]);
    println!("cmp(2y, p): {:?}", two_y.cmp(&p));
    let debug_inv = pow_mod_debug(two_y, fermat_exp, p);
    println!(
        "debug pow_mod result: {:016x} {:016x} {:016x} {:016x}",
        debug_inv.limbs[3], debug_inv.limbs[2], debug_inv.limbs[1], debug_inv.limbs[0]
    );
    use avila_crypto::bigint::U512;
    let simple = U512 { limbs: [1, 0, 0, 0, 0, 0, 0, 0] };
    let reduced_simple = simple.reduce(&p);
    println!(
        "U512(1) mod p: {:016x} {:016x} {:016x} {:016x}",
        reduced_simple.limbs[3], reduced_simple.limbs[2], reduced_simple.limbs[1], reduced_simple.limbs[0]
    );

    let three = U256 { limbs: [3, 0, 0, 0] };
    let exp_13 = U256 { limbs: [13, 0, 0, 0] };
    let mod_23 = U256 { limbs: [23, 0, 0, 0] };
    let pow_result = three.pow_mod(&exp_13, &mod_23);
    println!("3^13 mod 23 = {:016x} {:016x} {:016x} {:016x}", pow_result.limbs[3], pow_result.limbs[2], pow_result.limbs[1], pow_result.limbs[0]);
    println!("(2y)^-1: {:016x} {:016x} {:016x} {:016x}", two_y_inv.limbs[3], two_y_inv.limbs[2], two_y_inv.limbs[1], two_y_inv.limbs[0]);
    let mul_by_mod = two_y.mul_mod(&p, &p);
    println!(
        "(2y) * p mod p = {:016x} {:016x} {:016x} {:016x}",
        mul_by_mod.limbs[3], mul_by_mod.limbs[2], mul_by_mod.limbs[1], mul_by_mod.limbs[0]
    );
    let check_rev = two_y_inv.mul_mod(&two_y, &p);
    println!(
        "(2y)^-1 * (2y) mod p = {:016x} {:016x} {:016x} {:016x}",
        check_rev.limbs[3], check_rev.limbs[2], check_rev.limbs[1], check_rev.limbs[0]
    );

    let wide_product = mul_wide_reference(&two_y, &two_y_inv);
    let reduced_product = U512 { limbs: wide_product }.reduce(&p);
    println!(
        "reduce(mul_wide_reference(2y, inv)) = {:016x} {:016x} {:016x} {:016x}",
        reduced_product.limbs[3], reduced_product.limbs[2], reduced_product.limbs[1], reduced_product.limbs[0]
    );

    let modulus_small = U256 { limbs: [97, 0, 0, 0] };
    let value_small = U512 {
        limbs: [
            0x0123_4567_89ab_cdef,
            0x0fed_cba9_8765_4321,
            0, 0, 0, 0, 0, 0,
        ],
    };
    let reduced_small = value_small.reduce(&modulus_small);
    let value_small_u128 = ((value_small.limbs[1] as u128) << 64) | (value_small.limbs[0] as u128);
    let expected_small = (value_small_u128 % 97u128) as u64;
    println!(
        "small reduce = {:016x} {:016x} {:016x} {:016x}, expected = {:016x}",
        reduced_small.limbs[3], reduced_small.limbs[2], reduced_small.limbs[1], reduced_small.limbs[0], expected_small
    );

    println!(
        "\n2y * (2y)^-1 mod p = {:016x} {:016x} {:016x} {:016x}",
        check.limbs[3], check.limbs[2], check.limbs[1], check.limbs[0]
    );

    assert_eq!(two_g.x, expected_x, "2G x coordinate mismatch");
    assert_eq!(two_g.y, expected_y, "2G y coordinate mismatch");
}

fn pow_mod_debug(mut base: U256, mut exp: U256, modulus: U256) -> U256 {
    let mut result = U256::ONE;
    let mut iteration = 0u32;

    while !exp.is_zero() {
        if exp.limbs[0] & 1 == 1 {
            result = result.mul_mod(&base, &modulus);
            if iteration < 10 {
                println!(
                    "debug iter {} (bit=1): result={:016x} {:016x} {:016x} {:016x}",
                    iteration,
                    result.limbs[3], result.limbs[2], result.limbs[1], result.limbs[0]
                );
            }
        }

        let raw_square = base.mul(&base);
        let wide_square = mul_wide_reference(&base, &base);
        let reduced_square = U512 { limbs: wide_square }.reduce(&modulus);
        if iteration < 10 {
            println!(
                "debug iter {}: raw_square={:016x} {:016x} {:016x} {:016x}",
                iteration,
                raw_square.limbs[3], raw_square.limbs[2], raw_square.limbs[1], raw_square.limbs[0]
            );
            println!(
                "debug iter {}: wide_square limbs = {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x}",
                iteration,
                wide_square[7], wide_square[6], wide_square[5], wide_square[4],
                wide_square[3], wide_square[2], wide_square[1], wide_square[0]
            );
        }

        base = reduced_square;
        if iteration < 10 {
            println!(
                "debug iter {}: base={:016x} {:016x} {:016x} {:016x}",
                iteration,
                base.limbs[3], base.limbs[2], base.limbs[1], base.limbs[0]
            );
        }

        exp = exp.shr(1);
        iteration += 1;
    }

    result
}

fn mul_wide_reference(a: &U256, b: &U256) -> [u64; 8] {
    let mut result = [0u128; 8];

    for i in 0..4 {
        let ai = a.limbs[i] as u128;
        let mut carry = 0u128;

        for j in 0..4 {
            let idx = i + j;
            let bj = b.limbs[j] as u128;
            let temp = result[idx]
                + ai * bj
                + carry;
            result[idx] = temp & 0xFFFF_FFFF_FFFF_FFFF;
            carry = temp >> 64;
        }

        let mut idx = i + 4;
        while carry > 0 {
            let temp = result[idx] + carry;
            result[idx] = temp & 0xFFFF_FFFF_FFFF_FFFF;
            carry = temp >> 64;
            idx += 1;
        }
    }

    let mut out = [0u64; 8];
    for (i, limb) in result.iter().enumerate() {
        out[i] = *limb as u64;
    }

    out
}
