extern crate sodiumoxide;
extern crate libc;

use errors::crypto::CryptoError;
use self::libc::c_int;
use self::sodiumoxide::crypto::box_;
use self::sodiumoxide::crypto::sign;
use self::sodiumoxide::randombytes;
use std::convert::AsMut;

pub struct ED25519 {}

impl ED25519 {
    pub fn new() -> ED25519 {
        ED25519 {}
    }

    pub fn create_key_pair(&self) -> (Vec<u8>, Vec<u8>) {
        let (public_key, private_key) = box_::gen_keypair();
        (public_key[..].to_vec(), private_key[..].to_vec())
    }

    pub fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Vec<u8> {
        box_::seal(
            doc,
            &box_::Nonce(ED25519::_clone_into_array(nonce)),
            &box_::PublicKey(ED25519::_clone_into_array(public_key)),
            &box_::SecretKey(ED25519::_clone_into_array(private_key))
        )
    }

    pub fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
        box_::open(
            doc,
            &box_::Nonce(ED25519::_clone_into_array(nonce)),
            &box_::PublicKey(ED25519::_clone_into_array(public_key)),
            &box_::SecretKey(ED25519::_clone_into_array(private_key))
        )
            .map_err(|_| CryptoError::InvalidStructure("Unable to decrypt data".to_string()))
    }

    pub fn gen_nonce(&self) -> Vec<u8> {
        box_::gen_nonce()[..].to_vec()
    }

    pub fn create_key_pair_for_signature(&self, seed: Option<&[u8]>) -> (Vec<u8>, Vec<u8>) {
        let (public_key, private_key) =
            sign::keypair_from_seed(
                &sign::Seed(
                    ED25519::_clone_into_array(
                        seed.unwrap_or(&randombytes::randombytes(32)[..])
                    )
                )
            );

        (public_key[..].to_vec(), private_key[..].to_vec())
    }

    pub fn sign(&self, private_key: &[u8], doc: &[u8]) -> Vec<u8> {
        let mut pr_key: [u8; 64] = [0; 64];
        pr_key.clone_from_slice(private_key);

        sign::sign(
            doc,
            &sign::SecretKey(pr_key)
        )
    }

    pub fn verify(&self, public_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sign::verify(
            doc,
            &sign::PublicKey(ED25519::_clone_into_array(public_key))
        )
            .map_err(|_| CryptoError::InvalidStructure("Unable to decrypt data".to_string()))
    }

    fn _clone_into_array<A, T>(slice: &[T]) -> A
        where A: Sized + Default + AsMut<[T]>, T: Clone
    {
        let mut a = Default::default();
        <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
        a
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_works() {
        let ed25519 = ED25519::new();
        let text = randombytes::randombytes(16);
        let nonce = ed25519.gen_nonce();

        let (alice_pk, alice_sk) = ed25519.create_key_pair();
        let (bob_pk, bob_sk) = ed25519.create_key_pair();

        let bob_encrypted_text = ed25519.encrypt(&bob_sk, &alice_pk, &text, &nonce);
        let bob_decrypt_result = ed25519.decrypt(&alice_sk, &bob_pk, &bob_encrypted_text, &nonce);
        assert!(bob_decrypt_result.is_ok());
        assert_eq!(text, bob_decrypt_result.unwrap());

        let alice_encrypted_text = ed25519.encrypt(&alice_sk, &bob_pk, &text, &nonce);
        let alice_decrypted_text = ed25519.decrypt(&bob_sk, &alice_pk, &alice_encrypted_text, &nonce);
        assert!(alice_decrypted_text.is_ok());
        assert_eq!(text, alice_decrypted_text.unwrap());
    }

    #[test]
    fn signin_verify_works() {
        let ed25519 = ED25519::new();
        let seed = randombytes::randombytes(32);
        let text = randombytes::randombytes(16);

        let (public_key, secret_key) = ed25519.create_key_pair_for_signature(Some(&seed));
        let alice_signed_text = ed25519.sign(&secret_key, &text);
        let verified_data = ed25519.verify(&public_key, &alice_signed_text);

        assert!(verified_data.is_ok());
        assert_eq!(text, verified_data.unwrap());
    }
}