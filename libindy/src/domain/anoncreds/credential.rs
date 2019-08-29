use ursa::cl::{
    CredentialSignature,
    RevocationRegistry,
    SignatureCorrectnessProof,
    Witness
};

use super::DELIMITER;
use super::schema::SchemaId;
use super::credential_definition::CredentialDefinitionId;
use super::revocation_registry_definition::RevocationRegistryId;

use std::collections::HashMap;
use named_type::NamedType;

use utils::validation::Validatable;

#[derive(Debug, Deserialize, Serialize, NamedType)]
pub struct Credential {
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub rev_reg_id: Option<RevocationRegistryId>,
    pub values: CredentialValues,
    pub signature: CredentialSignature,
    pub signature_correctness_proof: SignatureCorrectnessProof,
    pub rev_reg: Option<RevocationRegistry>,
    pub witness: Option<Witness>
}

impl Credential {
    fn schema_parts(&self) -> Vec<&str> {
        self.schema_id.0.split_terminator(DELIMITER).collect::<Vec<&str>>()
    }

    pub fn schema_id(&self) -> String { self.schema_id.0.to_string() }

    pub fn schema_issuer_did(&self) -> String {
        self.schema_parts().get(0).map(|val| val.to_string()).unwrap_or_default()
    }

    pub fn schema_name(&self) -> String {
        self.schema_parts().get(2).map(|val| val.to_string()).unwrap_or_default()
    }

    pub fn schema_version(&self) -> String {
        self.schema_parts().get(3).map(|val| val.to_string()).unwrap_or_default()
    }

    pub fn issuer_did(&self) -> String {
        self.cred_def_id.0.split_terminator(DELIMITER).collect::<Vec<&str>>()[0].to_string()
    }

    pub fn cred_def_id(&self) -> String { self.cred_def_id.0.to_string() }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialInfo {
    pub referent: String,
    pub attrs: ShortCredentialValues,
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub rev_reg_id: Option<RevocationRegistryId>,
    pub cred_rev_id: Option<String>
}

pub type ShortCredentialValues = HashMap<String, String>;

pub type CredentialValues = HashMap<String, AttributeValues>;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct AttributeValues {
    pub raw: String,
    pub encoded: String
}

impl Validatable for CredentialValues {
    fn validate(&self) -> Result<(), String> {
        if self.is_empty() {
            return Err(String::from("CredentialValues validation failed: empty list has been passed"));
        }

        Ok(())
    }
}

impl Validatable for Credential {
    fn validate(&self) -> Result<(), String> {
        self.schema_id.validate()?;
        self.cred_def_id.validate()?;
        self.values.validate()?;

        if self.rev_reg_id.is_some() && (self.witness.is_none() ||self.rev_reg.is_none()){
            return Err(String::from("Credential validation failed: `witness` and `rev_reg` must be passed for revocable Credential"));
        }

        if self.values.is_empty() {
            return Err(String::from("Credential validation failed: `values` is empty"));
        }

        Ok(())
    }
}