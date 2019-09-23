use named_type::NamedType;

use regex::Regex;
use rust_base58::FromBase58;

use utils::validation::Validatable;
use utils::qualifier;

pub const ABBREVIATABLE_PREFIX: &'static str = "did:sov";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyDidInfo {
    pub did: Option<DidValue>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>,
    pub cid: Option<bool>,
    pub method_name: Option<String>,
}

impl Validatable for MyDidInfo {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref did) = self.did {
            did.validate()?;
        }
        if let Some(ref name) = self.method_name {
            lazy_static! {
                static ref REGEX_METHOD_NAME: Regex = Regex::new("^[a-z0-9]+$").unwrap();
            }
            if !REGEX_METHOD_NAME.is_match(name) {
                return Err(format!("Invalid default name: {}. It does not match the DID method name format.", name));
            }
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

qualifiable_type!(DidValue);

impl DidValue {
    pub const PREFIX: &'static str = "did";

    pub fn new(did: &str, method_name: Option<&str>) -> DidValue {
        match method_name {
            Some(method_name_) => DidValue(format!("{}:{}:{}", Self::PREFIX, method_name_, did)),
            None => DidValue(did.to_string())
        }
    }

    pub fn to_short(&self) -> ShortDidValue {
        ShortDidValue(self.unqualify().0)
    }

    pub fn from_short(did: &ShortDidValue, prefix: Option<String>) -> DidValue {
        match prefix {
            Some(prefix_) => DidValue(qualifier::qualify(&did.0, Self::PREFIX, &prefix_)),
            None => DidValue(did.0.clone())
        }
    }

    pub fn is_abbreviatable(&self) -> bool {
        if !self.is_fully_qualified() {
            return true;
        }
        if self.0.starts_with(ABBREVIATABLE_PREFIX) {
            return true;
        }
        false
    }
}

impl Validatable for DidValue {
    fn validate(&self) -> Result<(), String> {
        if self.is_fully_qualified() {
            // pass
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
