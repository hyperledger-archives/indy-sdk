extern crate sodiumoxide;

use self::sodiumoxide::crypto::aead::chacha20poly1305_ietf;
use errors::common::CommonError::InvalidStructure;
use errors::crypto::CryptoError;

pub const KEYBYTES: usize = chacha20poly1305_ietf::KEYBYTES;
pub const NONCEBYTES: usize = chacha20poly1305_ietf::NONCEBYTES;
pub const TAGBYTES: usize = chacha20poly1305_ietf::TAGBYTES;

sodium_type!(Key, chacha20poly1305_ietf::Key, KEYBYTES);

pub fn gen_key() -> Key {
    Key(chacha20poly1305_ietf::gen_key())
}
