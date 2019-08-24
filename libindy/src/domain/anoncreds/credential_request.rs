use ursa::cl::{
    BlindedCredentialSecrets,
    BlindedCredentialSecretsCorrectnessProof,
    CredentialSecretsBlindingFactors,
    Nonce
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequest {
    pub prover_did: String,
    pub cred_def_id: String,
    pub blinded_ms: BlindedCredentialSecrets,
    pub blinded_ms_correctness_proof: BlindedCredentialSecretsCorrectnessProof,
    pub nonce: Nonce,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequestMetadata {
    pub master_secret_blinding_data: CredentialSecretsBlindingFactors,
    pub nonce: Nonce,
    pub master_secret_name: String
}