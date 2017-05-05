pub mod types;
use utils::json::{JsonEncodable, JsonDecodable};
use errors::signus::SignusError;
use commands::signus::types::{DIDInfo};
use services::crypto::wrappers::ed25519::ED25519;

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum SignusCommand {
    CreateAndStoreMyDid(
        i32, // wallet handle
        String, // did json
        Box<Fn(Result<(String, String, String), SignusError>) + Send>),
    ReplaceKeys(
        i32, // wallet handle
        String, // identity json
        String, // did
        Box<Fn(Result<(String, String), SignusError>) + Send>),
    StoreTheirDid(
        i32, // wallet handle
        String, // identity json
        Box<Fn(Result<(), SignusError>) + Send>),
    Sign(
        i32, // wallet handle
        String, // did
        String, // msg
        Box<Fn(Result<String, SignusError>) + Send>),
    VerifySignature(
        i32, // wallet handle
        String, // did
        String, // msg
        String, // signature
        Box<Fn(Result<bool, SignusError>) + Send>),
    Encrypt(
        i32, // wallet handle
        String, // did
        String, // msg
        Box<Fn(Result<String, SignusError>) + Send>),
    Decrypt(
        i32, // wallet handle
        String, // did
        String, // encrypted msg
        Box<Fn(Result<String, SignusError>) + Send>)
}

pub struct SignusCommandExecutor {
    crypto_service: Rc<CryptoService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,

}

impl SignusCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> SignusCommandExecutor {
        SignusCommandExecutor {
            crypto_service: crypto_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, command: SignusCommand) {
        match command {
            SignusCommand::CreateAndStoreMyDid(walled_handle, did_json, cb) => {
                info!(target: "signus_command_executor", "CreateAndStoreMyDid command received");
                self.create_and_store_my_did(walled_handle, &did_json, cb);
            },
            SignusCommand::ReplaceKeys(walled_handle, identity_json, did, cb) => {
                info!(target: "signus_command_executor", "ReplaceKeys command received");
                self.replace_keys(walled_handle, &identity_json, &did, cb);
            },
            SignusCommand::StoreTheirDid(walled_handle, identity_json, cb) => {
                info!(target: "signus_command_executor", "StoreTheirDid command received");
                self.store_their_did(walled_handle, &identity_json, cb);
            },
            SignusCommand::Sign(walled_handle, did, msg, cb) => {
                info!(target: "signus_command_executor", "Sign command received");
                self.sign(walled_handle, &did, &msg, cb);
            },
            SignusCommand::VerifySignature(walled_handle, did, msg, signature, cb) => {
                info!(target: "signus_command_executor", "VerifySignature command received");
                self.verify_signature(walled_handle, &did, &msg, &signature, cb);
            },
            SignusCommand::Encrypt(walled_handle, did, msg, cb) => {
                info!(target: "signus_command_executor", "Encrypt command received");
                self.encrypt(walled_handle, &did, &msg, cb);
            },
            SignusCommand::Decrypt(walled_handle, did, encrypted_msg, cb) => {
                info!(target: "signus_command_executor", "Decrypt command received");
                self.decrypt(walled_handle, &did, &encrypted_msg, cb);
            }
        };
    }

    fn create_and_store_my_did(&self,
                               walled_handle: i32,
                               did_json: &str,
                               cb: Box<Fn(Result<(String, String, String), SignusError>) + Send>) {
        cb(self._create_and_store_my_did(walled_handle, did_json));
    }

    fn _create_and_store_my_did(&self, walled_handle: i32, did_json: &str) -> Result<(String, String, String), SignusError> {
        let instance = ED25519::new();
        let did_info = DIDInfo::from_json(&did_json)?;
        Ok(("".to_string(), "".to_string(), "".to_string()))
    }

    fn replace_keys(&self,
                    walled_handle: i32,
                    identity_json: &str,
                    did: &str,
                    cb: Box<Fn(Result<(String, String), SignusError>) + Send>) {
        cb(Ok(("".to_string(), "".to_string())));
    }

    fn store_their_did(&self,
                       walled_handle: i32,
                       identity_json: &str,
                       cb: Box<Fn(Result<(), SignusError>) + Send>) {
        cb(Ok(()));
    }

    fn sign(&self,
            walled_handle: i32,
            did: &str,
            msg: &str,
            cb: Box<Fn(Result<String, SignusError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn verify_signature(&self,
                        walled_handle: i32,
                        did: &str,
                        msg: &str,
                        signature: &str,
                        cb: Box<Fn(Result<bool, SignusError>) + Send>) {
        cb(Ok(true));
    }

    fn encrypt(&self,
               walled_handle: i32,
               did: &str,
               msg: &str,
               cb: Box<Fn(Result<String, SignusError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn decrypt(&self,
               walled_handle: i32,
               did: &str,
               encrypted_msg: &str,
               cb: Box<Fn(Result<String, SignusError>) + Send>) {
        cb(Ok("".to_string()));
    }
}