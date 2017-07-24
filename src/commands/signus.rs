use utils::json::{JsonDecodable, JsonEncodable};
use errors::signus::SignusError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use errors::indy::IndyError;
use services::signus::types::{MyDidInfo, MyKyesInfo, MyDid, TheirDidInfo, TheirDid};
use services::ledger::types::{Reply, GetNymResultData, GetNymReplyResult};
use services::anoncreds::AnoncredsService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::signus::SignusService;
use services::ledger::LedgerService;

use std::error::Error;
use std::rc::Rc;
use std::str;
use std::cell::RefCell;

use commands::ledger::{LedgerCommand};
use commands::{Command, CommandExecutor};
use std::collections::HashMap;
use utils::sequence::SequenceUtils;

use super::utils::check_wallet_and_pool_handles_consistency;

pub enum SignusCommand {
    CreateAndStoreMyDid(
        i32, // wallet handle
        String, // did json
        Box<Fn(Result<(String, String, String), IndyError>) + Send>),
    ReplaceKeys(
        i32, // wallet handle
        String, // identity json
        String, // did
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    StoreTheirDid(
        i32, // wallet handle
        String, // identity json
        Box<Fn(Result<(), IndyError>) + Send>),
    Sign(
        i32, // wallet handle
        String, // did
        String, // msg
        Box<Fn(Result<String, IndyError>) + Send>),
    //TODO divide on two commands
    VerifySignature(
        i32, // wallet handle
        i32, // pool_handle,
        String, // my_did
        String, // did
        String, // signed message
        Box<Fn(Result<bool, IndyError>) + Send>),
    VerifySignatureGetNymAck(
        i32, // wallet handle
        String, // signed message
        i32, //callback id
        Result<String, IndyError>) /* result json or error)*/,
    Encrypt(
        i32, // wallet handle
        i32, // pool handle
        String, // my_did
        String, // did
        String, // msg
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    EncryptGetNymAck(
        i32, // wallet handle
        String, // my_did
        String, // msg
        i32, //cb_id
        Result<String, IndyError> //result
    ),
    Decrypt(
        i32, // wallet handle
        String, // my_did
        String, // did
        String, // encrypted msg
        String, // nonce
        Box<Fn(Result<String, IndyError>) + Send>)
}

pub struct SignusCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,
    signus_service: Rc<SignusService>,
    ledger_service: Rc<LedgerService>,
    verify_callbacks: RefCell<HashMap<i32, Box<Fn(Result<bool, IndyError>)>>>,
    encrypt_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(String, String), IndyError>)>>>,

}

impl SignusCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>,
               signus_service: Rc<SignusService>,
               ledger_service: Rc<LedgerService>) -> SignusCommandExecutor {
        SignusCommandExecutor {
            anoncreds_service: anoncreds_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
            signus_service: signus_service,
            ledger_service: ledger_service,
            verify_callbacks: RefCell::new(HashMap::new()),
            encrypt_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: SignusCommand) {
        match command {
            SignusCommand::CreateAndStoreMyDid(wallet_handle, did_json, cb) => {
                info!(target: "signus_command_executor", "CreateAndStoreMyDid command received");
                self.create_and_store_my_did(wallet_handle, &did_json, cb);
            }
            SignusCommand::ReplaceKeys(wallet_handle, identity_json, did, cb) => {
                info!(target: "signus_command_executor", "ReplaceKeys command received");
                self.replace_keys(wallet_handle, &identity_json, &did, cb);
            }
            SignusCommand::StoreTheirDid(wallet_handle, identity_json, cb) => {
                info!(target: "signus_command_executor", "StoreTheirDid command received");
                self.store_their_did(wallet_handle, &identity_json, cb);
            }
            SignusCommand::Sign(wallet_handle, did, msg, cb) => {
                info!(target: "signus_command_executor", "Sign command received");
                self.sign(wallet_handle, &did, &msg, cb);
            }
            SignusCommand::VerifySignature(wallet_handle, pool_handle, my_did, did, signed_msg, cb) => {
                info!(target: "signus_command_executor", "VerifySignature command received");
                self.verify_signature(wallet_handle, pool_handle, &my_did, &did, &signed_msg, cb);
            }
            SignusCommand::VerifySignatureGetNymAck(wallet_handle, signed_msg, cb_id, result) => {
                info!(target: "signus_command_executor", "VerifySignatureGetNymAck command received");
                self.verify_signature_get_nym_ack(wallet_handle, &signed_msg, cb_id, result);
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
        };
    }

    fn create_and_store_my_did(&self,
                               wallet_handle: i32,
                               my_did_info_json: &str,
                               cb: Box<Fn(Result<(String, String, String), IndyError>) + Send>) {
        cb(self._create_and_store_my_did(wallet_handle, my_did_info_json));
    }

    fn _create_and_store_my_did(&self, wallet_handle: i32, my_did_info_json: &str) -> Result<(String, String, String), IndyError> {
        let my_did_info = MyDidInfo::from_json(&my_did_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid MyDidInfo json: {}", err.description())))?;

        let my_did = self.signus_service.create_my_did(&my_did_info)?;

        let my_did_json = MyDid::to_json(&my_did)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize MyDid: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", my_did.did), &my_did_json)?;
        Ok((my_did.did, my_did.verkey, my_did.pk))
    }

    fn replace_keys(&self,
                    wallet_handle: i32,
                    keys_info_json: &str,
                    did: &str,
                    cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        cb(self._replace_keys(wallet_handle, keys_info_json, did));
    }

    fn _replace_keys(&self,
                     wallet_handle: i32,
                     keys_info_json: &str,
                     did: &str) -> Result<(String, String), IndyError> {
        let keys_info: MyKyesInfo = MyKyesInfo::from_json(keys_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(format!("Invalid MyKyesInfo json: {}", err.description())))?;

        let my_did_info = MyDidInfo::new(
            Some(did.to_string()),
            keys_info.seed,
            keys_info.crypto_type,
            None);

        let my_did = self.signus_service.create_my_did(&my_did_info)?;

        let my_did_json = my_did.to_json()
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize MyDid: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", my_did.did), &my_did_json)?;

        Ok((my_did.verkey, my_did.pk))
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
                    format!("Can't serialize TheirDid: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;
        Ok(())
    }

    fn sign(&self,
            wallet_handle: i32,
            did: &str,
            msg: &str,
            cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._sign(wallet_handle, did, msg));
    }

    fn _sign(&self,
             wallet_handle: i32,
             did: &str,
             msg: &str) -> Result<String, IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", did))?;
        let my_did = MyDid::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState((format!("Invalid my did json"))))?;

        let signed_msg = self.signus_service.sign(&my_did, msg)?;
        Ok(signed_msg)
    }

    fn verify_signature(&self,
                        wallet_handle: i32,
                        pool_handle: i32,
                        my_did: &str,
                        did: &str,
                        signed_msg: &str,
                        cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        let load_verkey_from_ledger = move |cb: Box<Fn(Result<bool, IndyError>)>| {
            let signed_msg = signed_msg.to_string();
            let get_nym_request = self.ledger_service.build_get_nym_request(my_did, did);
            if get_nym_request.is_err() {
                return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Get Num Request")))))
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
                                        signed_msg.clone(),
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
            Ok(their_did_json) => {
                let their_did = TheirDid::from_json(&their_did_json);
                if their_did.is_err() {
                    return cb(Err(IndyError::SignusError(SignusError::CommonError(CommonError::InvalidStructure(format!("Invalid their did json"))))))
                }

                let their_did: TheirDid = their_did.unwrap();

                match their_did.verkey {
                    Some(_) => cb(self.signus_service.verify(&their_did, signed_msg).map_err(|err| IndyError::SignusError(err))),
                    None => load_verkey_from_ledger(cb)
                }
            }
            Err(WalletError::NotFound(_)) => load_verkey_from_ledger(cb),
            Err(err) => cb(Err(IndyError::WalletError(err)))
        }
    }

    fn verify_signature_get_nym_ack(&self,
                                    wallet_handle: i32,
                                    signed_msg: &str,
                                    cb_id: i32,
                                    result: Result<String, IndyError>) {
        match self.verify_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                let cb = cbs.remove(&cb_id);

                if cb.is_none() {
                    return error!("Can't process Signus::VerifySignatureGetNymAck for handle {} - appropriate callback not found!", cb_id)
                }
                let cb = cb.unwrap();

                match result {
                    Ok(their_did_json) =>
                        cb(self._verify_signature_get_nym_ack(wallet_handle, &their_did_json, signed_msg)),
                    Err(err) => cb(Err(err))
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _verify_signature_get_nym_ack(&self,
                                     wallet_handle: i32,
                                     get_nym_response: &str,
                                     signed_msg: &str) -> Result<bool, IndyError> {
        let get_nym_response: Reply<GetNymReplyResult> = Reply::from_json(&get_nym_response)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid their did json")))?;

        let gen_nym_result_data = GetNymResultData::from_json(&get_nym_response.result.data)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid their did json")))?;

        let their_did_info = TheirDidInfo::new(gen_nym_result_data.dest, None, gen_nym_result_data.verkey, None);

        let their_did = self.signus_service.create_their_did(&their_did_info)?;

        let their_did_json = their_did.to_json()
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize TheirDid: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;
        self.signus_service.verify(&their_did, &signed_msg)
            .map_err(map_err_trace!())
            .map_err(|err| IndyError::SignusError(err))
    }

    fn encrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: &str,
               did: &str,
               msg: &str,
               cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        let load_public_key_from_ledger = move |cb: Box<Fn(Result<(String, String), IndyError>)>| {
            let msg = msg.to_string();
            let my_did = my_did.to_string();
            let get_nym_request = self.ledger_service.build_get_nym_request(&my_did, did);
            if get_nym_request.is_err() {
                return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Get Num Request")))))
            }
            let get_nym_request = get_nym_request.unwrap();
            let cb_id: i32 = SequenceUtils::get_next_id();

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
                                        msg.clone(),
                                        my_did.clone(),
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
                let their_did = TheirDid::from_json(&their_did_json);
                if their_did.is_err() {
                    return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid their did json")))))
                }
                let their_did: TheirDid = their_did.unwrap();

                let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did));
                if my_did_json.is_err() {
                    return cb(Err(IndyError::WalletError(WalletError::NotFound(format!("My Did not found")))))
                }
                let my_did_json = my_did_json.unwrap();

                let my_did = MyDid::from_json(&my_did_json);
                if my_did.is_err() {
                    return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid my did json")))))
                }
                let my_did: MyDid = my_did.unwrap();

                match their_did.pk {
                    Some(_) => cb(self.signus_service.encrypt(&my_did, &their_did, msg).map_err(|err| IndyError::SignusError(err))),
                    None => load_public_key_from_ledger(cb)
                }
            }
            Err(WalletError::NotFound(_)) => load_public_key_from_ledger(cb),
            Err(err) => cb(Err(IndyError::WalletError(err)))
        }
    }

    fn encrypt_get_nym_ack(&self,
                           wallet_handle: i32,
                           my_did: &str,
                           msg: &str,
                           cb_id: i32,
                           result: Result<String, IndyError>) {
        match self.encrypt_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                let cb = cbs.remove(&cb_id);

                if cb.is_none() {
                    return error!("Can't process Signus::EncryptGetNymAck for handle {} - appropriate callback not found!", cb_id)
                }
                let cb = cb.unwrap();

                match result {
                    Ok(their_did_json) =>
                        cb(self._encrypt_get_nym_ack(wallet_handle, my_did, &their_did_json, msg)),
                    Err(err) => cb(Err(err))
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _encrypt_get_nym_ack(&self,
                            wallet_handle: i32,
                            my_did: &str,
                            their_did_json: &str,
                            msg: &str) -> Result<(String, String), IndyError> {
        let my_did = MyDid::from_json(&my_did)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid my did json")))?;
        let their_did = TheirDid::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|_| CommonError::InvalidState(format!("Invalid their did json")))?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;

        self.signus_service.encrypt(&my_did, &their_did, &msg)
            .map_err(map_err_trace!())
            .map_err(|err| IndyError::SignusError(err))
    }

    fn _encrypt(&self,
                wallet_handle: i32,
                my_did: &str,
                did: &str,
                msg: &str) -> Result<(String, String), IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        let my_did = MyDid::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did))?;
        let their_did = TheirDid::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        self.signus_service.encrypt(&my_did, &their_did, msg)
            .map_err(map_err_trace!())
            .map_err(|err| IndyError::SignusError(err))
    }

    fn decrypt(&self,
               wallet_handle: i32,
               my_did: &str,
               did: &str,
               encrypted_msg: &str,
               nonce: &str,
               cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._decrypt(wallet_handle, my_did, did, encrypted_msg, nonce));
    }

    fn _decrypt(&self,
                wallet_handle: i32,
                my_did: &str,
                did: &str,
                encrypted_msg: &str,
                nonce: &str) -> Result<String, IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        let my_did = MyDid::from_json(&my_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did))?;
        let their_did = TheirDid::from_json(&their_did_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(err.to_string()))?;

        self.signus_service.decrypt(&my_did, &their_did, encrypted_msg, nonce)
            .map_err(|err| IndyError::SignusError(err))
    }
}