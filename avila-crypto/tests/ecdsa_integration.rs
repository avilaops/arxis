use avila_crypto::bigint::{BigInt, U256};
use avila_crypto::curves::{EllipticCurve, secp256k1::Secp256k1};
use avila_crypto::signatures::ecdsa::{Signature, PublicKey};

#[test]
fn test_ecdsa_sign_verify_cycle() {
    // 1. Gerar chave privada
    let private_key = U256::from_bytes_be(&[
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0x30, 0x39,
    ]);
    
    // 2. Derivar chave pública: Q = d  G
    let public_key_point = Secp256k1::scalar_mul(&private_key, &Secp256k1::generator());
    let public_key = PublicKey {
        x: public_key_point.x,
        y: public_key_point.y,
    };
    
    // 3. Hash da mensagem
    let message_hash = U256::from_bytes_be(&[
        0x56, 0x40, 0x59, 0x56, 0x2b, 0x91, 0xbf, 0x41,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    
    // 4. Nonce determinístico
    let k = U256::from_bytes_be(&[
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0xD4, 0x31,
    ]);
    
    // 5. Calcular R = k  G
    let r_point = Secp256k1::scalar_mul(&k, &Secp256k1::generator());
    let r = r_point.x;
    
    // 6. Calcular s = k¹  (hash + r  d) mod n
    let curve_order = Secp256k1::N;
    let k_inv = k.inv_mod(&curve_order).expect("k deve ter inverso");
    let r_priv = r.mul_mod(&private_key, &curve_order);
    let hash_plus_r_priv = message_hash.add_mod(&r_priv, &curve_order);
    let s = k_inv.mul_mod(&hash_plus_r_priv, &curve_order);
    
    // 7. Criar assinatura
    let signature = Signature { r, s };
    
    println!("\n=== ECDSA Test ===");
    println!("Private key: {:016x}", private_key.limbs[0]);
    println!("Public key X: {:016x}", public_key.x.limbs[0]);
    println!("Message hash: {:016x}", message_hash.limbs[0]);
    println!("Signature r: {:016x}", signature.r.limbs[0]);
    println!("Signature s: {:016x}", signature.s.limbs[0]);
    
    // 8. Verificar
    let result = signature.verify(&message_hash, &public_key);
    println!("Verification: {:?}", result);
    
    assert!(result.is_ok(), "ECDSA signature verification should succeed");
}
