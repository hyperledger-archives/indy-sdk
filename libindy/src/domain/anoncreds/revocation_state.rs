extern crate indy_crypto;

use self::indy_crypto::cl::{Witness, RevocationRegistry};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use named_type::NamedType;

#[derive(Clone, Debug, Serialize, Deserialize, NamedType)]
pub struct RevocationState {
    pub witness: Witness,
    pub rev_reg: RevocationRegistry,
    pub timestamp: u64
}

impl JsonEncodable for RevocationState {}

impl<'a> JsonDecodable<'a> for RevocationState {}