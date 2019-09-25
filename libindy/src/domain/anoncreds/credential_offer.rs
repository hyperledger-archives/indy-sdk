use ursa::cl::{CredentialKeyCorrectnessProof, Nonce};

use super::schema::SchemaId;
use super::credential_definition::CredentialDefinitionId;

use utils::validation::Validatable;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOffer {
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub key_correctness_proof: CredentialKeyCorrectnessProof,
    pub nonce: Nonce,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
}

impl CredentialOffer {
    pub fn disqualify(self) -> CredentialOffer {
        CredentialOffer {
            schema_id: self.schema_id.disqualify(),
            cred_def_id: self.cred_def_id.disqualify(),
            key_correctness_proof: self.key_correctness_proof,
            nonce: self.nonce,
            method_name: if self.schema_id.is_fully_qualified(){ self.schema_id.get_method()} else { None },
        }
    }
}

impl Validatable for CredentialOffer {
    fn validate(&self) -> Result<(), String> {
        self.schema_id.validate()?;
        self.cred_def_id.validate()?;
        Ok(())
    }
}