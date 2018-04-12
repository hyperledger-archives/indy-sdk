extern crate indy_crypto;
extern crate serde;
extern crate serde_json;

use self::indy_crypto::utils::json::{JsonEncodable, JsonDecodable};
use self::indy_crypto::cl::{RevocationRegistryDelta as RegistryDelta};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDeltaV1 {
    pub value: RegistryDelta
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "ver")]
pub enum RevocationRegistryDelta {
    #[serde(rename = "1.0")]
    RevocationRegistryDeltaV1(RevocationRegistryDeltaV1)
}

impl JsonEncodable for RevocationRegistryDelta {}

impl<'a> JsonDecodable<'a> for RevocationRegistryDelta {}

impl From<RevocationRegistryDelta> for RevocationRegistryDeltaV1 {
    fn from(rev_reg_delta: RevocationRegistryDelta) -> Self {
        match rev_reg_delta {
            RevocationRegistryDelta::RevocationRegistryDeltaV1(rev_reg_delta) => rev_reg_delta,
        }
    }
}