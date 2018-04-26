extern crate indy_crypto;
extern crate rmp_serde;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Serialize, Deserialize)]
pub struct KeyInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

impl JsonEncodable for KeyInfo {}

impl<'a> JsonDecodable<'a> for KeyInfo {}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyDidInfo {
    pub did: Option<String>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>,
    pub cid: Option<bool>
}

impl<'a> JsonDecodable<'a> for MyDidInfo {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheirDidInfo {
    pub did: String,
    pub verkey: Option<String>
}

impl TheirDidInfo {
    pub fn new(did: String, verkey: Option<String>) -> TheirDidInfo {
        TheirDidInfo {
            did,
            verkey
        }
    }
}

impl JsonEncodable for TheirDidInfo {}

impl<'a> JsonDecodable<'a> for TheirDidInfo {}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

impl JsonEncodable for Key {}

impl<'a> JsonDecodable<'a> for Key {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Did {
    pub did: String,
    pub verkey: String
}

impl Did {
    pub fn new(did: String, verkey: String) -> Did {
        Did {
            did,
            verkey
        }
    }
}

impl JsonEncodable for Did {}

impl<'a> JsonDecodable<'a> for Did {}

#[derive(Serialize, Deserialize)]
pub struct ComboBox {
    pub msg: String,
    pub sender: String,
    pub nonce: String
}

impl ComboBox {
    pub fn to_msg_pack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::encode::to_vec_named(self)
    }

    pub fn from_msg_pack(bytes: &[u8]) -> Result<ComboBox, rmp_serde::decode::Error> {
        rmp_serde::decode::from_slice(bytes)
    }
}