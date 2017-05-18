use utils::json::{JsonDecodable, JsonEncodable};
use errors::signus::SignusError;
use errors::crypto::CryptoError;
use errors::wallet::WalletError;
use errors::ledger::LedgerError;
use services::signus::types::{MyDidInfo, MyIdentityInfo, MyDid, TheirDid};

use services::anoncreds::AnoncredsService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::signus::SignusService;
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
        i32, // pool_handle,
        String, // did
        String, // signed message
        Box<Fn(Result<bool, SignusError>) + Send>),
    VerifySignatureGetNymAck(
        i32, // wallet handle
        String, // signed message
        i32, //callback id
        Result<String, LedgerError>) /* result json or error)*/,
    Encrypt(
        i32, // wallet handle
        i32, // pool handle
        String, // my_did
        String, // did
        String, // msg
        Box<Fn(Result<(String, String), SignusError>) + Send>),
    EncryptGetNymAck(
        i32, // wallet handle
        String, // my_did
        String, // msg
        i32, //cb_id
        Result<String, LedgerError> //result
    ),
    Decrypt(
        i32, // wallet handle
        String, // my_did
        String, // did
        String, // encrypted msg
        String, // nonce
        Box<Fn(Result<String, SignusError>) + Send>)
}

pub struct SignusCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,
    signus_service: Rc<SignusService>,
    verify_callbacks: RefCell<HashMap<i32, Box<Fn(Result<bool, SignusError>)>>>,
    encrypt_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(String, String), SignusError>)>>>,

}

impl SignusCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>,
               signus_service: Rc<SignusService>) -> SignusCommandExecutor {
        SignusCommandExecutor {
            anoncreds_service: anoncreds_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
            signus_service: signus_service,
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
            SignusCommand::VerifySignature(wallet_handle, pool_handle, did, signed_msg, cb) => {
                info!(target: "signus_command_executor", "VerifySignature command received");
                self.verify_signature(wallet_handle, pool_handle, &did, &signed_msg, cb);
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
                               did_json: &str,
                               cb: Box<Fn(Result<(String, String, String), SignusError>) + Send>) {
        cb(self._create_and_store_my_did(wallet_handle, did_json));
    }

    fn _create_and_store_my_did(&self, wallet_handle: i32, did_json: &str) -> Result<(String, String, String), SignusError> {
        let did_info = MyDidInfo::from_json(&did_json)?;

        let my_did = self.signus_service.create_my_did(&did_info)?;
        let my_did_json = my_did.to_json()?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", my_did.did), &my_did_json)?;

        Ok((my_did.did, my_did.ver_key, my_did.public_key))
    }

    fn replace_keys(&self,
                    wallet_handle: i32,
                    identity_json: &str,
                    did: &str,
                    cb: Box<Fn(Result<(String, String), SignusError>) + Send>) {
        cb(self._replace_keys(wallet_handle, identity_json, did));
    }

    fn _replace_keys(&self,
                     wallet_handle: i32,
                     identity_json: &str,
                     did: &str) -> Result<(String, String), SignusError> {
        let identity_info: MyIdentityInfo = MyIdentityInfo::from_json(identity_json)?;

        let did_info = MyDidInfo::new(Some(did.to_string()), identity_info.seed, identity_info.crypto_type);

        let my_did = self.signus_service.create_my_did(&did_info)?;
        let my_did_json = my_did.to_json()?;

        self.wallet_service.set(wallet_handle, &format!("my_did::{}", my_did.did), &my_did_json)?;

        Ok((my_did.ver_key, my_did.public_key))
    }

    fn store_their_did(&self,
                       wallet_handle: i32,
                       identity_json: &str,
                       cb: Box<Fn(Result<(), SignusError>) + Send>) {
        cb(self._store_their_did(wallet_handle, identity_json));
    }

    fn _store_their_did(&self,
                        wallet_handle: i32,
                        identity_json: &str) -> Result<(), SignusError> {
        let their_did = TheirDid::from_json(identity_json)?;

        if their_did.pk.is_none() && their_did.verkey.is_none() {
            return Err(SignusError::CryptoError(
                CryptoError::InvalidStructure(format!("Either public_key or verkey must be provided"))
            ))
        }

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &identity_json)?;
        Ok(())
    }

    fn sign(&self,
            wallet_handle: i32,
            did: &str,
            msg: &str,
            cb: Box<Fn(Result<String, SignusError>) + Send>) {
        cb(self._sign(wallet_handle, did, msg));
    }

    fn _sign(&self,
             wallet_handle: i32,
             did: &str,
             msg: &str) -> Result<String, SignusError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", did))?;
        let my_did = MyDid::from_json(&my_did_json)?;

        let signed_msg = self.signus_service.sign(&my_did, msg)?;
        Ok(signed_msg)
    }

    fn verify_signature(&self,
                        wallet_handle: i32,
                        pool_handle: i32,
                        did: &str,
                        signed_msg: &str,
                        cb: Box<Fn(Result<bool, SignusError>) + Send>) {
        let load_verkey_from_ledger = move |cb: Box<Fn(Result<bool, SignusError>)>| {
            let signed_msg = signed_msg.to_string();
            let get_nym_request = "".to_string(); //TODO add build_nym_request function in ledger service
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
                Err(err) => cb(Err(SignusError::CryptoError(CryptoError::BackendError(format!("{:?}", err)))))
            }
        };

        match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", did)) {
            Ok(their_did_json) => {
                let their_did = TheirDid::from_json(&their_did_json);
                if their_did.is_err() {
                    return cb(Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Invalid their did json")))))
                }

                let their_did: TheirDid = their_did.unwrap();

                match their_did.verkey {
                    Some(_) => cb(self.signus_service.verify(&their_did, signed_msg)),
                    None => load_verkey_from_ledger(cb)
                }
            }
            Err(WalletError::NotFound(_)) => load_verkey_from_ledger(cb),
            _ => cb(Err(SignusError::WalletError(WalletError::BackendError(format!("Wallet error")))))
        }
    }

    fn verify_signature_get_nym_ack(&self,
                                    wallet_handle: i32,
                                    signed_msg: &str,
                                    cb_id: i32,
                                    result: Result<String, LedgerError>) {
        match self.verify_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                let cb = cbs.remove(&cb_id);

                if cb.is_none() {
                    error!("Can't process Signus::VerifySignatureGetNymAck for handle {} - appropriate callback not found!", cb_id)
                }
                let cb = cb.unwrap();

                match result {
                    Ok(their_did_json) =>
                        cb(self._verify_signature_get_nym_ack(wallet_handle, &their_did_json, signed_msg)),
                    Err(err) => cb(Err(SignusError::LedgerError(err)))
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _verify_signature_get_nym_ack(&self,
                                     wallet_handle: i32,
                                     their_did_json: &str,
                                     signed_msg: &str) -> Result<bool, SignusError> {
        let their_did = TheirDid::from_json(&their_did_json)?;
        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;
        self.signus_service.verify(&their_did, &signed_msg)
    }

    fn encrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: &str,
               did: &str,
               msg: &str,
               cb: Box<Fn(Result<(String, String), SignusError>) + Send>) {
        let load_public_key_from_ledger = move |cb| {
            let msg = msg.to_string();
            let my_did = my_did.to_string();
            let get_nym_request = "".to_string(); //TODO add build_nym_request function in ledger service
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
                Err(err) => cb(Err(SignusError::CryptoError(CryptoError::BackendError(format!("{:?}", err)))))
            }
        };

        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb);

        match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", did)) {
            Ok(their_did_json) => {
                let their_did = TheirDid::from_json(&their_did_json);
                if their_did.is_err() {
                    return cb(Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Invalid their did json")))))
                }
                let their_did: TheirDid = their_did.unwrap();

                let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did));
                if my_did_json.is_err() {
                    return cb(Err(SignusError::WalletError(WalletError::NotFound(format!("My Did not found")))))
                }
                let my_did_json = my_did_json.unwrap();

                let my_did = MyDid::from_json(&my_did_json);
                if my_did.is_err() {
                    return cb(Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Invalid my did json")))))
                }
                let my_did: MyDid = my_did.unwrap();

                match their_did.pk {
                    Some(_) => cb(self.signus_service.encrypt(&my_did, &their_did, msg)),
                    None => load_public_key_from_ledger(cb)
                }
            }
            Err(WalletError::NotFound(_)) => load_public_key_from_ledger(cb),
            _ => cb(Err(SignusError::WalletError(WalletError::BackendError(format!("Wallet error")))))
        }
    }

    fn encrypt_get_nym_ack(&self,
                           wallet_handle: i32,
                           my_did: &str,
                           msg: &str,
                           cb_id: i32,
                           result: Result<String, LedgerError>) {
        match self.encrypt_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                let cb = cbs.remove(&cb_id);

                if cb.is_none() {
                    error!("Can't process Signus::EncryptGetNymAck for handle {} - appropriate callback not found!", cb_id)
                }
                let cb = cb.unwrap();

                match result {
                    Ok(their_did_json) =>
                        cb(self._encrypt_get_nym_ack(wallet_handle, my_did, &their_did_json, msg)),
                    Err(err) => cb(Err(SignusError::LedgerError(err)))
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _encrypt_get_nym_ack(&self,
                            wallet_handle: i32,
                            my_did: &str,
                            their_did_json: &str,
                            msg: &str) -> Result<(String, String), SignusError> {
        let my_did = MyDid::from_json(&my_did)?;
        let their_did = TheirDid::from_json(&their_did_json)?;

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;

        self.signus_service.encrypt(&my_did, &their_did, &msg)
    }

    fn _encrypt(&self,
                wallet_handle: i32,
                my_did: &str,
                did: &str,
                msg: &str) -> Result<(String, String), SignusError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        let my_did = MyDid::from_json(&my_did_json)?;

        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did))?;
        let their_did = TheirDid::from_json(&their_did_json)?;

        self.signus_service.encrypt(&my_did, &their_did, msg)
    }

    fn decrypt(&self,
               wallet_handle: i32,
               my_did: &str,
               did: &str,
               encrypted_msg: &str,
               nonce: &str,
               cb: Box<Fn(Result<String, SignusError>) + Send>) {
        cb(self._decrypt(wallet_handle, my_did, did, encrypted_msg, nonce));
    }

    fn _decrypt(&self,
                wallet_handle: i32,
                my_did: &str,
                did: &str,
                encrypted_msg: &str,
                nonce: &str) -> Result<String, SignusError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        let my_did = MyDid::from_json(&my_did_json)?;

        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did))?;
        let their_did = TheirDid::from_json(&their_did_json)?;

        self.signus_service.decrypt(&my_did, &their_did, encrypted_msg, nonce)
    }
}