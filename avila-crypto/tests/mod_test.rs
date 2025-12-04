use avila_crypto::bigint::{BigInt, U256};

#[test]
fn test_mul_mod() {
    let p = U256 {
        limbs: [
            0xFFFFFFFFFFFFFC2F,
            0xFFFFFFFFFFFFFFFE,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
        ],
    };
    
    // Test: 2 * 2 mod p = 4
    let two = U256 { limbs: [2, 0, 0, 0] };
    let four = two.mul_mod(&two, &p);
    assert_eq!(four.limbs[0], 4);
    assert_eq!(four.limbs[1], 0);
    
    // Test case from double: lambda = 0x325eaf6900a1737afbad3cc023148c68bf9bec20c36bd95370203cdde5037052
    let lambda = U256::from_bytes_be(&[
        0x32, 0x5e, 0xaf, 0x69, 0x00, 0xa1, 0x73, 0x7a,
        0xfb, 0xad, 0x3c, 0xc0, 0x23, 0x14, 0x8c, 0x68,
        0xbf, 0x9b, 0xec, 0x20, 0xc3, 0x6b, 0xd9, 0x53,
        0x70, 0x20, 0x3c, 0xdd, 0xe5, 0x03, 0x70, 0x52,
    ]);
    
    let lambda_sq = lambda.mul_mod(&lambda, &p);
    
    println!("lambda:   {:016x}{:016x}{:016x}{:016x}", lambda.limbs[3], lambda.limbs[2], lambda.limbs[1], lambda.limbs[0]);
    println!("lambda²:  {:016x}{:016x}{:016x}{:016x}", lambda_sq.limbs[3], lambda_sq.limbs[2], lambda_sq.limbs[1], lambda_sq.limbs[0]);
    println!("Expected: 0ff86c00e655744645933409a2c4f7af721bb7ecbc812629a5c5f16480e60c06");
    
    let expected = U256::from_bytes_be(&[
        0x0f, 0xf8, 0x6c, 0x00, 0xe6, 0x55, 0x74, 0x46,
        0x45, 0x93, 0x34, 0x09, 0xa2, 0xc4, 0xf7, 0xaf,
        0x72, 0x1b, 0xb7, 0xec, 0xbc, 0x81, 0x26, 0x29,
        0xa5, 0xc5, 0xf1, 0x64, 0x80, 0xe6, 0x0c, 0x06,
    ]);
    
    assert_eq!(lambda_sq, expected, "lambda² mismatch");
}
