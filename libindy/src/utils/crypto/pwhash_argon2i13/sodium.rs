extern crate sodiumoxide;
extern crate libc;
extern crate errno;

use errors::common::CommonError;

use self::sodiumoxide::crypto::pwhash;
use self::libc::{size_t, c_ulonglong, c_int};

pub struct PwhashArgon2i13 {}

impl PwhashArgon2i13 {
    pub const SALTBYTES: usize = pwhash::SALTBYTES;

    pub fn gen_salt() -> [u8; pwhash::SALTBYTES] {
        let pwhash::Salt(salt) = pwhash::gen_salt();
        salt
    }

    pub fn derive_key<'a>(key: &'a mut [u8], passwd: &[u8], salt: &[u8; pwhash::SALTBYTES]) -> Result<&'a [u8], CommonError> {
        let opslimit = unsafe { crypto_pwhash_opslimit_moderate() };
        let memlimit = unsafe { crypto_pwhash_memlimit_moderate() };
        let alg = unsafe { crypto_pwhash_alg_argon2i13() };

        let res = unsafe {
            crypto_pwhash(key.as_mut_ptr(),
                          key.len() as c_ulonglong,
                          passwd.as_ptr(),
                          passwd.len() as c_ulonglong,
                          salt,
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
}

extern {
    fn crypto_pwhash_alg_argon2i13() -> c_int;
    fn crypto_pwhash_opslimit_moderate() -> size_t;
    fn crypto_pwhash_memlimit_moderate() -> size_t;

    fn crypto_pwhash(out: *mut u8,
                     outlen: c_ulonglong,
                     passwd: *const u8,
                     passwdlen: c_ulonglong,
                     salt: *const [u8; 32], // SODIUM_CRYPTO_PWHASH_SALTBYTES
                     opslimit: c_ulonglong,
                     memlimit: size_t,
                     alg: c_int) -> c_int;
}


#[cfg(test)]
mod tests {
    use super::*;
    use self::sodiumoxide::crypto::secretbox;
    use utils::crypto::chacha20poly1305_ietf::ChaCha20Poly1305IETF;

    #[test]
    fn crypto_pwhash_works() {
        let passwd = b"Correct Horse Battery Staple";
        let secretbox::Key(ref mut kb) = secretbox::Key([0; ChaCha20Poly1305IETF::KEYBYTES]);
        let pwhash::Salt(ref salt) = pwhash::gen_salt();

        let _key = PwhashArgon2i13::derive_key(kb, passwd, salt).unwrap();
    }
}