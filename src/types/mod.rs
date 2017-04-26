use utils::json::{JsonDecodable, JsonEncodable};
use services::crypto::anoncreds::types::ClaimRequest;

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