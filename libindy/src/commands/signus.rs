use utils::json::{JsonDecodable, JsonEncodable};
use errors::signus::SignusError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use errors::indy::IndyError;
use services::signus::types::{KeyInfo, MyDidInfo, TheirDidInfo, Did, Key, Endpoint};
use services::ledger::types::{Reply, GetNymResultData, GetNymReplyResult};
use services::pool::PoolService;
use services::wallet::WalletService;
use services::signus::SignusService;
use services::ledger::LedgerService;

use std::error::Error;
use std::rc::Rc;
use std::str;
use std::cell::RefCell;

use commands::ledger::LedgerCommand;
use commands::{Command, CommandExecutor};
use std::collections::HashMap;
use utils::sequence::SequenceUtils;

use super::utils::check_wallet_and_pool_handles_consistency;

use utils::crypto::base58::Base58;

#[derive()]
pub enum SignusCommand {
    CreateAndStoreMyDid(
        i32, // wallet handle
        String, // my did info json
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    ReplaceKeysStart(
        i32, // wallet handle
        String, // key info json
        String, // did
        Box<Fn(Result<String, IndyError>) + Send>),
    ReplaceKeysApply(
        i32, // wallet handle
        String, // my did
        Box<Fn(Result<(), IndyError>) + Send>),
    StoreTheirDid(
        i32, // wallet handle
        String, // their did info json
        Box<Fn(Result<(), IndyError>) + Send>),
    Sign(
        i32, // wallet handle
        String, // my did
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    VerifySignature(
        i32, // wallet handle
        i32, // pool handle,
        String, // their did
        Vec<u8>, // msg
        Vec<u8>, // signature
        Box<Fn(Result<bool, IndyError>) + Send>),
    Encrypt(
        i32, // wallet handle
        i32, // pool handle
        String, // my did
        String, // their did
        Vec<u8>, // msg
        Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>) + Send>),
    Decrypt(
        i32, // wallet handle
        i32, // pool handle
        String, // my did
        String, // their did
        Vec<u8>, // encrypted msg
        Vec<u8>, // nonce
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    EncryptSealed(
        i32, // wallet handle
        i32, // pool handle
        String, // their did
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    DecryptSealed(
        i32, // wallet handle
        String, // my did
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
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
    KeyForDid(
        i32, // pool handle
        i32, // wallet handle
        String, // did (my or their)
        Box<Fn(Result<String/*key*/, IndyError>) + Send>),
    SetEndpointForDid(
        i32, // wallet handle
        String, // did
        String, // address
        String, // transport_key
        Box<Fn(Result<(), IndyError>) + Send>),
    GetEndpointForDid(
        i32, // wallet handle
        String, // did
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    SetDidMetadata(
        i32, // wallet handle
        String, // did
        String, // metadata
        Box<Fn(Result<(), IndyError>) + Send>),
    GetDidMetadata(
        i32, // wallet handle
        String, // did
        Box<Fn(Result<String, IndyError>) + Send>),
    // Internal commands
    GetNymAck(
        i32, // wallet_handle
        Result<String, IndyError>, // GetNym Result
        i32, // deferred cmd id
    )
}

pub struct SignusCommandExecutor {
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,
    signus_service: Rc<SignusService>,
    ledger_service: Rc<LedgerService>,
    deferred_commands: RefCell<HashMap<i32, SignusCommand>>,
}

impl SignusCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>,
               signus_service: Rc<SignusService>,
               ledger_service: Rc<LedgerService>) -> SignusCommandExecutor {
        SignusCommandExecutor {
            pool_service,
            wallet_service,
            signus_service,
            ledger_service,
            deferred_commands: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: SignusCommand) {
        match command {
            SignusCommand::CreateAndStoreMyDid(wallet_handle, my_did_info_json, cb) => {
                info!(target: "signus_command_executor", "CreateAndStoreMyDid command received");
                self.create_and_store_my_did(wallet_handle, &my_did_info_json, cb);
            }
            SignusCommand::ReplaceKeysStart(wallet_handle, key_info_json, did, cb) => {
                info!(target: "signus_command_executor", "ReplaceKeysStart command received");
                self.replace_keys_start(wallet_handle, &key_info_json, &did, cb);
            }
            SignusCommand::ReplaceKeysApply(wallet_handle, did, cb) => {
                info!(target: "signus_command_executor", "ReplaceKeysApply command received");
                self.replace_keys_apply(wallet_handle, &did, cb);
            }
            SignusCommand::StoreTheirDid(wallet_handle, identity_json, cb) => {
                info!(target: "signus_command_executor", "StoreTheirDid command received");
                self.store_their_did(wallet_handle, &identity_json, cb);
            }
            SignusCommand::Sign(wallet_handle, did, msg, cb) => {
                info!(target: "signus_command_executor", "Sign command received");
                self.sign(wallet_handle, &did, &msg, cb);
            }
            SignusCommand::VerifySignature(wallet_handle, pool_handle, their_did, msg, signature, cb) => {
                info!(target: "signus_command_executor", "VerifySignature command received");
                self.verify_signature(wallet_handle, pool_handle, their_did, msg, signature, cb);
            }
            SignusCommand::Encrypt(wallet_handle, pool_handle, my_did, their_did, msg, cb) => {
                info!(target: "signus_command_executor", "Encrypt command received");
                self.encrypt(wallet_handle, pool_handle, my_did, their_did, msg, cb);
            }
            SignusCommand::Decrypt(wallet_handle, pool_handle, my_did, their_did, encrypted_msg, nonce, cb) => {
                info!(target: "signus_command_executor", "Decrypt command received");
                self.decrypt(wallet_handle, pool_handle, my_did, their_did, encrypted_msg, nonce, cb);
            }
            SignusCommand::EncryptSealed(wallet_handle, pool_handle, their_did, msg, cb) => {
                info!(target: "signus_command_executor", "SealedEncrypt command received");
                self.encrypt_sealed(wallet_handle, pool_handle, their_did, msg, cb);
            }
            SignusCommand::DecryptSealed(wallet_handle, did, encrypted_msg, cb) => {
                info!(target: "signus_command_executor", "DecryptSealed command received");
                self.decrypt_sealed(wallet_handle, &did, &encrypted_msg, cb);
            }
            SignusCommand::CreateKey(wallet_handle, key_info_json, cb) => {
                info!(target: "signus_command_executor", "CreateKey command received");
                self.create_key(wallet_handle, &key_info_json, cb);
            }
            SignusCommand::SetKeyMetadata(wallet_handle, verkey, metadata, cb) => {
                info!(target: "signus_command_executor", "SetKeyMetadata command received");
                self.set_key_metadata(wallet_handle, &verkey, &metadata, cb);
            }
            SignusCommand::GetKeyMetadata(wallet_handle, verkey, cb) => {
                info!(target: "signus_command_executor", "GetKeyMetadata command received");
                self.get_key_metadata(wallet_handle, &verkey, cb);
            }
            SignusCommand::KeyForDid(pool_handle, wallet_handle, did, cb) => {
                info!(target: "signus_command_executor", "KeyForDid command received");
                self.key_for_did(pool_handle, wallet_handle, &did, cb);
            }
            SignusCommand::SetEndpointForDid(wallet_handle, did, address, transport_key, cb) => {
                info!(target: "signus_command_executor", "SetEndpointForDid command received");
                self.set_endpoint_for_did(wallet_handle, &did, &address, &transport_key, cb);
            }
            SignusCommand::GetEndpointForDid(wallet_handle, did, cb) => {
                info!(target: "signus_command_executor", "GetEndpointForDid command received");
                self.get_endpoint_for_did(wallet_handle, &did, cb);
            }
            SignusCommand::SetDidMetadata(wallet_handle, did, metadata, cb) => {
                info!(target: "signus_command_executor", "SetDidMetadata command received");
                self.set_did_metadata(wallet_handle, &did, &metadata, cb);
            }
            SignusCommand::GetDidMetadata(wallet_handle, did, cb) => {
                info!(target: "signus_command_executor", "GetDidMetadata command received");
                self.get_did_metadata(wallet_handle, &did, cb);
            }
            SignusCommand::GetNymAck(wallet_handle, result, deferred_cmd_id) => {
                info!(target: "signus_command_executor", "GetNymAck command received");
                self.get_nym_ack(wallet_handle, result, deferred_cmd_id);
            }
        };
    }

    fn create_and_store_my_did(&self,
                               wallet_handle: i32,
                               my_did_info_json: &str,
                               cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        cb(self._create_and_store_my_did(wallet_handle, my_did_info_json));
    }

    fn _create_and_store_my_did(&self, wallet_handle: i32, my_did_info_json: &str) -> Result<(String, String), IndyError> {
        let my_did_info = MyDidInfo::from_json(&my_did_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid MyDidInfo json: {}", err.description())))?;

        let (my_did, my_key) = self.signus_service.create_my_did(&my_did_info)?;

        let my_did_json = Did::to_json(&my_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Did: {}", err.description())))?;

        let my_key_json = Key::to_json(&my_key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", my_did.did), &my_did_json)?;
        self.wallet_service.set(wallet_handle, &format!("my_key::{}", my_key.verkey), &my_key_json)?;
        Ok((my_did.did, my_did.verkey))
    }

    fn replace_keys_start(&self,
                          wallet_handle: i32,
                          key_info_json: &str,
                          did: &str,
                          cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._replace_keys_start(wallet_handle, key_info_json, did));
    }

    fn _replace_keys_start(&self,
                           wallet_handle: i32,
                           key_info_json: &str,
                           did: &str) -> Result<String, IndyError> {
        self.wallet_service.get(wallet_handle, &format!("my_did::{}", did))?;

        let key_info: KeyInfo = KeyInfo::from_json(key_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(format!("Invalid KeyInfo json: {}", err.description())))?;

        let my_key = self.signus_service.create_key(&key_info)?;
        let my_did = Did::new(did.to_owned(), my_key.verkey.clone());

        let did_json = Did::to_json(&my_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Did: {}", err.description())))?;

        let key_json = Key::to_json(&my_key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_did_temporary::{}", my_did.did), &did_json)?;
        self.wallet_service.set(wallet_handle, &format!("my_key::{}", my_key.verkey), &key_json)?;

        Ok(my_did.verkey)
    }

    fn replace_keys_apply(&self,
                          wallet_handle: i32,
                          did: &str,
                          cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._replace_keys_apply(wallet_handle, did));
    }

    fn _replace_keys_apply(&self,
                           wallet_handle: i32,
                           did: &str) -> Result<(), IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did_temporary::{}", did))?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", did), &my_did_json)?;

        Ok(())
    }

    fn store_their_did(&self,
                       wallet_handle: i32,
                       their_did_info_json: &str,
                       cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._store_their_did(wallet_handle, their_did_info_json));
    }

    fn _store_their_did(&self,
                        wallet_handle: i32,
                        their_did_info_json: &str) -> Result<(), IndyError> {
        let their_did_info = TheirDidInfo::from_json(their_did_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(format!("Invalid TheirDidInfo json: {}", err.description())))?;

        let their_did = self.signus_service.create_their_did(&their_did_info)?;

        let their_did_json = their_did.to_json()
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Did: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;
        Ok(())
    }

    fn sign(&self,
            wallet_handle: i32,
            did: &str,
            msg: &[u8],
            cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        cb(self._sign(wallet_handle, did, msg));
    }

    fn _sign(&self,
             wallet_handle: i32,
             did: &str,
             msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", did))?;
        let my_did = Did::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState((format!("Invalid Did json"))))?;

        let key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey))?;
        let key = Key::from_json(&key_json)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState((format!("Invalid key json"))))?;

        let signed_msg = self.signus_service.sign(&key, msg)?;
        Ok(signed_msg)
    }

    fn verify_signature(&self,
                        wallet_handle: i32,
                        pool_handle: i32,
                        their_did: String,
                        msg: Vec<u8>,
                        signature: Vec<u8>,
                        cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb);

        let their_did = match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", their_did)) {
            Ok(their_did_json) => {
                if let Ok(their_did) = Did::from_json(&their_did_json) {
                    their_did
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid their Did json"))))));
                }
            }
            Err(WalletError::NotFound(_)) => {
                // No their their_did present in the wallet. Deffer this command until it is fetched from ledger.
                let deferred_cmd_id = self._defer_command(
                    SignusCommand::VerifySignature(
                        wallet_handle,
                        pool_handle,
                        their_did.clone(), // TODO: FIXME: Try to avoid cloning
                        msg,
                        signature,
                        cb));
                return self._fetch_their_did_from_ledger(wallet_handle, pool_handle, &their_did, deferred_cmd_id);
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let res = self.signus_service.verify(&their_did.verkey, &msg, &signature)
            .map_err(|err| IndyError::SignusError(err));

        cb(res);
    }

    fn encrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: String,
               their_did: String,
               msg: Vec<u8>,
               cb: Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>) + Send>) {
        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb);

        let my_did = match self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did)) {
            Ok(my_did_json) => {
                if let Ok(my_did) = Did::from_json(&my_did_json) {
                    my_did
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid my Did json"))))));
                }
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let my_key = match self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey)) {
            Ok(my_key_json) => {
                if let Ok(my_key) = Key::from_json(&my_key_json) {
                    my_key
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid my Key json"))))));
                }
            }
            Err(WalletError::NotFound(_)) => {
                return cb(Err(IndyError::SignusError(
                    SignusError::CommonError(CommonError::InvalidState(format!("No Key for my DID"))))));
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let their_did = match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", their_did)) {
            Ok(their_did_json) => {
                if let Ok(their_did) = Did::from_json(&their_did_json) {
                    their_did
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid their Did json"))))));
                }
            }
            Err(WalletError::NotFound(_)) => {
                // No their their_did present in the wallet. Deffer this command until it is fetched from ledger.
                let deferred_cmd_id = self._defer_command(
                    SignusCommand::Encrypt(
                        wallet_handle,
                        pool_handle,
                        my_did.did.clone(), // TODO: FIXME: Try to avoid cloning
                        their_did.clone(), // TODO: FIXME: Try to avoid cloning
                        msg,
                        cb));
                return self._fetch_their_did_from_ledger(wallet_handle, pool_handle, &their_did, deferred_cmd_id);
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let res = self.signus_service.encrypt(&my_key, &their_did.verkey, &msg)
            .map_err(|err| IndyError::SignusError(err));

        cb(res);
    }

    fn decrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: String,
               their_did: String,
               encrypted_msg: Vec<u8>,
               nonce: Vec<u8>,
               cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb);

        let my_did = match self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did)) {
            Ok(my_did_json) => {
                if let Ok(my_did) = Did::from_json(&my_did_json) {
                    my_did
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid my Did json"))))));
                }
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let my_key = match self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey)) {
            Ok(my_key_json) => {
                if let Ok(my_key) = Key::from_json(&my_key_json) {
                    my_key
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid my Key json"))))));
                }
            }
            Err(WalletError::NotFound(_)) => {
                return cb(Err(IndyError::SignusError(
                    SignusError::CommonError(CommonError::InvalidState(format!("No Key for my DID"))))));
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let their_did = match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", their_did)) {
            Ok(their_did_json) => {
                if let Ok(their_did) = Did::from_json(&their_did_json) {
                    their_did
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid their Did json"))))));
                }
            }
            Err(WalletError::NotFound(_)) => {
                // No their their_did present in the wallet. Deffer this command until it is fetched from ledger.
                let deferred_cmd_id = self._defer_command(
                    SignusCommand::Decrypt(
                        wallet_handle,
                        pool_handle,
                        my_did.did.clone(), // TODO: FIXME: Try to avoid cloning
                        their_did.clone(), // TODO: FIXME: Try to avoid cloning
                        encrypted_msg,
                        nonce,
                        cb));
                return self._fetch_their_did_from_ledger(wallet_handle, pool_handle, &their_did, deferred_cmd_id);
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let res = self.signus_service.decrypt(&my_key, &their_did.verkey, &encrypted_msg, &nonce)
            .map_err(|err| IndyError::SignusError(err));

        cb(res);
    }

    fn encrypt_sealed(&self,
                      wallet_handle: i32,
                      pool_handle: i32,
                      their_did: String,
                      msg: Vec<u8>,
                      cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        let their_did = match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", their_did)) {
            Ok(their_did_json) => {
                if let Ok(their_did) = Did::from_json(&their_did_json) {
                    their_did
                } else {
                    return cb(Err(IndyError::SignusError(
                        SignusError::CommonError(CommonError::InvalidState(format!("Invalid their Did json"))))));
                }
            }
            Err(WalletError::NotFound(_)) => {
                // No their their_did present in the wallet. Deffer this command until it is fetched from ledger.
                let deferred_cmd_id = self._defer_command(
                    SignusCommand::EncryptSealed(
                        wallet_handle,
                        pool_handle,
                        their_did.clone(), // TODO: FIXME: Try to avoid cloning
                        msg,
                        cb));
                return self._fetch_their_did_from_ledger(wallet_handle, pool_handle, &their_did, deferred_cmd_id);
            }
            Err(err) => return cb(Err(IndyError::WalletError(err)))
        };

        let res = self.signus_service.encrypt_sealed(&their_did.verkey, &msg)
            .map_err(|err| IndyError::SignusError(err));

        cb(res);
    }

    fn decrypt_sealed(&self,
                      wallet_handle: i32,
                      did: &str,
                      msg: &[u8],
                      cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        cb(self._decrypt_sealed(wallet_handle, did, msg));
    }

    fn _decrypt_sealed(&self,
                       wallet_handle: i32,
                       did: &str,
                       encrypted_msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", did))?;
        let my_did = Did::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        let key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey))?;
        let key = Key::from_json(&key_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        self.signus_service.decrypt_sealed(&key, encrypted_msg)
            .map_err(|err| IndyError::SignusError(err))
    }

    fn create_key(&self,
                  wallet_handle: i32,
                  key_info_json: &str,
                  cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._create_key(wallet_handle, key_info_json));
    }

    fn _create_key(&self, wallet_handle: i32, key_info_json: &str) -> Result<String, IndyError> {
        let key_info = KeyInfo::from_json(&key_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid KeyInfo json: {}", err.description())))?;

        let key = self.signus_service.create_key(&key_info)?;

        let key_json = Key::to_json(&key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("key::{}", key.verkey), &key_json)?;
        Ok(key.verkey)
    }

    fn set_key_metadata(&self,
                        wallet_handle: i32,
                        verkey: &str,
                        metadata: &str,
                        cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._set_key_metadata(wallet_handle, verkey, metadata));
    }

    fn _set_key_metadata(&self, wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), IndyError> {
        Base58::decode(verkey)?;
        self.wallet_service.set(wallet_handle, &format!("key::{}::metadata", verkey), metadata)?;
        Ok(())
    }

    fn get_key_metadata(&self,
                        wallet_handle: i32,
                        verkey: &str,
                        cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.wallet_service.get(wallet_handle, &format!("key::{}::metadata", verkey))
            .map_err(|err| IndyError::WalletError(err)));
    }

    fn key_for_did(&self,
                   pool_handle: i32,
                   wallet_handle: i32,
                   did: &str,
                   cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._key_for_did(pool_handle, wallet_handle, did));
    }

    fn _key_for_did(&self, pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, IndyError> {
        // TODO: FIXME: It works only for my did now!!!
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", did))?;
        let my_did = Did::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        Ok(my_did.verkey)
    }

    fn set_endpoint_for_did(&self,
                            wallet_handle: i32,
                            did: &str,
                            address: &str,
                            transport_key: &str,
                            cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._set_endpoint_for_did(wallet_handle, did, address, transport_key));
    }

    fn _set_endpoint_for_did(&self, wallet_handle: i32, did: &str, address: &str, transport_key: &str) -> Result<(), IndyError> {
        Base58::decode(did)?;
        Base58::decode(transport_key)?;

        let endpoint = Endpoint::new(address.to_string(), transport_key.to_string());
        let endpoint_json = endpoint.to_json()
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(format!("Can't serialize Endpoint: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("did::{}::endpoint", did), &endpoint_json)?;
        Ok(())
    }

    fn get_endpoint_for_did(&self,
                            wallet_handle: i32,
                            did: &str,
                            cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        cb(self._get_endpoint_for_did(wallet_handle, did));
    }

    fn _get_endpoint_for_did(&self, wallet_handle: i32, did: &str) -> Result<(String, String), IndyError> {
        let endpoint_json = self.wallet_service.get(wallet_handle, &format!("did::{}::endpoint", did))?;
        let endpoint: Endpoint = Endpoint::from_json(&endpoint_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(format!("Can't deserialize Endpoint: {}", err.description())))?;

        Ok((endpoint.ha, endpoint.verkey))
    }

    fn set_did_metadata(&self,
                        wallet_handle: i32,
                        did: &str,
                        metadata: &str,
                        cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._set_did_metadata(wallet_handle, did, metadata));
    }

    fn _set_did_metadata(&self, wallet_handle: i32, did: &str, metadata: &str) -> Result<(), IndyError> {
        Base58::decode(did)?;

        self.wallet_service.set(wallet_handle, &format!("did::{}::metadata", did), metadata)?;
        Ok(())
    }

    fn get_did_metadata(&self,
                        wallet_handle: i32,
                        did: &str,
                        cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.wallet_service.get(wallet_handle, &format!("did::{}::metadata", did))
            .map_err(|err| IndyError::WalletError(err)));
    }

    fn get_nym_ack(&self,
                   wallet_handle: i32,
                   get_nym_reply_result: Result<String, IndyError>,
                   deferred_cmd_id: i32) {
        let res = self._get_nym_ack(wallet_handle, get_nym_reply_result);
        self._execute_deferred_command(deferred_cmd_id, res.err());
    }

    fn _get_nym_ack(&self, wallet_handle: i32, get_nym_reply_result: Result<String, IndyError>) -> Result<(), IndyError> {
        let get_nym_reply = get_nym_reply_result?;

        let get_nym_response: Reply<GetNymReplyResult> = Reply::from_json(&get_nym_reply)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid GetNymReplyResult json")))?;

        let gen_nym_result_data = GetNymResultData::from_json(&get_nym_response.result.data)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid GetNymResultData json")))?;

        let their_did_info = TheirDidInfo::new(gen_nym_result_data.dest, gen_nym_result_data.verkey);

        let their_did = self.signus_service.create_their_did(&their_did_info)?;

        let their_did_json = their_did.to_json()
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Did: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;

        Ok(())
    }

    fn _defer_command(&self, cmd: SignusCommand) -> i32 {
        let deferred_cmd_id = SequenceUtils::get_next_id();
        self.deferred_commands.borrow_mut().insert(deferred_cmd_id, cmd);
        deferred_cmd_id
    }

    fn _execute_deferred_command(&self, deferred_cmd_id: i32, err: Option<IndyError>) {
        if let Some(cmd) = self.deferred_commands.borrow_mut().remove(&deferred_cmd_id) {
            if let Some(err) = err {
                self._call_error_cb(cmd, err);
            } else {
                self.execute(cmd);
            }
        } else {
            error!("No deferred command for id: {}", deferred_cmd_id)
        }
    }

    fn _call_error_cb(&self, command: SignusCommand, err: IndyError) {
        match command {
            SignusCommand::CreateAndStoreMyDid(_, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::ReplaceKeysStart(_, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::ReplaceKeysApply(_, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::StoreTheirDid(_, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::Sign(_, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::VerifySignature(_, _, _, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::Encrypt(_, _, _, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::Decrypt(_, _, _, _, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::EncryptSealed(_, _, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::DecryptSealed(_, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::CreateKey(_, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::KeyForDid(_, _, _, cb) => {
                return cb(Err(err));
            }
            _ => {}
        }
    }

    fn _fetch_their_did_from_ledger(&self,
                                    wallet_handle: i32, pool_handle: i32,
                                    did: &str, deferred_cmd_id: i32) {
        // TODO we need passing of my_did as identifier
        let get_nym_request = self.ledger_service.build_get_nym_request(did, did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    // TODO: FIXME: Remove this unwrap by sending GetNymAck with the error.
                    format!("Invalid Get Num Request: {}", err.description()))).unwrap();

        CommandExecutor::instance()
            .send(Command::Ledger(LedgerCommand::SubmitRequest(
                pool_handle,
                get_nym_request,
                Box::new(move |result| {
                    CommandExecutor::instance()
                        .send(Command::Signus(SignusCommand::GetNymAck(
                            wallet_handle,
                            result,
                            deferred_cmd_id
                        ))).unwrap();
                })
            ))).unwrap();
    }
}