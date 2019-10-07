use ursa::cl::MasterSecret as CryptoMasterSecret;

use named_type::NamedType;

use indy_api_types::validation::Validatable;

#[derive(Debug, Deserialize, Serialize, NamedType)]
pub struct MasterSecret {
    pub value: CryptoMasterSecret,
}

impl Validatable for MasterSecret {}
