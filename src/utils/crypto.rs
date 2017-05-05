extern crate libc;

use self::libc::c_int;
pub struct Ed25519ToCurve25519 {}

extern {
    // TODO: fix hack:
    // this functions isn't included to sodiumoxide rust wrappers,
    // temporary local binding is used to call libsodium-sys function
    pub fn crypto_sign_ed25519_pk_to_curve25519(
        curve25519_pk: *mut [u8; 32],
        ed25519_pk: *const [u8; 32]) -> c_int;
    pub fn crypto_sign_ed25519_sk_to_curve25519(
        curve25519_sk: *mut [u8; 32],
        ed25519_sk: *const [u8; 64]) -> c_int;
}

impl Ed25519ToCurve25519 {
    pub fn crypto_sign_ed25519_sk_to_curve25519(sk: &Vec<u8>) -> Vec<u8> {
        let mut from: [u8; 64] = [0; 64];
        from.clone_from_slice(sk.as_slice());
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_sk_to_curve25519(&mut to, &from);
        }
        to.iter().cloned().collect()
    }

    pub fn crypto_sign_ed25519_pk_to_curve25519(pk: &Vec<u8>) -> Vec<u8> {
        let mut from: [u8; 32] = [0; 32];
        from.clone_from_slice(pk.as_slice());
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_pk_to_curve25519(&mut to, &from);
        }
        to.iter().cloned().collect()
    }
}
