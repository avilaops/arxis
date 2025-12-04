use avila_crypto::bigint::{BigInt, U256};
use avila_crypto::curves::{Point, EllipticCurve, secp256k1::Secp256k1};
use avila_crypto::signatures::ecdsa::{Signature, PublicKey};

#[test]
fn test_ecdsa_debug() {
    // Valores de teste
    let private_key = U256::from_bytes_be(&[
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0x30, 0x39,
    ]);
    
    let k = U256::from_bytes_be(&[
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0xD4, 0x31,
    ]);
    
    let message_hash = U256::from_bytes_be(&[
        0x56, 0x40, 0x59, 0x56, 0x2b, 0x91, 0xbf, 0x41,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    
    let n = Secp256k1::N;
    
    // === ASSINATURA ===
    println!("\n=== ASSINATURA ===");
    println!("d (private): {:016x} {:016x} {:016x} {:016x}", 
        private_key.limbs[3], private_key.limbs[2], private_key.limbs[1], private_key.limbs[0]);
    println!("k (nonce):   {:016x} {:016x} {:016x} {:016x}", 
        k.limbs[3], k.limbs[2], k.limbs[1], k.limbs[0]);
    println!("z (hash):    {:016x} {:016x} {:016x} {:016x}", 
        message_hash.limbs[3], message_hash.limbs[2], message_hash.limbs[1], message_hash.limbs[0]);
    
    // R = k  G
    let r_point = Secp256k1::scalar_mul(&k, &Secp256k1::generator());
    let r = r_point.x;
    println!("\nR = k  G:");
    println!("r (R.x):     {:016x} {:016x} {:016x} {:016x}", 
        r.limbs[3], r.limbs[2], r.limbs[1], r.limbs[0]);
    
    // Q = d  G
    let public_key_point = Secp256k1::scalar_mul(&private_key, &Secp256k1::generator());
    let public_key = PublicKey {
        x: public_key_point.x,
        y: public_key_point.y,
    };
    println!("\nQ = d  G:");
    println!("Q.x:         {:016x} {:016x} {:016x} {:016x}", 
        public_key.x.limbs[3], public_key.x.limbs[2], public_key.x.limbs[1], public_key.x.limbs[0]);
    println!("Q.y:         {:016x} {:016x} {:016x} {:016x}", 
        public_key.y.limbs[3], public_key.y.limbs[2], public_key.y.limbs[1], public_key.y.limbs[0]);
    
    // s = k¹  (z + r  d) mod n
    let k_inv = k.inv_mod(&n).unwrap();
    println!("\nk¹ mod n:   {:016x} {:016x} {:016x} {:016x}", 
        k_inv.limbs[3], k_inv.limbs[2], k_inv.limbs[1], k_inv.limbs[0]);
    
    let r_d = r.mul_mod(&private_key, &n);
    println!("r  d:       {:016x} {:016x} {:016x} {:016x}", 
        r_d.limbs[3], r_d.limbs[2], r_d.limbs[1], r_d.limbs[0]);
    
    let z_plus_rd = message_hash.add_mod(&r_d, &n);
    println!("z + rd:     {:016x} {:016x} {:016x} {:016x}", 
        z_plus_rd.limbs[3], z_plus_rd.limbs[2], z_plus_rd.limbs[1], z_plus_rd.limbs[0]);
    
    let s = k_inv.mul_mod(&z_plus_rd, &n);
    println!("s = k¹...: {:016x} {:016x} {:016x} {:016x}", 
        s.limbs[3], s.limbs[2], s.limbs[1], s.limbs[0]);
    
    let signature = Signature { r, s };
    
    // === VERIFICAÇÃO ===
    println!("\n=== VERIFICAÇÃO ===");
    
    // s¹ mod n
    let s_inv = s.inv_mod(&n).unwrap();
    println!("s¹ mod n:   {:016x} {:016x} {:016x} {:016x}", 
        s_inv.limbs[3], s_inv.limbs[2], s_inv.limbs[1], s_inv.limbs[0]);
    
    // u1 = z  s¹ mod n
    let u1 = message_hash.mul_mod(&s_inv, &n);
    println!("u1 = zs¹:  {:016x} {:016x} {:016x} {:016x}", 
        u1.limbs[3], u1.limbs[2], u1.limbs[1], u1.limbs[0]);
    
    // u2 = r  s¹ mod n
    let u2 = r.mul_mod(&s_inv, &n);
    println!("u2 = rs¹:  {:016x} {:016x} {:016x} {:016x}", 
        u2.limbs[3], u2.limbs[2], u2.limbs[1], u2.limbs[0]);
    
    // P = u1G + u2Q
    let g = Secp256k1::generator();
    let q = Point {
        x: public_key.x,
        y: public_key.y,
        infinity: false,
    };
    
    let u1g = Secp256k1::scalar_mul(&u1, &g);
    println!("\nu1G:");
    println!("  x:         {:016x} {:016x} {:016x} {:016x}", 
        u1g.x.limbs[3], u1g.x.limbs[2], u1g.x.limbs[1], u1g.x.limbs[0]);
    println!("  y:         {:016x} {:016x} {:016x} {:016x}", 
        u1g.y.limbs[3], u1g.y.limbs[2], u1g.y.limbs[1], u1g.y.limbs[0]);
    
    let u2q = Secp256k1::scalar_mul(&u2, &q);
    println!("\nu2Q:");
    println!("  x:         {:016x} {:016x} {:016x} {:016x}", 
        u2q.x.limbs[3], u2q.x.limbs[2], u2q.x.limbs[1], u2q.x.limbs[0]);
    println!("  y:         {:016x} {:016x} {:016x} {:016x}", 
        u2q.y.limbs[3], u2q.y.limbs[2], u2q.y.limbs[1], u2q.y.limbs[0]);
    
    let p = Secp256k1::add(&u1g, &u2q);
    println!("\nP = u1G + u2Q:");
    println!("  x:         {:016x} {:016x} {:016x} {:016x}", 
        p.x.limbs[3], p.x.limbs[2], p.x.limbs[1], p.x.limbs[0]);
    println!("  y:         {:016x} {:016x} {:016x} {:016x}", 
        p.y.limbs[3], p.y.limbs[2], p.y.limbs[1], p.y.limbs[0]);
    
    println!("\nComparação:");
    println!("r (esperado):  {:016x} {:016x} {:016x} {:016x}", 
        r.limbs[3], r.limbs[2], r.limbs[1], r.limbs[0]);
    println!("P.x (obtido):  {:016x} {:016x} {:016x} {:016x}", 
        p.x.limbs[3], p.x.limbs[2], p.x.limbs[1], p.x.limbs[0]);
    
    let result = signature.verify(&message_hash, &public_key);
    println!("\nResultado: {:?}", result);
    
    // Verificação matemática: u1G + u2Q deveria ser igual a R
    // Isso só acontece se: u1 = zs¹ e u2 = rs¹
    // E a identidade: (zs¹)G + (rs¹)Q = kG
    // Simplificando: s¹(zG + rQ) = kG
    // Se Q = dG: s¹(zG + rdG) = kG
    // s¹G(z + rd) = kG
    // s¹(z + rd) = k
    // s = k¹(z + rd) 
    
    assert!(result.is_ok(), "ECDSA verification failed");
}
