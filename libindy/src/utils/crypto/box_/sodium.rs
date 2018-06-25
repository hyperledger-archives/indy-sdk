extern crate sodiumoxide;
extern crate libc;

use errors::common::CommonError;

use self::libc::c_int;
use self::sodiumoxide::crypto::box_;
use self::sodiumoxide::crypto::sign;
use self::sodiumoxide::randombytes;
use utils::byte_array::_clone_into_array;

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

pub struct CryptoBox {}

impl CryptoBox {
    pub fn encrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        if nonce.len() != 24 {
            return Err(CommonError::InvalidStructure("Invalid nonce".to_string()))
        }

        Ok(box_::seal(
            doc,
            &box_::Nonce(_clone_into_array(nonce)),
            &box_::PublicKey(_clone_into_array(public_key)),
            &box_::SecretKey(_clone_into_array(private_key))
        ))
    }

    pub fn decrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        if nonce.len() != 24 {
            return Err(CommonError::InvalidStructure("Invalid nonce".to_string()))
        }

        box_::open(
            doc,
            &box_::Nonce(_clone_into_array(nonce)),
            &box_::PublicKey(_clone_into_array(public_key)),
            &box_::SecretKey(_clone_into_array(private_key))
        )
            .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
    }

    pub fn gen_nonce() -> Vec<u8> {
        box_::gen_nonce()[..].to_vec()
    }

    pub fn create_key_pair_for_signature(seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CommonError> {
        if seed.is_some() && seed.unwrap().len() != 32 {
            return Err(CommonError::InvalidStructure("Invalid seed".to_string()));
        }

        let (public_key, private_key) =
            sign::keypair_from_seed(
                &sign::Seed(
                    _clone_into_array(
                        seed.unwrap_or(&randombytes::randombytes(32)[..])
                    )
                )
            );

        Ok((public_key[..].to_vec(), private_key[..].to_vec()))
    }

    pub fn sign(private_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        if private_key.len() != 64 {
            return Err(CommonError::InvalidStructure("Invalid sign key".to_string()));
        }

        let mut pr_key: [u8; 64] = [0; 64];
        pr_key.clone_from_slice(private_key);

        Ok(sign::sign_detached(
            doc,
            &sign::SecretKey(pr_key)
        )[..].to_vec())
    }

    pub fn verify(public_key: &[u8], doc: &[u8], sign: &[u8]) -> Result<bool, CommonError> {
        if sign.len() != 64 {
            return Err(CommonError::InvalidStructure("Invalid signature".to_string()));
        }

        if public_key.len() != 32 {
            return Err(CommonError::InvalidStructure("Invalid verkey".to_string()));
        }

        let mut signature: [u8; 64] = [0; 64];
        signature.clone_from_slice(sign);

        Ok(sign::verify_detached(
            &sign::Signature(signature),
            doc,
            &sign::PublicKey(_clone_into_array(public_key))
        ))
    }

    pub fn sk_to_curve25519(sk: &[u8]) -> Result<Vec<u8>, CommonError> {
        if sk.len() != 64 {
            return Err(CommonError::InvalidStructure("Invalid signkey".to_string()));
        }

        let mut from: [u8; 64] = [0; 64];
        from.clone_from_slice(sk);
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_sk_to_curve25519(&mut to, &from);
        }
        Ok(to.iter().cloned().collect())
    }

    pub fn vk_to_curve25519(pk: &[u8]) -> Result<Vec<u8>, CommonError> {
        if pk.len() != 32 {
            return Err(CommonError::InvalidStructure("Invalid verkey".to_string()));
        }

        let mut from: [u8; 32] = [0; 32];
        from.clone_from_slice(pk);
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_pk_to_curve25519(&mut to, &from);
        }
        Ok(to.iter().cloned().collect())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_works() {
        let text = randombytes::randombytes(16);
        let nonce = CryptoBox::gen_nonce();
        let seed = randombytes::randombytes(32);

        let (alice_ver_key, alice_sign_key) = CryptoBox::create_key_pair_for_signature(Some(&seed)).unwrap();
        let alice_pk = CryptoBox::vk_to_curve25519(&alice_ver_key).unwrap();
        let alice_sk = CryptoBox::sk_to_curve25519(&alice_sign_key).unwrap();

        let (bob_ver_key, bob_sign_key) = CryptoBox::create_key_pair_for_signature(Some(&seed)).unwrap();
        let bob_pk = CryptoBox::vk_to_curve25519(&bob_ver_key).unwrap();
        let bob_sk = CryptoBox::sk_to_curve25519(&bob_sign_key).unwrap();

        let bob_encrypted_text = CryptoBox::encrypt(&bob_sk, &alice_pk, &text, &nonce).unwrap();
        let bob_decrypt_result = CryptoBox::decrypt(&alice_sk, &bob_pk, &bob_encrypted_text, &nonce);
        assert!(bob_decrypt_result.is_ok());
        assert_eq!(text, bob_decrypt_result.unwrap());

        let alice_encrypted_text = CryptoBox::encrypt(&alice_sk, &bob_pk, &text, &nonce).unwrap();
        let alice_decrypted_text = CryptoBox::decrypt(&bob_sk, &alice_pk, &alice_encrypted_text, &nonce);
        assert!(alice_decrypted_text.is_ok());
        assert_eq!(text, alice_decrypted_text.unwrap());
    }

    #[test]
    fn signin_verify_works() {
        let seed = randombytes::randombytes(32);
        let text = randombytes::randombytes(16);

        let (public_key, secret_key) = CryptoBox::create_key_pair_for_signature(Some(&seed)).unwrap();
        let alice_signed_text = CryptoBox::sign(&secret_key, &text).unwrap();
        let verified = CryptoBox::verify(&public_key, &text, &alice_signed_text).unwrap();

        assert!(verified);
    }

    #[test]
    fn pk_to_curve25519_works() {
        let pk = vec!(236, 191, 114, 144, 108, 87, 211, 244, 148, 23, 20, 175, 122, 6, 159, 254, 85, 99, 145, 152, 178, 133, 230, 236, 192, 69, 35, 136, 141, 194, 243, 134);
        let pkc_test = CryptoBox::vk_to_curve25519(&pk).unwrap();
        let pkc_exp = vec!(8, 45, 124, 147, 248, 201, 112, 171, 11, 51, 29, 248, 34, 127, 197, 241, 60, 158, 84, 47, 4, 176, 238, 166, 110, 39, 207, 58, 127, 110, 76, 42);
        assert_eq!(pkc_exp, pkc_test);
    }

    #[test]
    fn sk_to_curve25519_works() {
        let sk = vec!(78, 67, 205, 99, 150, 131, 75, 110, 56, 154, 76, 61, 27, 142, 36, 141, 44, 223, 122, 199, 14, 230, 12, 163, 4, 255, 94, 230, 21, 242, 97, 200, 236, 191, 114, 144, 108, 87, 211, 244, 148, 23, 20, 175, 122, 6, 159, 254, 85, 99, 145, 152, 178, 133, 230, 236, 192, 69, 35, 136, 141, 194, 243, 134);
        let skc_test = CryptoBox::sk_to_curve25519(&sk).unwrap();
        let skc_exp = vec!(144, 112, 64, 101, 69, 167, 61, 44, 220, 148, 58, 187, 108, 73, 11, 247, 130, 161, 158, 40, 100, 1, 40, 27, 76, 148, 209, 240, 195, 35, 153, 121);
        assert_eq!(skc_exp, skc_test);
    }
}
