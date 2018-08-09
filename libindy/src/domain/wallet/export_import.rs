use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionMethod {
    ChaCha20Poly1305IETF {
        // **ChaCha20-Poly1305-IETF** cypher in blocks per chunk_size bytes
        salt: Vec<u8>,
        // pwhash_argon2i13::Salt as bytes. Random salt used for deriving of key from passphrase
        nonce: Vec<u8>,
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        chunk_size: usize,
        // size of encrypted chunk
    },
    ChaCha20Poly1305IETFWithSimplify {
        // **ChaCha20-Poly1305-IETF** cypher in blocks per chunk_size bytes
        salt: Vec<u8>,
        // pwhash_argon2i13::Salt as bytes. Random salt used for deriving of key from passphrase
        nonce: Vec<u8>,
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        chunk_size: usize
    },
}

impl EncryptionMethod {
    pub fn simplify_security(&self) -> bool {
        match self {
            EncryptionMethod::ChaCha20Poly1305IETF { .. } => false,
            EncryptionMethod::ChaCha20Poly1305IETFWithSimplify { .. } => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub encryption_method: EncryptionMethod,
    // Method of encryption for encrypted stream
    pub time: u64,
    // Export time in seconds from UNIX Epoch
    pub version: u32
}

// Note that we use externally tagged enum serialization and header will be represented as:
//
// {
//   "encryption_method": {
//     "ChaCha20Poly1305IETF": {
//       "salt": ..,
//       "nonce": ..,
//       "chunk_size": ..,
//     },
//   },
//   "time": ..,
//   "version": ..,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "type")]
    pub type_: String,
    // Wallet record type
    pub id: String,
    // Wallet record id
    pub value: String,
    // Wallet record value
    pub tags: HashMap<String, String>,
    // Wallet record tags
}