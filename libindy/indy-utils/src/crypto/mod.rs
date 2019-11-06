#[macro_use]
pub mod sodium_type;

#[cfg(feature = "base64_rust_base64")]
#[path = "base64/rust_base64.rs"]
pub mod base64;

#[cfg(feature = "chacha20poly1305_ietf_sodium")]
#[path = "chacha20poly1305_ietf/sodium.rs"]
pub mod chacha20poly1305_ietf;

#[cfg(feature = "hash_openssl")]
#[path = "hash/openssl.rs"]
pub mod hash;

#[cfg(feature = "hmacsha256_sodium")]
#[path = "hmacsha256/sodium.rs"]
pub mod hmacsha256;

#[cfg(feature = "pwhash_argon2i13_sodium")]
#[path = "pwhash_argon2i13/sodium.rs"]
pub mod pwhash_argon2i13;
