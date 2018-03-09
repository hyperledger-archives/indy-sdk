use utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::wallet::WalletError;
use errors::indy::IndyError;
use services::signus::types::{KeyInfo, MyDidInfo, TheirDidInfo, Did, Key};
use services::ledger::types::{Reply, GetNymResultData, GetNymReplyResult, GetAttribReplyResult, Endpoint, AttribData};
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
use commands::crypto::CryptoCommandExecutor;
use std::collections::HashMap;
use utils::sequence::SequenceUtils;

use super::utils::check_wallet_and_pool_handles_consistency;

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
    KeyForDid(
        i32, // pool handle
        i32, // wallet handle
        String, // did (my or their)
        Box<Fn(Result<String/*key*/, IndyError>) + Send>),
    KeyForLocalDid(
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
        i32, // pool handle
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
    ),
    // Internal commands
    GetAttribAck(
        i32, // wallet_handle
        Result<String, IndyError>, // GetAttrib Result
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

macro_rules! ensure_their_did {
    ($self_:ident, $wallet_handle:ident, $pool_handle:ident, $their_did:ident, $deferred_cmd:expr, $cb:ident) => (match $self_._wallet_get_their_did($wallet_handle, &$their_did) {
          Ok(val) => val,
          Err(IndyError::WalletError(WalletError::NotFound(_))) => {

              check_wallet_and_pool_handles_consistency!($self_.wallet_service, $self_.pool_service,
                                                         $wallet_handle, $pool_handle, $cb);

              // No their their_did present in the wallet. Deffer this command until it is fetched from ledger.
              return $self_._fetch_their_did_from_ledger($wallet_handle, $pool_handle, &$their_did, $deferred_cmd);
            }
            Err(err) => return $cb(Err(err))
        });
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
                info!("CreateAndStoreMyDid command received");
                cb(self.create_and_store_my_did(wallet_handle, &my_did_info_json));
            }
            SignusCommand::ReplaceKeysStart(wallet_handle, key_info_json, did, cb) => {
                info!("ReplaceKeysStart command received");
                cb(self.replace_keys_start(wallet_handle, &key_info_json, &did));
            }
            SignusCommand::ReplaceKeysApply(wallet_handle, did, cb) => {
                info!("ReplaceKeysApply command received");
                cb(self.replace_keys_apply(wallet_handle, &did));
            }
            SignusCommand::StoreTheirDid(wallet_handle, identity_json, cb) => {
                info!("StoreTheirDid command received");
                cb(self.store_their_did(wallet_handle, &identity_json));
            }
            SignusCommand::Sign(wallet_handle, did, msg, cb) => {
                info!("Sign command received");
                cb(self.sign(wallet_handle, &did, &msg));
            }
            SignusCommand::VerifySignature(wallet_handle, pool_handle, their_did, msg, signature, cb) => {
                info!("VerifySignature command received");
                self.verify_signature(wallet_handle, pool_handle, their_did, msg, signature, cb);
            }
            SignusCommand::Encrypt(wallet_handle, pool_handle, my_did, their_did, msg, cb) => {
                info!("Encrypt command received");
                self.encrypt(wallet_handle, pool_handle, my_did, their_did, msg, cb);
            }
            SignusCommand::Decrypt(wallet_handle, pool_handle, my_did, their_did, encrypted_msg, nonce, cb) => {
                info!("Decrypt command received");
                self.decrypt(wallet_handle, pool_handle, my_did, their_did, encrypted_msg, nonce, cb);
            }
            SignusCommand::EncryptSealed(wallet_handle, pool_handle, their_did, msg, cb) => {
                info!("SealedEncrypt command received");
                self.encrypt_sealed(wallet_handle, pool_handle, their_did, msg, cb);
            }
            SignusCommand::DecryptSealed(wallet_handle, my_did, encrypted_msg, cb) => {
                info!("DecryptSealed command received");
                cb(self.decrypt_sealed(wallet_handle, my_did, encrypted_msg));
            }
            SignusCommand::KeyForDid(pool_handle, wallet_handle, did, cb) => {
                info!("KeyForDid command received");
                self.key_for_did(pool_handle, wallet_handle, did, cb);
            }
            SignusCommand::KeyForLocalDid(wallet_handle, did, cb) => {
                info!("KeyForLocalDid command received");
                cb(self.key_for_local_did(wallet_handle, did));
            }
            SignusCommand::SetEndpointForDid(wallet_handle, did, address, transport_key, cb) => {
                info!("SetEndpointForDid command received");
                cb(self.set_endpoint_for_did(wallet_handle, did, address, transport_key));
            }
            SignusCommand::GetEndpointForDid(wallet_handle, pool_handle, did, cb) => {
                info!("GetEndpointForDid command received");
                self.get_endpoint_for_did(wallet_handle, pool_handle, did, cb);
            }
            SignusCommand::SetDidMetadata(wallet_handle, did, metadata, cb) => {
                info!("SetDidMetadata command received");
                cb(self.set_did_metadata(wallet_handle, did, metadata));
            }
            SignusCommand::GetDidMetadata(wallet_handle, did, cb) => {
                info!("GetDidMetadata command received");
                cb(self.get_did_metadata(wallet_handle, did));
            }
            SignusCommand::GetNymAck(wallet_handle, result, deferred_cmd_id) => {
                info!("GetNymAck command received");
                self.get_nym_ack(wallet_handle, result, deferred_cmd_id);
            }
            SignusCommand::GetAttribAck(wallet_handle, result, deferred_cmd_id) => {
                info!("GetAttribAck command received");
                self.get_attrib_ack(wallet_handle, result, deferred_cmd_id);
            }
        };
    }

    fn create_and_store_my_did(&self, wallet_handle: i32, my_did_info_json: &str) -> Result<(String, String), IndyError> {
        let my_did_info = MyDidInfo::from_json(&my_did_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid MyDidInfo json: {}", err.description())))?;

        let (my_did, key) = self.signus_service.create_my_did(&my_did_info)?;

        self._wallet_set_my_did(wallet_handle, &my_did)?;
        self._wallet_set_key(wallet_handle, &key)?;

        let res = (my_did.did, my_did.verkey);
        Ok(res)
    }

    fn replace_keys_start(&self,
                          wallet_handle: i32,
                          key_info_json: &str,
                          my_did: &str) -> Result<String, IndyError> {
        self.signus_service.validate_did(my_did)?;

        let key_info = SignusService::get_key_info_from_json(key_info_json.to_string())?;

        let my_did = self._wallet_get_my_did(wallet_handle, my_did)?;

        let temporary_key = self.signus_service.create_key(&key_info)?;
        let my_temporary_did = Did::new(my_did.did, temporary_key.verkey.clone());

        self._wallet_set_key(wallet_handle, &temporary_key)?;
        self._wallet_set_my_temporary_did(wallet_handle, &my_temporary_did)?;

        let res = my_temporary_did.verkey;
        Ok(res)
    }

    fn replace_keys_apply(&self,
                          wallet_handle: i32,
                          my_did: &str) -> Result<(), IndyError> {
        self.signus_service.validate_did(my_did)?;

        let my_did = self._wallet_get_my_did(wallet_handle, my_did)?;
        let my_temporary_did = self._wallet_get_my_temporary_did(wallet_handle, &my_did.did)?;

        self._wallet_set_my_did(wallet_handle, &my_temporary_did)?;

        Ok(())
    }

    fn store_their_did(&self,
                       wallet_handle: i32,
                       their_did_info_json: &str) -> Result<(), IndyError> {
        let their_did_info = TheirDidInfo::from_json(their_did_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(format!("Invalid TheirDidInfo json: {}", err.description())))?;

        let their_did = self.signus_service.create_their_did(&their_did_info)?;
        self._wallet_set_their_did(wallet_handle, &their_did)?;

        Ok(())
    }

    fn sign(&self,
            wallet_handle: i32,
            my_did: &str,
            msg: &[u8]) -> Result<Vec<u8>, IndyError> {
        self.signus_service.validate_did(my_did)?;

        let my_did = self._wallet_get_my_did(wallet_handle, my_did)?;
        let key = self._wallet_get_key(wallet_handle, &my_did.verkey)?;

        let res = self.signus_service.sign(&key, msg)?;
        Ok(res)
    }

    fn verify_signature(&self,
                        wallet_handle: i32,
                        pool_handle: i32,
                        their_did: String,
                        msg: Vec<u8>,
                        signature: Vec<u8>,
                        cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        try_cb!(self.signus_service.validate_did(&their_did), cb);

        let their_did = ensure_their_did!(self,
                                          wallet_handle,
                                          pool_handle,
                                          their_did,
                                          SignusCommand::VerifySignature(
                                              wallet_handle,
                                              pool_handle,
                                              their_did.clone(),
                                              msg,
                                              signature,
                                              cb),
                                           cb);

        let res = try_cb!(self.signus_service.verify(&their_did.verkey, &msg, &signature), cb);
        cb(Ok(res))
    }

    fn encrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: String,
               their_did: String,
               msg: Vec<u8>,
               cb: Box<Fn(Result<(Vec<u8>, Vec<u8>), IndyError>) + Send>) {
        try_cb!(self.signus_service.validate_did(&my_did), cb);
        try_cb!(self.signus_service.validate_did(&their_did), cb);

        let my_did = try_cb!(self._wallet_get_my_did(wallet_handle, &my_did), cb);
        let my_key = try_cb!(self._wallet_get_key(wallet_handle, &my_did.verkey), cb);

        let their_did = ensure_their_did!(self,
                                          wallet_handle,
                                          pool_handle,
                                          their_did,
                                          SignusCommand::Encrypt(
                                              wallet_handle,
                                              pool_handle,
                                              my_did.did.clone(),
                                              their_did.clone(),
                                              msg,
                                              cb),
                                           cb);

        let res = try_cb!(self.signus_service.encrypt(&my_key, &their_did.verkey, &msg), cb);
        cb(Ok(res))
    }

    fn decrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: String,
               their_did: String,
               encrypted_msg: Vec<u8>,
               nonce: Vec<u8>,
               cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        try_cb!(self.signus_service.validate_did(&my_did), cb);
        try_cb!(self.signus_service.validate_did(&their_did), cb);

        let my_did = try_cb!(self._wallet_get_my_did(wallet_handle, &my_did), cb);
        let my_key = try_cb!(self._wallet_get_key(wallet_handle, &my_did.verkey), cb);

        let their_did = ensure_their_did!(self,
                                          wallet_handle,
                                          pool_handle,
                                          their_did,
                                          SignusCommand::Decrypt(
                                              wallet_handle,
                                              pool_handle,
                                              my_did.did.clone(),
                                              their_did.clone(),
                                              encrypted_msg,
                                              nonce,
                                              cb),
                                           cb);

        let res = try_cb!(self.signus_service.decrypt(&my_key, &their_did.verkey, &encrypted_msg, &nonce), cb);
        cb(Ok(res))
    }

    fn encrypt_sealed(&self,
                      wallet_handle: i32,
                      pool_handle: i32,
                      their_did: String,
                      msg: Vec<u8>,
                      cb: Box<Fn(Result<Vec<u8>, IndyError>) + Send>) {
        try_cb!(self.signus_service.validate_did(&their_did), cb);

        let their_did = ensure_their_did!(self,
                                          wallet_handle,
                                          pool_handle,
                                          their_did,
                                          SignusCommand::EncryptSealed(
                                              wallet_handle,
                                              pool_handle,
                                              their_did.clone(),
                                              msg,
                                              cb),
                                           cb);

        let res = try_cb!(self.signus_service.encrypt_sealed(&their_did.verkey, &msg), cb);
        cb(Ok(res))
    }

    fn decrypt_sealed(&self,
                      wallet_handle: i32,
                      my_did: String,
                      encrypted_msg: Vec<u8>) -> Result<Vec<u8>, IndyError> {
        self.signus_service.validate_did(&my_did)?;

        let my_did = self._wallet_get_my_did(wallet_handle, &my_did)?;
        let key = self._wallet_get_key(wallet_handle, &my_did.verkey)?;

        let res = self.signus_service.decrypt_sealed(&key, &encrypted_msg)?;
        Ok(res)
    }

    fn key_for_did(&self,
                   pool_handle: i32,
                   wallet_handle: i32,
                   did: String,
                   cb: Box<Fn(Result<String, IndyError>) + Send>) {
        try_cb!(self.signus_service.validate_did(&did), cb);

        // Look to my did
        match self._wallet_get_my_did(wallet_handle, &did) {
            Ok(my_did) => return cb(Ok(my_did.verkey)),
            Err(IndyError::WalletError(WalletError::NotFound(_))) => {}
            Err(err) => return cb(Err(err))
        };

        // look to their did
        let their_did = ensure_their_did!(self,
                                          wallet_handle,
                                          pool_handle,
                                          did,
                                          SignusCommand::KeyForDid(
                                              pool_handle,
                                              wallet_handle,
                                              did.clone(),
                                              cb),
                                           cb);

        let res = their_did.verkey;
        cb(Ok(res))
    }

    fn key_for_local_did(&self,
                         wallet_handle: i32,
                         did: String) -> Result<String, IndyError> {
        self.signus_service.validate_did(&did)?;

        // Look to my did
        match self._wallet_get_my_did(wallet_handle, &did) {
            Ok(my_did) => return Ok(my_did.verkey),
            Err(IndyError::WalletError(WalletError::NotFound(_))) => {}
            Err(err) => return Err(err)
        };

        // look to their did
        let their_did = self._wallet_get_local_their_did(wallet_handle, &did)?;

        let res = their_did.verkey;
        Ok(res)
    }

    fn set_endpoint_for_did(&self,
                            wallet_handle: i32,
                            did: String,
                            address: String,
                            transport_key: String) -> Result<(), IndyError> {
        self.signus_service.validate_did(&did)?;
        self.signus_service.validate_key(&transport_key)?;

        let endpoint = Endpoint::new(address.to_string(), transport_key.to_string());

        self._wallet_set_did_endpoint(wallet_handle, &did, &endpoint)?;
        Ok(())
    }

    fn get_endpoint_for_did(&self,
                            wallet_handle: i32,
                            pool_handle: i32,
                            did: String,
                            cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        try_cb!(self.signus_service.validate_did(&did), cb);

        match self._wallet_get_did_endpoint(wallet_handle, &did) {
            Ok(endpoint) => cb(Ok((endpoint.ha, endpoint.verkey))),
            Err(IndyError::WalletError(WalletError::NotFound(_))) => {
                check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                           wallet_handle, pool_handle, cb);

                return self._fetch_attrib_from_ledger(wallet_handle,
                                                      pool_handle,
                                                      &did,
                                                      SignusCommand::GetEndpointForDid(
                                                          wallet_handle,
                                                          pool_handle,
                                                          did.clone(),
                                                          cb));
            }
            Err(err) => cb(Err(err))
        };
    }

    fn set_did_metadata(&self, wallet_handle: i32, did: String, metadata: String) -> Result<(), IndyError> {
        self.signus_service.validate_did(&did)?;
        self._wallet_set_did_metadata(wallet_handle, &did, &metadata)?;
        Ok(())
    }

    fn get_did_metadata(&self,
                        wallet_handle: i32,
                        did: String) -> Result<String, IndyError> {
        self.signus_service.validate_did(&did)?;
        let res = self._wallet_get_did_metadata(wallet_handle, &did)?;
        Ok(res)
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

    fn get_attrib_ack(&self,
                      wallet_handle: i32,
                      get_attrib_reply_result: Result<String, IndyError>,
                      deferred_cmd_id: i32) {
        let res = self._get_attrib_ack(wallet_handle, get_attrib_reply_result);
        self._execute_deferred_command(deferred_cmd_id, res.err());
    }

    fn _get_attrib_ack(&self, wallet_handle: i32, get_attrib_reply_result: Result<String, IndyError>) -> Result<(), IndyError> {
        let get_attrib_reply = get_attrib_reply_result?;

        let get_attrib_response: Reply<GetAttribReplyResult> = Reply::from_json(&get_attrib_reply)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid GetAttribReplyResult json")))?;

        let attrib_data: AttribData = AttribData::from_json(&get_attrib_response.result.data)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid GetAttribResultData json")))?;

        let endpoint = Endpoint::new(attrib_data.endpoint.ha, attrib_data.endpoint.verkey);

        self._wallet_set_did_endpoint(wallet_handle, &get_attrib_response.result.dest, &endpoint)?;

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
            SignusCommand::KeyForDid(_, _, _, cb) => {
                return cb(Err(err));
            }
            SignusCommand::GetEndpointForDid(_, _, _, cb) => {
                return cb(Err(err));
            }
            _ => {}
        }
    }

    fn _fetch_their_did_from_ledger(&self,
                                    wallet_handle: i32, pool_handle: i32,
                                    did: &str, deferred_cmd: SignusCommand) {
        // Deffer this command until their did is fetched from ledger.
        let deferred_cmd_id = self._defer_command(deferred_cmd);

        // TODO we need passing of my_did as identifier
        let get_nym_request = self.ledger_service.build_get_nym_request(did, did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    // TODO: FIXME: Remove this unwrap by sending GetNymAck with the error.
                    format!("Invalid Get Nym Request: {}", err.description()))).unwrap();

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

    fn _fetch_attrib_from_ledger(&self,
                                 wallet_handle: i32, pool_handle: i32,
                                 did: &str, deferred_cmd: SignusCommand) {
        // Deffer this command until their did is fetched from ledger.
        let deferred_cmd_id = self._defer_command(deferred_cmd);

        // TODO we need passing of my_did as identifier
        let get_attrib_request = self.ledger_service.build_get_attrib_request(did, did, "endpoint")
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    // TODO: FIXME: Remove this unwrap by sending GetAttribAck with the error.
                    format!("Invalid Get Attrib Request: {}", err.description()))).unwrap();

        CommandExecutor::instance()
            .send(Command::Ledger(LedgerCommand::SubmitRequest(
                pool_handle,
                get_attrib_request,
                Box::new(move |result| {
                    CommandExecutor::instance()
                        .send(Command::Signus(SignusCommand::GetAttribAck(
                            wallet_handle,
                            result,
                            deferred_cmd_id
                        ))).unwrap();
                })
            ))).unwrap();
    }

    fn _wallet_set_my_did(&self, wallet_handle: i32, my_did: &Did) -> Result<(), IndyError> {
        let my_did_json = Did::to_json(my_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize my Did: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", my_did.did), &my_did_json)?;
        Ok(())
    }

    fn _wallet_get_my_did(&self, wallet_handle: i32, my_did: &str) -> Result<Did, IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;

        let res = Did::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize my Did: {}", err.description())))?;
        Ok(res)
    }

    fn _wallet_set_their_did(&self, wallet_handle: i32, their_did: &Did) -> Result<(), IndyError> {
        let their_did_json = Did::to_json(their_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize their Did: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;
        Ok(())
    }

    fn _wallet_get_their_did(&self, wallet_handle: i32, their_did: &str) -> Result<Did, IndyError> {
        let their_did_json = self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", their_did))?;

        let res = Did::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize their Did: {}", err.description())))?;
        Ok(res)
    }

    fn _wallet_get_local_their_did(&self, wallet_handle: i32, their_did: &str) -> Result<Did, IndyError> {
        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", their_did))?;

        let res = Did::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize their Did: {}", err.description())))?;
        Ok(res)
    }

    fn _wallet_set_my_temporary_did(&self, wallet_handle: i32, my_temporary_did: &Did) -> Result<(), IndyError> {
        let my_temporary_did_json = Did::to_json(my_temporary_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize my temporary Did: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_temporary_did::{}", my_temporary_did.did), &my_temporary_did_json)?;
        Ok(())
    }

    fn _wallet_get_my_temporary_did(&self, wallet_handle: i32, my_temporary_did: &str) -> Result<Did, IndyError> {
        let my_temporary_did_json = self.wallet_service.get(wallet_handle, &format!("my_temporary_did::{}", my_temporary_did))?;

        let res = Did::from_json(&my_temporary_did_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize my temporary Did: {}", err.description())))?;
        Ok(res)
    }

    fn _wallet_set_key(&self, wallet_handle: i32, key: &Key) -> Result<(), IndyError> {
        let key_json = Key::to_json(&key)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Key: {}", err.description())))?;

        self.wallet_service.set(wallet_handle,
                                &CryptoCommandExecutor::_verkey_to_wallet_key(&key.verkey), &key_json)?;
        Ok(())
    }

    fn _wallet_get_key(&self, wallet_handle: i32, key: &str) -> Result<Key, IndyError> {
        let key_json = self.wallet_service.get(wallet_handle,
                                               &CryptoCommandExecutor::_verkey_to_wallet_key(key))?;

        let res = Key::from_json(&key_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize Key: {}", err.description())))?;
        Ok(res)
    }

    fn _wallet_set_did_endpoint(&self, wallet_handle: i32, did: &str, endpoint: &Endpoint) -> Result<(), IndyError> {
        let endpoint_json = Endpoint::to_json(&endpoint)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Endpoint: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("did::{}::endpoint", did), &endpoint_json)?;
        Ok(())
    }

    fn _wallet_get_did_endpoint(&self, wallet_handle: i32, did: &str) -> Result<Endpoint, IndyError> {
        let endpoint_json = self.wallet_service.get(wallet_handle, &format!("did::{}::endpoint", did))?;

        let res = Endpoint::from_json(&endpoint_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't deserialize Endpoint: {}", err.description())))?;
        Ok(res)
    }

    fn _wallet_set_did_metadata(&self, wallet_handle: i32, did: &str, metadata: &str) -> Result<(), IndyError> {
        self.wallet_service.set(wallet_handle, &format!("did::{}::metadata", did), metadata)?;
        Ok(())
    }

    fn _wallet_get_did_metadata(&self, wallet_handle: i32, did: &str) -> Result<String, IndyError> {
        let res = self.wallet_service.get(wallet_handle, &format!("did::{}::metadata", did))?;
        Ok(res)
    }
}
