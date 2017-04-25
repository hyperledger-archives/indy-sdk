use utils::json::{JsonDecodable, JsonEncodable};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub claim_def_seq_no: String
}

impl JsonEncodable for ClaimOffer {}

impl<'a> JsonDecodable<'a> for ClaimOffer {}
