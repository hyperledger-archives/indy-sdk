use named_type::NamedType;

use regex::Regex;
use rust_base58::FromBase58;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering}
};
use utils::validation::Validatable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyDidInfo {
    pub did: Option<DidValue>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>,
    pub cid: Option<bool>,
    pub method_name: Option<String>
}

impl Validatable for MyDidInfo {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref did) = self.did {
            did.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheirDidInfo {
    pub did: DidValue,
    pub verkey: Option<String>
}

impl TheirDidInfo {
    pub fn new(did: DidValue, verkey: Option<String>) -> TheirDidInfo {
        TheirDidInfo {
            did,
            verkey
        }
    }
}

impl Validatable for TheirDidInfo {
    fn validate(&self) -> Result<(), String> {
        self.did.validate()?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, NamedType)]
pub struct Did {
    pub did: DidValue,
    pub verkey: String
}

impl Did {
    pub fn new(did: DidValue, verkey: String) -> Did {
        Did {
            did,
            verkey
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DidValue(pub String);

impl DidValue {
    pub fn new(did: &str, method_name: Option<&str>) -> DidValue {
        if DidProtocolVersion::get() == 1 {
            let method_name = method_name.map(String::from).unwrap_or(DidProtocolVersion::get_default_method_name());
            DidValue(format!("did:{}:{}", method_name, did))
        } else {
            DidValue(did.to_string())
        }
    }

    pub fn to_short(&self) -> Result<ShortDidValue, &'static str> {
        let unqualified_did = self.unqualify();
        if DidProtocolVersion::get() == 1 {
            let did_ = unqualified_did.ok_or("Did does not match mask")?;
            Ok(ShortDidValue(did_))
        } else {
            Ok(ShortDidValue(unqualified_did.unwrap_or(self.0.to_string())))
        }
    }

    pub fn from_short(did: &ShortDidValue) -> DidValue {
        if DidProtocolVersion::get() == 1 {
        DidValue::new(&did.0, None)
        } else {
            DidValue(did.0.to_string())
        }
    }

    pub fn is_fully_qualified(&self) -> bool {
        REGEX.is_match(&self.0)
    }

    pub fn is_abbreviatable(&self) -> bool {
        if DidProtocolVersion::get() == 0 {
            return true;
        }
        if self.0.starts_with(&format!("did:{}:", DEFAULT_METHOD)) {
            return true;
        }
        false
    }

    fn unqualify(&self) -> Option<String> {
        trace!("unqualify_did: did: {:?}", self);
        let s = REGEX.captures(&self.0);
        trace!("unqualify_did: matches: {:?}", s);
        match s {
            None => None,
            Some(caps) => {
                caps.get(1).map(|m| m.as_str().to_string())
            }
        }
    }
}

impl Validatable for DidValue {
    fn validate(&self) -> Result<(), String> {
        if DidProtocolVersion::get() == 1 {
            self.unqualify()
                .ok_or("Did does not match mask".to_string())?;
        } else {
            let did = self.0.from_base58()
                .map_err(|err| err.to_string())?;

            if did.len() != 16 && did.len() != 32 {
                return Err(format!("Trying to use DID with unexpected length: {}. \
                               The 16- or 32-byte number upon which a DID is based should be 22/23 or 44/45 bytes when encoded as base58.", did.len()));
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ShortDidValue(pub String);

impl Validatable for ShortDidValue {
    fn validate(&self) -> Result<(), String> {
        let did = self.0.from_base58()
            .map_err(|err| err.to_string())?;

        if did.len() != 16 && did.len() != 32 {
            return Err(format!("Trying to use DID with unexpected length: {}. \
                               The 16- or 32-byte number upon which a DID is based should be 22/23 or 44/45 bytes when encoded as base58.", did.len()));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct DidMetadata {
    pub value: String
}

#[derive(Serialize, Clone, Debug, NamedType)]
#[serde(rename_all = "camelCase")]
pub struct DidWithMeta {
    pub did: DidValue,
    pub verkey: String,
    pub temp_verkey: Option<String>,
    pub metadata: Option<String>
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TheirDid {
    pub did: DidValue,
    pub verkey: String
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TemporaryDid {
    pub did: DidValue,
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
pub const DEFAULT_VERSION: usize = 0;

pub struct DidProtocolVersion {}

lazy_static! {
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
}
