use ursa::cl::{Witness, RevocationRegistry};
use std::collections::HashMap;

use named_type::NamedType;

use indy_api_types::validation::Validatable;

use super::revocation_registry_definition::RevocationRegistryId;

#[derive(Clone, Debug, Serialize, Deserialize, NamedType)]
pub struct RevocationState {
    pub witness: Witness,
    pub rev_reg: RevocationRegistry,
    pub timestamp: u64
}

impl Validatable for RevocationState {
    fn validate(&self) -> Result<(), String> {
        if self.timestamp == 0 {
            return Err(String::from("RevocationState validation failed: `timestamp` must be greater than 0"));
        }
        Ok(())
    }
}

pub type RevocationStates = HashMap<RevocationRegistryId, HashMap<u64, RevocationState>>;
