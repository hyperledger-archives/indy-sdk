use ursa::cl::MasterSecret as CryptoMasterSecret;

use indy_api_types::validation::Validatable;

#[derive(Debug, Deserialize, Serialize)]
pub struct MasterSecret {
    pub value: CryptoMasterSecret,
}

impl Validatable for MasterSecret {}
