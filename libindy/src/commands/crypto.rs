use utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::indy::IndyError;
use services::signus::types::{KeyInfo, Key};
use services::wallet::WalletService;
use services::signus::SignusService;

use std::error::Error;
use std::rc::Rc;
use std::str;

pub enum CryptoCommand {
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
    CreateKey(
        i32, // wallet handle
        String, // key info json
        Box<Fn(Result<String/*verkey*/, IndyError>) + Send>),
    CryptoBox(
        i32, // wallet handle
        String, // my vk
        String, // their vk
        Vec<u8>, // msg
        Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>) + Send>),
    CryptoBoxOpen(
        i32, // wallet handle
        String, // my vk
        String, // their vk
        Vec<u8>, // encrypted msg
        Vec<u8>, // nonce
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    CryptoBoxSeal(
        String, // their did
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    CryptoBoxSealOpen(
        i32, // wallet handle
        String, // my vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    SetKeyMetadata(
        i32, // wallet handle
        String, // verkey
        String, // metadata
        Box<Fn(Result<(), IndyError>) + Send>),
    GetKeyMetadata(
        i32, // wallet handle
        String, // verkey
        Box<Fn(Result<String, IndyError>) + Send>),
}

pub struct CryptoCommandExecutor {
    wallet_service: Rc<WalletService>,
    signus_service: Rc<SignusService>,
}

impl CryptoCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
               signus_service: Rc<SignusService>,
    ) -> CryptoCommandExecutor {
        CryptoCommandExecutor {
            wallet_service,
            signus_service,
        }
    }

    pub fn execute(&self, command: CryptoCommand) {
        match command {
            CryptoCommand::CreateKey(wallet_handle, key_info_json, cb) => {
                info!("CreateKey command received");
                cb(self.create_key(wallet_handle, key_info_json));
            }
            CryptoCommand::CryptoSign(wallet_handle, my_vk, msg, cb) => {
                info!("CryptoSign command received");
                cb(self.crypto_sign(wallet_handle, &my_vk, &msg));
            }
            CryptoCommand::CryptoVerify(their_vk, msg, signature, cb) => {
                info!("CryptoVerify command received");
                cb(self.crypto_verify(their_vk, msg, signature));
            }
            CryptoCommand::CryptoBox(wallet_handle, my_vk, their_vk, msg, cb) => {
                info!("CryptoBox command received");
                cb(self.crypto_box(wallet_handle, my_vk, their_vk, msg));
            }
            CryptoCommand::CryptoBoxOpen(wallet_handle, my_vk, their_vk, encrypted_msg, nonce, cb) => {
                info!("CryptoBoxOpen command received");
                cb(self.crypto_box_open(wallet_handle, my_vk, their_vk, encrypted_msg, nonce));
            }
            CryptoCommand::CryptoBoxSeal(their_vk, msg, cb) => {
                info!("CryptoBoxSeal command received");
                cb(self.crypto_box_seal(their_vk, msg));
            }
            CryptoCommand::CryptoBoxSealOpen(wallet_handle, my_vk, encrypted_msg, cb) => {
                info!("CryptoBoxSealOpen command received");
                cb(self.crypto_box_seal_open(wallet_handle, my_vk, encrypted_msg));
            }
            CryptoCommand::SetKeyMetadata(wallet_handle, verkey, metadata, cb) => {
                info!("SetKeyMetadata command received");
                cb(self.set_key_metadata(wallet_handle, verkey, metadata));
            }
            CryptoCommand::GetKeyMetadata(wallet_handle, verkey, cb) => {
                info!("GetKeyMetadata command received");
                cb(self.get_key_metadata(wallet_handle, verkey));
            }
        };
    }

    fn create_key(&self, wallet_handle: i32, key_info_json: String) -> Result<String, IndyError> {
        /*let key_info = KeyInfo::from_json(&key_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid KeyInfo json: {}", err.description())))?;*/
        let key_info = SignusService::get_key_info_from_json(key_info_json)?;

        let key = self.signus_service.create_key(&key_info)?;
        self._wallet_set_key(wallet_handle, &key)?;

        let res = key.verkey;
        Ok(res)
    }

    fn crypto_sign(&self,
                   wallet_handle: i32,
                   my_vk: &str,
                   msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        self.signus_service.validate_key(my_vk)?;

        let key = self._wallet_get_key(wallet_handle, &my_vk)?;

        let res = self.signus_service.sign(&key, msg)?;
        Ok(res)
    }

    fn crypto_verify(&self,
                     their_vk: String,
                     msg: Vec<u8>,
                     signature: Vec<u8>) -> Result<bool, IndyError> {
        self.signus_service.validate_key(&their_vk)?;

        let res = self.signus_service.verify(&their_vk, &msg, &signature)?;
        Ok(res)
    }

    fn crypto_box(&self,
                  wallet_handle: i32,
                  my_vk: String,
                  their_vk: String,
                  msg: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), IndyError> {
        self.signus_service.validate_key(&my_vk)?;
        self.signus_service.validate_key(&their_vk)?;

        let my_key = self._wallet_get_key(wallet_handle, &my_vk)?;

        let res = self.signus_service.encrypt(&my_key, &their_vk, &msg)?;
        Ok(res)
    }

    fn crypto_box_open(&self,
                       wallet_handle: i32,
                       my_vk: String,
                       their_vk: String,
                       encrypted_msg: Vec<u8>,
                       nonce: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        self.signus_service.validate_key(&my_vk)?;
        self.signus_service.validate_key(&their_vk)?;

        let my_key = self._wallet_get_key(wallet_handle, &my_vk)?;

        let res = self.signus_service.decrypt(&my_key, &their_vk, &encrypted_msg, &nonce)?;
        Ok(res)
    }

    fn crypto_box_seal(&self,
                       their_vk: String,
                       msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        self.signus_service.validate_key(&their_vk)?;

        let res = self.signus_service.encrypt_sealed(&their_vk, &msg)?;
        Ok(res)
    }

    fn crypto_box_seal_open(&self,
                            wallet_handle: i32,
                            my_vk: String,
                            encrypted_msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        self.signus_service.validate_key(&my_vk)?;

        let key = self._wallet_get_key(wallet_handle, &my_vk)?;

        let res = self.signus_service.decrypt_sealed(&key, &encrypted_msg)?;
        Ok(res)
    }

    fn set_key_metadata(&self, wallet_handle: i32, verkey: String, metadata: String) -> Result<(), IndyError> {
        self.signus_service.validate_key(&verkey)?;
        self._wallet_set_key_metadata(wallet_handle, &verkey, &metadata)?;
        Ok(())
    }

    fn get_key_metadata(&self,
                        wallet_handle: i32,
                        verkey: String) -> Result<String, IndyError> {
        self.signus_service.validate_key(&verkey)?;
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
        /*let key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", key))?;

        let res = Key::from_json(&key_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize Key: {}", err.description())))?;
        Ok(res)*/
        CryptoCommandExecutor::__wallet_get_key(self.wallet_service.clone(), wallet_handle, key)
    }

    pub fn __wallet_get_key(wallet_service: Rc<WalletService>, wallet_handle: i32, key: &str) -> Result<Key, IndyError> {
        let key_json = wallet_service.get(wallet_handle, &format!("key::{}", key))?;

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
