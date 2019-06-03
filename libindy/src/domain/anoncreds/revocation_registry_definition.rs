use ursa::cl::{RevocationKeyPublic, RevocationKeyPrivate};

use super::DELIMITER;

use std::collections::{HashMap, HashSet};
use named_type::NamedType;

pub const CL_ACCUM: &str = "CL_ACCUM";
pub const REV_REG_DEG_MARKER: &str = "4";

#[derive(Deserialize, Debug, Serialize)]
pub struct RevocationRegistryConfig {
    pub issuance_type: Option<String>,
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
    pub id: String,
    pub revoc_def_type: RegistryType,
    pub tag: String,
    pub cred_def_id: String,
    pub value: RevocationRegistryDefinitionValue
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
#[serde(tag = "ver")]
pub enum RevocationRegistryDefinition {
    #[serde(rename = "1.0")]
    RevocationRegistryDefinitionV1(RevocationRegistryDefinitionV1)
}

impl RevocationRegistryDefinition {
    pub fn rev_reg_id(did: &str, cred_def_id: &str, rev_reg_type: &RegistryType, tag: &str) -> String {
        format!("{}{}{}{}{}{}{}{}{}", did, DELIMITER, REV_REG_DEG_MARKER, DELIMITER, cred_def_id, DELIMITER, rev_reg_type.to_str(), DELIMITER, tag)
    }
}

impl From<RevocationRegistryDefinition> for RevocationRegistryDefinitionV1 {
    fn from(rev_reg_def: RevocationRegistryDefinition) -> Self {
        match rev_reg_def {
            RevocationRegistryDefinition::RevocationRegistryDefinitionV1(rev_reg_def) => rev_reg_def
        }
    }
}

pub fn rev_reg_defs_map_to_rev_reg_defs_v1_map(rev_reg_defs: HashMap<String, RevocationRegistryDefinition>) -> HashMap<String, RevocationRegistryDefinitionV1> {
    let mut rev_reg_defs_v1: HashMap<String, RevocationRegistryDefinitionV1> = HashMap::new();

    for (rev_reg_id, rev_reg_def) in rev_reg_defs {
        rev_reg_defs_v1.insert(rev_reg_id, RevocationRegistryDefinitionV1::from(rev_reg_def));
    }

    rev_reg_defs_v1
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
pub struct RevocationRegistryDefinitionPrivate {
    pub value: RevocationKeyPrivate
}

#[derive(Debug, Deserialize, Serialize, Clone, NamedType)]
pub struct RevocationRegistryInfo {
    pub id: String,
    pub curr_id: u32,
    pub used_ids: HashSet<u32>
}
