extern crate indy_crypto;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use named_type::NamedType;


#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Serialize, Deserialize, Clone, NamedType)]
pub struct Key {
    pub verkey: String,
    #[cfg(not(test))]
    #[derivative(Debug="ignore")]
    pub signkey: String,
    #[cfg(test)]
    pub signkey: String
}

impl Key {
    pub fn new(verkey: String, signkey: String) -> Key {
        Key {
            verkey,
            signkey
        }
    }
}

impl JsonEncodable for Key {}

impl<'a> JsonDecodable<'a> for Key {}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

impl JsonEncodable for KeyInfo {}

impl<'a> JsonDecodable<'a> for KeyInfo {}