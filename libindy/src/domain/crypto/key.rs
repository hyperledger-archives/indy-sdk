extern crate indy_crypto;

use named_type::NamedType;

#[derive(Serialize, Deserialize, Clone, Debug, NamedType)]
pub struct Key {
    pub verkey: String,
    pub signkey: String
}

impl Key {
    pub fn new(verkey: String, signkey: String) -> Key {
        Key {
            verkey,
            signkey
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}