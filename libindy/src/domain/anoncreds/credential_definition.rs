use super::DELIMITER;
use super::schema::SchemaId;
use super::super::ledger::request::ProtocolVersion;

use utils::validation::Validatable;

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
    pub id: CredentialDefinitionId,
    pub schema_id: SchemaId,
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

impl From<CredentialDefinition> for CredentialDefinitionV1 {
    fn from(cred_def: CredentialDefinition) -> Self {
        match cred_def {
            CredentialDefinition::CredentialDefinitionV1(cred_def) => cred_def
        }
    }
}

pub type CredentialDefinitions = HashMap<CredentialDefinitionId, CredentialDefinition>;

pub fn cred_defs_map_to_cred_defs_v1_map(cred_defs: CredentialDefinitions) -> HashMap<CredentialDefinitionId, CredentialDefinitionV1> {
    cred_defs
        .into_iter()
        .map(|(cred_def_id, cred_def)| (cred_def_id, CredentialDefinitionV1::from(cred_def)))
        .collect()
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct CredentialDefinitionPrivateKey {
    pub value: CredentialPrivateKey
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct CredentialDefinitionCorrectnessProof {
    pub value: CredentialKeyCorrectnessProof
}

impl Validatable for CredentialDefinition {
    fn validate(&self) -> Result<(), String> {
        match self {
            CredentialDefinition::CredentialDefinitionV1(cred_def) => {
                cred_def.id.validate()?;
                cred_def.schema_id.validate()?;
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CredentialDefinitionId(pub String);

impl CredentialDefinitionId {
    pub fn new(did: &str, schema_id: &SchemaId, signature_type: &str, tag: &str) -> CredentialDefinitionId {
        if ProtocolVersion::is_node_1_3() {
            CredentialDefinitionId(format!("{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id.0))
        } else {
            CredentialDefinitionId(format!("{}{}{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id.0, DELIMITER, tag))
        }
    }

    pub fn issuer_did(&self) -> Option<String> {
        self.0.split(DELIMITER).next().map(String::from)
    }
}

impl Validatable for CredentialDefinitionId {
    fn validate(&self) -> Result<(), String> {
        let parts: Vec<&str> = self.0.split_terminator(DELIMITER).collect::<Vec<&str>>();

        parts.get(0).ok_or_else(||format!("Credential Definition Id validation failed: issuer DID not found in: {}", self.0))?;
        parts.get(1).ok_or_else(||format!("Credential Definition Id validation failed: marker not found in: {}", self.0))?;
        parts.get(2).ok_or_else(||format!("Credential Definition Id validation failed: signature type not found in: {}", self.0))?;

        if parts.len() == 4 {
            // NcYxiDXkpYi6ov5FcYDi1e:3:CL:1
            parts.get(3)
                .ok_or_else(||format!("Credential Definition Id validation failed: schema id not found in: {}", self.0))?
                .parse::<i32>()
                .map_err(|_| format!("Credential Definition Id validation failed: schema id is invalid number: {}", self.0))?;
        } else if parts.len() == 5 {
            // NcYxiDXkpYi6ov5FcYDi1e:3:CL:1:tag
            parts.get(3)
                .ok_or_else(||format!("Credential Definition Id validation failed: schema id not found in: {}", self.0))?
                .parse::<i32>()
                .map_err(|_| format!("Credential Definition Id validation failed: schema id is invalid number: {}", self.0))?;
        } else if parts.len() == 7 {
            // NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0
            // nothing to do
        } else if parts.len() == 8 {
            // NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:TAG_1
            // nothing to do
        } else {
            return Err("Credential Definition Id validation failed: too much parts".to_string());
        }

        Ok(())
    }
}

impl Validatable for CredentialDefinitionConfig {}