use ursa::cl::{Witness, RevocationRegistry};

use named_type::NamedType;

#[derive(Clone, Debug, Serialize, Deserialize, NamedType)]
pub struct RevocationState {
    pub witness: Witness,
    pub rev_reg: RevocationRegistry,
    pub timestamp: u64
}
