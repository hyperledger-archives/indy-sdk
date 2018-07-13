use utils::crypto::{chacha20poly1305_ietf, pwhash_argon2i13};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionMethod {
    ChaCha20Poly1305IETF {salt: pwhash_argon2i13::Salt, nonce: chacha20poly1305_ietf::Nonce, chunk_size: usize},
}

impl EncryptionMethod {

    const CHUNK_SIZE: usize = 1024;

    pub fn chacha20poly1305_ietf() -> EncryptionMethod {
        EncryptionMethod::ChaCha20Poly1305IETF {
            salt: pwhash_argon2i13::gen_salt(),
            nonce: chacha20poly1305_ietf::gen_nonce(),
            chunk_size: EncryptionMethod::CHUNK_SIZE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub encryption_method: EncryptionMethod,
    pub time: u64,
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub value: String,
    pub tags: HashMap<String, String>,
}