extern crate indy_crypto;

use self::indy_crypto::cl::{
    BlindedMasterSecret,
    BlindedMasterSecretCorrectnessProof,
    MasterSecretBlindingData,
    Nonce
};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequest {
    pub prover_did: String,
    pub cred_def_id: String,
    pub blinded_ms: BlindedMasterSecret,
    pub blinded_ms_correctness_proof: BlindedMasterSecretCorrectnessProof,
    pub nonce: Nonce,
}

impl JsonEncodable for CredentialRequest {}

impl<'a> JsonDecodable<'a> for CredentialRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequestMetadata {
    pub master_secret_blinding_data: MasterSecretBlindingData,
    pub nonce: Nonce,
    pub master_secret_name: String
}

impl JsonEncodable for CredentialRequestMetadata {}

impl<'a> JsonDecodable<'a> for CredentialRequestMetadata {}