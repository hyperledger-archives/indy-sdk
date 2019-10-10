use std::collections::HashMap;

use ursa::cl::Proof as CryptoProof;

use super::schema::SchemaId;
use super::credential_definition::CredentialDefinitionId;
use super::revocation_registry_definition::RevocationRegistryId;
use crate::utils::validation::Validatable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    pub proof: CryptoProof,
    pub requested_proof: RequestedProof,
    pub identifiers: Vec<Identifier>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedProof {
    pub revealed_attrs: HashMap<String, RevealedAttributeInfo>,
    pub self_attested_attrs: HashMap<String, String>,
    pub unrevealed_attrs: HashMap<String, SubProofReferent>,
    pub predicates: HashMap<String, SubProofReferent>
}

impl Default for RequestedProof {
    fn default() -> Self {
        RequestedProof {
            revealed_attrs: HashMap::new(),
            self_attested_attrs: HashMap::new(),
            unrevealed_attrs: HashMap::new(),
            predicates: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubProofReferent {
    pub sub_proof_index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RevealedAttributeInfo {
    pub sub_proof_index: u32,
    pub raw: String,
    pub encoded: String
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub rev_reg_id: Option<RevocationRegistryId>,
    pub timestamp: Option<u64>
}

impl Validatable for Proof {}