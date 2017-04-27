use utils::json::{JsonDecodable, JsonEncodable};
use services::crypto::anoncreds::types::{ClaimRequest, Claims};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

impl JsonEncodable for ClaimOffer {}

impl<'a> JsonDecodable<'a> for ClaimOffer {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimRequestJson {
    pub claim_request: ClaimRequest,
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

impl JsonEncodable for ClaimRequestJson {}

impl<'a> JsonDecodable<'a> for ClaimRequestJson {}

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

impl JsonEncodable for ClaimJson {}

impl<'a> JsonDecodable<'a> for ClaimJson {}