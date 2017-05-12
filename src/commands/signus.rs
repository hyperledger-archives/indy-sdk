use utils::json::{JsonDecodable, JsonEncodable};
use errors::signus::SignusError;
use errors::crypto::CryptoError;
use errors::wallet::WalletError;
use services::signus::types::{MyDidInfo, MyIdentityInfo, MyDid, TheirDidInfo};

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
        String, // msg
        String, // signature
        Box<Fn(Result<bool, SignusError>) + Send>),
    VerifySignatureGetNymAck(
        i32, // wallet handle
        String, // nym_json
        String, // signature
        i32)/*callback id*/,
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
        String, // nym_json
        String, // msg
        i32)/*callback id*/,
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
            SignusCommand::VerifySignature(wallet_handle, pool_handle, did, msg, signature, cb) => {
                info!(target: "signus_command_executor", "VerifySignature command received");
                self.verify_signature(wallet_handle, pool_handle, &did, &msg, &signature, cb);
            }
            SignusCommand::VerifySignatureGetNymAck(wallet_handle, their_did, signature, cb_id) => {
                info!(target: "signus_command_executor", "VerifySignatureGetNymAck command received");
                self.verify_signature_get_nym_ack(wallet_handle, &their_did, &signature, cb_id);
            }
            SignusCommand::Encrypt(wallet_handle, pool_handle, my_did, did, msg, cb) => {
                info!(target: "signus_command_executor", "Encrypt command received");
                self.encrypt(wallet_handle, pool_handle, &my_did, &did, &msg, cb);
            }
            SignusCommand::EncryptGetNymAck(wallet_handle, my_did, their_did, msg, cb_id) => {
                info!(target: "signus_command_executor", "Encrypt command received");
                self.encrypt_get_nym_ack(wallet_handle, &my_did, &their_did, &msg, cb_id);
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
        let their_did_info = TheirDidInfo::from_json(identity_json)?;
        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did_info.did), &identity_json)?;
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

        let signature = self.signus_service.sign(&my_did, msg)?;
        Ok(signature)
    }

    fn verify_signature(&self,
                        wallet_handle: i32,
                        pool_handle: i32,
                        did: &str,
                        msg: &str,
                        signature: &str,
                        cb: Box<Fn(Result<bool, SignusError>) + Send>) {
        match self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", did)) {
            Ok(their_did_json) => {
                match TheirDidInfo::from_json(&their_did_json) {
                    Ok(their_did) => cb(self.signus_service.verify(&their_did, signature)),
                    _ => cb(Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Invalid their did json")))))
                }
            }
            Err(WalletError::NotFound(_)) => {
                let signature = signature.to_string();
                let get_nym_request = ""; //TODO add build_nym_request function in ledger service
                let cb_id: i32 = SequenceUtils::get_next_id();

                match self.verify_callbacks.try_borrow_mut() {
                    Ok(mut verify_callbacks) => {
                        verify_callbacks.insert(cb_id, cb);

                        CommandExecutor::instance()
                            .send(Command::Ledger(LedgerCommand::SubmitRequest(
                                pool_handle,
                                get_nym_request.to_string(),
                                Box::new(move |result| {
                                    let nym_json = result.unwrap();

                                    CommandExecutor::instance()
                                        .send(Command::Signus(SignusCommand::VerifySignatureGetNymAck(
                                            wallet_handle,
                                            nym_json,
                                            signature.clone(),
                                            cb_id
                                        ))).unwrap();
                                })
                            ))).unwrap();
                    }
                    Err(err) => cb(Err(SignusError::CryptoError(CryptoError::BackendError(format!("{:?}", err)))))
                }
            }
            _ => cb(Err(SignusError::WalletError(WalletError::BackendError(format!("Wallet error")))))
        }
    }

    fn verify_signature_get_nym_ack(&self,
                                    wallet_handle: i32,
                                    their_did_json: &str,
                                    signature: &str,
                                    cb_id: i32) {
        match self.verify_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                match cbs.remove(&cb_id) {
                    Some(cb) => {
                        let valid = self._verify_signature_get_nym_ack(wallet_handle, their_did_json, signature);
                        cb(valid)
                    }
                    None =>
                        error!("Can't process Signus::VerifySignatureGetNymAck for handle {} - appropriate callback not found!", cb_id)
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _verify_signature_get_nym_ack(&self,
                                     wallet_handle: i32,
                                     their_did_json: &str,
                                     signature: &str) -> Result<bool, SignusError> {
        let their_did = TheirDidInfo::from_json(&their_did_json)?;
        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;
        self.signus_service.verify(&their_did, &signature)
    }

    fn encrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: &str,
               did: &str,
               msg: &str,
               cb: Box<Fn(Result<(String, String), SignusError>) + Send>) {
        //TODO fix it
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did)).unwrap();
        let my_did = MyDid::from_json(&my_did_json).unwrap();

        let their_did_json = self.wallet_service.get(wallet_handle, &format!("their_did::{}", did)).unwrap();
        let their_did = TheirDidInfo::from_json(&their_did_json).unwrap();

        match their_did.pk {
            Some(pk) => cb(self.signus_service.encrypt(&my_did, &pk, msg)),
            _ => {
                let msg = msg.to_string();
                let my_did_json = my_did_json.to_string();
                let get_nym_request = ""; //TODO add build_nym_request function in ledger service
                let cb_id: i32 = SequenceUtils::get_next_id();

                match self.encrypt_callbacks.try_borrow_mut() {
                    Ok(mut encrypt_callbacks) => {
                        encrypt_callbacks.insert(cb_id, cb);

                        CommandExecutor::instance()
                            .send(Command::Ledger(LedgerCommand::SubmitRequest(
                                pool_handle,
                                get_nym_request.to_string(),
                                Box::new(move |result| {
                                    let nym_json = result.unwrap();

                                    CommandExecutor::instance()
                                        .send(Command::Signus(SignusCommand::EncryptGetNymAck(
                                            wallet_handle,
                                            my_did_json.clone(),
                                            nym_json,
                                            msg.clone(),
                                            cb_id
                                        ))).unwrap();
                                })
                            ))).unwrap();
                    }
                    Err(err) => cb(Err(SignusError::CryptoError(CryptoError::BackendError(format!("{:?}", err)))))
                }
            }
        }
    }

    fn encrypt_get_nym_ack(&self,
                           wallet_handle: i32,
                           my_did_json: &str,
                           their_did_json: &str,
                           msg: &str,
                           cb_id: i32) {
        match self.encrypt_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                match cbs.remove(&cb_id) {
                    Some(cb) => {
                        let res = self._encrypt_get_nym_ack(wallet_handle, my_did_json, their_did_json, msg);
                        cb(res)
                    }
                    None =>
                        error!("Can't process Signus::EncryptGetNymAck for handle {} - appropriate callback not found!", cb_id)
                }
            }
            Err(err) => error!("{:?}", err)
        }
    }

    fn _encrypt_get_nym_ack(&self,
                            wallet_handle: i32,
                            my_did_json: &str,
                            their_did_json: &str,
                            msg: &str) -> Result<(String, String), SignusError> {
        let my_did = MyDid::from_json(&their_did_json)?;
        let their_did = TheirDidInfo::from_json(&their_did_json)?;

        if their_did.pk.is_none() {
            return Err(SignusError::CryptoError(CryptoError::BackendError(format!("Public key not found"))))
        }

        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;

        self.signus_service.encrypt(&my_did, &their_did.pk.unwrap(), &msg)
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
        let their_did = TheirDidInfo::from_json(&their_did_json)?;

        self.signus_service.decrypt(&my_did, &their_did, encrypted_msg, nonce)
    }
}