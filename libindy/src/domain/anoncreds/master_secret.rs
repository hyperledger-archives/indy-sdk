extern crate indy_crypto;

use self::indy_crypto::cl::MasterSecret as CryptoMasterSecret;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use named_type::NamedType;

#[derive(Debug, Deserialize, Serialize, NamedType)]
pub struct MasterSecret {
    pub value: CryptoMasterSecret,
}

impl JsonEncodable for MasterSecret {}

impl<'a> JsonDecodable<'a> for MasterSecret {}