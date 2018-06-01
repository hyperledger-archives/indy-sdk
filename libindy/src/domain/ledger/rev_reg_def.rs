extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::{REVOC_REG_DEF, GET_REVOC_REG_DEF};

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use super::response::GetReplyResultV1;
use super::super::anoncreds::revocation_registry_definition::{RevocationRegistryDefinitionV1, RevocationRegistryDefinitionValue};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RevRegDefOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    #[serde(rename = "revocDefType")]
    pub type_: String,
    pub tag: String,
    pub cred_def_id: String,
    pub value: RevocationRegistryDefinitionValue
}

impl RevRegDefOperation {
    pub fn new(rev_reg_def: RevocationRegistryDefinitionV1) -> RevRegDefOperation {
        RevRegDefOperation {
            _type: REVOC_REG_DEF.to_string(),
            id: rev_reg_def.id.to_string(),
            type_: rev_reg_def.revoc_def_type.to_str().to_string(),
            tag: rev_reg_def.tag.to_string(),
            cred_def_id: rev_reg_def.cred_def_id.to_string(),
            value: rev_reg_def.value
        }
    }
}

impl JsonEncodable for RevRegDefOperation {}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegDefOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String
}

impl GetRevRegDefOperation {
    pub fn new(id: &str) -> GetRevRegDefOperation {
        GetRevRegDefOperation {
            _type: GET_REVOC_REG_DEF.to_string(),
            id: id.to_string()
        }
    }
}

impl JsonEncodable for GetRevRegDefOperation {}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetRevocRegDefReplyResult {
    GetRevocRegDefReplyResultV0(GetRevocRegDefResultV0),
    GetRevocRegDefReplyResultV1(GetReplyResultV1<RevocationRegistryDefinitionV1>)
}

impl<'a> JsonDecodable<'a> for GetRevocRegDefReplyResult {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegDefResultV0 {
    pub seq_no: i32,
    pub data: RevocationRegistryDefinitionV1
}

impl<'a> JsonDecodable<'a> for GetRevocRegDefResultV0 {}