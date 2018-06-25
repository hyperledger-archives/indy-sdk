extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::{REVOC_REG_ENTRY, GET_REVOC_REG, GET_REVOC_REG_DELTA};

use self::indy_crypto::cl::{RevocationRegistry, RevocationRegistryDelta};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use super::response::GetReplyResultV1;
use super::super::anoncreds::revocation_registry::RevocationRegistryV1;
use super::super::anoncreds::revocation_registry_delta::RevocationRegistryDeltaV1;

use std::collections::HashSet;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RevRegEntryOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: String,
    pub revoc_def_type: String,
    pub value: RevocationRegistryDelta,
}

impl RevRegEntryOperation {
    pub fn new(rev_def_type: &str, revoc_reg_def_id: &str, value: RevocationRegistryDeltaV1) -> RevRegEntryOperation {
        RevRegEntryOperation {
            _type: REVOC_REG_ENTRY.to_string(),
            revoc_def_type: rev_def_type.to_string(),
            revoc_reg_def_id: revoc_reg_def_id.to_string(),
            value: value.value
        }
    }
}

impl JsonEncodable for RevRegEntryOperation {}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: String,
    pub timestamp: i64,
}

impl GetRevRegOperation {
    pub fn new(revoc_reg_def_id: &str, timestamp: i64) -> GetRevRegOperation {
        GetRevRegOperation {
            _type: GET_REVOC_REG.to_string(),
            revoc_reg_def_id: revoc_reg_def_id.to_string(),
            timestamp
        }
    }
}

impl JsonEncodable for GetRevRegOperation {}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegDeltaOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    pub to: i64
}

impl GetRevRegDeltaOperation {
    pub fn new(revoc_reg_def_id: &str, from: Option<i64>, to: i64) -> GetRevRegDeltaOperation {
        GetRevRegDeltaOperation {
            _type: GET_REVOC_REG_DELTA.to_string(),
            revoc_reg_def_id: revoc_reg_def_id.to_string(),
            from,
            to
        }
    }
}

impl JsonEncodable for GetRevRegDeltaOperation {}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetRevocRegReplyResult {
    GetRevocRegReplyResultV0(GetRevocRegResultV0),
    GetRevocRegReplyResultV1(GetReplyResultV1<GetRevocRegDataV1>)
}

impl<'a> JsonDecodable<'a> for GetRevocRegReplyResult {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegResultV0 {
    pub seq_no: i32,
    pub revoc_reg_def_id: String,
    pub data: RevocationRegistryV1,
    pub txn_time: u64
}

impl<'a> JsonDecodable<'a> for GetRevocRegResultV0 {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegDataV1 {
    pub revoc_reg_def_id: String,
    pub value: RevocationRegistryV1
}

impl<'a> JsonDecodable<'a> for GetRevocRegDataV1 {}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDeltaData {
    pub value: RevocationRegistryDeltaValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevocationRegistryDeltaValue {
    pub accum_from: Option<AccumulatorState>,
    pub accum_to: AccumulatorState,
    pub issued: HashSet<u32>,
    pub revoked: HashSet<u32>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccumulatorState {
    pub value: RevocationRegistry,
    pub txn_time: u64
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetRevocRegDeltaReplyResult {
    GetRevocRegDeltaReplyResultV0(GetRevocRegDeltaResultV0),
    GetRevocRegDeltaReplyResultV1(GetReplyResultV1<GetRevocRegDeltaDataV1>)
}

impl<'a> JsonDecodable<'a> for GetRevocRegDeltaReplyResult {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegDeltaResultV0 {
    pub seq_no: i32,
    pub revoc_reg_def_id: String,
    pub data: RevocationRegistryDeltaData
}

impl<'a> JsonDecodable<'a> for GetRevocRegDeltaResultV0 {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegDeltaDataV1 {
    pub revoc_reg_def_id: String,
    pub value: RevocationRegistryDeltaData
}

impl<'a> JsonDecodable<'a> for GetRevocRegDeltaDataV1 {}
