use std::collections::{HashMap, HashSet};
use std::cell::RefCell;

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinition {
    #[serde(rename = "ref")]
    pub schema_seq_no: i32,
    #[serde(rename = "origin")]
    pub issuer_did: String,
    pub signature_type: String,
    pub data: ClaimDefinitionData
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinitionData {
    #[serde(rename = "primary")]
    pub public_key: PublicKey,
    #[serde(rename = "revocation")]
    pub public_key_revocation: Option<String>,
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct GetSchemaResultData {
    pub keys: HashSet<String>,
    pub name: String,
    pub origin: String,
    pub version: String
}

#[derive(Deserialize, PartialEq, Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTxnResult {
    pub identifier: String,
    #[serde(rename = "reqId")]
    pub req_id: u64,
    #[serde(rename = "seqNo")]
    pub seq_no: Option<i32>,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: String
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
    pub data: Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "seqNo")]
    pub seq_no: i32,
    pub data: SchemaData
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaData {
    pub name: String,
    pub version: String,
    pub keys: HashSet<String>
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub schema_seq_no: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofClaimsJson {
    pub attrs: HashMap<String, Vec<ClaimInfo>>,
    pub predicates: HashMap<String, Vec<ClaimInfo>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequestJson {
    pub nonce: String,
    pub name: String,
    pub version: String,
    pub requested_attrs: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, Predicate>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Predicate {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32,
    pub schema_seq_no: Option<i32>,
    pub issuer_did: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttributeInfo {
    pub name: String,
    pub schema_seq_no: Option<i32>,
    pub issuer_did: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ClaimInfo {
    pub claim_uuid: String,
    pub issuer_did: String,
    pub revoc_reg_seq_no: Option<i32>,
    pub schema_seq_no: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimRequestJson {
    pub blinded_ms: ClaimRequest,
    pub issuer_did: String,
    pub schema_seq_no: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub prover_did: String,
    pub u: String,
    pub ur: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimJson {
    pub claim: HashMap<String, Vec<String>>,
    pub revoc_reg_seq_no: Option<i32>,
    pub schema_seq_no: Option<i32>,
    pub signature: ClaimSignature,
    pub issuer_did: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimSignature {
    pub primary_claim: PrimaryClaim,
    pub non_revocation_claim: Option<RefCell<String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimaryClaim {
    pub m2: String,
    pub a: String,
    pub e: String,
    pub v: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofJson {
    pub proofs: HashMap<String, ClaimProof>,
    pub aggregated_proof: AggregatedProof,
    pub requested_proof: RequestedProofJson
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    pub primary_proof: PrimaryProof,
    pub non_revoc_proof: Option<NonRevocProof>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NonRevocProof {
    pub x_list: NonRevocProofXList,
    pub c_list: NonRevocProofCList
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonRevocProofCList {
    pub e: String,
    pub d: String,
    pub a: String,
    pub g: String,
    pub w: String,
    pub s: String,
    pub u: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonRevocProofXList {
    pub rho: String,
    pub r: String,
    pub r_prime: String,
    pub r_prime_prime: String,
    pub r_prime_prime_prime: String,
    pub o: String,
    pub o_prime: String,
    pub m: String,
    pub m_prime: String,
    pub t: String,
    pub t_prime: String,
    pub m2: String,
    pub s: String,
    pub c: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryProof {
    pub eq_proof: PrimaryEqualProof,
    pub ge_proofs: Vec<PrimaryPredicateGEProof>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryPredicateGEProof {
    pub u: HashMap<String, String>,
    pub r: HashMap<String, String>,
    pub mj: String,
    pub alpha: String,
    pub t: HashMap<String, String>,
    pub predicate: Predicate
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryEqualProof {
    pub revealed_attrs: HashMap<String, String>,
    pub a_prime: String,
    pub e: String,
    pub v: String,
    pub m: HashMap<String, String>,
    pub m1: String,
    pub m2: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimProof {
    pub proof: Proof,
    pub revoc_reg_seq_no: Option<i32>,
    pub schema_seq_no: i32,
    pub issuer_did: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregatedProof {
    pub c_hash: String,
    pub c_list: Vec<Vec<u8>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedProofJson {
    pub revealed_attrs: HashMap<String, (String, String, String)>,
    pub unrevealed_attrs: HashMap<String, String>,
    pub self_attested_attrs: HashMap<String, String>,
    pub predicates: HashMap<String, String>
}
