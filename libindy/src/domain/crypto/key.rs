extern crate indy_crypto;
extern crate zeroize;

use self::zeroize::Zeroize;

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

//Can't figure out why this macro can't be found in `cargo test` because
//if works for src/utils/crypto/randombytes/sodium
//memzeroize!(Key, signkey);

impl Zeroize for Key {
    fn zeroize(&mut self) { self.signkey.zeroize(); }
}

impl Drop for Key {
    fn drop(&mut self) {
        self.signkey.zeroize();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct KeyMetadata {
    pub value: String
}