mod ed25519;
pub mod types;

extern crate serde_json;
use self::serde_json::Value;

use self::ed25519::ED25519Signus;
use self::types::{
    MyDidInfo,
    MyDid,
    TheirDid
};
use utils::crypto::base58::Base58;
use utils::crypto::signature_serializer::serialize_signature;

use errors::crypto::CryptoError;
use errors::signus::SignusError;
use std::collections::HashMap;
use std::str;

const DEFAULT_CRYPTO_TYPE: &'static str = "ed25519";

trait CryptoType {
    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Vec<u8>;
    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn gen_nonce(&self) -> Vec<u8>;
    fn create_key_pair_for_signature(&self, seed: Option<&[u8]>) -> (Vec<u8>, Vec<u8>);
    fn sign(&self, private_key: &[u8], doc: &[u8]) -> Vec<u8>;
    fn verify(&self, public_key: &[u8], doc: &[u8], signature: &[u8]) -> bool;
    fn get_key_pair_for_encryption(&self, pk: &[u8], sk: &[u8]) -> (Vec<u8>, Vec<u8>);
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
        let (ver_key, sign_key) = signus.create_key_pair_for_signature(seed);
        let (public_key, secret_key) = signus.get_key_pair_for_encryption(&ver_key, &sign_key);


        let did = did_info.did.as_ref().map(|did| Base58::decode(did)).unwrap_or(Ok(ver_key[0..16].to_vec()))?;


        let my_did = MyDid::new(Base58::encode(&did),
                                xtype.clone(),
                                Base58::encode(&public_key),
                                Base58::encode(&secret_key),
                                Base58::encode(&ver_key),
                                Base58::encode(&sign_key));
        println!("did {:?}", my_did.did);

        Ok(my_did)
    }

    pub fn sign(&self, my_did: &MyDid, doc: &str) -> Result<String, CryptoError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(CryptoError::UnknownType(my_did.crypto_type.clone()));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();

        let sign_key = Base58::decode(&my_did.sign_key)?;
        let mut msg: Value = serde_json::from_str(doc)?;

        let signature = serialize_signature(msg.clone())?;
        let signature = signus.sign(&sign_key, signature.as_bytes());
        let signature = Base58::encode(&signature);
        msg["signature"] = Value::String(signature);
        let signed_msg: String = serde_json::to_string(&msg)?;
        Ok(signed_msg)
    }

    pub fn verify(&self, their_did: &TheirDid, signed_msg: &str) -> Result<bool, SignusError> {
        let xtype = their_did.crypto_type.clone().unwrap_or(DEFAULT_CRYPTO_TYPE.to_string());

        if !self.crypto_types.contains_key(&xtype.as_str()) {
            return Err(SignusError::CryptoError(CryptoError::UnknownType(xtype)));
        }

        if their_did.verkey.is_none() {
            return Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Verkey key not found"))));
        }
        let verkey = their_did.verkey.clone().unwrap();

        let signus = self.crypto_types.get(&xtype.as_str()).unwrap();

        let verkey = Base58::decode(&verkey)?;
        let signed_msg: Value = serde_json::from_str(signed_msg)?;

        if let Some(signature) = signed_msg["signature"].as_str() {
            let signature = Base58::decode(signature)?;
            let mut message: Value = Value::Object(serde_json::map::Map::new());
            for key in signed_msg.as_object().unwrap().keys() {
                if key != "signature" {
                    message[key] = signed_msg[key].clone();
                }
            }
            Ok(signus.verify(&verkey, &serialize_signature(message)?.as_bytes(), &signature))
        } else {
            return Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Signature key not found"))));
        }
    }

    pub fn encrypt(&self, my_did: &MyDid, their_did: &TheirDid, doc: &str) -> Result<(String, String), SignusError> {
        if !self.crypto_types.contains_key(&my_did.crypto_type.as_str()) {
            return Err(SignusError::CryptoError(CryptoError::UnknownType(my_did.crypto_type.clone())));
        }

        if their_did.pk.is_none() {
            return Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Public key not found"))));
        }

        let signus = self.crypto_types.get(&my_did.crypto_type.as_str()).unwrap();
        let public_key = their_did.pk.clone().unwrap();

        let nonce = signus.gen_nonce();

        let secret_key = Base58::decode(&my_did.secret_key)?;
        let public_key = Base58::decode(&public_key)?;

        let encrypted_doc = signus.encrypt(&secret_key, &public_key, &doc.as_bytes(), &nonce);
        let encrypted_doc = Base58::encode(&encrypted_doc);
        let nonce = Base58::encode(&nonce);

        Ok((encrypted_doc, nonce))
    }

    pub fn decrypt(&self, my_did: &MyDid, their_did: &TheirDid, doc: &str, nonce: &str) -> Result<String, SignusError> {
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
        let nonce = Base58::decode(&nonce)?;
        let doc = Base58::decode(&doc)?;

        let decrypted_doc = signus.decrypt(&secret_key, &public_key, &doc, &nonce)?;

        let decrypted_doc = str::from_utf8(&decrypted_doc)?;
        Ok(decrypted_doc.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use services::signus::types::MyDidInfo;

    #[test]
    fn create_my_did_with_empty_input_works() {
        let service = SignusService::new();

        let did_info = MyDidInfo {
            did: None,
            seed: None,
            crypto_type: None
        };

        let res = service.create_my_did(&did_info);

        assert!(res.is_ok());
    }

    #[test]
    fn create_my_did_with_did_in_input_works() {
        let service = SignusService::new();

        let did = Some("Dbf2fjCbsiq2kfns".to_string());
        let did_info = MyDidInfo {
            did: did.clone(),
            seed: None,
            crypto_type: None
        };

        let res = service.create_my_did(&did_info);
        assert!(res.is_ok());

        assert_eq!(did.unwrap(), did_info.did.unwrap());
    }

    #[test]
    fn try_create_my_did_with_invalid_crypto_type() {
        let service = SignusService::new();

        let did = Some("Dbf2fjCbsiq2kfns".to_string());
        let crypto_type = Some("type".to_string());
        let did_info = MyDidInfo {
            did: did.clone(),
            seed: None,
            crypto_type: crypto_type
        };

        let res = service.create_my_did(&did_info);
        assert!(res.is_err());
    }

    #[test]
    fn create_my_did_with_seed_type() {
        let service = SignusService::new();

        let did = Some("Dbf2fjCbsiq2kfns".to_string());
        let seed = Some("DJASbewkdUY3265HJFDSbds278sdDSnA".to_string());
        let did_info_with_seed = MyDidInfo {
            did: did.clone(),
            seed: seed,
            crypto_type: None
        };
        let did_info_without_seed = MyDidInfo {
            did: did.clone(),
            seed: None,
            crypto_type: None
        };

        let res_with_seed = service.create_my_did(&did_info_with_seed);
        let res_without_seed = service.create_my_did(&did_info_without_seed);

        assert!(res_with_seed.is_ok());
        assert!(res_without_seed.is_ok());

        assert_ne!(res_with_seed.unwrap().ver_key, res_without_seed.unwrap().ver_key)
    }

    #[test]
    fn sign_works() {
        let service = SignusService::new();

        let did_info = MyDidInfo {
            did: None,
            seed: None,
            crypto_type: None
        };
        let message = r#"{
            "reqId":1495034346617224651,
            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            "operation":{
                "type":"1",
                "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
            }
        }"#;

        let res = service.create_my_did(&did_info);
        assert!(res.is_ok());
        let my_did = res.unwrap();

        let signature = service.sign(&my_did, message);
        assert!(signature.is_ok());
    }

    #[test]
    fn sign_verify_works() {
        let service = SignusService::new();

        let did_info = MyDidInfo {
            did: None,
            seed: None,
            crypto_type: None
        };
        let message = r#"{
            "reqId":1495034346617224651,
            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            "operation":{
                "type":"1",
                "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
            }
        }"#;

        let res = service.create_my_did(&did_info);
        assert!(res.is_ok());
        let my_did = res.unwrap();

        let signature = service.sign(&my_did, message);
        assert!(signature.is_ok());
        let signature = signature.unwrap();

        let their_did = TheirDid {
            did: "sw2SA2jCbsiq2kfns".to_string(),
            crypto_type: Some(DEFAULT_CRYPTO_TYPE.to_string()),
            pk: None,
            verkey: Some(my_did.ver_key)
        };

        let res = service.verify(&their_did, &signature);
        assert!(res.is_ok());
        let valid = res.unwrap();
        assert!(valid);
    }

    #[test]
    fn try_verify_with_invalid_verkey() {
        let service = SignusService::new();

        let did_info = MyDidInfo {
            did: None,
            seed: None,
            crypto_type: None
        };
        let message = r#"{
            "reqId":1495034346617224651,
            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            "operation":{
                "type":"1",
                "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
            }
        }"#;

        let res = service.create_my_did(&did_info);
        assert!(res.is_ok());
        let my_did = res.unwrap();

        let signature = service.sign(&my_did, message);
        assert!(signature.is_ok());
        let signature = signature.unwrap();

        let their_did = TheirDid {
            did: "sw2SA2jCbsiq2kfns".to_string(),
            crypto_type: Some(DEFAULT_CRYPTO_TYPE.to_string()),
            pk: None,
            verkey: Some("AnnxV4t3LUHKZaxVQDWoVaG44NrGmeDYMA4Gz6C2tCZd".to_string())
        };

        let res = service.verify(&their_did, &signature);
        assert!(res.is_ok());
        assert_eq!(false, res.unwrap());
    }

    #[test]
    fn encrypt_works() {
        let service = SignusService::new();

        let msg = "some message";

        let did_info = MyDidInfo {
            did: None,
            seed: None,
            crypto_type: None
        };
        let res = service.create_my_did(&did_info);
        assert!(res.is_ok());
        let my_did = res.unwrap();


        let res = service.create_my_did(&did_info.clone());
        assert!(res.is_ok());
        let their_did = res.unwrap();

        let their_did = TheirDid {
            did: their_did.did,
            crypto_type: Some(DEFAULT_CRYPTO_TYPE.to_string()),
            pk: Some(their_did.public_key),
            verkey: Some(their_did.ver_key)
        };

        let encrypted_message = service.encrypt(&my_did, &their_did, msg);
        assert!(encrypted_message.is_ok());
    }

    #[test]
    fn encrypt_decrypt_works() {
        let service = SignusService::new();

        let msg = "some message";

        let did_info = MyDidInfo {
            did: None,
            seed: None,
            crypto_type: None
        };
        let res = service.create_my_did(&did_info);
        assert!(res.is_ok());
        let my_did = res.unwrap();

        let my_did_for_encrypt = my_did.clone();

        let their_did_for_decrypt = TheirDid {
            did: my_did.did,
            crypto_type: Some(DEFAULT_CRYPTO_TYPE.to_string()),
            pk: Some(my_did.public_key),
            verkey: Some(my_did.ver_key)
        };


        let res = service.create_my_did(&did_info.clone());
        assert!(res.is_ok());
        let their_did = res.unwrap();

        let my_did_for_decrypt = their_did.clone();

        let their_did_for_encrypt = TheirDid {
            did: their_did.did,
            crypto_type: Some(DEFAULT_CRYPTO_TYPE.to_string()),
            pk: Some(their_did.public_key),
            verkey: Some(their_did.ver_key)
        };

        let encrypted_message = service.encrypt(&my_did_for_encrypt, &their_did_for_encrypt, msg);
        assert!(encrypted_message.is_ok());
        let (encrypted_message, noce) = encrypted_message.unwrap();

        let decrypted_message = service.decrypt(&my_did_for_decrypt, &their_did_for_decrypt, &encrypted_message, &noce);
        assert!(decrypted_message.is_ok());
        let decrypted_message = decrypted_message.unwrap();

        assert_eq!(msg.to_string(), decrypted_message);
    }
}