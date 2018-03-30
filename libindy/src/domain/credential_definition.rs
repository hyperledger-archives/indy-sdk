extern crate indy_crypto;
extern crate serde;
extern crate serde_json;

use self::serde::ser::{self, Serialize, Serializer};
use self::serde::de::{self, Deserialize, Deserializer};
use serde_json::Value;

use super::build_id;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use self::indy_crypto::cl::{CredentialPrimaryPublicKey, CredentialRevocationPublicKey};

use std::collections::HashMap;

pub const CL_SIGNATURE_TYPE: &'static str = "CL";
pub const CRED_DEF_MARKER: &'static str = "\x03";

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub enum SignatureType {
    CL
}

impl<'a> JsonDecodable<'a> for SignatureType {}

impl SignatureType {
    pub fn to_str(&self) -> &'static str {
        match self {
            &SignatureType::CL => CL_SIGNATURE_TYPE
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionConfig {
    pub support_revocation: bool
}

impl<'a> JsonDecodable<'a> for CredentialDefinitionConfig {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionV0 {
    pub primary: CredentialPrimaryPublicKey,
    pub revocation: Option<CredentialRevocationPublicKey>
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinitionV1 {
    pub id: String,
    pub schema_id: String,
    #[serde(rename = "type")]
    pub signature_type: SignatureType,
    pub tag: String,
    pub value: CredentialDefinitionV0
}

impl CredentialDefinitionV1 {
    const VERSION: &'static str = "1";
}

#[derive(Debug)]
pub enum CredentialDefinition {
    CredentialDefinitionV0(CredentialDefinitionV0),
    CredentialDefinitionV1(CredentialDefinitionV1)
}

impl CredentialDefinition {
    pub fn cred_def_id(did: &str, schema_id: &str, signature_type: &SignatureType, tag: &str) -> String {
        build_id(did, CRED_DEF_MARKER, Some(schema_id), signature_type.to_str(), tag)
    }
}

impl JsonEncodable for CredentialDefinition {}

impl<'a> JsonDecodable<'a> for CredentialDefinition {}

impl<'de> Deserialize<'de> for CredentialDefinition
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct VersionHelper {
            ver: Option<String>,
        }

        let v = Value::deserialize(deserializer)?;

        let helper = VersionHelper::deserialize(&v).map_err(de::Error::custom)?;

        match helper.ver {
            Some(version) => {
                match version.as_ref() {
                    CredentialDefinitionV1::VERSION => {
                        let cred_def = CredentialDefinitionV1::deserialize(v).map_err(de::Error::custom)?;
                        Ok(CredentialDefinition::CredentialDefinitionV1(cred_def))
                    }
                    _ => Err(de::Error::unknown_variant(&version, &[CredentialDefinitionV1::VERSION]))
                }
            }
            None => {
                let cred_def = CredentialDefinitionV0::deserialize(v).map_err(de::Error::custom)?;
                Ok(CredentialDefinition::CredentialDefinitionV0(cred_def))
            }
        }
    }
}

impl Serialize for CredentialDefinition
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            CredentialDefinition::CredentialDefinitionV1(ref cred_def) => {
                let mut v = serde_json::to_value(cred_def).map_err(ser::Error::custom)?;
                v["ver"] = serde_json::Value::String(CredentialDefinitionV1::VERSION.to_string());
                v.serialize(serializer)
            }
            CredentialDefinition::CredentialDefinitionV0(ref cred_def) => cred_def.serialize(serializer),
        }
    }
}

impl From<CredentialDefinitionV1> for CredentialDefinitionV0 {
    fn from(cred_def: CredentialDefinitionV1) -> Self {
        cred_def.value
    }
}

impl From<CredentialDefinition> for CredentialDefinitionV0 {
    fn from(cred_def: CredentialDefinition) -> Self {
        match cred_def {
            CredentialDefinition::CredentialDefinitionV0(cred_def) => cred_def,
            CredentialDefinition::CredentialDefinitionV1(cred_def) => CredentialDefinitionV0::from(cred_def)
        }
    }
}

pub fn cred_defs_map_to_cred_defs_v0_map(cred_defs: HashMap<String, CredentialDefinition>) -> HashMap<String, CredentialDefinitionV0> {
    let mut cred_defs_v0: HashMap<String, CredentialDefinitionV0> = HashMap::new();

    for (cred_def_id, cred_def) in cred_defs {
        cred_defs_v0.insert(cred_def_id, CredentialDefinitionV0::from(cred_def));
    }

    cred_defs_v0
}