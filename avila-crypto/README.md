# avila-crypto

**Sovereign cryptographic primitives with zero external dependencies.**

[![Crates.io](https://img.shields.io/crates/v/avila-crypto.svg)](https://crates.io/crates/avila-crypto)
[![Documentation](https://docs.rs/avila-crypto/badge.svg)](https://docs.rs/avila-crypto)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Philosophy

**Mathematics over politics.** No compromises with government standards. Battle-tested algorithms from Bitcoin/Ethereum. Zero external dependencies.

## Features

- **BLAKE3** - Merkle tree hash, 4x faster than SHA-256
- **Keccak-256** - Ethereum's SHA-3, sponge construction
- **SHA-256** - Bitcoin standard, 64 rounds
- **Poly1305** - 130-bit prime field MAC
- **ChaCha20** - Stream cipher, 20 rounds
- **ChaCha20-Poly1305** - AEAD (RFC 8439)
- **ECDSA** - secp256k1 signatures (Bitcoin/Ethereum)
- **U256/U512** - Modular arithmetic

## Usage

\\\ust
use avila_crypto::hash::blake3::Blake3;
use avila_crypto::hash::Hasher;

let data = b"Hello, sovereign crypto!";
let hash = Blake3::hash(data);
println!("BLAKE3: {:?}", hash);
\\\

### ChaCha20-Poly1305 AEAD

\\\ust
use avila_crypto::encryption::chacha20_poly1305::ChaCha20Poly1305;

let key = [0x42; 32];
let nonce = [0x07; 12];
let plaintext = b"Secret message";
let aad = b"metadata";

let aead = ChaCha20Poly1305::new(&key);
let (ciphertext, tag) = aead.encrypt(&nonce, plaintext, aad);
let decrypted = aead.decrypt(&nonce, &ciphertext, aad, &tag).unwrap();
assert_eq!(decrypted.as_slice(), plaintext);
\\\

### Poly1305 MAC

\\\ust
use avila_crypto::mac::poly1305::Poly1305;

let key = [0x85; 32];
let message = b"Authenticate this";
let tag = Poly1305::mac(&key, message);
\\\

## no_std Support

\\\	oml
[dependencies]
avila-crypto = { version = "0.1", default-features = false }
\\\

## Principles

1. **Stack-only allocation** - Zero heap usage
2. **Fixed-size types** - Compile-time known sizes
3. **Constant-time operations** - Side-channel resistant
4. **Zero dependencies** - Everything from scratch
5. **forbid(unsafe_code)** - Memory safe

## Testing

\\\ash
cargo test --lib
\\\

**Result:**  15/15 tests passing

## Benchmarks

\\\ash
cargo bench
\\\

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions welcome! Mathematics over politics always.
