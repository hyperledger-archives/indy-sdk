extern crate indy_crypto;

use std::collections::HashMap;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use super::credential::CredentialInfo;
use super::proof_request::NonRevocedInterval;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialsForProofRequest {
    pub attrs: HashMap<String, Vec<RequestedCredential>>,
    pub predicates: HashMap<String, Vec<RequestedCredential>>
}

impl JsonEncodable for CredentialsForProofRequest {}

impl<'a> JsonDecodable<'a> for CredentialsForProofRequest {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestedCredential {
    pub cred_info: CredentialInfo,
    pub interval: Option<NonRevocedInterval>
}