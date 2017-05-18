use errors::ledger::LedgerError;
use errors::pool::PoolError;
use errors::wallet::WalletError;

use services::anoncreds::AnoncredsService;
use services::pool::PoolService;
use services::signus::SignusService;
use services::signus::types::MyDid;
use services::wallet::WalletService;

use utils::json::JsonDecodable;

use super::utils::check_wallet_and_pool_handles_consistency;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum LedgerCommand {
    SignAndSubmitRequest(
        i32, // pool handle
        i32, // wallet handle
        String, // submitter did
        String, // request json
        Box<Fn(Result<String, LedgerError>) + Send>),
    SubmitRequest(
        i32, // pool handle
        String, // request json
        Box<Fn(Result<String, LedgerError>) + Send>),
    SubmitAck(
        i32, // cmd_id
        Result<String, LedgerError>, // result json or error
    ),
    BuildGetDdoRequest(
        String, // submitter did
        String, // target did
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildNymRequest(
        String, // submitter did
        String, // target did
        String, // verkey
        String, // xref
        String, // data
        String, // role
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildAttribRequest(
        String, // submitter did
        String, // target did
        String, // hash
        String, // raw
        String, // enc
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildGetAttribRequest(
        String, // submitter did
        String, // target did
        String, // data
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildGetNymRequest(
        String, // submitter did
        String, // target did
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildSchemaRequest(
        String, // submitter did
        String, // data
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildGetSchemaRequest(
        String, // submitter did
        String, // data
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildClaimDefRequest(
        String, // submitter did
        String, // xref
        String, // data
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildGetClaimDefRequest(
        String, // submitter did
        String, // xref
        Box<Fn(Result<String, LedgerError>) + Send>),
    BuildNodeRequest(
        String, // submitter did
        String, // target_did
        String, // data
        Box<Fn(Result<String, LedgerError>) + Send>)
}

pub struct LedgerCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
    pool_service: Rc<PoolService>,
    signus_service: Rc<SignusService>,
    wallet_service: Rc<WalletService>,

    send_callbacks: RefCell<HashMap<i32, Box<Fn(Result<String, LedgerError>)>>>,
}

impl LedgerCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               pool_service: Rc<PoolService>,
               signus_service: Rc<SignusService>,
               wallet_service: Rc<WalletService>) -> LedgerCommandExecutor {
        LedgerCommandExecutor {
            anoncreds_service: anoncreds_service,
            pool_service: pool_service,
            signus_service: signus_service,
            wallet_service: wallet_service,
            send_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: LedgerCommand) {
        match command {
            LedgerCommand::SignAndSubmitRequest(pool_handle, wallet_handle, submitter_did, request_json, cb) => {
                info!(target: "ledger_command_executor", "SignAndSubmitRequest command received");
                self.sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request_json, cb);
            }
            LedgerCommand::SubmitRequest(handle, request_json, cb) => {
                info!(target: "ledger_command_executor", "SubmitRequest command received");
                self.submit_request(handle, &request_json, cb);
            }
            LedgerCommand::SubmitAck(handle, result) => {
                info!(target: "ledger_command_executor", "SubmitAck command received");
                self.send_callbacks.borrow_mut().remove(&handle)
                    .expect("Expect callback to process ack command")
                    (result);
            }
            LedgerCommand::BuildGetDdoRequest(submitter_did, target_did, cb) => {
                info!(target: "ledger_command_executor", "BuildGetDdoRequest command received");
                self.build_get_ddo_request(&submitter_did, &target_did, cb);
            }
            LedgerCommand::BuildNymRequest(submitter_did, target_did, verkey, xref, data, role, cb) => {
                info!(target: "ledger_command_executor", "BuildNymRequest command received");
                self.build_nym_request(&submitter_did, &target_did, &verkey, &xref, &data, &role, cb);
            }
            LedgerCommand::BuildAttribRequest(submitter_did, target_did, hash, raw, enc, cb) => {
                info!(target: "ledger_command_executor", "BuildAttribRequest command received");
                self.build_attrib_request(&submitter_did, &target_did, &hash, &raw, &enc, cb);
            }
            LedgerCommand::BuildGetAttribRequest(submitter_did, target_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildGetAttribRequest command received");
                self.build_get_attrib_request(&submitter_did, &target_did, &data, cb);
            }
            LedgerCommand::BuildGetNymRequest(submitter_did, target_did, cb) => {
                info!(target: "ledger_command_executor", "BuildGetNymRequest command received");
                self.build_get_nym_request(&submitter_did, &target_did, cb);
            }
            LedgerCommand::BuildSchemaRequest(submitter_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildSchemaRequest command received");
                self.build_schema_request(&submitter_did, &data, cb);
            }
            LedgerCommand::BuildGetSchemaRequest(submitter_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildGetSchemaRequest command received");
                self.build_get_schema_request(&submitter_did, &data, cb);
            }
            LedgerCommand::BuildClaimDefRequest(submitter_did, xref, data, cb) => {
                info!(target: "ledger_command_executor", "BuildClaimDefRequest command received");
                self.build_issuer_key_request(&submitter_did, &xref, &data, cb);
            }
            LedgerCommand::BuildGetClaimDefRequest(submitter_did, xref, cb) => {
                info!(target: "ledger_command_executor", "BuildGetClaimDefRequest command received");
                self.build_get_issuer_key_request(&submitter_did, &xref, cb);
            }
            LedgerCommand::BuildNodeRequest(submitter_did, target_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildNodeRequest command received");
                self.build_node_key_request(&submitter_did, &target_did, &data, cb);
            }
        };
    }

    fn sign_and_submit_request(&self,
                               pool_handle: i32,
                               wallet_handle: i32,
                               submitter_did: &str,
                               request_json: &str,
                               cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        {
            // FIXME REMOVE
            // FIXME just remove with block after errors refactoring
            use errors::signus::SignusError;
            let cb = |se: Result<(), SignusError>| {
                cb(Err(LedgerError::from(se.err().unwrap())))
            };
            //FIXME REMOVE code above and extract next line from the block
            check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb
                                                   );
        }
        match self._sign_request(wallet_handle, submitter_did, request_json) {
            Ok(signed_request) => self.submit_request(pool_handle, signed_request.as_str(), cb),
            Err(err) => cb(Err(err))
        }
    }

    fn _sign_request(&self,
                     wallet_handle: i32,
                     submitter_did: &str,
                     request_json: &str,
    ) -> Result<String, LedgerError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", submitter_did))?;
        let my_did = MyDid::from_json(&my_did_json).map_err(WalletError::from)?;

        let signed_request = self.signus_service.sign(&my_did, request_json)?;
        Ok(signed_request)
    }

    fn submit_request(&self,
                      handle: i32,
                      request_json: &str,
                      cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        let x: Result<i32, PoolError> = self.pool_service.send_tx(handle, request_json);
        match x {
            Ok(cmd_id) => { self.send_callbacks.borrow_mut().insert(cmd_id, cb); }
            Err(err) => { cb(Err(LedgerError::PoolError(err))); }
        };
    }

    fn build_get_ddo_request(&self,
                             submitter_did: &str,
                             target_did: &str,
                             cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_nym_request(&self,
                         submitter_did: &str,
                         target_did: &str,
                         verkey: &str,
                         xref: &str,
                         data: &str,
                         role: &str,
                         cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_attrib_request(&self,
                            submitter_did: &str,
                            target_did: &str,
                            hash: &str,
                            raw: &str,
                            enc: &str,
                            cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_get_attrib_request(&self,
                                submitter_did: &str,
                                target_did: &str,
                                data: &str,
                                cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_get_nym_request(&self,
                             submitter_did: &str,
                             target_did: &str,
                             cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_schema_request(&self,
                            submitter_did: &str,
                            data: &str,
                            cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_get_schema_request(&self,
                                submitter_did: &str,
                                data: &str,
                                cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_issuer_key_request(&self,
                                submitter_did: &str,
                                xref: &str,
                                data: &str,
                                cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_get_issuer_key_request(&self,
                                    submitter_did: &str,
                                    xref: &str,
                                    cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn build_node_key_request(&self,
                              submitter_did: &str,
                              target_did: &str,
                              data: &str,
                              cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        cb(Ok("".to_string()));
    }
}