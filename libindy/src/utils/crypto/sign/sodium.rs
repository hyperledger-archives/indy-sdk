extern crate sodiumoxide;
extern crate libc;

use errors::crypto::CryptoError;

use self::libc::c_int;
use self::sodiumoxide::crypto::sign;
use self::sodiumoxide::randombytes;

use utils::crypto::box_;

pub const SEEDBYTES: usize = sign::SEEDBYTES;
pub const PUBLICKEYBYTES: usize = sign::PUBLICKEYBYTES;
pub const SECRETKEYBYTES: usize = sign::SECRETKEYBYTES;
pub const SIGNATUREBYTES: usize = sign::SIGNATUREBYTES;

sodium_type!(Seed, sign::Seed, SEEDBYTES);
sodium_type!(PublicKey, sign::PublicKey, PUBLICKEYBYTES);
sodium_type!(SecretKey, sign::SecretKey, SECRETKEYBYTES);
sodium_type!(Signature, sign::Signature, SIGNATUREBYTES);

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

pub struct CryptoSign {}

impl CryptoSign {
    pub fn create_key_pair_for_signature(seed: Option<&Seed>) -> Result<(PublicKey, SecretKey), CryptoError> {
        let (public_key, secret_key) =
            sign::keypair_from_seed(
                &seed.unwrap_or(
                    &Seed::from_slice(&randombytes::randombytes(32)).unwrap()
                ).0
            );

        Ok((PublicKey(public_key), SecretKey(secret_key)))
    }

    pub fn sign(secret_key: &SecretKey, doc: &[u8]) -> Result<Signature, CryptoError> {
        Ok(Signature(
            sign::sign_detached(
                doc,
                &secret_key.0))
        )
    }

    pub fn verify(public_key: &PublicKey, doc: &[u8], signature: &Signature) -> Result<bool, CryptoError> {
        Ok(sign::verify_detached(
            &signature.0,
            doc,
            &public_key.0
        ))
    }

    pub fn sk_to_curve25519(sk: &SecretKey) -> Result<box_::SecretKey, CryptoError> {
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_sk_to_curve25519(&mut to, &(sk.0).0);
        }
        box_::SecretKey::from_slice(&to)
    }

    pub fn vk_to_curve25519(pk: &PublicKey) -> Result<box_::PublicKey, CryptoError> {
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_pk_to_curve25519(&mut to, &(pk.0).0);
        }
        box_::PublicKey::from_slice(&to)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::crypto::box_;

    #[test]
    fn signin_verify_works() {
        let seed = Seed::from_slice(&randombytes::randombytes(32)).unwrap();
        let text = randombytes::randombytes(16);

        let (public_key, secret_key) = CryptoSign::create_key_pair_for_signature(Some(&seed)).unwrap();
        let alice_signed_text = CryptoSign::sign(&secret_key, &text).unwrap();
        let verified = CryptoSign::verify(&public_key, &text, &alice_signed_text).unwrap();

        assert!(verified);
    }

    #[test]
    fn pk_to_curve25519_works() {
        let pk = vec!(236, 191, 114, 144, 108, 87, 211, 244, 148, 23, 20, 175, 122, 6, 159, 254, 85, 99, 145, 152, 178, 133, 230, 236, 192, 69, 35, 136, 141, 194, 243, 134);
        let pk = PublicKey::from_slice(&pk).unwrap();
        let pkc_test = CryptoSign::vk_to_curve25519(&pk).unwrap();
        let pkc_exp = vec!(8, 45, 124, 147, 248, 201, 112, 171, 11, 51, 29, 248, 34, 127, 197, 241, 60, 158, 84, 47, 4, 176, 238, 166, 110, 39, 207, 58, 127, 110, 76, 42);
        let pkc_exp = box_::PublicKey::from_slice(&pkc_exp).unwrap();
        assert_eq!(pkc_exp, pkc_test);
    }

    #[test]
    fn sk_to_curve25519_works() {
        let sk = vec!(78, 67, 205, 99, 150, 131, 75, 110, 56, 154, 76, 61, 27, 142, 36, 141, 44, 223, 122, 199, 14, 230, 12, 163, 4, 255, 94, 230, 21, 242, 97, 200, 236, 191, 114, 144, 108, 87, 211, 244, 148, 23, 20, 175, 122, 6, 159, 254, 85, 99, 145, 152, 178, 133, 230, 236, 192, 69, 35, 136, 141, 194, 243, 134);
        let sk = SecretKey::from_slice(&sk).unwrap();
        let skc_test = CryptoSign::sk_to_curve25519(&sk).unwrap();
        let skc_exp = vec!(144, 112, 64, 101, 69, 167, 61, 44, 220, 148, 58, 187, 108, 73, 11, 247, 130, 161, 158, 40, 100, 1, 40, 27, 76, 148, 209, 240, 195, 35, 153, 121);
        let skc_exp = box_::SecretKey::from_slice(&skc_exp).unwrap();
        assert_eq!(skc_exp, skc_test);
    }
}
