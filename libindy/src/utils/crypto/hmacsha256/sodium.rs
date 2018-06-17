extern crate sodiumoxide;

use sodiumoxide::crypto::auth::hmacsha256;
use utils::byte_array::_clone_into_array;


pub struct HMACSHA256Key {
    key: hmacsha256::Key,
}

impl HMACSHA256Key {
    pub fn get_bytes(&self) -> &[u8] {
        &self.key.0
    }
}


pub struct HMACSHA256 {}

impl HMACSHA256 {
    pub const TAGBYTES: usize = hmacsha256::TAGBYTES;

    pub fn generate_key() -> HMACSHA256Key {
        HMACSHA256Key { key: hmacsha256::gen_key() }
    }

    pub fn clone_key_from_slice(bytes: &[u8]) -> HMACSHA256Key {
        HMACSHA256Key { key: hmacsha256::Key(_clone_into_array(bytes)) }
    }

    pub fn create_tag(data: &[u8], key: &HMACSHA256Key) -> [u8; hmacsha256::TAGBYTES]{
        let hmacsha256::Tag(tag) = hmacsha256::authenticate(data, &key.key);
        tag
    }
}