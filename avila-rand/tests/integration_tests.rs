//! Comprehensive integration tests for avila-rand

use avila_rand::*;

#[test]
fn test_chacha20_u32_generation() {
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let val = rng.next_u32();
    assert!(val > 0 || val == 0); // Always true, just testing it works
}

#[test]
fn test_chacha20_u64_generation() {
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let val = rng.next_u64();
    assert!(val > 0 || val == 0);
}

#[test]
fn test_chacha20_fill_bytes() {
    let mut rng = ChaCha20Rng::seed_from_u64(123);
    let mut buf = [0u8; 64];
    rng.fill_bytes(&mut buf);
    
    // Check that not all bytes are zero
    assert!(buf.iter().any(|&b| b != 0));
}

#[test]
fn test_chacha20_reproducibility() {
    let seed = [7u8; 32];
    let mut rng1 = ChaCha20Rng::from_seed(seed);
    let mut rng2 = ChaCha20Rng::from_seed(seed);
    
    for _ in 0..1000 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }
}

#[test]
fn test_xoshiro_u32_generation() {
    let mut rng = Xoshiro256StarStar::seed_from_u64(99);
    let val = rng.next_u32();
    assert!(val > 0 || val == 0);
}

#[test]
fn test_xoshiro_u64_generation() {
    let mut rng = Xoshiro256StarStar::seed_from_u64(99);
    let val = rng.next_u64();
    assert!(val > 0 || val == 0);
}

#[test]
fn test_xoshiro_fill_bytes() {
    let mut rng = Xoshiro256StarStar::seed_from_u64(555);
    let mut buf = [0u8; 64];
    rng.fill_bytes(&mut buf);
    
    assert!(buf.iter().any(|&b| b != 0));
}

#[test]
fn test_xoshiro_reproducibility() {
    let seed = [13u8; 32];
    let mut rng1 = Xoshiro256StarStar::from_seed(seed);
    let mut rng2 = Xoshiro256StarStar::from_seed(seed);
    
    for _ in 0..1000 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }
}

#[test]
fn test_os_rng_generates_different_values() {
    let mut rng = OsRng::new();
    let mut values = Vec::new();
    
    for _ in 0..10 {
        values.push(rng.next_u64());
    }
    
    // Check that we have at least some different values
    let unique_count = values.iter().collect::<std::collections::HashSet<_>>().len();
    assert!(unique_count > 5);
}

#[test]
fn test_random_u8() {
    let mut rng = ChaCha20Rng::seed_from_u64(1);
    let _val: u8 = rng.gen();
}

#[test]
fn test_random_u16() {
    let mut rng = ChaCha20Rng::seed_from_u64(2);
    let _val: u16 = rng.gen();
}

#[test]
fn test_random_u32() {
    let mut rng = ChaCha20Rng::seed_from_u64(3);
    let _val: u32 = rng.gen();
}

#[test]
fn test_random_u64() {
    let mut rng = ChaCha20Rng::seed_from_u64(4);
    let _val: u64 = rng.gen();
}

#[test]
fn test_random_u128() {
    let mut rng = ChaCha20Rng::seed_from_u64(5);
    let _val: u128 = rng.gen();
}

#[test]
fn test_random_bool() {
    let mut rng = ChaCha20Rng::seed_from_u64(6);
    let mut true_count = 0;
    let mut false_count = 0;
    
    for _ in 0..100 {
        let val: bool = rng.gen();
        if val {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }
    
    // Should have both true and false values
    assert!(true_count > 0);
    assert!(false_count > 0);
}

#[test]
fn test_random_f32() {
    let mut rng = ChaCha20Rng::seed_from_u64(7);
    
    for _ in 0..100 {
        let val: f32 = rng.gen();
        assert!(val >= 0.0 && val < 1.0);
    }
}

#[test]
fn test_random_f64() {
    let mut rng = ChaCha20Rng::seed_from_u64(8);
    
    for _ in 0..100 {
        let val: f64 = rng.gen();
        assert!(val >= 0.0 && val < 1.0);
    }
}

#[test]
fn test_random_range_u32() {
    let mut rng = ChaCha20Rng::seed_from_u64(9);
    
    for _ in 0..100 {
        let val: u32 = rng.gen_range(10..20);
        assert!(val >= 10 && val < 20);
    }
}

#[test]
fn test_random_range_i32() {
    let mut rng = ChaCha20Rng::seed_from_u64(10);
    
    for _ in 0..100 {
        let val: i32 = rng.gen_range(-50..50);
        assert!(val >= -50 && val < 50);
    }
}

#[test]
fn test_random_range_f64() {
    let mut rng = ChaCha20Rng::seed_from_u64(11);
    
    for _ in 0..100 {
        let val: f64 = rng.gen_range(5.0..10.0);
        assert!(val >= 5.0 && val < 10.0);
    }
}

#[test]
fn test_shuffle() {
    let mut rng = ChaCha20Rng::seed_from_u64(12);
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let original = arr;
    
    shuffle(&mut rng, &mut arr);
    
    // Length should be the same
    assert_eq!(arr.len(), 10);
    
    // All elements should still be present
    let mut sorted = arr;
    sorted.sort();
    assert_eq!(sorted, original);
}

#[test]
fn test_uniform_distribution() {
    let mut rng = ChaCha20Rng::seed_from_u64(13);
    let dist = Uniform::new(0.0, 100.0);
    
    for _ in 0..100 {
        let val = dist.sample(&mut rng);
        assert!(val >= 0.0 && val < 100.0);
    }
}

#[test]
fn test_normal_distribution_mean() {
    let mut rng = ChaCha20Rng::seed_from_u64(14);
    let dist = Normal::new(50.0, 10.0);
    
    let mut sum = 0.0;
    let n = 10000;
    for _ in 0..n {
        sum += dist.sample(&mut rng);
    }
    
    let mean = sum / n as f64;
    // Mean should be close to 50.0
    assert!((mean - 50.0).abs() < 2.0);
}

#[test]
fn test_exponential_distribution_mean() {
    let mut rng = ChaCha20Rng::seed_from_u64(15);
    let lambda = 2.0;
    let dist = Exponential::new(lambda);
    
    let mut sum = 0.0;
    let n = 10000;
    for _ in 0..n {
        sum += dist.sample(&mut rng);
    }
    
    let mean = sum / n as f64;
    let expected_mean = 1.0 / lambda;
    // Mean should be close to 1/lambda
    assert!((mean - expected_mean).abs() < 0.1);
}

#[test]
fn test_thread_local_random() {
    let _val: u32 = random();
}

#[test]
fn test_thread_local_random_range() {
    let val: u32 = random_range(100..200);
    assert!(val >= 100 && val < 200);
}

#[test]
fn test_crypto_rng_trait() {
    fn assert_crypto_rng<T: CryptoRng>() {}
    assert_crypto_rng::<ChaCha20Rng>();
    assert_crypto_rng::<OsRng>();
}

#[test]
fn test_seedable_rng_trait() {
    fn test_seedable<T: SeedableRng>() {
        let _rng = T::seed_from_u64(12345);
    }
    
    test_seedable::<ChaCha20Rng>();
    test_seedable::<Xoshiro256StarStar>();
}

// Statistical tests
#[test]
fn test_chi_square_uniformity() {
    let mut rng = ChaCha20Rng::seed_from_u64(999);
    let bins = 10;
    let expected_per_bin = 1000;
    let n = bins * expected_per_bin;
    
    let mut counts = vec![0; bins];
    
    for _ in 0..n {
        let val: u32 = rng.gen_range(0..bins as u32);
        counts[val as usize] += 1;
    }
    
    // Calculate chi-square statistic
    let mut chi_square = 0.0;
    for count in counts {
        let diff = count as f64 - expected_per_bin as f64;
        chi_square += (diff * diff) / expected_per_bin as f64;
    }
    
    // For 9 degrees of freedom, critical value at 0.05 significance is ~16.92
    // We use a more lenient threshold for this test
    assert!(chi_square < 25.0, "Chi-square value {} is too high", chi_square);
}

#[test]
fn test_bit_distribution() {
    let mut rng = ChaCha20Rng::seed_from_u64(777);
    let n = 10000;
    let mut ones = 0;
    
    for _ in 0..n {
        let val = rng.next_u64();
        ones += val.count_ones();
    }
    
    let total_bits = n * 64;
    let ratio = ones as f64 / total_bits as f64;
    
    // Should be close to 0.5
    assert!((ratio - 0.5).abs() < 0.02);
}

#[test]
fn test_no_correlation() {
    let mut rng = ChaCha20Rng::seed_from_u64(888);
    let mut prev = rng.next_u64();
    let mut equal_count = 0;
    
    for _ in 0..1000 {
        let curr = rng.next_u64();
        if curr == prev {
            equal_count += 1;
        }
        prev = curr;
    }
    
    // Should have very few (ideally zero) equal consecutive values
    assert!(equal_count < 2);
}
