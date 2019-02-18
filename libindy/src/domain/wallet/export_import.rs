use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionMethod {
    // **ChaCha20-Poly1305-IETF** cypher in blocks per chunk_size bytes
    ChaCha20Poly1305IETF {
        // pwhash_argon2i13::Salt as bytes. Random salt used for deriving of key from passphrase
        salt: Vec<u8>,
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        nonce: Vec<u8>,
        // size of encrypted chunk
        chunk_size: usize,
    },
    // **ChaCha20-Poly1305-IETF interactive key derivation** cypher in blocks per chunk_size bytes
    ChaCha20Poly1305IETFInteractive {
        // pwhash_argon2i13::Salt as bytes. Random salt used for deriving of key from passphrase
        salt: Vec<u8>,
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        nonce: Vec<u8>,
        // size of encrypted chunk
        chunk_size: usize,
    },
    // **ChaCha20-Poly1305-IETF raw key** cypher in blocks per chunk_size bytes
    ChaCha20Poly1305IETFRaw {
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        nonce: Vec<u8>,
        // size of encrypted chunk
        chunk_size: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    // Method of encryption for encrypted stream
    pub encryption_method: EncryptionMethod,
    // Export time in seconds from UNIX Epoch
    pub time: u64,
    // Version of header
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
    // Wallet record type
    #[serde(rename = "type")]
    pub type_: String,
    // Wallet record id
    pub id: String,
    // Wallet record value
    pub value: String,
    // Wallet record tags
    pub tags: HashMap<String, String>,
}