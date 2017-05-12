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
        String, // msg
        String, // signature
        i32, //callback id
        Result<String, LedgerError>) /* result json or error)*/,
    Encrypt(
        i32, // wallet handle
        i32, // pool handle
        String, // my_did
        String, // did
        String, // msg
        Box<Fn(Result<(String, String), SignusError>) + Send>),
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
            SignusCommand::VerifySignatureGetNymAck(wallet_handle, msg, signature, cb_id, result) => {
                info!(target: "signus_command_executor", "VerifySignatureGetNymAck command received");
                self.verify_signature_get_nym_ack(wallet_handle, &msg, &signature, cb_id, result);
            }
            SignusCommand::Encrypt(wallet_handle, pool_handle, my_did, did, msg, cb) => {
                info!(target: "signus_command_executor", "Encrypt command received");
                self.encrypt(wallet_handle, pool_handle, &my_did, &did, &msg, cb);
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
                match TheirDid::from_json(&their_did_json) {
                    Ok(their_did) => cb(self.signus_service.verify(&their_did, msg, signature)),
                    _ => cb(Err(SignusError::CryptoError(CryptoError::InvalidStructure(format!("Invalid their did json")))))
                }
            }
            Err(WalletError::NotFound(_)) => {
                let msg = msg.to_string();
                let signature = signature.to_string();
                let get_nym_request = "".to_string(); //TODO add build_nym_request function in ledger service
                let cb_id: i32 = SequenceUtils::get_next_id();

                match self.verify_callbacks.try_borrow_mut() {
                    Ok(mut verify_callbacks) => {
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
                    Err(err) => cb(Err(SignusError::CryptoError(CryptoError::BackendError(format!("{:?}", err)))))
                }
            }
            _ => cb(Err(SignusError::WalletError(WalletError::BackendError(format!("Wallet error")))))
        }
    }

    fn verify_signature_get_nym_ack(&self,
                                    wallet_handle: i32,
                                    msg: &str,
                                    signature: &str,
                                    cb_id: i32,
                                    result: Result<String, LedgerError>) {
        match self.verify_callbacks.try_borrow_mut() {
            Ok(mut cbs) => {
                match cbs.remove(&cb_id) {
                    Some(cb) => {
                        match result {
                            Ok(their_did_json) => {
                                let valid = self._verify_signature_get_nym_ack(wallet_handle, &their_did_json, msg, signature);
                                cb(valid)
                            }
                            Err(err) => cb(Err(SignusError::LedgerError(err)))
                        }
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
                                     msg: &str,
                                     signature: &str) -> Result<bool, SignusError> {
        let their_did = TheirDid::from_json(&their_did_json)?;
        self.wallet_service.set(wallet_handle, &format!("their_did::{}", their_did.did), &their_did_json)?;
        self.signus_service.verify(&their_did, &msg, &signature)
    }

    fn encrypt(&self,
               wallet_handle: i32,
               pool_handle: i32,
               my_did: &str,
               did: &str,
               msg: &str,
               cb: Box<Fn(Result<(String, String), SignusError>) + Send>) {
        cb(self._encrypt(wallet_handle, pool_handle, my_did, did, msg))
    }

    fn _encrypt(&self,
                wallet_handle: i32,
                pool_handle: i32,
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