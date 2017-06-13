use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Debug, PartialEq)]
pub struct ClaimDefinition {
    pub public_key: PublicKey,
    pub public_key_revocation: Option<String>,
    pub schema_seq_no: i32,
    pub signature_type: String
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Eq)]
pub struct PublicKey {
    pub n: String,
    pub s: String,
    pub rms: String,
    pub r: HashMap<String, String>,
    pub rctxt: String,
    pub z: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub op: String,
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
    pub  raw: String,
    pub  seq_no: Option<i32>
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
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

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetSchemaResultData {
    pub keys: HashSet<String>,
    pub name: String,
    pub origin: String,
    pub version: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct GetClaimDefReplyResult {
    pub identifier: String,
    #[serde(rename = "reqId")]
    pub req_id: u64,
    #[serde(rename = "seqNo")]
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: ClaimDefinitionData,
    pub origin: String,
    pub signature_type: String,
    #[serde(rename = "ref")]
    pub  _ref: i32
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub keys: HashSet<String>,
    pub seq_no: i32
}

#[derive(Deserialize, Debug, Serialize, Eq, PartialEq)]
pub struct ClaimDefinitionData {
    pub primary: PublicKey,
    pub revocation: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub claim_def_seq_no: i32,
    pub schema_seq_no: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofClaimsJson {
    pub attrs: HashMap<String, Vec<ClaimInfo>>,
    pub predicates: HashMap<String, Vec<ClaimInfo>>
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ClaimInfo {
    pub claim_uuid: String,
    pub claim_def_seq_no: i32,
    pub revoc_reg_seq_no: Option<i32>,
    pub schema_seq_no: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimRequestJson {
    pub claim_request: ClaimRequest,
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub prover_did: String,
    pub u: String,
    pub ur: Option<String>
}