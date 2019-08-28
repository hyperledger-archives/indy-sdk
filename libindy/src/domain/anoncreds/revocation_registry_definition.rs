use ursa::cl::{RevocationKeyPublic, RevocationKeyPrivate};

use super::DELIMITER;
use super::credential_definition::CredentialDefinitionId;

use std::collections::{HashMap, HashSet};
use named_type::NamedType;

use utils::validation::Validatable;

pub const CL_ACCUM: &str = "CL_ACCUM";
pub const REV_REG_DEG_MARKER: &str = "4";

#[derive(Deserialize, Debug, Serialize)]
pub struct RevocationRegistryConfig {
    pub issuance_type: Option<IssuanceType>,
    pub max_cred_num: Option<u32>
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub enum IssuanceType {
    ISSUANCE_BY_DEFAULT,
    ISSUANCE_ON_DEMAND
}

impl IssuanceType {
    pub fn to_bool(&self) -> bool {
        self.clone() == IssuanceType::ISSUANCE_BY_DEFAULT
    }
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub enum RegistryType {
    CL_ACCUM,
}

impl RegistryType {
    pub fn to_str(&self) -> &'static str {
        match *self {
            RegistryType::CL_ACCUM => CL_ACCUM
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDefinitionValue {
    pub issuance_type: IssuanceType,
    pub max_cred_num: u32,
    pub public_keys: RevocationRegistryDefinitionValuePublicKeys,
    pub tails_hash: String,
    pub tails_location: String
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDefinitionValuePublicKeys {
    pub accum_key: RevocationKeyPublic
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDefinitionV1 {
    pub id: RevocationRegistryId,
    pub revoc_def_type: RegistryType,
    pub tag: String,
    pub cred_def_id: CredentialDefinitionId,
    pub value: RevocationRegistryDefinitionValue
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
#[serde(tag = "ver")]
pub enum RevocationRegistryDefinition {
    #[serde(rename = "1.0")]
    RevocationRegistryDefinitionV1(RevocationRegistryDefinitionV1)
}

impl From<RevocationRegistryDefinition> for RevocationRegistryDefinitionV1 {
    fn from(rev_reg_def: RevocationRegistryDefinition) -> Self {
        match rev_reg_def {
            RevocationRegistryDefinition::RevocationRegistryDefinitionV1(rev_reg_def) => rev_reg_def
        }
    }
}

pub type RevocationRegistryDefinitions = HashMap<RevocationRegistryId, RevocationRegistryDefinition>;

pub fn rev_reg_defs_map_to_rev_reg_defs_v1_map(rev_reg_defs: RevocationRegistryDefinitions) -> HashMap<RevocationRegistryId, RevocationRegistryDefinitionV1> {
    rev_reg_defs
        .into_iter()
        .map(|(rev_reg_id, rev_reg_def)| (rev_reg_id, RevocationRegistryDefinitionV1::from(rev_reg_def)))
        .collect()
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct RevocationRegistryDefinitionPrivate {
    pub value: RevocationKeyPrivate
}

#[derive(Debug, Deserialize, Serialize, Clone, NamedType)]
pub struct RevocationRegistryInfo {
    pub id: RevocationRegistryId,
    pub curr_id: u32,
    pub used_ids: HashSet<u32>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RevocationRegistryId(pub String);

impl RevocationRegistryId {
    pub fn new(did: &str, cred_def_id: &str, rev_reg_type: &RegistryType, tag: &str) -> RevocationRegistryId {
        RevocationRegistryId(format!("{}{}{}{}{}{}{}{}{}", did, DELIMITER, REV_REG_DEG_MARKER, DELIMITER, cred_def_id, DELIMITER, rev_reg_type.to_str(), DELIMITER, tag))
    }
}

impl Validatable for RevocationRegistryConfig {
    fn validate(&self) -> Result<(), String> {
        if let Some(num_) = self.max_cred_num {
            if num_ == 0 {
                return Err(String::from("RevocationRegistryConfig validation failed: `max_cred_num` must be greater than 0"));
            }
        }
        Ok(())
    }
}

impl Validatable for RevocationRegistryId {
    fn validate(&self) -> Result<(), String> {
        let parts: Vec<&str> = self.0.split_terminator(DELIMITER).collect::<Vec<&str>>();

        parts.get(0).ok_or_else(||format!("Revocation Registry Id validation failed: issuer DID not found in: {}", self.0))?;
        parts.get(1).ok_or_else(||format!("Revocation Registry Id validation failed: marker not found in: {}", self.0))?;
        parts.get(2).ok_or_else(||format!("Revocation Registry Id validation failed: signature type not found in: {}", self.0))?;

        if parts.len() != 8 && parts.len() != 9 && parts.len() != 11 && parts.len() != 12 {
            return Err("Revocation Registry Id validation failed: invalid number of parts".to_string());
        }

        Ok(())
    }
}

impl Validatable for RevocationRegistryDefinition {
    fn validate(&self) -> Result<(), String> {
        match self {
            RevocationRegistryDefinition::RevocationRegistryDefinitionV1(revoc_reg_def) => {
                revoc_reg_def.id.validate()?;
            }
        }
        Ok(())
    }
}
