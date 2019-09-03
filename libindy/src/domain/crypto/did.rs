use named_type::NamedType;

use errors::{IndyError, IndyErrorKind};
use regex::Regex;
use rust_base58::FromBase58;
use std::convert::TryFrom;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering}
};

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

#[derive(Serialize, Deserialize, Clone, Debug, NamedType, PartialEq)]
pub struct DidValue(pub String);

impl TryFrom<String> for DidValue {
    type Error = IndyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if DidProtocolVersion::get() == 1 {
            if let Some(s) = DidProtocolVersion::unqualify_did(&value) {
                Ok(DidValue(s))
            } else {
                Err(IndyError::from_msg(IndyErrorKind::InvalidStructure, "Did does not match mask"))
            }
        } else {
            let did = value.from_base58()?;

            if did.len() != 16 && did.len() != 32 {
                return Err(IndyError::from_msg(IndyErrorKind::InvalidStructure, &format!("Trying to use DID with unexpected length: {}. \
                               The 16- or 32-byte number upon which a DID is based should be 22/23 or 44/45 bytes when encoded as base58.", did.len())));
            }
            Ok(DidValue(value.to_string()))
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

pub const DEFAULT_METHOD: &'static str = "sov";
pub const DEFAULT_VERSION: usize = 1;

pub struct DidProtocolVersion {}

lazy_static!{
    pub static ref DID_PROTOCOL_VERSION: AtomicUsize = AtomicUsize::new(DEFAULT_VERSION);
    pub static ref DID_DEFAULT_METHOD_NAME: Mutex<String> = Mutex::new(DEFAULT_METHOD.to_string());
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
