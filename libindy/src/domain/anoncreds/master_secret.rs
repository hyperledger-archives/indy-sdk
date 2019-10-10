use ursa::cl::MasterSecret as CryptoMasterSecret;

use named_type::NamedType;

use crate::utils::validation::Validatable;

#[derive(Debug, Deserialize, Serialize, NamedType)]
pub struct MasterSecret {
    pub value: CryptoMasterSecret,
}

impl Validatable for MasterSecret {}
