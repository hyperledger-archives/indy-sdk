extern crate indy_crypto;
extern crate serde;
extern crate serde_json;

use super::DELIMITER;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use self::indy_crypto::cl::{
    CredentialPrimaryPublicKey,
    CredentialRevocationPublicKey,
    CredentialPrivateKey,
    CredentialKeyCorrectnessProof
};
use super::super::ledger::request::ProtocolVersion;

use std::collections::HashMap;
use named_type::NamedType;

pub const CL_SIGNATURE_TYPE: &'static str = "CL";
pub const CRED_DEF_MARKER: &'static str = "3";

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

fn default_false() -> bool { false }

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionConfig {
    #[serde(default = "default_false")]
    pub support_revocation: bool
}

impl<'a> JsonDecodable<'a> for CredentialDefinitionConfig {}

impl Default for CredentialDefinitionConfig {
    fn default() -> Self {
        CredentialDefinitionConfig {
            support_revocation: false
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionData {
    pub primary: CredentialPrimaryPublicKey,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub value: CredentialDefinitionData
}

impl<'a> JsonDecodable<'a> for CredentialDefinitionV1 {}

#[derive(Debug, Serialize, Deserialize, NamedType)]
#[serde(tag = "ver")]
pub enum CredentialDefinition {
    #[serde(rename = "1.0")]
    CredentialDefinitionV1(CredentialDefinitionV1)
}

impl CredentialDefinition {
    pub fn cred_def_id(did: &str, schema_id: &str, signature_type: &str, tag: &str) -> String {
        if ProtocolVersion::is_node_1_3(){
            format!("{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id)
        } else {
            format!("{}{}{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id, DELIMITER, tag)
        }
    }
}

impl JsonEncodable for CredentialDefinition {}

impl<'a> JsonDecodable<'a> for CredentialDefinition {}

impl From<CredentialDefinition> for CredentialDefinitionV1 {
    fn from(cred_def: CredentialDefinition) -> Self {
        match cred_def {
            CredentialDefinition::CredentialDefinitionV1(cred_def) => cred_def
        }
    }
}

pub fn cred_defs_map_to_cred_defs_v1_map(cred_defs: HashMap<String, CredentialDefinition>) -> HashMap<String, CredentialDefinitionV1> {
    let mut cred_defs_v1: HashMap<String, CredentialDefinitionV1> = HashMap::new();

    for (cred_def_id, cred_def) in cred_defs {
        cred_defs_v1.insert(cred_def_id, CredentialDefinitionV1::from(cred_def));
    }

    cred_defs_v1
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct CredentialDefinitionPrivateKey {
    pub value: CredentialPrivateKey
}

impl JsonEncodable for CredentialDefinitionPrivateKey {}

impl<'a> JsonDecodable<'a> for CredentialDefinitionPrivateKey {}

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct CredentialDefinitionCorrectnessProof {
    pub value: CredentialKeyCorrectnessProof
}

impl JsonEncodable for CredentialDefinitionCorrectnessProof {}

impl<'a> JsonDecodable<'a> for CredentialDefinitionCorrectnessProof {}