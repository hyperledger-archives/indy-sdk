extern crate sodiumoxide;

use self::sodiumoxide::crypto::auth::hmacsha256;

pub const KEYBYTES: usize = hmacsha256::KEYBYTES;
pub const TAGBYTES: usize = hmacsha256::TAGBYTES;

sodium_type!(Key, hmacsha256::Key, KEYBYTES);
sodium_type!(Tag, hmacsha256::Tag, TAGBYTES);

pub fn gen_key() -> Key {
    Key(hmacsha256::gen_key())
}

pub fn authenticate(data: &[u8], key: &Key) -> Tag {
    Tag(hmacsha256::authenticate(data, &key.0))
}