use std::collections::HashMap;

use super::credential::CredentialInfo;
use super::proof_request::NonRevocedInterval;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialsForProofRequest {
    pub attrs: HashMap<String, Vec<RequestedCredential>>,
    pub predicates: HashMap<String, Vec<RequestedCredential>>
}

impl Default for CredentialsForProofRequest {
    fn default() -> Self {
        CredentialsForProofRequest { attrs: HashMap::new(), predicates: HashMap::new() }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestedCredential {
    pub cred_info: CredentialInfo,
    pub interval: Option<NonRevocedInterval>
}