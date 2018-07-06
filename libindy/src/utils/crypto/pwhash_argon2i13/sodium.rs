extern crate sodiumoxide;
extern crate libc;
extern crate errno;
extern crate serde;

use errors::common::CommonError;

use self::sodiumoxide::crypto::pwhash;
use self::libc::{size_t, c_ulonglong, c_int};

pub const SALTBYTES: usize = pwhash::SALTBYTES;

sodium_type!(Salt, pwhash::Salt, SALTBYTES);

pub fn gen_salt() -> Salt {
    Salt(pwhash::gen_salt())
}

pub fn pwhash<'a>(key: &'a mut [u8], passwd: &[u8], salt: &Salt) -> Result<&'a [u8], CommonError> {
    let opslimit = unsafe { crypto_pwhash_opslimit_moderate() };
    let memlimit = unsafe { crypto_pwhash_memlimit_moderate() };
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
        Err(CommonError::InvalidStructure(format!("{:?}", errno::errno())))
    }
}

extern {
    fn crypto_pwhash_alg_argon2i13() -> c_int;
    fn crypto_pwhash_opslimit_moderate() -> size_t;
    fn crypto_pwhash_memlimit_moderate() -> size_t;

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
    use super::*;
    use rmp_serde;

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
        let _key = pwhash(&mut key, passwd, &salt).unwrap();
    }
}