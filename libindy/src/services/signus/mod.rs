mod ed25519;
pub mod types;

use self::ed25519::ED25519Signus;
use self::types::{
    MyDidInfo,
    MyDid,
    TheirDidInfo,
    TheirDid
};
use utils::crypto::base58::Base58;
use utils::crypto::verkey_builder::build_full_verkey;

use errors::common::CommonError;
use errors::signus::SignusError;

use std::collections::HashMap;
use std::str;

const DEFAULT_CRYPTO_TYPE: &'static str = "ed25519";

trait CryptoType {
    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn gen_nonce(&self) -> Vec<u8>;
    fn create_key_pair_for_signature(&self, seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CommonError>;
    fn sign(&self, private_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn verify(&self, public_key: &[u8], doc: &[u8], signature: &[u8]) -> Result<bool, CommonError>;
    fn verkey_to_public_key(&self, vk: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn signkey_to_private_key(&self, sk: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn encrypt_sealed(&self, public_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn decrypt_sealed(&self, public_key: &[u8], private_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
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

    pub fn create_my_did(&self, my_did_info: &MyDidInfo) -> Result<MyDid, SignusError> {
        let xtype = my_did_info.crypto_type.clone().unwrap_or(DEFAULT_CRYPTO_TYPE.to_string());

        if !self.crypto_types.contains_key(&xtype.as_str()) {
            return Err(
                SignusError::UnknownCryptoError(
                    format!("MyDidInfo info contains unknown crypto: {}", xtype)));
        }

        let signus = self.crypto_types.get(&xtype.as_str()).unwrap();

        let seed = my_did_info.seed.as_ref().map(String::as_bytes);
        let (ver_key, sign_key) = signus.create_key_pair_for_signature(seed)?;

        let public_key = signus.verkey_to_public_key(&ver_key)?;
        let secret_key = signus.signkey_to_private_key(&sign_key)?;

        let did = match my_did_info.did {
            Some(ref did) => Base58::decode(did)?,
            _ if my_did_info.cid == Some(true) => ver_key.clone(),
            _ => ver_key[0..16].to_vec()
        };

        let my_did = MyDid::new(Base58::encode(&did),
                                xtype.clone(),
                                Base58::encode(&public_key),
                                Base58::encode(&secret_key),
                                Base58::encode(&ver_key),
                                Base58::encode(&sign_key));

        Ok(my_did)
    }

    pub fn create_their_did(&self, their_did_info: &TheirDidInfo) -> Result<TheirDid, SignusError> {
        let xtype = their_did_info.crypto_type.clone().unwrap_or(DEFAULT_CRYPTO_TYPE.to_string());

        if !self.crypto_types.contains_key(&xtype.as_str()) {
            return Err(
                SignusError::UnknownCryptoError(
                    format!("TheirDidInfo info contains unknown crypto: {}", xtype)));
        }

        let signus = self.crypto_types.get(&xtype.as_str()).unwrap();

        // Check did is correct Base58
        Base58::decode(&their_did_info.did)?;

        let (verkey, pk) = match their_did_info.verkey {
            Some(ref verkey) => {
                let full_verkey = build_full_verkey(&their_did_info.did, &Some(verkey.clone()))
                    .map_err(|err| CommonError::InvalidState(format!("Invalid verkey {:?}", err)))?;
                (Some(Base58::encode(&full_verkey)),
                 Some(Base58::encode(&signus.verkey_to_public_key(&full_verkey)?)))
            }
            None => (None, None)
        };

        let their_did = TheirDid::new(their_did_info.did.clone(),
                                      xtype.clone(),
                                      verkey,
                                      pk,
                                      their_did_info.endpoint.as_ref().cloned());
        Ok(their_did)
    }

    pub fn sign(&self, my_did: &MyDid, doc: &[u8]) -> Result<Vec<u8>, SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(
                SignusError::UnknownCryptoError(
                    format!("Trying to sign message with unknown crypto: {}", my_did.crypto_type)));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();

        let sign_key = Base58::decode(&my_did.signkey)?;

        let signature = signus.sign(&sign_key, doc)?;

        Ok(signature)
    }

    pub fn verify(&self, their_did: &TheirDid, msg: &[u8], signature: &[u8]) -> Result<bool, SignusError> {
        if !self.crypto_types.contains_key(their_did.crypto_type.as_str()) {
            return Err(SignusError::UnknownCryptoError(format!("Trying to verify message with unknown crypto: {}", their_did.crypto_type)));
        }

        let signus = self.crypto_types.get(their_did.crypto_type.as_str()).unwrap();

        let verkey = match their_did.verkey {
            Some(ref verkey) => Base58::decode(&verkey)?,
            None => return Err(SignusError::CommonError(CommonError::InvalidStructure(format!("TheirDid doesn't contain verkey: {}", their_did.did))))
        };

        Ok(signus.verify(&verkey, msg, signature)?)
    }

    pub fn encrypt(&self, my_did: &MyDid, their_did: &TheirDid, doc: &[u8]) -> Result<(Vec<u8>, Vec<u8>), SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(SignusError::UnknownCryptoError(format!("Trying to encrypt message with unknown crypto: {}", my_did.crypto_type)));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();

        if their_did.pk.is_none() {
            return Err(SignusError::CommonError(CommonError::InvalidStructure(format!("TheirDid doesn't contain pk: {}", their_did.did))));
        }

        let public_key = their_did.pk.clone().unwrap();

        let nonce = signus.gen_nonce();

        let secret_key = Base58::decode(&my_did.sk)?;
        let public_key = Base58::decode(&public_key)?;

        let encrypted_doc = signus.encrypt(&secret_key, &public_key, doc, &nonce)?;
        Ok((encrypted_doc, nonce))
    }

    pub fn decrypt(&self, my_did: &MyDid, their_did: &TheirDid, doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(SignusError::UnknownCryptoError(format!("MyDid crypto is unknown: {}, {}", my_did.did, my_did.crypto_type)));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();

        if their_did.pk.is_none() {
            return Err(SignusError::CommonError(
                CommonError::InvalidStructure(format!("No pk in TheirDid: {}", their_did.did))));
        }

        let public_key = their_did.pk.clone().unwrap();

        let secret_key = Base58::decode(&my_did.sk)?;
        let public_key = Base58::decode(&public_key)?;

        let decrypted_doc = signus.decrypt(&secret_key, &public_key, &doc, &nonce)?;

        Ok(decrypted_doc)
    }

    pub fn encrypt_sealed(&self, their_did: &TheirDid, doc: &[u8]) -> Result<Vec<u8>, SignusError> {
        if !self.crypto_types.contains_key(&their_did.crypto_type.as_str()) {
            return Err(SignusError::UnknownCryptoError(format!("Trying to encrypt message with unknown crypto: {}", their_did.crypto_type)));
        }

        let signus = self.crypto_types.get(&their_did.crypto_type.as_str()).unwrap();

        if their_did.pk.is_none() {
            return Err(SignusError::CommonError(CommonError::InvalidStructure(format!("TheirDid doesn't contain pk: {}", their_did.did))));
        }

        let public_key = their_did.pk.clone().unwrap();
        let public_key = Base58::decode(&public_key)?;

        let encrypted_doc = signus.encrypt_sealed(&public_key, doc)?;
        Ok(encrypted_doc)
    }

    pub fn decrypt_sealed(&self, my_did: &MyDid, doc: &[u8]) -> Result<Vec<u8>, SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(SignusError::UnknownCryptoError(format!("Trying to encrypt message with unknown crypto: {}", my_did.crypto_type)));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();

        let public_key = Base58::decode(&my_did.pk)?;
        let private_key = Base58::decode(&my_did.sk)?;

        let decrypted_doc = signus.decrypt_sealed(&public_key, &private_key, doc)?;
        Ok(decrypted_doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use services::signus::types::MyDidInfo;

    #[test]
    fn create_my_did_with_works_for_empty_info() {
        let service = SignusService::new();
        let did_info = MyDidInfo::new(None, None, None, None);
        service.create_my_did(&did_info).unwrap();
    }

    #[test]
    fn create_my_did_works_for_passed_did() {
        let service = SignusService::new();

        let did = Some("Dbf2fjCbsiq2kfns".to_string());
        let did_info = MyDidInfo::new(did.clone(), None, None, None);

        let my_did = service.create_my_did(&did_info).unwrap();

        assert_eq!(did.unwrap(), my_did.did);
    }

    #[test]
    fn create_my_did_not_works_for_invalid_crypto_type() {
        let service = SignusService::new();

        let did = Some("Dbf2fjCbsiq2kfns".to_string());
        let crypto_type = Some("type".to_string());

        let did_info = MyDidInfo::new(did.clone(), None, crypto_type, None);

        assert!(service.create_my_did(&did_info).is_err());
    }

    #[test]
    fn create_my_did_works_for_seed() {
        let service = SignusService::new();

        let did = Some("Dbf2fjCbsiq2kfns".to_string());
        let seed = Some("DJASbewkdUY3265HJFDSbds278sdDSnA".to_string());

        let did_info_with_seed = MyDidInfo::new(did.clone(), seed, None, None);
        let did_info_without_seed = MyDidInfo::new(did.clone(), None, None, None);

        let res_with_seed = service.create_my_did(&did_info_with_seed).unwrap();
        let res_without_seed = service.create_my_did(&did_info_without_seed).unwrap();

        assert_ne!(res_with_seed.verkey, res_without_seed.verkey)
    }

    #[test]
    fn create_their_did_works_without_verkey() {
        let service = SignusService::new();
        let did = "8wZcEriaNLNKtteJvx7f8i";
        let their_did_info = TheirDidInfo::new(did.to_string(), None, None, None);
        let their_did: TheirDid = service.create_their_did(&their_did_info).unwrap();

        assert_eq!(did.to_string(), their_did.did);
        assert_eq!(None, their_did.verkey);
    }

    #[test]
    fn create_their_did_works_for_full_verkey() {
        let service = SignusService::new();
        let did = "8wZcEriaNLNKtteJvx7f8i";
        let verkey = "5L2HBnzbu6Auh2pkDRbFt5f4prvgE2LzknkuYLsKkacp";
        let their_did_info = TheirDidInfo::new(did.to_string(), None, Some(verkey.to_string()), None);
        let their_did: TheirDid = service.create_their_did(&their_did_info).unwrap();

        assert_eq!(did.to_string(), their_did.did);
        assert_eq!(verkey, their_did.verkey.unwrap());
    }

    #[test]
    fn create_their_did_works_for_abbreviated_verkey() {
        let service = SignusService::new();
        let did = "8wZcEriaNLNKtteJvx7f8i";
        let their_did_info = TheirDidInfo::new(did.to_string(), None, Some("~NcYxiDXkpYi6ov5FcYDi1e".to_string()), None);
        let their_did: TheirDid = service.create_their_did(&their_did_info).unwrap();

        assert_eq!(did.to_string(), their_did.did);
        assert_eq!("5L2HBnzbu6Auh2pkDRbFt5f4prvgE2LzknkuYLsKkacp", their_did.verkey.unwrap());
    }

    #[test]
    fn sign_works() {
        let service = SignusService::new();

        let did_info = MyDidInfo::new(None, None, None, None);

        let message = r#"{
            "reqId":1495034346617224651,
            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            "operation":{
                "type":"1",
                "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
            }
        }"#;

        let my_did = service.create_my_did(&did_info).unwrap();

        service.sign(&my_did, message.as_bytes()).unwrap();
    }

    #[test]
    fn sign_works_for_invalid_signkey() {
        let service = SignusService::new();

        let message = r#"{
            "reqId":1495034346617224651,
            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            "operation":{
                "type":"1",
                "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
            }
        }"#;

        let my_did = MyDid::new("NcYxiDXkpYi6ov5FcYDi1e".to_string(),
                                DEFAULT_CRYPTO_TYPE.to_string(),
                                "pk".to_string(),
                                "sk".to_string(),
                                "verkey".to_string(),
                                "signkey".to_string());

        assert!(service.sign(&my_did, message.as_bytes()).is_err());
    }

    #[test]
    fn sign_verify_works() {
        let service = SignusService::new();

        let did_info = MyDidInfo::new(None, None, None, None);

        let message = r#"{
            "reqId":1495034346617224651,
            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            "operation":{
                "type":"1",
                "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
            }
        }"#;

        let my_did = service.create_my_did(&did_info).unwrap();

        let signature = service.sign(&my_did, message.as_bytes()).unwrap();

        let their_did = TheirDid {
            did: "sw2SA2jCbsiq2kfns".to_string(),
            crypto_type: DEFAULT_CRYPTO_TYPE.to_string(),
            pk: None,
            endpoint: None,
            verkey: Some(my_did.verkey)
        };

        let valid = service.verify(&their_did, message.as_bytes(), &signature).unwrap();
        assert!(valid);
    }

    #[test]
    fn verify_not_works_for_invalid_verkey() {
        let service = SignusService::new();

        let did_info = MyDidInfo::new(None, None, None, None);

        let message = r#"{
            "reqId":1495034346617224651,
            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            "operation":{
                "type":"1",
                "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
            }
        }"#;

        let my_did = service.create_my_did(&did_info).unwrap();

        let signature = service.sign(&my_did, message.as_bytes()).unwrap();

        let their_did = TheirDid {
            did: "sw2SA2jCbsiq2kfns".to_string(),
            crypto_type: DEFAULT_CRYPTO_TYPE.to_string(),
            pk: None,
            endpoint: None,
            verkey: Some("AnnxV4t3LUHKZaxVQDWoVaG44NrGmeDYMA4Gz6C2tCZd".to_string())
        };

        let valid = service.verify(&their_did, message.as_bytes(), &signature).unwrap();
        assert_eq!(false, valid);
    }

    #[test]
    fn encrypt_works() {
        let service = SignusService::new();

        let msg = "some message";

        let did_info = MyDidInfo::new(None, None, None, None);

        let my_did = service.create_my_did(&did_info).unwrap();

        let their_did = service.create_my_did(&did_info.clone()).unwrap();

        let their_did = TheirDid {
            did: their_did.did,
            crypto_type: DEFAULT_CRYPTO_TYPE.to_string(),
            pk: Some(their_did.pk),
            endpoint: None,
            verkey: Some(their_did.verkey)
        };

        service.encrypt(&my_did, &their_did, msg.as_bytes()).unwrap();
    }

    #[test]
    fn encrypt_decrypt_works() {
        let service = SignusService::new();

        let msg = "some message";

        let did_info = MyDidInfo::new(None, None, None, None);

        let my_did = service.create_my_did(&did_info).unwrap();

        let my_did_for_encrypt = my_did.clone();

        let their_did_for_decrypt = TheirDid {
            did: my_did.did,
            crypto_type: DEFAULT_CRYPTO_TYPE.to_string(),
            pk: Some(my_did.pk),
            endpoint: None,
            verkey: Some(my_did.verkey)
        };

        let their_did = service.create_my_did(&did_info.clone()).unwrap();

        let my_did_for_decrypt = their_did.clone();

        let their_did_for_encrypt = TheirDid {
            did: their_did.did,
            crypto_type: DEFAULT_CRYPTO_TYPE.to_string(),
            pk: Some(their_did.pk),
            endpoint: None,
            verkey: Some(their_did.verkey)
        };

        let (encrypted_message, noce) = service.encrypt(&my_did_for_encrypt, &their_did_for_encrypt, msg.as_bytes()).unwrap();

        let decrypted_message = service.decrypt(&my_did_for_decrypt, &their_did_for_decrypt, &encrypted_message, &noce).unwrap();

        assert_eq!(msg.as_bytes().to_vec(), decrypted_message);
    }

    #[test]
    fn encrypt_sealed_works() {
        let service = SignusService::new();

        let msg = "some message";

        let did_info = MyDidInfo::new(None, None, None, None);

        let did = service.create_my_did(&did_info.clone()).unwrap();

        let did = TheirDid {
            did: did.did,
            crypto_type: DEFAULT_CRYPTO_TYPE.to_string(),
            pk: Some(did.pk),
            endpoint: None,
            verkey: Some(did.verkey)
        };

        service.encrypt_sealed(&did, msg.as_bytes()).unwrap();
    }

    #[test]
    fn encrypt_decrypt_sealed_works() {
        let service = SignusService::new();

        let msg = "some message".as_bytes();

        let did_info = MyDidInfo::new(None, None, None, None);

        let did = service.create_my_did(&did_info.clone()).unwrap();

        let encrypt_did = TheirDid {
            did: did.did.clone(),
            crypto_type: DEFAULT_CRYPTO_TYPE.to_string(),
            pk: Some(did.pk.clone()),
            endpoint: None,
            verkey: Some(did.verkey.clone())
        };

        let encrypted_message = service.encrypt_sealed(&encrypt_did, msg).unwrap();
        let decrypted_message = service.decrypt_sealed(&did, &encrypted_message).unwrap();
        assert_eq!(msg, decrypted_message.as_slice());
    }
}