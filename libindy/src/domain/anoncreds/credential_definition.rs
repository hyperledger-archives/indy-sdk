use super::DELIMITER;
use super::super::ledger::request::ProtocolVersion;

use ursa::cl::{
    CredentialPrimaryPublicKey,
    CredentialRevocationPublicKey,
    CredentialPrivateKey,
    CredentialKeyCorrectnessProof
};

use std::collections::HashMap;
use named_type::NamedType;

pub const CL_SIGNATURE_TYPE: &str = "CL";
pub const CRED_DEF_MARKER: &str = "3";

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub enum SignatureType {
    CL
}

impl SignatureType {
    pub fn to_str(&self) -> &'static str {
        match *self {
            SignatureType::CL => CL_SIGNATURE_TYPE
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialDefinitionConfig {
    #[serde(default)]
    pub support_revocation: bool
}

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

#[derive(Debug, Serialize, Deserialize, NamedType)]
#[serde(tag = "ver")]
pub enum CredentialDefinition {
    #[serde(rename = "1.0")]
    CredentialDefinitionV1(CredentialDefinitionV1)
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct TemporaryCredentialDefinition {
    pub cred_def: CredentialDefinition,
    pub cred_def_priv_key: CredentialDefinitionPrivateKey,
    pub cred_def_correctness_proof: CredentialDefinitionCorrectnessProof
}

impl CredentialDefinition {
    pub fn cred_def_id(did: &str, schema_id: &str, signature_type: &str, tag: &str) -> String {
        if ProtocolVersion::is_node_1_3() {
            format!("{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id)
        } else {
            format!("{}{}{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id, DELIMITER, tag)
        }
    }

    pub fn issuer_did(cred_def_id: &str) -> Option<String> {
        cred_def_id.split(':').next().map(String::from)
    }
}

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

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct CredentialDefinitionCorrectnessProof {
    pub value: CredentialKeyCorrectnessProof
}