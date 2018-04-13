extern crate indy_crypto;

use self::indy_crypto::cl::{CredentialKeyCorrectnessProof, Nonce};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOffer {
    pub schema_id: String,
    pub cred_def_id: String,
    pub key_correctness_proof: CredentialKeyCorrectnessProof,
    pub nonce: Nonce
}

impl JsonEncodable for CredentialOffer {}

impl<'a> JsonDecodable<'a> for CredentialOffer {}