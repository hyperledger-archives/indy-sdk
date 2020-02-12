use std::iter;
use rand::prelude::*;
use rand::distributions::Alphanumeric;
use sha2::Digest;

pub fn get_rand_string(len: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(()).map(|()| rng.sample(Alphanumeric)).take(len).collect()
}

pub fn gen_rand_signature(address: &str, message: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(address.as_bytes());
    bytes.extend_from_slice(message);
    sha2::Sha256::digest(bytes.as_slice()).to_vec()
}
