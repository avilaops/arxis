use avila_crypto::bigint::{BigInt, U256};
use avila_crypto::curves::secp256k1::Secp256k1;

#[test]
fn test_sub_mod_direct() {
    let p = Secp256k1::P;
    
    // From debug: ?(x-x3) = 0x6ff80973306ad3901ce4e0ee7d6a03455ee32f266bd9cf8b140a67cfa7af9875
    let y3_temp = U256::from_bytes_be(&[
        0x6f, 0xf8, 0x09, 0x73, 0x30, 0x6a, 0xd3, 0x90,
        0x1c, 0xe4, 0xe0, 0xee, 0x7d, 0x6a, 0x03, 0x45,
        0x5e, 0xe3, 0x2f, 0x26, 0x6b, 0x9c, 0xf8, 0x8b,
        0x14, 0x0a, 0x67, 0xcf, 0xa7, 0xaf, 0x98, 0x75,
    ]);
    
    // G.y = 0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8
    let gy = Secp256k1::GY;
    
    println!("y3_temp: {:016x}{:016x}{:016x}{:016x}", y3_temp.limbs[3], y3_temp.limbs[2], y3_temp.limbs[1], y3_temp.limbs[0]);
    println!("gy:      {:016x}{:016x}{:016x}{:016x}", gy.limbs[3], gy.limbs[2], gy.limbs[1], gy.limbs[0]);
    println!("p:       {:016x}{:016x}{:016x}{:016x}", p.limbs[3], p.limbs[2], p.limbs[1], p.limbs[0]);
    
    // Manual: y3_temp > gy, so just subtract
    use core::cmp::Ordering;
    println!("\nComparison: {:?}", y3_temp.cmp(&gy));
    
    let y3_manual = if y3_temp >= gy {
        y3_temp.sub(&gy)
    } else {
        p.sub(&gy.sub(&y3_temp))
    };
    
    println!("\nManual y3:   {:016x}{:016x}{:016x}{:016x}", y3_manual.limbs[3], y3_manual.limbs[2], y3_manual.limbs[1], y3_manual.limbs[0]);
    println!("Expected y3: 1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a");
}
