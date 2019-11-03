use std::collections::HashMap;

use indy_api_types::validation::Validatable;

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

impl Validatable for RequestedCredentials {
    fn validate(&self) -> Result<(), String> {
        if self.self_attested_attributes.is_empty() && self.requested_attributes.is_empty() && self.requested_predicates.is_empty() {
            return Err(String::from("Requested Credentials validation failed: `self_attested_attributes` and `requested_attributes` and `requested_predicates` are empty"));
        }
        Ok(())
    }
}