extern crate indy_crypto;

use std::collections::HashMap;

use self::indy_crypto::cl::Proof as CryptoProof;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    pub proof: CryptoProof,
    pub requested_proof: RequestedProof,
    pub identifiers: Vec<Identifier>
}

impl JsonEncodable for Proof {}

impl<'a> JsonDecodable<'a> for Proof {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedProof {
    pub revealed_attrs: HashMap<String, RevealedAttributeInfo>,
    pub self_attested_attrs: HashMap<String, String>,
    pub unrevealed_attrs: HashMap<String, SubProofReferent>,
    pub predicates: HashMap<String, SubProofReferent>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubProofReferent {
    pub sub_proof_index: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RevealedAttributeInfo {
    pub sub_proof_index: i32,
    pub raw: String,
    pub encoded: String
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub timestamp: Option<u64>
}