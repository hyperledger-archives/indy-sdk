extern crate indy_crypto;

use std::collections::HashMap;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCredentials {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attributes: HashMap<String, RequestedAttribute>,
    pub requested_predicates: HashMap<String, ProvingCredentialKey>
}

impl JsonEncodable for RequestedCredentials {}

impl<'a> JsonDecodable<'a> for RequestedCredentials {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedAttribute {
    pub cred_id: String,
    pub timestamp: Option<u64>,
    pub revealed: bool
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub struct ProvingCredentialKey {
    pub cred_id: String,
    pub timestamp: Option<u64>
}