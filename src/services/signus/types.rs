use utils::json::{JsonEncodable, JsonDecodable};

#[derive(Serialize, Deserialize)]
pub struct DIDInfo {
    pub did: Option<String>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

impl DIDInfo {
    pub fn new(did: Option<String>, seed: Option<String>, crypto_type: Option<String>) -> DIDInfo {
        DIDInfo {
            did: did,
            seed: seed,
            crypto_type: crypto_type
        }
    }
}

impl JsonEncodable for DIDInfo {}

impl<'a> JsonDecodable<'a> for DIDInfo {}