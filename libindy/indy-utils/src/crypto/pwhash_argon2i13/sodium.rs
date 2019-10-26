extern crate serde;
extern crate sodiumoxide;

use indy_api_types::domain::wallet::KeyDerivationMethod;
use indy_api_types::errors::prelude::*;
use libc::{c_int, c_ulonglong, size_t};
use self::sodiumoxide::crypto::pwhash;

pub const SALTBYTES: usize = pwhash::SALTBYTES;

sodium_type!(Salt, pwhash::Salt, SALTBYTES);

pub fn gen_salt() -> Salt {
    Salt(pwhash::gen_salt())
}

pub fn pwhash<'a>(key: &'a mut [u8], passwd: &[u8], salt: &Salt, key_derivation_method: &KeyDerivationMethod) -> Result<&'a [u8], IndyError> {
    let (opslimit, memlimit) = unsafe {
        match key_derivation_method {
            KeyDerivationMethod::ARGON2I_MOD => (crypto_pwhash_argon2i_opslimit_moderate(), crypto_pwhash_argon2i_memlimit_moderate()),
            KeyDerivationMethod::ARGON2I_INT => (crypto_pwhash_argon2i_opslimit_interactive(), crypto_pwhash_argon2i_memlimit_interactive()),
            KeyDerivationMethod::RAW => return Err(IndyError::from_msg(IndyErrorKind::InvalidStructure, "RAW key derivation method is not acceptable"))
        }
    };

    let alg = unsafe { crypto_pwhash_alg_argon2i13() };

    let res = unsafe {
        crypto_pwhash(key.as_mut_ptr(),
                      key.len() as c_ulonglong,
                      passwd.as_ptr(),
                      passwd.len() as c_ulonglong,
                      (salt.0).0.as_ptr(),
                      opslimit as c_ulonglong,
                      memlimit,
                      alg)
    };

    if res == 0 {
        Ok(key)
    } else {
        Err(IndyError::from_msg(IndyErrorKind::InvalidState, "Sodium pwhash failed"))
    }
}

extern {
    fn crypto_pwhash_alg_argon2i13() -> c_int;
    fn crypto_pwhash_argon2i_opslimit_moderate() -> size_t;
    fn crypto_pwhash_argon2i_memlimit_moderate() -> size_t;
    fn crypto_pwhash_argon2i_opslimit_interactive() -> size_t;
    fn crypto_pwhash_argon2i_memlimit_interactive() -> size_t;

    fn crypto_pwhash(out: *mut u8,
                     outlen: c_ulonglong,
                     passwd: *const u8,
                     passwdlen: c_ulonglong,
                     salt: *const u8, // SODIUM_CRYPTO_PWHASH_SALTBYTES
                     opslimit: c_ulonglong,
                     memlimit: size_t,
                     alg: c_int) -> c_int;
}


#[cfg(test)]
mod tests {
    use rmp_serde;
    use super::*;

    #[test]
    fn get_salt_works() {
        let salt = gen_salt();
        assert_eq!(salt[..].len(), SALTBYTES)
    }

    #[test]
    fn salt_serialize_deserialize_works() {
        let salt = gen_salt();
        let serialized = rmp_serde::to_vec(&salt).unwrap();
        let deserialized: Salt = rmp_serde::from_slice(&serialized).unwrap();

        assert_eq!(serialized.len(), SALTBYTES + 2);
        assert_eq!(salt, deserialized)
    }

    #[test]
    fn pwhash_works() {
        let passwd = b"Correct Horse Battery Staple";
        let mut key = [0u8; 64];

        let salt = gen_salt();
        let _key = pwhash(&mut key, passwd, &salt, &KeyDerivationMethod::ARGON2I_MOD).unwrap();
    }

    #[test]
    fn pwhash_works_for_interactive_method() {
        let passwd = b"Correct Horse Battery Staple";

        let salt = gen_salt();

        let mut key = [0u8; 64];
        let key_moderate = pwhash(&mut key, passwd, &salt, &KeyDerivationMethod::ARGON2I_MOD).unwrap();

        let mut key = [0u8; 64];
        let key_interactive = pwhash(&mut key, passwd, &salt, &KeyDerivationMethod::ARGON2I_INT).unwrap();

        assert_ne!(key_moderate, key_interactive);
    }
}