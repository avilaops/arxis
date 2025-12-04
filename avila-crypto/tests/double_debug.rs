use avila_crypto::bigint::{BigInt, U256};
use avila_crypto::curves::{Point, EllipticCurve, secp256k1::Secp256k1};

#[test]
fn test_double_debug() {
    let g = Secp256k1::generator();
    let p = Secp256k1::P;
    
    println!("G:");
    println!("  x: {:016x}{:016x}{:016x}{:016x}", g.x.limbs[3], g.x.limbs[2], g.x.limbs[1], g.x.limbs[0]);
    println!("  y: {:016x}{:016x}{:016x}{:016x}", g.y.limbs[3], g.y.limbs[2], g.y.limbs[1], g.y.limbs[0]);
    
    println!("\np = {:016x}{:016x}{:016x}{:016x}", p.limbs[3], p.limbs[2], p.limbs[1], p.limbs[0]);
    
    // Manual calculation
    let x_sq = g.x.mul_mod(&g.x, &p);
    println!("\nx²: {:016x}{:016x}{:016x}{:016x}", x_sq.limbs[3], x_sq.limbs[2], x_sq.limbs[1], x_sq.limbs[0]);
    
    let three_x_sq = x_sq.add_mod(&x_sq, &p).add_mod(&x_sq, &p);
    println!("3x²: {:016x}{:016x}{:016x}{:016x}", three_x_sq.limbs[3], three_x_sq.limbs[2], three_x_sq.limbs[1], three_x_sq.limbs[0]);
    
    let two_y = g.y.add_mod(&g.y, &p);
    println!("2y: {:016x}{:016x}{:016x}{:016x}", two_y.limbs[3], two_y.limbs[2], two_y.limbs[1], two_y.limbs[0]);
    
    let two_y_inv = two_y.inv_mod(&p).unwrap();
    println!("(2y)¹: {:016x}{:016x}{:016x}{:016x}", two_y_inv.limbs[3], two_y_inv.limbs[2], two_y_inv.limbs[1], two_y_inv.limbs[0]);
    
    let lambda = three_x_sq.mul_mod(&two_y_inv, &p);
    println!("\nλ = 3x²/(2y): {:016x}{:016x}{:016x}{:016x}", lambda.limbs[3], lambda.limbs[2], lambda.limbs[1], lambda.limbs[0]);
    
    let lambda_sq = lambda.mul_mod(&lambda, &p);
    println!("λ²: {:016x}{:016x}{:016x}{:016x}", lambda_sq.limbs[3], lambda_sq.limbs[2], lambda_sq.limbs[1], lambda_sq.limbs[0]);
    
    let two_x = g.x.add_mod(&g.x, &p);
    println!("2x: {:016x}{:016x}{:016x}{:016x}", two_x.limbs[3], two_x.limbs[2], two_x.limbs[1], two_x.limbs[0]);
    
    // x3 = λ² - 2x mod p
    let x3 = if lambda_sq >= two_x {
        lambda_sq.sub(&two_x)
    } else {
        p.sub(&two_x.sub(&lambda_sq))
    };
    println!("x3 = λ² - 2x: {:016x}{:016x}{:016x}{:016x}", x3.limbs[3], x3.limbs[2], x3.limbs[1], x3.limbs[0]);
    
    let x_minus_x3 = if g.x >= x3 {
        g.x.sub(&x3)
    } else {
        p.sub(&x3.sub(&g.x))
    };
    println!("x - x3: {:016x}{:016x}{:016x}{:016x}", x_minus_x3.limbs[3], x_minus_x3.limbs[2], x_minus_x3.limbs[1], x_minus_x3.limbs[0]);
    
    let y3_temp = lambda.mul_mod(&x_minus_x3, &p);
    println!("λ(x-x3): {:016x}{:016x}{:016x}{:016x}", y3_temp.limbs[3], y3_temp.limbs[2], y3_temp.limbs[1], y3_temp.limbs[0]);
    
    let y3 = if y3_temp >= g.y {
        y3_temp.sub(&g.y)
    } else {
        p.sub(&g.y.sub(&y3_temp))
    };
    println!("y3 = λ(x-x3) - y: {:016x}{:016x}{:016x}{:016x}", y3.limbs[3], y3.limbs[2], y3.limbs[1], y3.limbs[0]);
    
    println!("\nResult from double():");
    let two_g = Secp256k1::double(&g);
    println!("  x: {:016x}{:016x}{:016x}{:016x}", two_g.x.limbs[3], two_g.x.limbs[2], two_g.x.limbs[1], two_g.x.limbs[0]);
    println!("  y: {:016x}{:016x}{:016x}{:016x}", two_g.y.limbs[3], two_g.y.limbs[2], two_g.y.limbs[1], two_g.y.limbs[0]);
    
    println!("\nExpected:");
    println!("  x: c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5");
    println!("  y: 1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a");
}
