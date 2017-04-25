use utils::json::{JsonDecodable, JsonEncodable};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimDefinition {
    pub public_key: String,
    pub schema: String,
    pub signature_type: String
}

impl JsonEncodable for ClaimDefinition {}
impl<'a> JsonDecodable<'a> for ClaimDefinition {}
