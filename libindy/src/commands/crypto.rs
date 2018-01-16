extern crate indy_crypto;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::indy::IndyError;
use services::crypto::types::{KeyInfo, Key, ComboBox};
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
            CryptoCommand::CryptoSign(wallet_handle, my_vk, msg, cb) => {
                info!("CryptoSign command received");
                cb(self.crypto_sign(wallet_handle, &my_vk, &msg));
            }
            CryptoCommand::CryptoVerify(their_vk, msg, signature, cb) => {
                info!("CryptoVerify command received");
                cb(self.crypto_verify(their_vk, msg, signature));
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
        info!("create_key >>> wallet_handle: {:?}, key_info_json: {:?}", wallet_handle, key_info_json);

        let key_info = KeyInfo::from_json(&key_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid KeyInfo json: {}", err.description())))?;

        let key = self.crypto_service.create_key(&key_info)?;
        self._wallet_set_key(wallet_handle, &key)?;

        let res = key.verkey;

        info!("create_key <<< res: {:?}", res);

        Ok(res)
    }

    fn crypto_sign(&self,
                   wallet_handle: i32,
                   my_vk: &str,
                   msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        info!("crypto_sign >>> wallet_handle: {:?}, sender_vk: {:?}, msg: {:?}", wallet_handle, my_vk, msg);

        self.crypto_service.validate_key(my_vk)?;

        let key = self._wallet_get_key(wallet_handle, &my_vk)?;

        let res = self.crypto_service.sign(&key, msg)?;

        info!("crypto_sign <<< res: {:?}", res);

        Ok(res)
    }

    fn crypto_verify(&self,
                     their_vk: String,
                     msg: Vec<u8>,
                     signature: Vec<u8>) -> Result<bool, IndyError> {
        info!("crypto_verify >>> their_vk: {:?}, msg: {:?}, signature: {:?}", their_vk, msg, signature);

        self.crypto_service.validate_key(&their_vk)?;

        let res = self.crypto_service.verify(&their_vk, &msg, &signature)?;

        info!("crypto_verify <<< res: {:?}", res);

        Ok(res)
    }

    fn authenticated_encrypt(&self,
                             wallet_handle: i32,
                             my_vk: String,
                             their_vk: String,
                             msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        info!("authenticated_encrypt >>> wallet_handle: {:?}, my_vk: {:?}, their_vk: {:?}, msg: {:?}", wallet_handle, my_vk, their_vk, msg);

        self.crypto_service.validate_key(&my_vk)?;
        self.crypto_service.validate_key(&their_vk)?;

        let my_key = CryptoCommandExecutor::_wallet_get_key(&self, wallet_handle, &my_vk)?;

        let msg = self.crypto_service.create_combo_box(&my_key, &their_vk, msg.as_slice())?;

        let msg = msg.to_msg_pack()
            .map_err(|e| CommonError::InvalidState(format!("Can't serialize ComboBox: {:?}", e)))?;

        let res = self.crypto_service.encrypt_sealed(&their_vk, &msg)?;

        info!("authenticated_encrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn authenticated_decrypt(&self,
                             wallet_handle: i32,
                             my_vk: String,
                             msg: Vec<u8>) -> Result<(String, Vec<u8>), IndyError> {
        info!("authenticated_decrypt >>> wallet_handle: {:?}, my_vk: {:?}, msg: {:?}", wallet_handle, my_vk, msg);

        self.crypto_service.validate_key(&my_vk)?;

        let my_key = CryptoCommandExecutor::_wallet_get_key(&self, wallet_handle, &my_vk)?;

        let decrypted_msg = self.crypto_service.decrypt_sealed(&my_key, &msg)?;

        let parsed_msg = ComboBox::from_msg_pack(decrypted_msg.as_slice())
            .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize ComboBox: {:?}", err)))?;

        let doc: Vec<u8> = base64::decode(&parsed_msg.msg)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode internal msg filed from base64 {}", err)))?;

        let nonce: Vec<u8> = base64::decode(&parsed_msg.nonce)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode nonce from base64 {}", err)))?;

        let decrypted_msg = self.crypto_service.decrypt(&my_key, &parsed_msg.sender, &doc, &nonce)?;

        let res = (parsed_msg.sender.clone(), decrypted_msg);

        info!("authenticated_decrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn anonymous_encrypt(&self,
                         their_vk: String,
                         msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        info!("anonymous_encrypt >>> their_vk: {:?}, msg: {:?}", their_vk, msg);

        self.crypto_service.validate_key(&their_vk)?;

        let res = self.crypto_service.encrypt_sealed(&their_vk, &msg)?;

        info!("anonymous_encrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn anonymous_decrypt(&self,
                         wallet_handle: i32,
                         my_vk: String,
                         encrypted_msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        info!("anonymous_decrypt >>> wallet_handle: {:?}, my_vk: {:?}, encrypted_msg: {:?}", wallet_handle, my_vk, encrypted_msg);

        self.crypto_service.validate_key(&my_vk)?;

        let my_key = CryptoCommandExecutor::_wallet_get_key(&self, wallet_handle, &my_vk)?;

        let res = self.crypto_service.decrypt_sealed(&my_key, &encrypted_msg)?;

        info!("anonymous_decrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn set_key_metadata(&self, wallet_handle: i32, verkey: String, metadata: String) -> Result<(), IndyError> {
        info!("set_key_metadata >>> wallet_handle: {:?}, verkey: {:?}, metadata: {:?}", wallet_handle, verkey, metadata);

        self.crypto_service.validate_key(&verkey)?;
        let res = self._wallet_set_key_metadata(wallet_handle, &verkey, &metadata)?;

        info!("set_key_metadata <<< res: {:?}", res);

        Ok(res)
    }

    fn get_key_metadata(&self,
                        wallet_handle: i32,
                        verkey: String) -> Result<String, IndyError> {
        info!("get_key_metadata >>> wallet_handle: {:?}, verkey: {:?}", wallet_handle, verkey);

        self.crypto_service.validate_key(&verkey)?;
        let res = self._wallet_get_key_metadata(wallet_handle, &verkey)?;

        info!("get_key_metadata <<< res: {:?}", res);

        Ok(res)
    }

    fn _wallet_set_key(&self, wallet_handle: i32, key: &Key) -> Result<(), IndyError> {
        info!("_wallet_set_key >>> wallet_handle: {:?}, key: {:?}", wallet_handle, key);

        let key_json = Key::to_json(&key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        let res = self.wallet_service.set(wallet_handle, &format!("key::{}", key.verkey), &key_json)?;

        info!("_wallet_set_key <<< res: {:?}", res);

        Ok(res)
    }

    fn _wallet_get_key(&self, wallet_handle: i32, key: &str) -> Result<Key, IndyError> {
        info!("_wallet_get_key >>> wallet_handle: {:?}, key: {:?}", wallet_handle, key);

        let key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", key))?;

        let res = Key::from_json(&key_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize Key: {}", err.description())))?;

        info!("_wallet_get_key <<< res: {:?}", res);

        Ok(res)
    }

    fn _wallet_set_key_metadata(&self, wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), IndyError> {
        info!("_wallet_set_key_metadata >>> wallet_handle: {:?}, verkey: {:?}, metadata: {:?}", wallet_handle, verkey, metadata);

        let res = self.wallet_service.set(wallet_handle, &format!("key::{}::metadata", verkey), metadata)?;

        info!("_wallet_set_key_metadata <<< res: {:?}", res);

        Ok(res)
    }

    fn _wallet_get_key_metadata(&self, wallet_handle: i32, verkey: &str) -> Result<String, IndyError> {
        info!("_wallet_get_key_metadata >>> wallet_handle: {:?}, verkey: {:?}", wallet_handle, verkey);

        let res = self.wallet_service.get(wallet_handle, &format!("key::{}::metadata", verkey))?;

        info!("_wallet_get_key_metadata <<< res: {:?}", res);

        Ok(res)
    }
}