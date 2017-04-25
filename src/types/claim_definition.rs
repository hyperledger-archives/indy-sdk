use utils::json::{JsonEncodable};

#[derive(Serialize, Debug)]
pub struct ClaimDefinition {
    pub public_key: String,
    pub schema: String,
    pub signature_type: String
}

impl JsonEncodable for ClaimDefinition {}
