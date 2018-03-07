extern crate serde_json;
extern crate indy_crypto;

use self::serde_json::Value;

use errors::common::CommonError;
use errors::pool::PoolError;
use errors::signus::SignusError;
use errors::indy::IndyError;

use services::pool::PoolService;
use services::signus::SignusService;
use services::signus::types::{Did, Key};
use services::wallet::WalletService;
use services::ledger::LedgerService;

use utils::json::JsonDecodable;

use super::utils::check_wallet_and_pool_handles_consistency;

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use utils::crypto::base58::Base58;

use utils::crypto::signature_serializer::serialize_signature;
use self::indy_crypto::bn::{BigNumber};


pub enum LedgerCommand {
    SignAndSubmitRequest(
        i32, // pool handle
        i32, // wallet handle
        String, // submitter did
        String, // request json
        Box<Fn(Result<String, IndyError>) + Send>),
    SubmitRequest(
        i32, // pool handle
        String, // request json
        Box<Fn(Result<String, IndyError>) + Send>),
    SubmitAck(
        i32, // cmd_id
        Result<String, PoolError>, // result json or error
    ),
    SignRequest(
        i32, // wallet handle
        String, // submitter did
        String, // request json
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetDdoRequest(
        String, // submitter did
        String, // target did
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildNymRequest(
        String, // submitter did
        String, // target did
        Option<String>, // verkey
        Option<String>, // alias
        Option<String>, // role
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildAttribRequest(
        String, // submitter did
        String, // target did
        Option<String>, // hash
        Option<String>, // raw
        Option<String>, // enc
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetAttribRequest(
        String, // submitter did
        String, // target did
        String, // data
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetNymRequest(
        String, // submitter did
        String, // target did
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildSchemaRequest(
        String, // submitter did
        String, // data
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetSchemaRequest(
        String, // submitter did
        String, // dest
        String, // data
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildClaimDefRequest(
        String, // submitter did
        i32, // xref
        String, // signature_type
        String, // data
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetClaimDefRequest(
        String, // submitter did
        i32, // xref
        String, // signature_type
        String, // origin
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildNodeRequest(
        String, // submitter did
        String, // target_did
        String, // data
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetTxnRequest(
        String, // submitter did
        i32, // data
        Box<Fn(Result<String, IndyError>) + Send>),
    // ------------------------------------------------ AUTHZ --------------------------------------

    BuildAgentAuthzRequest(
        String, // submitter
        String, // address
        Option<String>, // verkey
        Option<u32>, // authz
        Option<String>, // comm
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetAgentAuthzRequest(
        String, // submitter
        String, // address
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetAgentAuthzAccumRequest(
        String, // submitter
        String, // accumulator id
        Box<Fn(Result<String, IndyError>) + Send>)

}

pub struct LedgerCommandExecutor {
    pool_service: Rc<PoolService>,
    signus_service: Rc<SignusService>,
    wallet_service: Rc<WalletService>,
    ledger_service: Rc<LedgerService>,

    send_callbacks: RefCell<HashMap<i32, Box<Fn(Result<String, IndyError>)>>>,
}

impl LedgerCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>,
               signus_service: Rc<SignusService>,
               wallet_service: Rc<WalletService>,
               ledger_service: Rc<LedgerService>) -> LedgerCommandExecutor {
        LedgerCommandExecutor {
            pool_service: pool_service,
            signus_service: signus_service,
            wallet_service: wallet_service,
            ledger_service: ledger_service,
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
                    (result.map_err(IndyError::from));
            }
            LedgerCommand::SignRequest(wallet_handle, submitter_did, request_json, cb) => {
                info!(target: "ledger_command_executor", "SignRequest command received");
                self.sign_request(wallet_handle, &submitter_did, &request_json, cb);
            }
            LedgerCommand::BuildGetDdoRequest(submitter_did, target_did, cb) => {
                info!(target: "ledger_command_executor", "BuildGetDdoRequest command received");
                self.build_get_ddo_request(&submitter_did, &target_did, cb);
            }
            LedgerCommand::BuildNymRequest(submitter_did, target_did, verkey, alias, role, cb) => {
                info!(target: "ledger_command_executor", "BuildNymRequest command received");
                self.build_nym_request(&submitter_did, &target_did,
                                       verkey.as_ref().map(String::as_str),
                                       alias.as_ref().map(String::as_str),
                                       role.as_ref().map(String::as_str),
                                       cb);
            }
            LedgerCommand::BuildAttribRequest(submitter_did, target_did, hash, raw, enc, cb) => {
                info!(target: "ledger_command_executor", "BuildAttribRequest command received");
                self.build_attrib_request(&submitter_did, &target_did,
                                          hash.as_ref().map(String::as_str),
                                          raw.as_ref().map(String::as_str),
                                          enc.as_ref().map(String::as_str),
                                          cb);
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
            LedgerCommand::BuildGetSchemaRequest(submitter_did, dest, data, cb) => {
                info!(target: "ledger_command_executor", "BuildGetSchemaRequest command received");
                self.build_get_schema_request(&submitter_did, &dest, &data, cb);
            }
            LedgerCommand::BuildClaimDefRequest(submitter_did, xref, signature_type, data, cb) => {
                info!(target: "ledger_command_executor", "BuildClaimDefRequest command received");
                self.build_claim_def_request(&submitter_did, xref, &signature_type, &data, cb);
            }
            LedgerCommand::BuildGetClaimDefRequest(submitter_did, xref, signature_type, origin, cb) => {
                info!(target: "ledger_command_executor", "BuildGetClaimDefRequest command received");
                self.build_get_claim_def_request(&submitter_did, xref, &signature_type, &origin, cb);
            }
            LedgerCommand::BuildNodeRequest(submitter_did, target_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildNodeRequest command received");
                self.build_node_key_request(&submitter_did, &target_did, &data, cb);
            }
            LedgerCommand::BuildGetTxnRequest(submitter_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildGetTxnRequest command received");
                self.build_get_txn_request(&submitter_did, data, cb);
            }
            // ------------------------------------------------ AUTHZ -------------------------

            LedgerCommand::BuildAgentAuthzRequest(submitter, address, verkey, auth, comm, cb) => {
                info!(target: "ledger_command_executor", "BuildAgentAuthzRequest command received");
                cb(self.build_agent_authz_request(&submitter, &address,
                                                  verkey.as_ref().map(String::as_str),
                                                  auth,
                                                  comm.as_ref().map(String::as_str)));
            }
            LedgerCommand::BuildGetAgentAuthzRequest(submitter, address, cb) => {
                info!(target: "ledger_command_executor", "BuildGetAgentAuthzRequest command received");
                cb(self.build_get_agent_authz_request(&submitter, &address));
            }
            LedgerCommand::BuildGetAgentAuthzAccumRequest(submitter, accum_id, cb) => {
                info!(target: "ledger_command_executor", "BuildGetAgentAuthzAccumRequest command received");
                cb(self.build_get_agent_authz_accum_request(&submitter, &accum_id));
            }
        };
    }

    fn sign_and_submit_request(&self,
                               pool_handle: i32,
                               wallet_handle: i32,
                               submitter_did: &str,
                               request_json: &str,
                               cb: Box<Fn(Result<String, IndyError>) + Send>) {
        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service,
                                                   wallet_handle, pool_handle, cb);
        match self._sign_request(wallet_handle, submitter_did, request_json) {
            Ok(signed_request) => self.submit_request(pool_handle, signed_request.as_str(), cb),
            Err(err) => cb(Err(err))
        }
    }

    fn _sign_request(&self,
                     wallet_handle: i32,
                     submitter_did: &str,
                     request_json: &str,
    ) -> Result<String, IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", submitter_did))?;
        let my_did = Did::from_json(&my_did_json)
            .map_err(|err| CommonError::InvalidState(format!("Invalid my_did_json: {}", err.to_string())))?;

        let my_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", my_did.verkey))?;
        let my_key = Key::from_json(&my_key_json)
            .map_err(|err| CommonError::InvalidState(format!("Invalid my_key_json: {}", err.to_string())))?;

        let mut request: Value = serde_json::from_str(request_json)
            .map_err(|err|
                IndyError::SignusError(SignusError::CommonError(
                    CommonError::InvalidStructure(format!("Message is invalid json: {}", err.description())))))?;

        if !request.is_object() {
            return Err(IndyError::SignusError(SignusError::CommonError(
                CommonError::InvalidStructure(format!("Message is invalid json: {}", request)))));
        }
        let serialized_request = serialize_signature(request.clone())?;
        let signature = self.signus_service.sign(&my_key, &serialized_request.as_bytes().to_vec())?;

        request["signature"] = Value::String(Base58::encode(&signature));
        let signed_request: String = serde_json::to_string(&request)
            .map_err(|err|
                IndyError::SignusError(SignusError::CommonError(
                    CommonError::InvalidState(format!("Can't serialize message after signing: {}", err.description())))))?;

        Ok(signed_request)
    }

    fn submit_request(&self,
                      handle: i32,
                      request_json: &str,
                      cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let x: Result<i32, PoolError> = self.pool_service.send_tx(handle, request_json);
        match x {
            Ok(cmd_id) => { self.send_callbacks.borrow_mut().insert(cmd_id, cb); }
            Err(err) => { cb(Err(IndyError::PoolError(err))); }
        };
    }

    fn sign_request(&self,
                    wallet_handle: i32,
                    submitter_did: &str,
                    request_json: &str,
                    cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._sign_request(wallet_handle, submitter_did, request_json))
    }

    fn build_get_ddo_request(&self,
                             submitter_did: &str,
                             target_did: &str,
                             cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_get_ddo_request(submitter_did, target_did)
            .map_err(|err| IndyError::CommonError(err)))
    }

    fn build_nym_request(&self,
                         submitter_did: &str,
                         target_did: &str,
                         verkey: Option<&str>,
                         alias: Option<&str>,
                         role: Option<&str>,
                         cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_nym_request(submitter_did,
                                                 target_did,
                                                 verkey,
                                                 alias,
                                                 role
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_attrib_request(&self,
                            submitter_did: &str,
                            target_did: &str,
                            hash: Option<&str>,
                            raw: Option<&str>,
                            enc: Option<&str>,
                            cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_attrib_request(submitter_did,
                                                    target_did,
                                                    hash,
                                                    raw,
                                                    enc
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_get_attrib_request(&self,
                                submitter_did: &str,
                                target_did: &str,
                                data: &str,
                                cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_get_attrib_request(submitter_did,
                                                        target_did,
                                                        data
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_get_nym_request(&self,
                             submitter_did: &str,
                             target_did: &str,
                             cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_get_nym_request(submitter_did,
                                                     target_did
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_schema_request(&self,
                            submitter_did: &str,
                            data: &str,
                            cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_schema_request(submitter_did,
                                                    data
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_get_schema_request(&self,
                                submitter_did: &str,
                                dest: &str,
                                data: &str,
                                cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_get_schema_request(submitter_did,
                                                        dest,
                                                        data
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_claim_def_request(&self,
                               submitter_did: &str,
                               xref: i32,
                               signature_type: &str,
                               data: &str,
                               cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_claim_def_request(submitter_did,
                                                       xref,
                                                       signature_type,
                                                       data
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_get_claim_def_request(&self,
                                   submitter_did: &str,
                                   xref: i32,
                                   signature_type: &str,
                                   origin: &str,
                                   cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_get_claim_def_request(submitter_did,
                                                           xref,
                                                           signature_type,
                                                           origin
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_node_key_request(&self,
                              submitter_did: &str,
                              target_did: &str,
                              data: &str,
                              cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_node_request(submitter_did,
                                                  target_did,
                                                  data
        ).map_err(|err| IndyError::CommonError(err)))
    }

    fn build_get_txn_request(&self,
                             submitter_did: &str,
                             data: i32,
                             cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self.ledger_service.build_get_txn_request(submitter_did,
                                                     data
        ).map_err(|err| IndyError::CommonError(err)))
    }

    // ------------------------------------------------ AUTHZ --------------------------------------

    fn build_agent_authz_request(&self,
                                 submitter: &str,
                                 address: &str,
                                 verkey: Option<&str>,
                                 auth: Option<u32>,
                                 comm: Option<&str>) -> Result<String, IndyError> {
        info!("build_agent_authz_request >>> submitter: {:?}, address: {:?}, verkey: {:?}, auth: {:?}, comm: {:?}",
              submitter, address, verkey, auth, comm);

        self.signus_service.validate_key(submitter)?;
        // TODO: Need some validation
        let address = BigNumber::from_dec(address)?;
        let comm = comm.as_ref().map(|s| BigNumber::from_dec(s).unwrap());
        let res = self.ledger_service.build_agent_authz_request(submitter,
                                                                address,
                                                                verkey,
                                                                auth,
                                                                comm)?;

        info!("build_agent_authz_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_agent_authz_request(&self,
                                     submitter: &str,
                                     address: &str) -> Result<String, IndyError> {
        info!("build_get_agent_authz_request >>> submitter: {:?}, address: {:?}", submitter, address);

        self.signus_service.validate_did(submitter)?;
        // TODO: Need some validation
        let address = BigNumber::from_dec(address)?;
        let res = self.ledger_service.build_get_agent_authz_request(submitter,
                                                                    address)?;

        info!("build_get_agent_authz_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_agent_authz_accum_request(&self,
                                           submitter: &str,
                                           accum_id: &str) -> Result<String, IndyError> {
        info!("build_get_agent_authz_accum_request >>> submitter: {:?}, accum_id: {:?}", submitter, accum_id);

        self.signus_service.validate_did(submitter)?;
        // TODO: Need some validation
        let res = self.ledger_service.build_get_agent_authz_accum_request(submitter,
                                                                          accum_id)?;

        info!("build_get_agent_authz_accum_request <<< res: {:?}", res);

        Ok(res)
    }
}
