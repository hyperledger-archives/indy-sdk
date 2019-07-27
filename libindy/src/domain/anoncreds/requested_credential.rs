use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCredentials {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attributes: HashMap<String, RequestedAttribute>,
    pub requested_predicates: HashMap<String, ProvingCredentialKey>
}

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
