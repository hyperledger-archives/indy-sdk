use ursa::cl::{
    BlindedCredentialSecrets,
    BlindedCredentialSecretsCorrectnessProof,
    CredentialSecretsBlindingFactors,
    Nonce
};

use super::credential_definition::CredentialDefinitionId;

use utils::validation::Validatable;

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequest {
    pub prover_did: String,
    pub cred_def_id: CredentialDefinitionId,
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

impl Validatable for CredentialRequest {
    fn validate(&self) -> Result<(), String> {
        self.cred_def_id.validate()?;
        Ok(())
    }
}

impl Validatable for CredentialRequestMetadata {}