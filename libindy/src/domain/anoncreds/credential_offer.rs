use ursa::cl::{CredentialKeyCorrectnessProof, Nonce};

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOffer {
    pub schema_id: String,
    pub cred_def_id: String,
    pub key_correctness_proof: CredentialKeyCorrectnessProof,
    pub nonce: Nonce
}