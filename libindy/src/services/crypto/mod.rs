mod ed25519;
pub mod types;

use base64;

use self::ed25519::ED25519CryptoType;
use self::types::*;

use utils::crypto::base58::Base58;
use utils::crypto::verkey_builder::build_full_verkey;

use errors::common::CommonError;
use errors::crypto::CryptoError;

use std::collections::HashMap;
use std::str;

pub const DEFAULT_CRYPTO_TYPE: &'static str = "ed25519";

trait CryptoType {
    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn gen_nonce(&self) -> Vec<u8>;
    fn create_key(&self, seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CommonError>;
    fn validate_key(&self, vk: &[u8]) -> Result<(), CommonError>;
    fn sign(&self, sk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn verify(&self, vk: &[u8], doc: &[u8], signature: &[u8]) -> Result<bool, CommonError>;
    fn encrypt_sealed(&self, vk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
    fn decrypt_sealed(&self, vk: &[u8], sk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError>;
}

pub struct CryptoService {
    crypto_types: HashMap<&'static str, Box<CryptoType>>
}

impl CryptoService {
    pub fn new() -> CryptoService {
        let mut crypto_types: HashMap<&str, Box<CryptoType>> = HashMap::new();
        crypto_types.insert(DEFAULT_CRYPTO_TYPE, Box::new(ED25519CryptoType::new()));

        CryptoService {
            crypto_types
        }
    }

    pub fn create_key(&self, key_info: &KeyInfo) -> Result<Key, CryptoError> {
        let crypto_type_name = key_info.crypto_type
            .as_ref()
            .map(String::as_str)
            .unwrap_or(DEFAULT_CRYPTO_TYPE);

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(
                CryptoError::UnknownCryptoError(
                    format!("KeyInfo contains unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let seed = self.convert_seed(key_info.seed.as_ref().map(String::as_ref))?;
        let (vk, sk) = crypto_type.create_key(seed.as_ref().map(Vec::as_slice))?;
        let mut vk = Base58::encode(&vk);
        let sk = Base58::encode(&sk);

        if !crypto_type_name.eq(DEFAULT_CRYPTO_TYPE) {
            // Use suffix with crypto type name to store crypto type inside of vk
            vk = format!("{}:{}", vk, crypto_type_name);
        }

        Ok(Key::new(vk, sk))
    }

    pub fn create_my_did(&self, my_did_info: &MyDidInfo) -> Result<(Did, Key), CryptoError> {
        let crypto_type_name = my_did_info.crypto_type
            .as_ref()
            .map(String::as_str)
            .unwrap_or(DEFAULT_CRYPTO_TYPE);

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(
                CryptoError::UnknownCryptoError(
                    format!("MyDidInfo info contains unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let seed = self.convert_seed(my_did_info.seed.as_ref().map(String::as_ref))?;
        let (vk, sk) = crypto_type.create_key(seed.as_ref().map(Vec::as_slice))?;
        let did = match my_did_info.did {
            Some(ref did) => {
                self.validate_did(did)?;
                Base58::decode(did)?
            }
            _ if my_did_info.cid == Some(true) => vk.clone(),
            _ => vk[0..16].to_vec()
        };

        let did = Base58::encode(&did);
        let mut vk = Base58::encode(&vk);
        let sk = Base58::encode(&sk);

        if !crypto_type_name.eq(DEFAULT_CRYPTO_TYPE) {
            // Use suffix with crypto type name to store crypto type inside of vk
            vk = format!("{}:{}", vk, crypto_type_name);
        }

        Ok((Did::new(did, vk.clone()), Key::new(vk, sk)))
    }

    pub fn create_their_did(&self, their_did_info: &TheirDidInfo) -> Result<Did, CryptoError> {
        // Check did is correct Base58
        Base58::decode(&their_did_info.did)?;

        let verkey = build_full_verkey(their_did_info.did.as_str(),
                                       their_did_info.verkey.as_ref().map(String::as_str))?;

        self.validate_key(&verkey)?;

        let did = Did::new(their_did_info.did.clone(),
                           verkey);
        Ok(did)
    }

    pub fn sign(&self, my_key: &Key, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let crypto_type_name = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            splits[1]
        } else {
            DEFAULT_CRYPTO_TYPE
        };

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(
                CryptoError::UnknownCryptoError(
                    format!("Trying to sign message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let my_sk = Base58::decode(my_key.signkey.as_str())?;
        let signature = crypto_type.sign(&my_sk, doc)?;

        Ok(signature)
    }

    pub fn verify(&self, their_vk: &str, msg: &[u8], signature: &[u8]) -> Result<bool, CryptoError> {
        let (their_vk, crypto_type_name) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (their_vk, DEFAULT_CRYPTO_TYPE)
        };

        if !self.crypto_types.contains_key(crypto_type_name) {
            return Err(CryptoError::UnknownCryptoError(
                format!("Trying to verify message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let their_vk = Base58::decode(&their_vk)?;

        Ok(crypto_type.verify(&their_vk, msg, signature)?)
    }

    pub fn create_combo_box(&self, my_key: &Key, their_vk: &str, doc: &[u8]) -> Result<ComboBox, CryptoError> {
        let (msg, nonce) = self.encrypt(my_key, their_vk, doc)?;

        Ok(ComboBox {
            msg: base64::encode(msg.as_slice()),
            sender: my_key.verkey.to_string(),
            nonce: base64::encode(nonce.as_slice())
        })
    }

    pub fn encrypt(&self, my_key: &Key, their_vk: &str, doc: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        let (_my_vk, crypto_type_name) = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            (splits[0], splits[1])
        } else {
            (my_key.verkey.as_str(), DEFAULT_CRYPTO_TYPE)
        };

        let (their_vk, their_crypto_type_name) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (their_vk, DEFAULT_CRYPTO_TYPE)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(CryptoError::UnknownCryptoError(format!("Trying to encrypt message with unknown crypto: {}", crypto_type_name)));
        }

        if !crypto_type_name.eq(their_crypto_type_name) {
            // TODO: FIXME: Use dedicated error code
            return Err(CryptoError::UnknownCryptoError(
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

    pub fn decrypt(&self, my_key: &Key, their_vk: &str, doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let (_my_vk, crypto_type_name) = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            (splits[0], splits[1])
        } else {
            (my_key.verkey.as_str(), DEFAULT_CRYPTO_TYPE)
        };

        let (their_vk, their_crypto_type_name) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (their_vk, DEFAULT_CRYPTO_TYPE)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(CryptoError::UnknownCryptoError(
                format!("Trying to decrypt message with unknown crypto: {}", crypto_type_name)));
        }

        if !crypto_type_name.eq(their_crypto_type_name) {
            // TODO: FIXME: Use dedicated error code
            return Err(CryptoError::UnknownCryptoError(
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

    pub fn encrypt_sealed(&self, their_vk: &str, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let (their_vk, crypto_type_name) = if their_vk.contains(":") {
            let splits: Vec<&str> = their_vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (their_vk, DEFAULT_CRYPTO_TYPE)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(CryptoError::UnknownCryptoError(format!("Trying to encrypt sealed message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let their_vk = Base58::decode(their_vk)?;

        let encrypted_doc = crypto_type.encrypt_sealed(&their_vk, doc)?;
        Ok(encrypted_doc)
    }

    pub fn decrypt_sealed(&self, my_key: &Key, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let (my_vk, crypto_type_name) = if my_key.verkey.contains(":") {
            let splits: Vec<&str> = my_key.verkey.split(":").collect();
            (splits[0], splits[1])
        } else {
            (my_key.verkey.as_str(), DEFAULT_CRYPTO_TYPE)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(CryptoError::UnknownCryptoError(
                format!("Trying to decrypt sealed message with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let my_vk = Base58::decode(my_vk)?;
        let my_sk = Base58::decode(my_key.signkey.as_str())?;

        let decrypted_doc = crypto_type.decrypt_sealed(&my_vk, &my_sk, doc)?;
        Ok(decrypted_doc)
    }

    pub fn convert_seed(&self, seed: Option<&str>) -> Result<Option<Vec<u8>>, CryptoError> {
        Ok(match seed {
            Some(ref seed) =>
                if seed.ends_with("=") {
                    Some(base64::decode(&seed)
                        .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Seed from Base64 string: {:?}", err)))?)
                } else {
                    Some(seed.as_bytes().to_vec())
                },
            None => None
        })
    }

    pub fn validate_key(&self, vk: &str) -> Result<(), CryptoError> {
        let (vk, crypto_type_name) = if vk.contains(":") {
            let splits: Vec<&str> = vk.split(":").collect();
            (splits[0], splits[1])
        } else {
            (vk, DEFAULT_CRYPTO_TYPE)
        };

        if !self.crypto_types.contains_key(&crypto_type_name) {
            return Err(CryptoError::UnknownCryptoError(format!("Trying to use key with unknown crypto: {}", crypto_type_name)));
        }

        let crypto_type = self.crypto_types.get(crypto_type_name).unwrap();

        let vk = if vk.starts_with("~") { &vk[1..] } else { vk };
        let vk = Base58::decode(vk)?;

        crypto_type.validate_key(&vk)?;
        Ok(())
    }

    pub fn validate_did(&self, did: &str) -> Result<(), CryptoError> {
        let did = Base58::decode(did)?;

        if did.len() != 16 && did.len() != 32 {
            return Err(CryptoError::CommonError(
                CommonError::InvalidStructure(
                    format!("Trying to use did with unexpected len: {}", did.len()))));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use services::crypto::types::MyDidInfo;

    #[test]
    fn create_my_did_with_works_for_empty_info() {
        let service = CryptoService::new();
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        service.create_my_did(&did_info).unwrap();
    }

    #[test]
    fn create_my_did_works_for_passed_did() {
        let service = CryptoService::new();

        let did = "NcYxiDXkpYi6ov5FcYDi1e";
        let did_info = MyDidInfo { did: Some(did.to_string()), cid: None, seed: None, crypto_type: None };

        let (my_did, _) = service.create_my_did(&did_info).unwrap();
        assert_eq!(did, my_did.did);
    }

    #[test]
    fn create_my_did_not_works_for_invalid_crypto_type() {
        let service = CryptoService::new();

        let did = Some("NcYxiDXkpYi6ov5FcYDi1e".to_string());
        let crypto_type = Some("type".to_string());

        let did_info = MyDidInfo { did: did.clone(), cid: None, seed: None, crypto_type: crypto_type };

        assert!(service.create_my_did(&did_info).is_err());
    }

    #[test]
    fn create_my_did_works_for_seed() {
        let service = CryptoService::new();

        let did = Some("NcYxiDXkpYi6ov5FcYDi1e".to_string());
        let seed = Some("00000000000000000000000000000My1".to_string());

        let did_info_with_seed = MyDidInfo { did: did.clone(), cid: None, seed, crypto_type: None };
        let did_info_without_seed = MyDidInfo { did: did.clone(), cid: None, seed: None, crypto_type: None };

        let (did_with_seed, _) = service.create_my_did(&did_info_with_seed).unwrap();
        let (did_without_seed, _) = service.create_my_did(&did_info_without_seed).unwrap();

        assert_ne!(did_with_seed.verkey, did_without_seed.verkey)
    }

    #[test]
    fn create_their_did_works_without_verkey() {
        let service = CryptoService::new();
        let did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";

        let their_did_info = TheirDidInfo::new(did.to_string(), None);
        let their_did = service.create_their_did(&their_did_info).unwrap();

        assert_eq!(did.to_string(), their_did.did);
        assert_eq!(did.to_string(), their_did.verkey);
    }

    #[test]
    fn create_their_did_works_for_full_verkey() {
        let service = CryptoService::new();
        let did = "8wZcEriaNLNKtteJvx7f8i";
        let verkey = "5L2HBnzbu6Auh2pkDRbFt5f4prvgE2LzknkuYLsKkacp";

        let their_did_info = TheirDidInfo::new(did.to_string(), Some(verkey.to_string()));
        let their_did = service.create_their_did(&their_did_info).unwrap();

        assert_eq!(did.to_string(), their_did.did);
        assert_eq!(verkey, their_did.verkey);
    }

    #[test]
    fn create_their_did_works_for_abbreviated_verkey() {
        let service = CryptoService::new();
        let did = "8wZcEriaNLNKtteJvx7f8i";
        let their_did_info = TheirDidInfo::new(did.to_string(), Some("~NcYxiDXkpYi6ov5FcYDi1e".to_string()));
        let their_did = service.create_their_did(&their_did_info).unwrap();

        assert_eq!(did.to_string(), their_did.did);
        assert_eq!("5L2HBnzbu6Auh2pkDRbFt5f4prvgE2LzknkuYLsKkacp", their_did.verkey);
    }

    #[test]
    fn sign_works() {
        let service = CryptoService::new();
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };

        let message = r#"message"#;
        let (_, my_key) = service.create_my_did(&did_info).unwrap();
        service.sign(&my_key, message.as_bytes()).unwrap();
    }

    #[test]
    fn sign_works_for_invalid_signkey() {
        let service = CryptoService::new();
        let message = r#"message"#;
        let my_key = Key::new("8wZcEriaNLNKtteJvx7f8i".to_string(), "5L2HBnzbu6Auh2pkDRbFt5f4prvgE2LzknkuYLsKkacp".to_string());
        assert!(service.sign(&my_key, message.as_bytes()).is_err());
    }

    #[test]
    fn sign_verify_works() {
        let service = CryptoService::new();
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        let message = r#"message"#;
        let (my_did, my_key) = service.create_my_did(&did_info).unwrap();
        let signature = service.sign(&my_key, message.as_bytes()).unwrap();
        let valid = service.verify(&my_did.verkey, message.as_bytes(), &signature).unwrap();
        assert!(valid);
    }

    #[test]
    fn sign_verify_works_for_verkey_contained_crypto_type() {
        let service = CryptoService::new();
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        let message = r#"message"#;
        let (my_did, my_key) = service.create_my_did(&did_info).unwrap();
        let signature = service.sign(&my_key, message.as_bytes()).unwrap();
        let verkey = my_did.verkey + ":ed25519";
        let valid = service.verify(&verkey, message.as_bytes(), &signature).unwrap();
        assert!(valid);
    }


    #[test]
    fn sign_verify_works_for_verkey_contained_invalid_crypto_type() {
        let service = CryptoService::new();
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        let message = r#"message"#;
        let (my_did, my_key) = service.create_my_did(&did_info).unwrap();
        let signature = service.sign(&my_key, message.as_bytes()).unwrap();
        let verkey = format!("crypto_type:{}", my_did.verkey);
        assert!(service.verify(&verkey, message.as_bytes(), &signature).is_err());
    }

    #[test]
    fn verify_not_works_for_invalid_verkey() {
        let service = CryptoService::new();
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        let message = r#"message"#;
        let (_, my_key) = service.create_my_did(&did_info).unwrap();
        let signature = service.sign(&my_key, message.as_bytes()).unwrap();
        let verkey = "AnnxV4t3LUHKZaxVQDWoVaG44NrGmeDYMA4Gz6C2tCZd";
        let valid = service.verify(verkey, message.as_bytes(), &signature).unwrap();
        assert_eq!(false, valid);
    }

    #[test]
    fn encrypt_works() {
        let service = CryptoService::new();
        let msg = "some message";
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        let (_, my_key) = service.create_my_did(&did_info).unwrap();
        let (their_did, _) = service.create_my_did(&did_info.clone()).unwrap();
        let their_did = Did::new(their_did.did, their_did.verkey);
        service.encrypt(&my_key, &their_did.verkey, msg.as_bytes()).unwrap();
    }

    #[test]
    fn encrypt_decrypt_works() {
        let service = CryptoService::new();

        let msg = "some message";

        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };

        let (my_did, my_key) = service.create_my_did(&did_info).unwrap();

        let my_key_for_encrypt = my_key.clone();

        let their_did_for_decrypt = Did::new(my_did.did, my_did.verkey);

        let (their_did, their_key) = service.create_my_did(&did_info.clone()).unwrap();

        let my_key_for_decrypt = their_key.clone();

        let their_did_for_encrypt = Did::new(their_did.did, their_did.verkey);

        let (encrypted_message, noce) = service.encrypt(&my_key_for_encrypt, &their_did_for_encrypt.verkey, msg.as_bytes()).unwrap();

        let decrypted_message = service.decrypt(&my_key_for_decrypt, &their_did_for_decrypt.verkey, &encrypted_message, &noce).unwrap();

        assert_eq!(msg.as_bytes().to_vec(), decrypted_message);
    }


    #[test]
    fn encrypt_decrypt_works_for_verkey_contained_crypto_type() {
        let service = CryptoService::new();

        let msg = "some message";

        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };

        let (my_did, my_key) = service.create_my_did(&did_info).unwrap();

        let my_key_for_encrypt = my_key.clone();

        let their_did_for_decrypt = Did::new(my_did.did, my_did.verkey);

        let (their_did, their_key) = service.create_my_did(&did_info.clone()).unwrap();
        let my_key_for_decrypt = their_key.clone();

        let their_did_for_encrypt = Did::new(their_did.did, their_did.verkey);

        let (encrypted_message, noce) = service.encrypt(&my_key_for_encrypt, &their_did_for_encrypt.verkey, msg.as_bytes()).unwrap();

        let verkey = their_did_for_decrypt.verkey + ":ed25519";

        let decrypted_message = service.decrypt(&my_key_for_decrypt, &verkey, &encrypted_message, &noce).unwrap();

        assert_eq!(msg.as_bytes().to_vec(), decrypted_message);
    }

    #[test]
    fn encrypt_sealed_works() {
        let service = CryptoService::new();
        let msg = "some message";
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        let (did, _) = service.create_my_did(&did_info.clone()).unwrap();
        let did = Did::new(did.did, did.verkey);
        service.encrypt_sealed(&did.verkey, msg.as_bytes()).unwrap();
    }

    #[test]
    fn encrypt_decrypt_sealed_works() {
        let service = CryptoService::new();
        let msg = "some message".as_bytes();
        let did_info = MyDidInfo { did: None, cid: None, seed: None, crypto_type: None };
        let (did, key) = service.create_my_did(&did_info.clone()).unwrap();
        let encrypt_did = Did::new(did.did.clone(), did.verkey.clone());
        let encrypted_message = service.encrypt_sealed(&encrypt_did.verkey, msg).unwrap();
        let decrypted_message = service.decrypt_sealed(&key, &encrypted_message).unwrap();
        assert_eq!(msg, decrypted_message.as_slice());
    }
}
