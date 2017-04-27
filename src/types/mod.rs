use utils::json::{JsonDecodable, JsonEncodable};
use services::crypto::anoncreds::types::{AccumulatorPublicKey, ClaimRequest, Claims, Predicate};
use services::crypto::wrappers::pair::PointG1;
use services::crypto::wrappers::bn::BigNumber;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

impl ClaimOffer {
    pub fn new(issuer_did: String, claim_def_seq_no: i32) -> ClaimOffer {
        ClaimOffer {
            issuer_did: issuer_did,
            claim_def_seq_no: claim_def_seq_no
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimRequestJson {
    pub claim_request: ClaimRequest,
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

impl ClaimRequestJson {
    pub fn new(claim_request: ClaimRequest, issuer_did: String, claim_def_seq_no: i32) -> ClaimRequestJson {
        ClaimRequestJson {
            claim_request: claim_request,
            issuer_did: issuer_did,
            claim_def_seq_no: claim_def_seq_no
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimJson {
    pub claim: HashMap<String, Vec<String>>,
    pub claim_def_seq_no: i32,
    pub revoc_reg_seq_no: i32,
    pub signature: Claims
}

impl ClaimJson {
    pub fn new(claim: HashMap<String, Vec<String>>, claim_def_seq_no: i32, revoc_reg_seq_no: i32,
               signature: Claims) -> ClaimJson {
        ClaimJson {
            claim: claim,
            claim_def_seq_no: claim_def_seq_no,
            revoc_reg_seq_no: revoc_reg_seq_no,
            signature: signature
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequestJson {
    nonce: BigNumber,
    requested_attr: HashMap<String, String>,
    requested_predicate: HashMap<String, Predicate>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RevocationRegistryJson {
    pub claim_def_seq_no: i32,
    pub accumulator: PointG1,
    pub v: HashSet<i32>,
    pub accumulator_pk: AccumulatorPublicKey,
}

impl RevocationRegistryJson {
    pub fn new(claim_def_seq_no: i32, accumulator: PointG1, v: HashSet<i32>,
               accumulator_pk: AccumulatorPublicKey) -> RevocationRegistryJson {
        RevocationRegistryJson {
            claim_def_seq_no: claim_def_seq_no,
            accumulator: accumulator,
            v: v,
            accumulator_pk: accumulator_pk
        }
    }
}

impl JsonEncodable for ClaimOffer {}

impl<'a> JsonDecodable<'a> for ClaimOffer {}

impl JsonEncodable for ClaimRequestJson {}

impl<'a> JsonDecodable<'a> for ClaimRequestJson {}

impl JsonEncodable for ClaimJson {}

impl<'a> JsonDecodable<'a> for ClaimJson {}

impl JsonEncodable for RevocationRegistryJson {}

impl<'a> JsonDecodable<'a> for RevocationRegistryJson {}

impl JsonEncodable for ProofRequestJson {}

impl<'a> JsonDecodable<'a> for ProofRequestJson {}