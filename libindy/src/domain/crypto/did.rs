extern crate indy_crypto;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use named_type::NamedType;


#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug, NamedType)]
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

#[derive(Serialize, Clone, Debug, NamedType)]
pub struct DidWithMeta {
    pub did: String,
    pub verkey: String,
    pub metadata: Option<String>
}

impl JsonEncodable for DidWithMeta {}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TheirDid {
    pub did: String,
    pub verkey: String
}

impl JsonEncodable for TheirDid {}

impl<'a> JsonDecodable<'a> for TheirDid {}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TemporaryDid {
    pub did: String,
    pub verkey: String
}

impl JsonEncodable for TemporaryDid {}

impl<'a> JsonDecodable<'a> for TemporaryDid {}

impl From<TemporaryDid> for Did {
    fn from(temp_did: TemporaryDid) -> Self {
        Did {
            did: temp_did.did,
            verkey: temp_did.verkey
        }
    }
}

