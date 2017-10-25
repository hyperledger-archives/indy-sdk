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
        String, // did json
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    ReplaceKeysStart(
        i32, // wallet handle
        String, // identity json
        String, // did
        Box<Fn(Result<String, IndyError>) + Send>),
    ReplaceKeysApply(
        i32, // wallet handle
        String, // did
        Box<Fn(Result<(), IndyError>) + Send>),
    StoreTheirDid(
        i32, // wallet handle
        String, // identity json
        Box<Fn(Result<(), IndyError>) + Send>),
    Sign(
        i32, // wallet handle
        String, // did
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    //TODO divide on two commands
    VerifySignature(
        i32, // wallet handle
        i32, // pool_handle,
        String, // did
        Vec<u8>, // msg
        Vec<u8>, // signature
        Box<Fn(Result<bool, IndyError>) + Send>),
    VerifySignatureGetNymAck(
        i32, // wallet handle
        Vec<u8>, // message
        Vec<u8>, // signature
        i32, //callback id
        Result<String, IndyError>) /* result json or error)*/,
    Encrypt(
        i32, // wallet handle
        i32, // pool handle
        String, // my_did
        String, // did
        Vec<u8>, // msg
        Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>) + Send>),
    EncryptGetNymAck(
        i32, // wallet handle
        String, // my_did
        Vec<u8>, // msg
        i32, //cb_id
        Result<String, IndyError> //result
    ),
    Decrypt(
        i32, // wallet handle
        String, // my_did
        String, // did
        Vec<u8>, // encrypted msg
        Vec<u8>, // nonce
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    EncryptSealed(
        i32, // wallet handle
        i32, // pool handle
        String, // did
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    EncryptSealedGetNymAck(
        i32, // wallet handle
        Vec<u8>, // msg
        i32, //cb_id
        Result<String, IndyError> //result
    ),
    DecryptSealed(
        i32, // wallet handle
        String, // did
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>, IndyError>) + Send>),
    CreateKey(
        i32, // wallet handle
        String, // key json
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
        String, // did
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
        Box<Fn(Result<String, IndyError>) + Send>)
}

pub struct SignusCommandExecutor {
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,
    signus_service: Rc<SignusService>,
    ledger_service: Rc<LedgerService>,
    verify_callbacks: RefCell<HashMap<i32, Box<Fn(Result<bool, IndyError>)>>>,
    encrypt_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>)>>>,
    encrypt_sealed_callbacks: RefCell<HashMap<i32, Box<Fn(Result<Vec<u8>, IndyError>)>>>,
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
            verify_callbacks: RefCell::new(HashMap::new()),
            encrypt_callbacks: RefCell::new(HashMap::new()),
            encrypt_sealed_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: SignusCommand) {
        match command {
            SignusCommand::CreateAndStoreMyDid(wallet_handle, did_json, cb) => {
                info!(target: "signus_command_executor", "CreateAndStoreMyDid command received");
                self.create_and_store_my_did(wallet_handle, &did_json, cb);
            }
            SignusCommand::ReplaceKeysStart(wallet_handle, identity_json, did, cb) => {
                info!(target: "signus_command_executor", "ReplaceKeysStart command received");
                self.replace_keys_start(wallet_handle, &identity_json, &did, cb);
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
            SignusCommand::VerifySignature(wallet_handle, pool_handle, did, msg, signature, cb) => {
                info!(target: "signus_command_executor", "VerifySignature command received");
                self.verify_signature(wallet_handle, pool_handle, &did, &msg, &signature, cb);
            }
            SignusCommand::VerifySignatureGetNymAck(wallet_handle, msg, signature, cb_id, result) => {
                info!(target: "signus_command_executor", "VerifySignatureGetNymAck command received");
                self.verify_signature_get_nym_ack(wallet_handle, &msg, &signature, cb_id, result);
            }
            SignusCommand::Encrypt(wallet_handle, pool_handle, my_did, did, msg, cb) => {
                info!(target: "signus_command_executor", "Encrypt command received");
                self.encrypt(wallet_handle, pool_handle, &my_did, &did, &msg, cb);
            }
            SignusCommand::EncryptGetNymAck(wallet_handle, my_did, msg, cb_id, result) => {
                info!(target: "signus_command_executor", "EncryptGetNymAck command received");
                self.encrypt_get_nym_ack(wallet_handle, &my_did, &msg, cb_id, result);
            }
            SignusCommand::Decrypt(wallet_handle, my_did, did, encrypted_msg, nonce, cb) => {
                info!(target: "signus_command_executor", "Decrypt command received");
                self.decrypt(wallet_handle, &my_did, &did, &encrypted_msg, &nonce, cb);
            }
            SignusCommand::EncryptSealed(wallet_handle, pool_handle, did, msg, cb) => {
                info!(target: "signus_command_executor", "SealedEncrypt command received");
                self.encrypt_sealed(wallet_handle, pool_handle, &did, &msg, cb);
            }
            SignusCommand::EncryptSealedGetNymAck(wallet_handle, msg, cb_id, result) => {
                info!(target: "signus_command_executor", "EncryptsealedGetNymAck command received");
                self.encrypt_sealed_get_nym_ack(wallet_handle, &msg, cb_id, result);
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

        let (my_did, key) = self.signus_service.create_my_did(&my_did_info)?;

        let my_did_json = Did::to_json(&my_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Did: {}", err.description())))?;

        let key_json = Key::to_json(&key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", my_did.did), &my_did_json)?;
        self.wallet_service.set(wallet_handle, &format!("key::{}", key.verkey), &key_json)?;
        Ok((my_did.did, my_did.verkey))
    }

    fn replace_keys_start(&self,
                          wallet_handle: i32,
                          keys_info_json: &str,
                          did: &str,
                          cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._replace_keys_start(wallet_handle, keys_info_json, did));
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

        let key = self.signus_service.create_key(&key_info)?;
        let my_did = Did::new(did.to_owned(), key.verkey.clone());

        let did_json = Did::to_json(&my_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Did: {}", err.description())))?;

        let key_json = Key::to_json(&key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_did_temporary::{}", my_did.did), &did_json)?;
        self.wallet_service.set(wallet_handle, &format!("key::{}", key.verkey), &key_json)?;

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
                        did: &str,
                        msg: &[u8],
                        signature: &[u8],
                        cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        let load_did_from_ledger = move |cb: Box<Fn(Result<bool, IndyError>)>| {
            let msg = msg.to_owned();
            let signature = signature.to_owned();
            let get_nym_request = self.ledger_service.build_get_nym_request(did, did); //TODO we need pass my_did as identifier
            if get_nym_request.is_err() {
                return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Get Num Request")))));
            }
            let get_nym_request = get_nym_request.unwrap();
            let cb_id: i32 = SequenceUtils::get_next_id();

            match self.verify_callbacks.try_borrow_mut() {
                Ok(mut verify_callbacks) => {
                    check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb); //TODO pop at top level ?

                    verify_callbacks.insert(cb_id, cb);

                    CommandExecutor::instance()
                        .send(Command::Ledger(LedgerCommand::SubmitRequest(
                            pool_handle,
                            get_nym_request,
                            Box::new(move |result| {
                                CommandExecutor::instance()
                                    .send(Command::Signus(SignusCommand::VerifySignatureGetNymAck(
                                        wallet_handle,
                                        msg.clone(),
                                        signature.clone(),
                                        cb_id,
                                        result
                                    ))).unwrap();
                            })
                        ))).unwrap();
                }
                Err(err) => cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("{:?}", err)))))
            }
        };

        match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", did)) {
            Ok(did_json) => {
                let did = Did::from_json(&did_json);
                if did.is_err() {
                    return cb(Err(IndyError::SignusError(SignusError::CommonError(CommonError::InvalidStructure(format!("Invalid their did json"))))));
                }

                let did: Did = did.unwrap();
                cb(self.signus_service.verify(&did.verkey, msg, signature).map_err(|err| IndyError::SignusError(err)));
            }
            Err(WalletError::NotFound(_)) => load_did_from_ledger(cb),
            Err(err) => cb(Err(IndyError::WalletError(err)))
        }
    }

    fn verify_signature_get_nym_ack(&self,
                                    wallet_handle: i32,
                                    msg: &[u8],
                                    signature: &[u8],
                                    cb_id: i32,
                                    result: Result<String, IndyError>) {
        match self.verify_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                let cb = cbs.remove(&cb_id);

                if cb.is_none() {
                    return error!("Can't process Signus::VerifySignatureGetNymAck for handle {} - appropriate callback not found!", cb_id);
                }
                let cb = cb.unwrap();

                match result {
                    Ok(get_nym_response) =>
                        cb(self._verify_signature_get_nym_ack(wallet_handle, &get_nym_response, msg, signature)),
                    Err(err) => cb(Err(err))
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }


    fn _get_their_did_from_nym(&self, get_nym_response: &str, wallet_handle: i32) -> Result<Did, IndyError> {
        let get_nym_response: Reply<GetNymReplyResult> = Reply::from_json(&get_nym_response)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid their their_did json")))?;

        let gen_nym_result_data = GetNymResultData::from_json(&get_nym_response.result.data)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid their their_did json")))?;

        let their_did_info = TheirDidInfo::new(gen_nym_result_data.dest, gen_nym_result_data.verkey);

        let their_did = self.signus_service.create_their_did(&their_did_info)?;

        let their_did_json = their_did.to_json()
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Did: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;

        return Ok(their_did);
    }

    fn _verify_signature_get_nym_ack(&self,
                                     wallet_handle: i32,
                                     get_nym_response: &str,
                                     msg: &[u8],
                                     signature: &[u8]) -> Result<bool, IndyError> {
        let did = self._get_their_did_from_nym(get_nym_response, wallet_handle)?;
        self.signus_service.verify(&did.verkey, msg, signature)
            .map_err(map_err_trace!())
            .map_err(|err| IndyError::SignusError(err))
    }

    fn encrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: &str,
               did: &str,
               msg: &[u8],
               cb: Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>) + Send>) {
        let load_did_from_ledger = move |cb: Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>)>| {
            let msg = msg.to_owned();
            let my_did = my_did.to_string();
            let did = did.to_string();
            let cb_id: i32 = SequenceUtils::get_next_id();
            let get_nym_request = self.ledger_service.build_get_nym_request(&my_did, &did);
            if get_nym_request.is_err() {
                return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Get Num Request")))));
            }
            let get_nym_request = get_nym_request.unwrap();

            match self.encrypt_callbacks.try_borrow_mut() {
                Ok(mut encrypt_callbacks) => {
                    encrypt_callbacks.insert(cb_id, cb);

                    CommandExecutor::instance()
                        .send(Command::Ledger(LedgerCommand::SubmitRequest(
                            pool_handle,
                            get_nym_request,
                            Box::new(move |result| {
                                CommandExecutor::instance()
                                    .send(Command::Signus(SignusCommand::EncryptGetNymAck(
                                        wallet_handle,
                                        my_did.clone(),
                                        msg.clone(),
                                        cb_id,
                                        result
                                    ))).unwrap();
                            })
                        ))).unwrap();
                }
                Err(err) => cb(Err(
                    IndyError::CommonError(
                        CommonError::InvalidState(format!("{:?}", err)))))
            }
        };

        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb);

        match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", did)) {
            Ok(their_did_json) => {
                let their_did = Did::from_json(&their_did_json);
                if their_did.is_err() {
                    return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Did json")))));
                }
                let their_did: Did = their_did.unwrap();

                let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did));
                if my_did_json.is_err() {
                    return cb(Err(IndyError::WalletError(WalletError::NotFound(format!("My Did not found")))));
                }
                let my_did_json = my_did_json.unwrap();

                let my_did = Did::from_json(&my_did_json);
                if my_did.is_err() {
                    return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Did json")))));
                }
                let my_did: Did = my_did.unwrap();

                let my_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey));
                if my_key_json.is_err() {
                    return cb(Err(IndyError::WalletError(WalletError::NotFound(format!("Key not found")))));
                }
                let my_key_json = my_key_json.unwrap();

                let my_key = Key::from_json(&my_key_json);
                if my_key.is_err() {
                    return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Key json")))));
                }
                let my_key: Key = my_key.unwrap();

                cb(self.signus_service.encrypt(&my_key, &their_did.verkey, msg).map_err(|err| IndyError::SignusError(err)))
            }
            Err(WalletError::NotFound(_)) => load_did_from_ledger(cb),
            Err(err) => cb(Err(IndyError::WalletError(err)))
        }
    }

    fn encrypt_get_nym_ack(&self,
                           wallet_handle: i32,
                           my_did: &str,
                           msg: &[u8],
                           cb_id: i32,
                           result: Result<String, IndyError>) {
        match self.encrypt_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                let cb = cbs.remove(&cb_id);

                if cb.is_none() {
                    return error!("Can't process Signus::EncryptGetNymAck for handle {} - appropriate callback not found!", cb_id);
                }
                let cb = cb.unwrap();

                match result {
                    Ok(get_nym_response) =>
                        cb(self._encrypt_get_nym_ack(wallet_handle, my_did, &get_nym_response, msg)),
                    Err(err) => cb(Err(err))
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _encrypt_get_nym_ack(&self,
                            wallet_handle: i32,
                            my_did: &str,
                            get_nym_response: &str,
                            msg: &[u8]) -> Result<(Vec<u8>, Vec<u8>), IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        let my_did = Did::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState((format!("Invalid Did json"))))?;

        let my_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey))?;
        let my_key = Key::from_json(&my_key_json)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState((format!("Invalid Key json"))))?;

        let their_did = self._get_their_did_from_nym(get_nym_response, wallet_handle)?;

        self.signus_service.encrypt(&my_key, &their_did.verkey, &msg)
            .map_err(map_err_trace!())
            .map_err(|err| IndyError::SignusError(err))
    }

    fn _encrypt(&self,
                wallet_handle: i32,
                my_did: &str,
                did: &str,
                msg: &[u8]) -> Result<(Vec<u8>, Vec<u8>), IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        let my_did = Did::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        let my_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey))?;
        let my_key = Key::from_json(&my_key_json)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState((format!("Invalid Key json"))))?;

        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did))?;
        let their_did = Did::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        self.signus_service.encrypt(&my_key, &their_did.verkey, msg)
            .map_err(map_err_trace!())
            .map_err(|err| IndyError::SignusError(err))
    }

    fn decrypt(&self,
               wallet_handle: i32,
               my_did: &str,
               did: &str,
               encrypted_msg: &[u8],
               nonce: &[u8],
               cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        cb(self._decrypt(wallet_handle, my_did, did, encrypted_msg, nonce));
    }

    fn _decrypt(&self,
                wallet_handle: i32,
                my_did: &str,
                did: &str,
                encrypted_msg: &[u8],
                nonce: &[u8]) -> Result<Vec<u8>, IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        let my_did = Did::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        let my_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey))?;
        let my_key = Key::from_json(&my_key_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did))?;
        let their_did = Did::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        self.signus_service.decrypt(&my_key, &their_did.verkey, encrypted_msg, nonce)
            .map_err(|err| IndyError::SignusError(err))
    }

    fn encrypt_sealed(&self,
                      wallet_handle: i32,
                      pool_handle: i32,
                      did: &str,
                      msg: &[u8],
                      cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        let load_did_from_ledger = move |cb: Box<Fn(Result<Vec<u8>, IndyError>)>| {
            let msg = msg.to_owned();
            let cb_id: i32 = SequenceUtils::get_next_id();
            let get_nym_request = self.ledger_service.build_get_nym_request(did, did); //TODO we need pass my_did as identifier
            if get_nym_request.is_err() {
                return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Get Num Request")))));
            }
            let get_nym_request = get_nym_request.unwrap();

            match self.encrypt_sealed_callbacks.try_borrow_mut() {
                Ok(mut encrypt_callbacks) => {
                    encrypt_callbacks.insert(cb_id, cb);

                    CommandExecutor::instance()
                        .send(Command::Ledger(LedgerCommand::SubmitRequest(
                            pool_handle,
                            get_nym_request,
                            Box::new(move |result| {
                                CommandExecutor::instance()
                                    .send(Command::Signus(SignusCommand::EncryptSealedGetNymAck(
                                        wallet_handle,
                                        msg.clone(),
                                        cb_id,
                                        result
                                    ))).unwrap();
                            })
                        ))).unwrap();
                }
                Err(err) => cb(Err(
                    IndyError::CommonError(
                        CommonError::InvalidState(format!("{:?}", err)))))
            }
        };


        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb);

        match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", did)) {
            Ok(their_did_json) => {
                let their_did = Did::from_json(&their_did_json);
                if their_did.is_err() {
                    return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid their did json")))));
                }
                let their_did: Did = their_did.unwrap();
                cb(self.signus_service.encrypt_sealed(&their_did.verkey, msg)
                    .map_err(|err| IndyError::SignusError(err)));
            }
            Err(WalletError::NotFound(_)) => load_did_from_ledger(cb),
            Err(err) => cb(Err(IndyError::WalletError(err)))
        }
    }

    fn encrypt_sealed_get_nym_ack(&self,
                                  wallet_handle: i32,
                                  msg: &[u8],
                                  cb_id: i32,
                                  result: Result<String, IndyError>) {
        match self.encrypt_sealed_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                let cb = cbs.remove(&cb_id);

                if cb.is_none() {
                    return error!("Can't process Signus::EncryptSealedGetNymAck for handle {} - appropriate callback not found!", cb_id);
                }
                let cb = cb.unwrap();

                match result {
                    Ok(get_nym_response) =>
                        cb(self._encrypt_sealed_get_nym_ack(wallet_handle, &get_nym_response, msg)),
                    Err(err) => cb(Err(err))
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _encrypt_sealed_get_nym_ack(&self,
                                   wallet_handle: i32,
                                   get_nym_response: &str,
                                   msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        let their_did = self._get_their_did_from_nym(get_nym_response, wallet_handle)?;

        self.signus_service.encrypt_sealed(&their_did.verkey, &msg)
            .map_err(map_err_trace!())
            .map_err(|err| IndyError::SignusError(err))
    }

    fn _encrypt_sealed(&self,
                       wallet_handle: i32,
                       did: &str,
                       msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did))?;
        let their_did = Did::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        self.signus_service.encrypt_sealed(&their_did.verkey, msg)
            .map_err(|err| IndyError::SignusError(err))
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
        cb(self._key_for_did(wallet_handle, did));
    }

    fn _key_for_did(&self, wallet_handle: i32, did: &str) -> Result<String, IndyError> {
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
}