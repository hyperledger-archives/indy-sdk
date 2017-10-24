mod ed25519;
pub mod types;

use self::ed25519::ED25519CryptoType;
use self::types::{
    KeyInfo,
    MyDidInfo,
    TheirDidInfo,
    Key,
    Did
};

use utils::crypto::base58::Base58;
use utils::crypto::verkey_builder::build_full_verkey;

use errors::common::CommonError;
use errors::signus::SignusError;

use std::collections::HashMap;
use std::str;

pub const DEFAULT_CRYPTO_TYPE: &'static str = "ed25519";

trait CryptoType {
    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn gen_nonce(&self) -> Vec<u8>;
    fn create_key(&self, seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CommonError>;
    fn sign(&self, sk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn verify(&self, vk: &[u8], doc: &[u8], signature: &[u8]) -> Result<bool, CommonError>;
    fn encrypt_sealed(&self, vk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn decrypt_sealed(&self, vk: &[u8], sk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
}

pub struct SignusService {
    crypto_types: HashMap<&'static str, Box<CryptoType>>
}

impl SignusService {
    pub fn new() -> SignusService {
        let mut crypto_types: HashMap<&str, Box<CryptoType>> = HashMap::new();
        crypto_types.insert(DEFAULT_CRYPTO_TYPE, Box::new(ED25519CryptoType::new()));

        SignusService {
            crypto_types: crypto_types
        }
    }

    pub fn create_key(&self, key_info: &KeyInfo) -> Result<Key, SignusError> {
        let crypto_type_name = key_info.crypto_type
            .as_ref()
            .map(String::as_str)
            .unwrap_or(DEFAULT_CRYPTO_TYPE);

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(
                SignusError::UnknownCryptoError(
                    format!("KeyInfo contains unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let seed = key_info.seed.as_ref().map(String::as_bytes);
        let (vk, sk) = crypto_type.create_key(seed)?;
        let vk = Base58::encode(&vk);
        let sk = Base58::encode(&sk);

        if !crypto_type_name.eq(DEFAULT_CRYPTO_TYPE) {
            // Use suffix with crypto type name to store crypto type inside of vk
            let vk = format!("{}:{}", vk, crypto_type_name);
        }

        Ok(Key::new(vk, sk))
    }

    pub fn create_my_did(&self, my_did_info: &MyDidInfo) -> Result<(Did, Key), SignusError> {
        let crypto_type_name = my_did_info.crypto_type
            .as_ref()
            .map(String::as_str)
            .unwrap_or(DEFAULT_CRYPTO_TYPE);

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(
                SignusError::UnknownCryptoError(
                    format!("MyDidInfo info contains unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let seed = my_did_info.seed.as_ref().map(String::as_bytes);
        let (vk, sk) = crypto_type.create_key(seed)?;

        let did = match my_did_info.did {
            Some(ref did) => Base58::decode(did)?,
            _ if my_did_info.cid == Some(true) => vk.clone(),
            _ => vk[0..16].to_vec()
        };

        let did = Base58::encode(&did);
        let vk = Base58::encode(&vk);
        let sk = Base58::encode(&sk);

        if !crypto_type_name.eq(DEFAULT_CRYPTO_TYPE) {
            // Use suffix with crypto type name to store crypto type inside of vk
            let vk = format!("{}:{}", vk, crypto_type_name);
        }

        Ok((Did::new(did, vk.clone()), Key::new(vk, sk)))
    }

    pub fn create_their_did(&self, their_did_info: &TheirDidInfo) -> Result<Did, SignusError> {
        // Check did is correct Base58
        Base58::decode(&their_did_info.did)?;

        let verkey = build_full_verkey(their_did_info.did.as_str(),
                                       their_did_info.verkey.as_ref().map(String::as_str))?;

        let did = Did::new(their_did_info.did.clone(),
                           verkey);
        Ok(did)
    }

    pub fn sign(&self, my_key: &Key, doc: &[u8]) -> Result<Vec<u8>, SignusError> {
        let crypto_type_name = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            splits[1]
        } else {
            DEFAULT_CRYPTO_TYPE
        };

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(
                SignusError::UnknownCryptoError(
                    format!("Trying to sign message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let my_sk = Base58::decode(my_key.signkey.as_str())?;
        let signature = crypto_type.sign(&my_sk, doc)?;

        Ok(signature)
    }

    pub fn verify(&self, their_vk: &str, msg: &[u8], signature: &[u8]) -> Result<bool, SignusError> {
        let (their_vk, crypto_type_name) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (DEFAULT_CRYPTO_TYPE, their_vk)
        };

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(SignusError::UnknownCryptoError(
                format!("Trying to verify message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let their_vk = Base58::decode(&their_vk)?;

        Ok(crypto_type.verify(&their_vk, msg, signature)?)
    }

    pub fn encrypt(&self, my_key: &Key, their_vk: &str, doc: &[u8]) -> Result<(Vec<u8>, Vec<u8>), SignusError> {
        let (crypto_type_name, my_vk) = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            (splits[0], splits[1])
        } else {
            (DEFAULT_CRYPTO_TYPE, my_key.verkey.as_str())
        };

        let (their_crypto_type_name, their_vk) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (DEFAULT_CRYPTO_TYPE, their_vk)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(SignusError::UnknownCryptoError(format!("Trying to encrypt message with unknown crypto: {}", crypto_type_name)));
        }

        if !crypto_type_name.eq(their_crypto_type_name) {
            // TODO: FIXME: Use dedicated error code
            return Err(SignusError::UnknownCryptoError(
                format!("My key crypto type is incompatible with their key crypto type: {} {}",
                        crypto_type_name,
                        their_crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(&crypto_type_name).unwrap();

        let my_sk = Base58::decode(my_key.signkey.as_str())?;
        let their_vk = Base58::decode(their_vk)?;
        let nonce = crypto_type.gen_nonce();

        let encrypted_doc = crypto_type.encrypt(&my_sk, &their_vk, doc, &nonce)?;
        Ok((encrypted_doc, nonce))
    }

    pub fn decrypt(&self, my_key: &Key, their_vk: &str, doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, SignusError> {
        let (crypto_type_name, my_vk) = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            (splits[0], splits[1])
        } else {
            (DEFAULT_CRYPTO_TYPE, my_key.verkey.as_str())
        };

        let (their_crypto_type_name, their_vk) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (DEFAULT_CRYPTO_TYPE, their_vk)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(SignusError::UnknownCryptoError(
                format!("Trying to decrypt message with unknown crypto: {}", crypto_type_name)));
        }

        if !crypto_type_name.eq(their_crypto_type_name) {
            // TODO: FIXME: Use dedicated error code
            return Err(SignusError::UnknownCryptoError(
                format!("My key crypto type is incompatible with their key crypto type: {} {}",
                        crypto_type_name,
                        their_crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let my_sk = Base58::decode(&my_key.signkey)?;
        let their_vk = Base58::decode(their_vk)?;

        let decrypted_doc = crypto_type.decrypt(&my_sk, &their_vk, &doc, &nonce)?;

        Ok(decrypted_doc)
    }

    pub fn encrypt_sealed(&self, their_vk: &str, doc: &[u8]) -> Result<Vec<u8>, SignusError> {
        let (crypto_type_name, their_vk) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (DEFAULT_CRYPTO_TYPE, their_vk)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(SignusError::UnknownCryptoError(format!("Trying to encrypt sealed message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let their_vk = Base58::decode(their_vk)?;

        let encrypted_doc = crypto_type.encrypt_sealed(&their_vk, doc)?;
        Ok(encrypted_doc)
    }

    pub fn decrypt_sealed(&self, my_key: &Key, doc: &[u8]) -> Result<Vec<u8>, SignusError> {
        let (crypto_type_name, my_vk) = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            (splits[0], splits[1])
        } else {
            (DEFAULT_CRYPTO_TYPE, my_key.verkey.as_str())
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(SignusError::UnknownCryptoError(
                format!("Trying to decrypt sealed message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let my_vk = Base58::decode(my_vk)?;
        let my_sk = Base58::decode(my_key.signkey.as_str())?;

        let decrypted_doc = crypto_type.decrypt_sealed(&my_vk, &my_sk, doc)?;
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