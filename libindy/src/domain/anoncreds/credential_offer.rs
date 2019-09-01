use ursa::cl::{CredentialKeyCorrectnessProof, Nonce};

use super::schema::SchemaId;
use super::credential_definition::CredentialDefinitionId;

use utils::validation::Validatable;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOffer {
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub key_correctness_proof: CredentialKeyCorrectnessProof,
    pub nonce: Nonce
}

impl Validatable for CredentialOffer {
    fn validate(&self) -> Result<(), String> {
        self.schema_id.validate()?;
        self.cred_def_id.validate()?;
        Ok(())
    }
}