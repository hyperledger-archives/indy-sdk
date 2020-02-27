use ursa::cl::{Witness, RevocationRegistry};
use std::collections::HashMap;

use indy_api_types::validation::Validatable;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

pub type RevocationStates = HashMap<String, HashMap<u64, RevocationState>>;
