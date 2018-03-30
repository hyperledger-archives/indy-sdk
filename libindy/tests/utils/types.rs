extern crate serde_json;

use std::collections::HashSet;

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub enum ResponseType {
    REQNACK,
    REPLY,
    REJECT
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub op: ResponseType,
    pub reason: String,
    pub req_id: u64,
    pub identifier: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct Reply<T> {
    pub op: String,
    pub result: T,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNymReplyResult {
    pub identifier: String,
    pub req_id: u64,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Option<String>,
    pub dest: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNymResultData {
    pub identifier: String,
    pub dest: String,
    pub role: Option<String>,
    pub verkey: Option<String>
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAttribReplyResult {
    pub     identifier: String,
    pub   req_id: u64,
    #[serde(rename = "type")]
    pub   _type: String,
    pub   data: Option<String>,
    pub  dest: String,
    pub  seq_no: Option<i32>
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSchemaReplyResult {
    pub identifier: String,
    pub req_id: u64,
    pub seq_no: Option<i32>,
    //For tests/ In normal case seq_no exists
    #[serde(rename = "type")]
    pub  _type: String,
    pub  data: Option<GetSchemaResultData>,
    pub  dest: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetOperationResult {
    pub identifier: String,
    pub req_id: u64,
    //For tests/ In normal case seq_no exists
    #[serde(rename = "type")]
    pub  _type: String,
    pub seq_no: Option<i32>,
    pub  data: Option<serde_json::Value>
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct GetSchemaResultData {
    pub attr_names: HashSet<String>,
    pub name: String,
    pub origin: Option<String>,
    pub version: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTxnResult {
    pub identifier: String,
    #[serde(rename = "reqId")]
    pub req_id: u64,
    #[serde(rename = "seqNo")]
    pub seq_no: Option<i32>,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Option<serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaResult {
    pub identifier: String,
    #[serde(rename = "reqId")]
    pub req_id: u64,
    #[serde(rename = "seqNo")]
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Option<SchemaData>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaData {
    pub name: String,
    pub version: String,
    pub attr_names: HashSet<String>
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialOfferInfo {
    pub cred_def_id: String
}