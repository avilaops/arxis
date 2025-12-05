use avila_rand_simple::prelude::*;

#[test]
fn test_basic_usage() {
    let mut pcg = Pcg64::new(42);
    let v1 = pcg.next_u64();
    let v2 = pcg.next_u64();
    assert_ne!(v1, v2);
}

#[test]
fn test_range_generation() {
    let mut rng = Pcg64::new(12345);
    for _ in 0..100 {
        let v = gen_range_u64(&mut rng, 1, 100);
        assert!(v >= 1 && v < 100);
    }
}

#[test]
fn test_float_generation() {
    let mut rng = Xorshift128Plus::new(42);
    for _ in 0..100 {
        let f = rng.next_f64();
        assert!(f >= 0.0 && f < 1.0);
    }
}
