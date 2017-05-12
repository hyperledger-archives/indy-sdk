mod ed25519;
pub mod types;

use self::ed25519::ED25519Signus;
use self::types::{
    MyDidInfo,
    MyDid,
    TheirDidInfo
};
use utils::crypto::base58::Base58;

use errors::crypto::CryptoError;
use errors::signus::SignusError;
use std::collections::HashMap;

const DEFAULT_CRYPTO_TYPE: &'static str = "ed25519";

trait CryptoType {
    fn create_key_pair(&self) -> (Vec<u8>, Vec<u8>);
    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Vec<u8>;
    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn gen_nonce(&self) -> Vec<u8>;
    fn create_key_pair_for_signature(&self, seed: Option<&[u8]>) -> (Vec<u8>, Vec<u8>);
    fn sign(&self, private_key: &[u8], doc: &[u8]) -> Vec<u8>;
    fn verify(&self, public_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

pub struct SignusService {
    crypto_types: HashMap<&'static str, Box<CryptoType>>
}

impl SignusService {
    pub fn new() -> SignusService {
        let mut crypto_types: HashMap<&str, Box<CryptoType>> = HashMap::new();
        crypto_types.insert(DEFAULT_CRYPTO_TYPE, Box::new(ED25519Signus::new()));

        SignusService {
            crypto_types: crypto_types
        }
    }

    pub fn create_my_did(&self, did_info: &MyDidInfo) -> Result<MyDid, SignusError> {
        let xtype = did_info.crypto_type.clone().unwrap_or(DEFAULT_CRYPTO_TYPE.to_string());

        if !self.crypto_types.contains_key(&xtype.as_str()) {
            return Err(SignusError::CryptoError(CryptoError::UnknownType(xtype)));
        }

        let signus = self.crypto_types.get(&xtype.as_str()).unwrap();

        let seed = did_info.seed.as_ref().map(String::as_bytes);
        let (public_key, secret_key) = signus.create_key_pair();
        let (ver_key, sign_key) = signus.create_key_pair_for_signature(seed);
        let did = did_info.did.as_ref().map(|did| Base58::decode(did)).unwrap_or(Ok(ver_key[0..16].to_vec()))?;

        let my_did = MyDid::new(Base58::encode(&did),
                                xtype.clone(),
                                Base58::encode(&public_key),
                                Base58::encode(&secret_key),
                                Base58::encode(&ver_key),
                                Base58::encode(&sign_key));

        Ok(my_did)
    }

    pub fn sign(&self, my_did: &MyDid, doc: &str) -> Result<String, SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(SignusError::CryptoError(CryptoError::UnknownType(my_did.crypto_type.clone())));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();

        let sign_key = Base58::decode(&my_did.sign_key)?;
        let signature = signus.sign(&sign_key, doc.as_bytes());
        let signature = Base58::encode(&signature);

        Ok(signature)
    }

    pub fn verify(&self, their_did: &TheirDidInfo, signature: &str) -> Result<bool, SignusError> {
        let xtype = their_did.crypto_type.clone().unwrap_or(DEFAULT_CRYPTO_TYPE.to_string());

        if !self.crypto_types.contains_key(&xtype.as_str()) {
            return Err(SignusError::CryptoError(CryptoError::UnknownType(xtype)));
        }

        let signus = self.crypto_types.get(&xtype.as_str()).unwrap();

        let verkey = Base58::decode(&their_did.verkey)?;
        let signature = Base58::decode(signature)?;

        let valid = signus.verify(&verkey, &signature).is_ok();

        Ok(valid)
    }

    pub fn encrypt(&self, my_did: &MyDid, public_key: &str, doc: &str) -> Result<(String, String), SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(SignusError::CryptoError(CryptoError::UnknownType(my_did.crypto_type.clone())));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();

        let nonce = signus.gen_nonce();

        let secret_key = Base58::decode(&my_did.secret_key)?;
        let public_key = Base58::decode(&public_key)?;
        let doc = Base58::decode(&doc)?;

        let encrypted_doc = signus.encrypt(&secret_key, &public_key, &doc, &nonce);
        let encrypted_doc = Base58::encode(&encrypted_doc);
        let nonce = Base58::encode(&nonce);
        Ok((encrypted_doc, nonce))
    }

    pub fn decrypt(&self, my_did: &MyDid, their_did: &TheirDidInfo, doc: &str, nonce: &str) -> Result<String, SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(SignusError::CryptoError(CryptoError::UnknownType(my_did.crypto_type.clone())));
        }

        if their_did.pk.is_none() {
            return Err(SignusError::CryptoError(CryptoError::BackendError(format!("Public key not found"))));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();
        let public_key = their_did.pk.clone().unwrap();

        let secret_key = Base58::decode(&my_did.secret_key)?;
        let public_key = Base58::decode(&public_key)?;
        let doc = Base58::decode(&doc)?;
        let nonce = Base58::decode(&nonce)?;

        let decrypted_doc = signus.decrypt(&secret_key, &public_key, &doc, &nonce)?;
        let decrypted_doc = Base58::encode(&decrypted_doc);
        Ok(decrypted_doc)
    }
}