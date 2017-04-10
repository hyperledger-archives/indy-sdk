extern crate rust_base58;
extern crate sodiumoxide;

use errors::crypto::CryptoError;
use self::rust_base58::{ToBase58, FromBase58};
use self::sodiumoxide::crypto::box_;
use self::sodiumoxide::crypto::secretbox;
use self::sodiumoxide::crypto::sign;
use self::sodiumoxide::randombytes;
use std::convert::AsMut;


pub fn sodium_symmetric_create_key() -> Vec<u8> {
    secretbox::gen_key()[..].to_vec()
}

pub fn sodium_symmetric_create_nonce() -> Vec<u8> {
    secretbox::gen_nonce()[..].to_vec()
}

pub fn sodium_symmetric_encrypt(key: &[u8], nonce: &[u8], doc: &[u8]) -> Vec<u8> {
    secretbox::seal(
        doc,
        &secretbox::Nonce(clone_into_array(nonce)),
        &secretbox::Key(clone_into_array(key))
    )
}

pub fn sodium_symmetric_decrypt(key: &[u8], nonce: &[u8], doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
    secretbox::open(
        doc,
        &secretbox::Nonce(clone_into_array(nonce)),
        &secretbox::Key(clone_into_array(key))
    )
        .map_err(|_| CryptoError::InvalidData("Unable to decrypt data".to_string()))
}


pub fn sodium_box_create_key_pair() -> (Vec<u8>, Vec<u8>) {
    let (public_key, private_key) = box_::gen_keypair();
    (public_key[..].to_vec(), private_key[..].to_vec())
}

pub fn sodium_encrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Vec<u8> {
    box_::seal(
        doc,
        &box_::Nonce(clone_into_array(nonce)),
        &box_::PublicKey(clone_into_array(public_key)),
        &box_::SecretKey(clone_into_array(private_key))
    )
}

pub fn sodium_decrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
    box_::open(
        doc,
        &box_::Nonce(clone_into_array(nonce)),
        &box_::PublicKey(clone_into_array(public_key)),
        &box_::SecretKey(clone_into_array(private_key))
    )
        .map_err(|_| CryptoError::InvalidData("Unable to decrypt data".to_string()))
}

pub fn get_nonce() -> Vec<u8> {
    box_::gen_nonce()[..].to_vec()
}

pub fn sodium_create_key_pair_for_signature(seed: Option<&[u8]>) -> (Vec<u8>, Vec<u8>) {
    let (public_key, private_key) =
        sign::keypair_from_seed(
            &sign::Seed(
                clone_into_array(
                    seed.unwrap_or(&randombytes::randombytes(32)[..])
                )
            )
        );

    (public_key[..].to_vec(), private_key[..].to_vec())
}

pub fn sodium_sign(private_key: &[u8], doc: &[u8]) -> Vec<u8> {
    let mut pr_key: [u8; 64] = [0; 64];
    pr_key.clone_from_slice(private_key);

    sign::sign(
        doc,
        &sign::SecretKey(pr_key)
    )
}

pub fn sodium_verify(public_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
    sign::verify(
        doc,
        &sign::PublicKey(clone_into_array(public_key))
    )
        .map_err(|_| CryptoError::InvalidData("Unable to decrypt data".to_string()))
}

pub fn base58_encode(doc: &[u8]) -> String {
    doc.to_base58()
}

pub fn base58_decode(doc: &String) -> Result<Vec<u8>, CryptoError> {
    doc.from_base58()
        .map_err(|err| CryptoError::InvalidData(format!("{}", err)))
}

fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>, T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypto_service_can_encode_decode_string() {
        let data = &[1, 2, 3];

        let encode_result = base58_encode(&data[..]);
        assert_eq!("Ldp", &encode_result, "Success encrypt data");

        let decrypted_data = base58_decode(&encode_result);
        assert!(decrypted_data.is_ok(), "Success decrypt data");
        assert_eq!(data, &decrypted_data.unwrap()[..], "Get correct data");
    }

    #[test]
    fn crypto_service_encode_decode_test() {
        let (alice_pk, alice_sk) = sodium_box_create_key_pair();
        let (bob_pk, bob_sk) = sodium_box_create_key_pair();

        let text = randombytes::randombytes(16);
        let nonce = get_nonce();

        let bob_encrypted_text = sodium_encrypt(&bob_sk, &alice_pk, &text[..], &nonce);
        let bob_decrypt_result = sodium_decrypt(&alice_sk, &bob_pk, &bob_encrypted_text, &nonce);
        assert!(bob_decrypt_result.is_ok());
        assert_eq!(text, bob_decrypt_result.unwrap());

        let alice_encrypted_text = sodium_encrypt(&alice_sk, &bob_pk, &text[..], &nonce);
        let alice_decrypted_text = sodium_decrypt(&bob_sk, &alice_pk, &alice_encrypted_text, &nonce);
        assert!(alice_decrypted_text.is_ok());
        assert_eq!(text, alice_decrypted_text.unwrap());
    }

    #[test]
    fn crypto_service_signin_verify_test() {
        let seed = randombytes::randombytes(32);

        let (public_key, secret_key) = sodium_create_key_pair_for_signature(Some(&seed[..]));

        let text = randombytes::randombytes(16);

        let alice_signed_text = sodium_sign(&secret_key, &text[..]);

        let verified_data = sodium_verify(&public_key, &alice_signed_text);
        assert!(verified_data.is_ok());

        assert_eq!(text, verified_data.unwrap());
    }
}