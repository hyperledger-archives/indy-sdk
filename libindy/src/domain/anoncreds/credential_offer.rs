use ursa::cl::{CredentialKeyCorrectnessProof, Nonce};

use super::schema::SchemaId;
use super::credential_definition::CredentialDefinitionId;

use crate::utils::validation::Validatable;

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
    pub fn to_unqualified(self) -> CredentialOffer {
        let method_name= if self.cred_def_id.is_fully_qualified(){ self.cred_def_id.get_method()} else { None };
        CredentialOffer {
            method_name,
            schema_id: self.schema_id.to_unqualified(),
            cred_def_id: self.cred_def_id.to_unqualified(),
            key_correctness_proof: self.key_correctness_proof,
            nonce: self.nonce,
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