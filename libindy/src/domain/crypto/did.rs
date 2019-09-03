use named_type::NamedType;

use regex::Regex;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering}
};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyDidInfo {
    pub did: Option<String>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>,
    pub cid: Option<bool>,
    pub method_name: Option<String>
}

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

impl TryFrom<String> for Did {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if DidProtocolVersion::get() == 1 {
            if let Some(s) = DidProtocolVersion::unqualify_did(&value) {
                Ok(Did{
                    did: s,
                    verkey: "".to_string()
                })
            } else {
                Err("Did does not match mask")
            }
        } else {
            Ok(Did {
                did: value.to_string(),
                verkey: "".to_string()
            })
        }
    }
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct DidMetadata {
    pub value: String
}

#[derive(Serialize, Clone, Debug, NamedType)]
#[serde(rename_all = "camelCase")]
pub struct DidWithMeta {
    pub did: String,
    pub verkey: String,
    pub temp_verkey: Option<String>,
    pub metadata: Option<String>
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TheirDid {
    pub did: String,
    pub verkey: String
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TemporaryDid {
    pub did: String,
    pub verkey: String
}

impl From<TemporaryDid> for Did {
    fn from(temp_did: TemporaryDid) -> Self {
        Did {
            did: temp_did.did,
            verkey: temp_did.verkey
        }
    }
}

pub struct DidProtocolVersion {}

lazy_static!{
    pub static ref DID_PROTOCOL_VERSION: AtomicUsize = AtomicUsize::new(0);
    pub static ref DID_DEFAULT_METHOD_NAME: Mutex<String> = Mutex::new("sov".to_string());
    pub static ref REGEX: Regex = Regex::new("did:[a-z0-9]+:([a-zA-Z0-9:.-_]*)").unwrap();
}

impl DidProtocolVersion {

    pub fn set(version: usize) {
        DID_PROTOCOL_VERSION.store(version, Ordering::Relaxed);
    }

    pub fn get() -> usize {
        DID_PROTOCOL_VERSION.load(Ordering::Relaxed)
    }

    pub fn get_default_method_name() -> String {
        DID_DEFAULT_METHOD_NAME.lock().unwrap().to_string()
    }

    pub fn set_default_method_name(method_name: &str) {
        let mut val = DID_DEFAULT_METHOD_NAME.lock().unwrap();
        *val = method_name.to_string();
    }

    pub fn unqualify_did(did: &str) -> Option<String> {
        trace!("unqualify_did: did: {}", did);
        let s = REGEX.captures(did);
        trace!("unqualify_did: matches: {:?}", s);
        match s {
            None => None,
            Some(caps) => {
                caps.get(1).map(|m| m.as_str().to_string())
            }
        }
    }
}
