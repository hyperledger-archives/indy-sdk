extern crate indy_crypto;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}