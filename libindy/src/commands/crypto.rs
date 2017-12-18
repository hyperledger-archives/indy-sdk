extern crate indy_crypto;
extern crate serde_json;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::indy::IndyError;
use services::crypto::types::{KeyInfo, Key};
use services::wallet::WalletService;
use services::crypto::CryptoService;

use std::error::Error;
use std::rc::Rc;
use std::str;

use base64;

pub enum CryptoCommand {
    CreateKey(
        i32, // wallet handle
        String, // key info json
        Box<Fn(Result<String/*verkey*/, IndyError>) + Send>),
    SetKeyMetadata(
        i32, // wallet handle
        String, // verkey
        String, // metadata
        Box<Fn(Result<(), IndyError>) + Send>),
    GetKeyMetadata(
        i32, // wallet handle
        String, // verkey
        Box<Fn(Result<String, IndyError>) + Send>),
    CryptoSign(
        i32, // wallet handle
        String, // my vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    CryptoVerify(
        String, // their vk
        Vec<u8>, // msg
        Vec<u8>, // signature
        Box<Fn(Result<bool, IndyError>) + Send>),
    AuthenticatedEncrypt(
        i32, // wallet handle
        String, // my vk
        String, // their vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    AuthenticatedDecrypt(
        i32, // wallet handle
        String, // my vk
        Vec<u8>, // encrypted msg
        Box<Fn(Result<(String, Vec<u8>), IndyError>) + Send>),
    AnonymousEncrypt(
        String, // their vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    AnonymousDecrypt(
        i32, // wallet handle
        String, // my vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>)
}

pub struct CryptoCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
}

impl CryptoCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
               crypto_service: Rc<CryptoService>,
    ) -> CryptoCommandExecutor {
        CryptoCommandExecutor {
            wallet_service,
            crypto_service,
        }
    }

    pub fn execute(&self, command: CryptoCommand) {
        match command {
            CryptoCommand::CreateKey(wallet_handle, key_info_json, cb) => {
                info!("CreateKey command received");
                cb(self.create_key(wallet_handle, key_info_json));
            }
            CryptoCommand::SetKeyMetadata(wallet_handle, verkey, metadata, cb) => {
                info!("SetKeyMetadata command received");
                cb(self.set_key_metadata(wallet_handle, verkey, metadata));
            }
            CryptoCommand::GetKeyMetadata(wallet_handle, verkey, cb) => {
                info!("GetKeyMetadata command received");
                cb(self.get_key_metadata(wallet_handle, verkey));
            }
            CryptoCommand::CryptoSign(wallet_handle, sender_vk, msg, cb) => {
                info!("CryptoSign command received");
                cb(self.crypto_sign(wallet_handle, &sender_vk, &msg));
            }
            CryptoCommand::CryptoVerify(recipient_vk, msg, signature, cb) => {
                info!("CryptoVerify command received");
                cb(self.crypto_verify(recipient_vk, msg, signature));
            }
            CryptoCommand::AuthenticatedEncrypt(wallet_handle, my_vk, their_vk, msg, cb) => {
                info!("AuthenticatedEncrypt command received");
                cb(self.authenticated_encrypt(wallet_handle, my_vk, their_vk, msg));
            }
            CryptoCommand::AuthenticatedDecrypt(wallet_handle, my_vk, encrypted_msg, cb) => {
                info!("AuthenticatedDecrypt command received");
                cb(self.authenticated_decrypt(wallet_handle, my_vk, encrypted_msg));
            }
            CryptoCommand::AnonymousEncrypt(their_vk, msg, cb) => {
                info!("AnonymousEncrypt command received");
                cb(self.anonymous_encrypt(their_vk, msg));
            }
            CryptoCommand::AnonymousDecrypt(wallet_handle, my_vk, encrypted_msg, cb) => {
                info!("AnonymousDecrypt command received");
                cb(self.anonymous_decrypt(wallet_handle, my_vk, encrypted_msg));
            }
        };
    }

    fn create_key(&self, wallet_handle: i32, key_info_json: String) -> Result<String, IndyError> {
        let key_info = KeyInfo::from_json(&key_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid KeyInfo json: {}", err.description())))?;

        let key = self.crypto_service.create_key(&key_info)?;
        self._wallet_set_key(wallet_handle, &key)?;

        let res = key.verkey;
        Ok(res)
    }

    fn crypto_sign(&self,
                   wallet_handle: i32,
                   sender_vk: &str,
                   msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        self.crypto_service.validate_key(sender_vk)?;

        let key = self._wallet_get_key(wallet_handle, &sender_vk)?;

        let res = self.crypto_service.sign(&key, msg)?;
        Ok(res)
    }

    fn crypto_verify(&self,
                     recipient_vk: String,
                     msg: Vec<u8>,
                     signature: Vec<u8>) -> Result<bool, IndyError> {
        self.crypto_service.validate_key(&recipient_vk)?;

        let res = self.crypto_service.verify(&recipient_vk, &msg, &signature)?;
        Ok(res)
    }

    fn authenticated_encrypt(&self,
                             wallet_handle: i32,
                             my_vk: String,
                             their_vk: String,
                             msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        let sender_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_vk))?;

        let sender_key = Key::from_json(&sender_key_json)
            .map_err(|err| CommonError::InvalidState(format!("Can't deserialize Key: {:?}", err)))?;

        let (msg, nonce) = self.crypto_service.encrypt(&sender_key, &their_vk, msg.as_slice())?;

        let msg = AuthenticatedEncryptedMessage {
            msg: base64::encode(msg.as_slice()),
            sender: my_vk.to_string(),
            nonce: base64::encode(nonce.as_slice())
        };

        let msg = msg.to_json()
            .map_err(|e| CommonError::InvalidState(format!("Can't serialize AuthenticatedEncryptedMessage: {:?}", e)))?;

        let res = self.crypto_service.encrypt_sealed(&their_vk, msg.as_bytes())
            .map_err(IndyError::CryptoError);

        res
    }

    fn authenticated_decrypt(&self,
                             wallet_handle: i32,
                             my_vk: String,
                             msg: Vec<u8>) -> Result<(String, Vec<u8>), IndyError> {
        let my_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_vk))?;

        let my_key = Key::from_json(&my_key_json)
            .map_err(|err| IndyError::CommonError(CommonError::InvalidState(format!("Can't deserialize Key: {:?}", err))))?;

        let decrypted_msg = self.crypto_service.decrypt_sealed(&my_key, &msg)?;

        let parsed_msg: AuthenticatedEncryptedMessage = serde_json::from_slice(decrypted_msg.as_slice())
            .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize AuthenticatedEncryptedMessage: {:?}", err)))?;

        let doc: Vec<u8> = base64::decode(&parsed_msg.msg)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode internal msg filed from base64 {}", err)))?;

        let nonce: Vec<u8> = base64::decode(&parsed_msg.nonce)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode nonce from base64 {}", err)))?;

        let decrypted_msg = self.crypto_service.decrypt(&my_key, &parsed_msg.sender, &doc, &nonce)?;
        Ok((parsed_msg.sender.clone(), decrypted_msg))
    }

    fn anonymous_encrypt(&self,
                         their_vk: String,
                         msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        let msg = AnonymousEncryptedMessage { msg: base64::encode(&msg) };
        let msg = msg.to_json()
            .map_err(|e| CommonError::InvalidState(format!("Can't serialize AnonymousEncryptedMessage: {:?}", e)))?;

        let res = self.crypto_service.encrypt_sealed(&their_vk, msg.as_bytes())
            .map_err(IndyError::from);

        res
    }

    fn anonymous_decrypt(&self,
                         wallet_handle: i32,
                         my_vk: String,
                         encrypted_msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        let my_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_vk))?;

        let my_key = Key::from_json(&my_key_json)
            .map_err(|err| IndyError::CommonError(CommonError::InvalidState(format!("Invalid Key json: {:?}", err))))?;

        let decrypted_msg = self.crypto_service.decrypt_sealed(&my_key, &encrypted_msg)?;

        let parsed_msg: AnonymousEncryptedMessage = serde_json::from_slice(decrypted_msg.as_slice())
            .map_err(|err| CommonError::InvalidStructure(format!("Can't determine internal msg format: {:?}", err)))?;

        let res = Ok(base64::decode(&parsed_msg.msg)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode internal msg filed from base64 {}", err)))?);

        res
    }

    fn set_key_metadata(&self, wallet_handle: i32, verkey: String, metadata: String) -> Result<(), IndyError> {
        self.crypto_service.validate_key(&verkey)?;
        self._wallet_set_key_metadata(wallet_handle, &verkey, &metadata)?;
        Ok(())
    }

    fn get_key_metadata(&self,
                        wallet_handle: i32,
                        verkey: String) -> Result<String, IndyError> {
        self.crypto_service.validate_key(&verkey)?;
        let res = self._wallet_get_key_metadata(wallet_handle, &verkey)?;
        Ok(res)
    }

    fn _wallet_set_key(&self, wallet_handle: i32, key: &Key) -> Result<(), IndyError> {
        let key_json = Key::to_json(&key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("key::{}", key.verkey), &key_json)?;
        Ok(())
    }

    fn _wallet_get_key(&self, wallet_handle: i32, key: &str) -> Result<Key, IndyError> {
        let key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", key))?;

        let res = Key::from_json(&key_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize Key: {}", err.description())))?;
        Ok(res)
    }

    fn _wallet_set_key_metadata(&self, wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), IndyError> {
        self.wallet_service.set(wallet_handle, &format!("key::{}::metadata", verkey), metadata)?;
        Ok(())
    }

    fn _wallet_get_key_metadata(&self, wallet_handle: i32, verkey: &str) -> Result<String, IndyError> {
        let res = self.wallet_service.get(wallet_handle, &format!("key::{}::metadata", verkey))?;
        Ok(res)
    }
}


#[derive(Serialize, Deserialize)]
struct AuthenticatedEncryptedMessage {
    msg: String,
    sender: String,
    nonce: String
}

impl JsonEncodable for AuthenticatedEncryptedMessage {}

impl<'a> JsonDecodable<'a> for AuthenticatedEncryptedMessage {}


#[derive(Serialize, Deserialize)]
struct AnonymousEncryptedMessage {
    msg: String
}

impl JsonEncodable for AnonymousEncryptedMessage {}

impl<'a> JsonDecodable<'a> for AnonymousEncryptedMessage {}