use indy_crypto::cl::MasterSecret as CryptoMasterSecret;

use named_type::NamedType;

#[derive(Debug, Deserialize, Serialize, NamedType)]
pub struct MasterSecret {
    pub value: CryptoMasterSecret,
}