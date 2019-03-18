use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use serde_json;
use serde_json::Value;

use api::ledger::{CustomFree, CustomTransactionParser};
use domain::anoncreds::credential_definition::{CredentialDefinition, CredentialDefinitionV1};
use domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::anoncreds::revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1};
use domain::anoncreds::schema::{Schema, SchemaV1};
use domain::crypto::did::Did;
use domain::crypto::key::Key;
use domain::ledger::node::NodeOperationData;
use errors::prelude::*;
use services::crypto::CryptoService;
use services::ledger::LedgerService;
use services::pool::{
    PoolService,
    parse_response_metadata
};
use services::wallet::{RecordOptions, WalletService};
use utils::crypto::base58;
use utils::crypto::signature_serializer::serialize_signature;
use api::WalletHandle;

pub enum LedgerCommand {
    SignAndSubmitRequest(
        i32, // pool handle
        WalletHandle,
        String, // submitter did
        String, // request json
        Box<Fn(IndyResult<String>) + Send>),
    SubmitRequest(
        i32, // pool handle
        String, // request json
        Box<Fn(IndyResult<String>) + Send>),
    SubmitAck(
        i32, // cmd_id
        IndyResult<String>, // result json or error
    ),
    SubmitAction(
        i32, // pool handle
        String, // request json
        Option<String>, // nodes
        Option<i32>, // timeout
        Box<Fn(IndyResult<String>) + Send>),
    SignRequest(
        WalletHandle,
        String, // submitter did
        String, // request json
        Box<Fn(IndyResult<String>) + Send>),
    MultiSignRequest(
        WalletHandle,
        String, // submitter did
        String, // request json
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetDdoRequest(
        Option<String>, // submitter did
        String, // target did
        Box<Fn(IndyResult<String>) + Send>),
    BuildNymRequest(
        String, // submitter did
        String, // target did
        Option<String>, // verkey
        Option<String>, // alias
        Option<String>, // role
        Box<Fn(IndyResult<String>) + Send>),
    BuildAttribRequest(
        String, // submitter did
        String, // target did
        Option<String>, // hash
        Option<String>, // raw
        Option<String>, // enc
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetAttribRequest(
        Option<String>, // submitter did
        String, // target did
        Option<String>, // raw
        Option<String>, // hash
        Option<String>, // enc
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetNymRequest(
        Option<String>, // submitter did
        String, // target did
        Box<Fn(IndyResult<String>) + Send>),
    BuildSchemaRequest(
        String, // submitter did
        Schema, // data
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetSchemaRequest(
        Option<String>, // submitter did
        String, // id
        Box<Fn(IndyResult<String>) + Send>),
    ParseGetSchemaResponse(
        String, // get schema response json
        Box<Fn(IndyResult<(String, String)>) + Send>),
    BuildCredDefRequest(
        String, // submitter did
        CredentialDefinition, // data
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetCredDefRequest(
        Option<String>, // submitter did
        String, // id
        Box<Fn(IndyResult<String>) + Send>),
    ParseGetCredDefResponse(
        String, // get cred definition response
        Box<Fn(IndyResult<(String, String)>) + Send>),
    BuildNodeRequest(
        String, // submitter did
        String, // target_did
        NodeOperationData, // data
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetValidatorInfoRequest(
        String, // submitter did
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetTxnRequest(
        Option<String>, // submitter did
        Option<String>, // ledger type
        i32, // data
        Box<Fn(IndyResult<String>) + Send>),
    BuildPoolConfigRequest(
        String, // submitter did
        bool, // writes
        bool, // force
        Box<Fn(IndyResult<String>) + Send>),
    BuildPoolRestartRequest(
        String, //submitter did
        String, //action
        Option<String>, //datetime
        Box<Fn(IndyResult<String>) + Send>),
    BuildPoolUpgradeRequest(
        String, // submitter did
        String, // name
        String, // version
        String, // action
        String, // sha256
        Option<u32>, // timeout
        Option<String>, // schedule
        Option<String>, // justification
        bool, // reinstall
        bool, // force
        Option<String>, // package
        Box<Fn(IndyResult<String>) + Send>),
    BuildRevocRegDefRequest(
        String, // submitter did
        RevocationRegistryDefinition, // data
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetRevocRegDefRequest(
        Option<String>, // submitter did
        String, // revocation registry definition id
        Box<Fn(IndyResult<String>) + Send>),
    ParseGetRevocRegDefResponse(
        String, // get revocation registry definition response
        Box<Fn(IndyResult<(String, String)>) + Send>),
    BuildRevocRegEntryRequest(
        String, // submitter did
        String, // revocation registry definition id
        String, // revocation registry definition type
        RevocationRegistryDelta, // value
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetRevocRegRequest(
        Option<String>, // submitter did
        String, // revocation registry definition id
        i64, // timestamp
        Box<Fn(IndyResult<String>) + Send>),
    ParseGetRevocRegResponse(
        String, // get revocation registry response
        Box<Fn(IndyResult<(String, String, u64)>) + Send>),
    BuildGetRevocRegDeltaRequest(
        Option<String>, // submitter did
        String, // revocation registry definition id
        Option<i64>, // from
        i64, // to
        Box<Fn(IndyResult<String>) + Send>),
    ParseGetRevocRegDeltaResponse(
        String, // get revocation registry delta response
        Box<Fn(IndyResult<(String, String, u64)>) + Send>),
    RegisterSPParser(
        String, // txn type
        CustomTransactionParser,
        CustomFree,
        Box<Fn(IndyResult<()>) + Send>),
    GetResponseMetadata(
        String, // response
        Box<Fn(IndyResult<String>) + Send>),
    BuildAuthRuleRequest(
        String, // submitter did
        String, // auth type
        String, // auth action
        String, // field
        Option<String>, // old value
        String, // new value
        String, // constraint
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetAuthRuleRequest(
        Option<String>, // submitter did
        Option<String>, // auth type
        Option<String>, // auth action
        Option<String>, // field
        Option<String>, // old value
        Option<String>, // new value
        Box<Fn(IndyResult<String>) + Send>),
}

pub struct LedgerCommandExecutor {
    pool_service: Rc<PoolService>,
    crypto_service: Rc<CryptoService>,
    wallet_service: Rc<WalletService>,
    ledger_service: Rc<LedgerService>,

    send_callbacks: RefCell<HashMap<i32, Box<Fn(IndyResult<String>)>>>,
}

impl LedgerCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>,
               crypto_service: Rc<CryptoService>,
               wallet_service: Rc<WalletService>,
               ledger_service: Rc<LedgerService>) -> LedgerCommandExecutor {
        LedgerCommandExecutor {
            pool_service,
            crypto_service,
            wallet_service,
            ledger_service,
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
                match self.send_callbacks.borrow_mut().remove(&handle) {
                    Some(cb) => cb(result.map_err(IndyError::from)),
                    None => {
                        error!("Can't process LedgerCommand::SubmitAck for handle {} with result {:?} - appropriate callback not found!",
                               handle, result);
                    }
                }
            }
            LedgerCommand::SubmitAction(handle, request_json, nodes, timeout, cb) => {
                info!(target: "ledger_command_executor", "SubmitRequest command received");
                self.submit_action(handle, &request_json, nodes.as_ref().map(String::as_str), timeout, cb);
            }
            LedgerCommand::RegisterSPParser(txn_type, parser, free, cb) => {
                info!(target: "ledger_command_executor", "RegisterSPParser command received");
                cb(self.register_sp_parser(&txn_type, parser, free));
            }
            LedgerCommand::SignRequest(wallet_handle, submitter_did, request_json, cb) => {
                info!(target: "ledger_command_executor", "SignRequest command received");
                cb(self.sign_request(wallet_handle, &submitter_did, &request_json));
            }
            LedgerCommand::MultiSignRequest(wallet_handle, submitter_did, request_json, cb) => {
                info!(target: "ledger_command_executor", "MultiSignRequest command received");
                cb(self.multi_sign_request(wallet_handle, &submitter_did, &request_json));
            }
            LedgerCommand::BuildGetDdoRequest(submitter_did, target_did, cb) => {
                info!(target: "ledger_command_executor", "BuildGetDdoRequest command received");
                cb(self.build_get_ddo_request(submitter_did.as_ref().map(String::as_str), &target_did));
            }
            LedgerCommand::BuildNymRequest(submitter_did, target_did, verkey, alias, role, cb) => {
                info!(target: "ledger_command_executor", "BuildNymRequest command received");
                cb(self.build_nym_request(&submitter_did, &target_did,
                                          verkey.as_ref().map(String::as_str),
                                          alias.as_ref().map(String::as_str),
                                          role.as_ref().map(String::as_str)));
            }
            LedgerCommand::BuildAttribRequest(submitter_did, target_did, hash, raw, enc, cb) => {
                info!(target: "ledger_command_executor", "BuildAttribRequest command received");
                cb(self.build_attrib_request(&submitter_did, &target_did,
                                             hash.as_ref().map(String::as_str),
                                             raw.as_ref().map(String::as_str),
                                             enc.as_ref().map(String::as_str)));
            }
            LedgerCommand::BuildGetAttribRequest(submitter_did, target_did, raw, hash, enc, cb) => {
                info!(target: "ledger_command_executor", "BuildGetAttribRequest command received");
                cb(self.build_get_attrib_request(submitter_did.as_ref().map(String::as_str), &target_did,
                                                 raw.as_ref().map(String::as_str),
                                                 hash.as_ref().map(String::as_str),
                                                 enc.as_ref().map(String::as_str)));
            }
            LedgerCommand::BuildGetNymRequest(submitter_did, target_did, cb) => {
                info!(target: "ledger_command_executor", "BuildGetNymRequest command received");
                cb(self.build_get_nym_request(submitter_did.as_ref().map(String::as_str), &target_did));
            }
            LedgerCommand::BuildSchemaRequest(submitter_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildSchemaRequest command received");
                cb(self.build_schema_request(&submitter_did, SchemaV1::from(data)));
            }
            LedgerCommand::BuildGetSchemaRequest(submitter_did, id, cb) => {
                info!(target: "ledger_command_executor", "BuildGetSchemaRequest command received");
                cb(self.build_get_schema_request(submitter_did.as_ref().map(String::as_str), &id));
            }
            LedgerCommand::ParseGetSchemaResponse(get_schema_response, cb) => {
                info!(target: "ledger_command_executor", "ParseGetSchemaResponse command received");
                cb(self.parse_get_schema_response(&get_schema_response));
            }
            LedgerCommand::BuildCredDefRequest(submitter_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildCredDefRequest command received");
                cb(self.build_cred_def_request(&submitter_did, CredentialDefinitionV1::from(data)));
            }
            LedgerCommand::BuildGetCredDefRequest(submitter_did, id, cb) => {
                info!(target: "ledger_command_executor", "BuildGetCredDefRequest command received");
                cb(self.build_get_cred_def_request(submitter_did.as_ref().map(String::as_str), &id));
            }
            LedgerCommand::ParseGetCredDefResponse(get_cred_def_response, cb) => {
                info!(target: "ledger_command_executor", "ParseGetCredDefResponse command received");
                cb(self.parse_get_cred_def_response(&get_cred_def_response));
            }
            LedgerCommand::BuildNodeRequest(submitter_did, target_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildNodeRequest command received");
                cb(self.build_node_request(&submitter_did, &target_did, data));
            }
            LedgerCommand::BuildGetValidatorInfoRequest(submitter_did, cb) => {
                info!(target: "ledger_command_executor", "BuildGetValidatorInfoRequest command received");
                cb(self.build_get_validator_info_request(&submitter_did));
            }
            LedgerCommand::BuildGetTxnRequest(submitter_did, ledger_type, seq_no, cb) => {
                info!(target: "ledger_command_executor", "BuildGetTxnRequest command received");
                cb(self.build_get_txn_request(submitter_did.as_ref().map(String::as_str), ledger_type.as_ref().map(String::as_str), seq_no));
            }
            LedgerCommand::BuildPoolConfigRequest(submitter_did, writes, force, cb) => {
                info!(target: "ledger_command_executor", "BuildPoolConfigRequest command received");
                cb(self.build_pool_config_request(&submitter_did, writes, force));
            }
            LedgerCommand::BuildPoolRestartRequest(submitter_did, action, datetime, cb) => {
                info!(target: "ledger_command_executor", "BuildPoolRestartRequest command received");
                cb(self.build_pool_restart_request(&submitter_did, &action, datetime.as_ref().map(String::as_str)));
            }
            LedgerCommand::BuildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, package, cb) => {
                info!(target: "ledger_command_executor", "BuildPoolUpgradeRequest command received");
                cb(self.build_pool_upgrade_request(&submitter_did, &name, &version, &action, &sha256, timeout,
                                                   schedule.as_ref().map(String::as_str),
                                                   justification.as_ref().map(String::as_str),
                                                   reinstall, force, package.as_ref().map(String::as_str)));
            }
            LedgerCommand::BuildRevocRegDefRequest(submitter_did, data, cb) => {
                info!(target: "ledger_command_executor", "BuildRevocRegDefRequest command received");
                cb(self.build_revoc_reg_def_request(&submitter_did, RevocationRegistryDefinitionV1::from(data)));
            }
            LedgerCommand::BuildGetRevocRegDefRequest(submitter_did, id, cb) => {
                info!(target: "ledger_command_executor", "BuildGetRevocRegDefRequest command received");
                cb(self.build_get_revoc_reg_def_request(submitter_did.as_ref().map(String::as_str), &id));
            }
            LedgerCommand::ParseGetRevocRegDefResponse(get_revoc_ref_def_response, cb) => {
                info!(target: "ledger_command_executor", "ParseGetRevocRegDefDefResponse command received");
                cb(self.parse_revoc_reg_def_response(&get_revoc_ref_def_response));
            }
            LedgerCommand::BuildRevocRegEntryRequest(submitter_did, revoc_reg_def_id, rev_def_type, value, cb) => {
                info!(target: "ledger_command_executor", "BuildRevocRegEntryRequest command received");
                cb(self.build_revoc_reg_entry_request(&submitter_did, &revoc_reg_def_id, &rev_def_type, RevocationRegistryDeltaV1::from(value)));
            }
            LedgerCommand::BuildGetRevocRegRequest(submitter_did, revoc_reg_def_id, timestamp, cb) => {
                info!(target: "ledger_command_executor", "BuildGetRevocRegRequest command received");
                cb(self.build_get_revoc_reg_request(submitter_did.as_ref().map(String::as_str), &revoc_reg_def_id, timestamp));
            }
            LedgerCommand::ParseGetRevocRegResponse(get_revoc_reg_response, cb) => {
                info!(target: "ledger_command_executor", "ParseGetRevocRegResponse command received");
                cb(self.parse_revoc_reg_response(&get_revoc_reg_response));
            }
            LedgerCommand::BuildGetRevocRegDeltaRequest(submitter_did, revoc_reg_def_id, from, to, cb) => {
                info!(target: "ledger_command_executor", "BuildGetRevocRegDeltaRequest command received");
                cb(self.build_get_revoc_reg_delta_request(submitter_did.as_ref().map(String::as_str), &revoc_reg_def_id, from, to));
            }
            LedgerCommand::ParseGetRevocRegDeltaResponse(get_revoc_reg_delta_response, cb) => {
                info!(target: "ledger_command_executor", "ParseGetRevocRegDeltaResponse command received");
                cb(self.parse_revoc_reg_delta_response(&get_revoc_reg_delta_response));
            }
            LedgerCommand::GetResponseMetadata(response, cb) => {
                info!(target: "ledger_command_executor", "GetResponseMetadata command received");
                cb(self.get_response_metadata(&response));
            }
            LedgerCommand::BuildAuthRuleRequest(submitter_did, txn_type, action, field, old_value, new_value, constraint, cb) => {
                info!(target: "ledger_command_executor", "BuildAuthRuleRequest command received");
                cb(self.build_auth_rule_request(&submitter_did, &txn_type, &action, &field, old_value.as_ref().map(String::as_str), &new_value, &constraint));
            }
            LedgerCommand::BuildGetAuthRuleRequest(submitter_did, txn_type, action, field, old_value, new_value, cb) => {
                info!(target: "ledger_command_executor", "BuildGetAuthRuleRequest command received");
                cb(self.build_get_auth_rule_request(submitter_did.as_ref().map(String::as_str),
                                                    txn_type.as_ref().map(String::as_str),
                                                    action.as_ref().map(String::as_str),
                                                    field.as_ref().map(String::as_str),
                                                    old_value.as_ref().map(String::as_str),
                                                    new_value.as_ref().map(String::as_str)));
            }
        };
    }

    fn register_sp_parser(&self, txn_type: &str,
                          parser: CustomTransactionParser, free: CustomFree) -> IndyResult<()> {
        debug!("register_sp_parser >>> txn_type: {:?}, parser: {:?}, free: {:?}",
               txn_type, parser, free);

        PoolService::register_sp_parser(txn_type, parser, free)
            .map_err(IndyError::from)
    }

    fn sign_and_submit_request(&self,
                               pool_handle: i32,
                               wallet_handle: WalletHandle,
                               submitter_did: &str,
                               request_json: &str,
                               cb: Box<Fn(IndyResult<String>) + Send>) {
        debug!("sign_and_submit_request >>> pool_handle: {:?}, wallet_handle: {:?}, submitter_did: {:?}, request_json: {:?}",
               pool_handle, wallet_handle, submitter_did, request_json);

        match self._sign_request(wallet_handle, submitter_did, request_json, SignatureType::Single) {
            Ok(signed_request) => self.submit_request(pool_handle, signed_request.as_str(), cb),
            Err(err) => cb(Err(err))
        }
    }

    fn _sign_request(&self,
                     wallet_handle: WalletHandle,
                     submitter_did: &str,
                     request_json: &str,
                     signature_type: SignatureType) -> IndyResult<String> {
        debug!("_sign_request >>> wallet_handle: {:?}, submitter_did: {:?}, request_json: {:?}", wallet_handle, submitter_did, request_json);

        let my_did: Did = self.wallet_service.get_indy_object(wallet_handle, &submitter_did, &RecordOptions::id_value())?;

        let my_key: Key = self.wallet_service.get_indy_object(wallet_handle, &my_did.verkey, &RecordOptions::id_value())?;

        let mut request: Value = serde_json::from_str(request_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Message is invalid json")?;

        if !request.is_object() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Message isn't json object"));
        }

        let serialized_request = serialize_signature(request.clone())?;
        let signature = self.crypto_service.sign(&my_key, &serialized_request.as_bytes().to_vec())?;

        match signature_type {
            SignatureType::Single => { request["signature"] = Value::String(base58::encode(&signature)); }
            SignatureType::Multi => {
                request.as_object_mut()
                    .map(|request| {
                        if !request.contains_key("signatures") {
                            request.insert("signatures".to_string(), Value::Object(serde_json::Map::new()));
                        }
                        request["signatures"].as_object_mut().unwrap().insert(submitter_did.to_string(), Value::String(base58::encode(&signature)));
                    });
            }
        }

        let res: String = serde_json::to_string(&request)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize message after signing")?;

        debug!("_sign_request <<< res: {:?}", res);

        Ok(res)
    }

    fn submit_request(&self,
                      handle: i32,
                      request_json: &str,
                      cb: Box<Fn(IndyResult<String>) + Send>) {
        debug!("submit_request >>> handle: {:?}, request_json: {:?}", handle, request_json);

        let x: IndyResult<i32> = self.pool_service.send_tx(handle, request_json);
        match x {
            Ok(cmd_id) => { self.send_callbacks.borrow_mut().insert(cmd_id, cb); }
            Err(err) => { cb(Err(err)); }
        };
    }

    fn submit_action(&self,
                     handle: i32,
                     request_json: &str,
                     nodes: Option<&str>,
                     timeout: Option<i32>,
                     cb: Box<Fn(IndyResult<String>) + Send>) {
        debug!("submit_action >>> handle: {:?}, request_json: {:?}, nodes: {:?}, timeout: {:?}", handle, request_json, nodes, timeout);

        if let Err(err) = self.ledger_service.validate_action(request_json) {
            return cb(Err(err));
        }

        let x: IndyResult<i32> = self.pool_service.send_action(handle, request_json, nodes, timeout);
        match x {
            Ok(cmd_id) => { self.send_callbacks.borrow_mut().insert(cmd_id, cb); }
            Err(err) => { cb(Err(err)); }
        };
    }

    fn sign_request(&self,
                    wallet_handle: WalletHandle,
                    submitter_did: &str,
                    request_json: &str) -> IndyResult<String> {
        debug!("sign_request >>> wallet_handle: {:?}, submitter_did: {:?}, request_json: {:?}", wallet_handle, submitter_did, request_json);

        let res = self._sign_request(wallet_handle, submitter_did, request_json, SignatureType::Single)?;

        debug!("sign_request <<< res: {:?}", res);

        Ok(res)
    }

    fn multi_sign_request(&self,
                          wallet_handle: WalletHandle,
                          submitter_did: &str,
                          request_json: &str) -> IndyResult<String> {
        debug!("multi_sign_request >>> wallet_handle: {:?}, submitter_did: {:?}, request_json: {:?}", wallet_handle, submitter_did, request_json);

        let res = self._sign_request(wallet_handle, submitter_did, request_json, SignatureType::Multi)?;

        debug!("multi_sign_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_ddo_request(&self,
                             submitter_did: Option<&str>,
                             target_did: &str) -> IndyResult<String> {
        debug!("build_get_ddo_request >>> submitter_did: {:?}, target_did: {:?}", submitter_did, target_did);

        let res = self.ledger_service.build_get_ddo_request(submitter_did, target_did)?;

        debug!("build_get_ddo_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_nym_request(&self,
                         submitter_did: &str,
                         target_did: &str,
                         verkey: Option<&str>,
                         alias: Option<&str>,
                         role: Option<&str>) -> IndyResult<String> {
        debug!("build_nym_request >>> submitter_did: {:?}, target_did: {:?}, verkey: {:?}, alias: {:?}, role: {:?}",
               submitter_did, target_did, verkey, alias, role);

        self.crypto_service.validate_did(submitter_did)?;
        self.crypto_service.validate_did(target_did)?;
        if let Some(vk) = verkey {
            self.crypto_service.validate_key(vk)?;
        }

        let res = self.ledger_service.build_nym_request(submitter_did,
                                                        target_did,
                                                        verkey,
                                                        alias,
                                                        role)?;

        debug!("build_nym_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_attrib_request(&self,
                            submitter_did: &str,
                            target_did: &str,
                            hash: Option<&str>,
                            raw: Option<&str>,
                            enc: Option<&str>) -> IndyResult<String> {
        debug!("build_attrib_request >>> submitter_did: {:?}, target_did: {:?}, hash: {:?}, raw: {:?}, enc: {:?}",
               submitter_did, target_did, hash, raw, enc);

        self.crypto_service.validate_did(submitter_did)?;
        self.crypto_service.validate_did(target_did)?;

        let res = self.ledger_service.build_attrib_request(submitter_did,
                                                           target_did,
                                                           hash,
                                                           raw,
                                                           enc)?;

        debug!("build_attrib_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_attrib_request(&self,
                                submitter_did: Option<&str>,
                                target_did: &str,
                                raw: Option<&str>,
                                hash: Option<&str>,
                                enc: Option<&str>) -> IndyResult<String> {
        debug!("build_get_attrib_request >>> submitter_did: {:?}, target_did: {:?}, raw: {:?}, hash: {:?}, enc: {:?}",
               submitter_did, target_did, raw, hash, enc);

        self.validate_opt_did(submitter_did)?;
        self.crypto_service.validate_did(target_did)?;

        let res = self.ledger_service.build_get_attrib_request(submitter_did,
                                                               target_did,
                                                               raw,
                                                               hash,
                                                               enc)?;

        debug!("build_get_attrib_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_nym_request(&self,
                             submitter_did: Option<&str>,
                             target_did: &str) -> IndyResult<String> {
        debug!("build_get_nym_request >>> submitter_did: {:?}, target_did: {:?}", submitter_did, target_did);

        self.validate_opt_did(submitter_did)?;
        self.crypto_service.validate_did(target_did)?;

        let res = self.ledger_service.build_get_nym_request(submitter_did,
                                                            target_did)?;

        debug!("build_get_attrib_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_schema_request(&self,
                            submitter_did: &str,
                            schema: SchemaV1) -> IndyResult<String> {
        debug!("build_schema_request >>> submitter_did: {:?}, schema: {:?}", submitter_did, schema);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_schema_request(submitter_did, schema)?;

        debug!("build_schema_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_schema_request(&self,
                                submitter_did: Option<&str>,
                                id: &str) -> IndyResult<String> {
        debug!("build_get_schema_request >>> submitter_did: {:?}, id: {:?}", submitter_did, id);

        self.validate_opt_did(submitter_did)?;

        let res = self.ledger_service.build_get_schema_request(submitter_did, id)?;

        debug!("build_get_schema_request <<< res: {:?}", res);

        Ok(res)
    }

    fn parse_get_schema_response(&self,
                                 get_schema_response: &str) -> IndyResult<(String, String)> {
        debug!("parse_get_schema_response >>> get_schema_response: {:?}", get_schema_response);

        let res = self.ledger_service.parse_get_schema_response(get_schema_response)?;

        debug!("parse_get_schema_response <<< res: {:?}", res);

        Ok(res)
    }

    fn build_cred_def_request(&self,
                              submitter_did: &str,
                              cred_def: CredentialDefinitionV1) -> IndyResult<String> {
        debug!("build_cred_def_request >>> submitter_did: {:?}, cred_def: {:?}",
               submitter_did, cred_def);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_cred_def_request(submitter_did, cred_def)?;

        debug!("build_cred_def_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_cred_def_request(&self,
                                  submitter_did: Option<&str>,
                                  id: &str) -> IndyResult<String> {
        debug!("build_get_cred_def_request >>> submitter_did: {:?}, id: {:?}", submitter_did, id);

        self.validate_opt_did(submitter_did)?;

        let res = self.ledger_service.build_get_cred_def_request(submitter_did, id)?;

        debug!("build_get_cred_def_request <<< res: {:?}", res);

        Ok(res)
    }

    fn parse_get_cred_def_response(&self,
                                   get_cred_def_response: &str) -> IndyResult<(String, String)> {
        debug!("parse_get_cred_def_response >>> get_cred_def_response: {:?}", get_cred_def_response);

        let res = self.ledger_service.parse_get_cred_def_response(get_cred_def_response)?;

        debug!("parse_get_cred_def_response <<< res: {:?}", res);

        Ok(res)
    }

    fn build_node_request(&self,
                          submitter_did: &str,
                          target_did: &str,
                          data: NodeOperationData) -> IndyResult<String> {
        debug!("build_node_request >>> submitter_did: {:?}, target_did: {:?}, data: {:?}",
               submitter_did, target_did, data);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_node_request(submitter_did, target_did, data)?;

        debug!("build_node_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_validator_info_request(&self,
                                        submitter_did: &str) -> IndyResult<String> {
        info!("build_get_validator_info_request >>> submitter_did: {:?}", submitter_did);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_get_validator_info_request(submitter_did)?;

        info!("build_get_validator_info_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_txn_request(&self,
                             submitter_did: Option<&str>,
                             ledger_type: Option<&str>,
                             seq_no: i32) -> IndyResult<String> {
        debug!("build_get_txn_request >>> submitter_did: {:?}, ledger_type: {:?}, seq_no: {:?}",
               submitter_did, ledger_type, seq_no);

        self.validate_opt_did(submitter_did)?;

        let res = self.ledger_service.build_get_txn_request(submitter_did, ledger_type, seq_no)?;

        debug!("build_get_txn_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_pool_config_request(&self,
                                 submitter_did: &str,
                                 writes: bool,
                                 force: bool) -> IndyResult<String> {
        debug!("build_pool_config_request >>> submitter_did: {:?}, writes: {:?}, force: {:?}",
               submitter_did, writes, force);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_pool_config(submitter_did, writes, force)?;

        debug!("build_pool_config_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn build_pool_restart_request(&self, submitter_did: &str, action: &str,
                                  datetime: Option<&str>) -> IndyResult<String> {
        debug!("build_pool_restart_request >>> submitter_did: {:?}, action: {:?}, datetime: {:?}", submitter_did, action, datetime);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_pool_restart(submitter_did, action, datetime)?;

        debug!("build_pool_config_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn build_pool_upgrade_request(&self,
                                  submitter_did: &str,
                                  name: &str,
                                  version: &str,
                                  action: &str,
                                  sha256: &str,
                                  timeout: Option<u32>,
                                  schedule: Option<&str>,
                                  justification: Option<&str>,
                                  reinstall: bool,
                                  force: bool,
                                  package: Option<&str>) -> IndyResult<String> {
        debug!("build_pool_upgrade_request >>> submitter_did: {:?}, name: {:?}, version: {:?}, action: {:?}, sha256: {:?},\
         timeout: {:?}, schedule: {:?}, justification: {:?}, reinstall: {:?}, force: {:?}, package: {:?}",
               submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, package);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_pool_upgrade(submitter_did, name, version, action, sha256,
                                                         timeout, schedule, justification, reinstall, force, package)?;

        debug!("build_pool_upgrade_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn build_revoc_reg_def_request(&self,
                                   submitter_did: &str,
                                   data: RevocationRegistryDefinitionV1) -> IndyResult<String> {
        debug!("build_revoc_reg_def_request >>> submitter_did: {:?}, data: {:?}", submitter_did, data);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_revoc_reg_def_request(submitter_did, data)?;

        debug!("build_revoc_reg_def_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_revoc_reg_def_request(&self,
                                       submitter_did: Option<&str>,
                                       id: &str) -> IndyResult<String> {
        debug!("build_get_revoc_reg_def_request >>> submitter_did: {:?}, id: {:?}", submitter_did, id);

        self.validate_opt_did(submitter_did)?;

        let res = self.ledger_service.build_get_revoc_reg_def_request(submitter_did, id)?;

        debug!("build_get_revoc_reg_def_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn parse_revoc_reg_def_response(&self,
                                    get_revoc_reg_def_response: &str) -> IndyResult<(String, String)> {
        debug!("parse_revoc_reg_def_response >>> get_revoc_reg_def_response: {:?}", get_revoc_reg_def_response);

        let res = self.ledger_service.parse_get_revoc_reg_def_response(get_revoc_reg_def_response)?;

        debug!("parse_revoc_reg_def_response <<< res: {:?}", res);

        Ok(res)
    }

    fn build_revoc_reg_entry_request(&self,
                                     submitter_did: &str,
                                     revoc_reg_def_id: &str,
                                     revoc_def_type: &str,
                                     value: RevocationRegistryDeltaV1) -> IndyResult<String> {
        debug!("build_revoc_reg_entry_request >>> submitter_did: {:?}, revoc_reg_def_id: {:?}, revoc_def_type: {:?}, value: {:?}",
               submitter_did, revoc_reg_def_id, revoc_def_type, value);

        self.crypto_service.validate_did(submitter_did)?;

        let res = self.ledger_service.build_revoc_reg_entry_request(submitter_did, revoc_reg_def_id, revoc_def_type, value)?;

        debug!("build_revoc_reg_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_revoc_reg_request(&self,
                                   submitter_did: Option<&str>,
                                   revoc_reg_def_id: &str,
                                   timestamp: i64) -> IndyResult<String> {
        debug!("build_get_revoc_reg_request >>> submitter_did: {:?}, revoc_reg_def_id: {:?}, timestamp: {:?}", submitter_did, revoc_reg_def_id, timestamp);

        self.validate_opt_did(submitter_did)?;

        let res = self.ledger_service.build_get_revoc_reg_request(submitter_did, revoc_reg_def_id, timestamp)?;

        debug!("build_get_revoc_reg_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn parse_revoc_reg_response(&self,
                                get_revoc_reg_response: &str) -> IndyResult<(String, String, u64)> {
        debug!("parse_revoc_reg_response >>> get_revoc_reg_response: {:?}", get_revoc_reg_response);

        let res = self.ledger_service.parse_get_revoc_reg_response(get_revoc_reg_response)?;

        debug!("parse_revoc_reg_response <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_revoc_reg_delta_request(&self,
                                         submitter_did: Option<&str>,
                                         revoc_reg_def_id: &str,
                                         from: Option<i64>,
                                         to: i64) -> IndyResult<String> {
        debug!("build_get_revoc_reg_delta_request >>> submitter_did: {:?}, revoc_reg_def_id: {:?}, from: {:?}, to: {:?}", submitter_did, revoc_reg_def_id, from, to);

        self.validate_opt_did(submitter_did)?;

        let res = self.ledger_service.build_get_revoc_reg_delta_request(submitter_did, revoc_reg_def_id, from, to)?;

        debug!("build_get_revoc_reg_delta_request  <<< res: {:?}", res);

        Ok(res)
    }

    fn parse_revoc_reg_delta_response(&self,
                                      get_revoc_reg_delta_response: &str) -> IndyResult<(String, String, u64)> {
        debug!("parse_revoc_reg_delta_response >>> get_revoc_reg_delta_response: {:?}", get_revoc_reg_delta_response);

        let res = self.ledger_service.parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response)?;

        debug!("parse_revoc_reg_delta_response <<< res: {:?}", res);

        Ok(res)
    }

    fn get_response_metadata(&self,
                             response: &str) -> IndyResult<String> {
        debug!("get_response_metadata >>> response: {:?}", response);

        let metadata = parse_response_metadata(response)?;

        let res = serde_json::to_string(&metadata)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize ResponseMetadata")?;

        debug!("get_response_metadata <<< res: {:?}", res);

        Ok(res)
    }

    fn build_auth_rule_request(&self,
                               submitter_did: &str,
                               txn_type: &str,
                               action: &str,
                               field: &str,
                               old_value: Option<&str>,
                               new_value: &str,
                               constraint: &str) -> IndyResult<String> {
        debug!("build_auth_rule_request >>> submitter_did: {:?}, txn_type: {:?}, action: {:?}, field: {:?}, \
            old_value: {:?}, new_value: {:?}, constraint: {:?}", submitter_did, txn_type, action, field, old_value, new_value, constraint);

        self.validate_opt_did(Some(submitter_did))?;

        let res = self.ledger_service.build_auth_rule_request(submitter_did, txn_type, action, field, old_value, new_value, constraint)?;

        debug!("build_auth_rule_request <<< res: {:?}", res);

        Ok(res)
    }

    fn build_get_auth_rule_request(&self,
                                   submitter_did: Option<&str>,
                                   txn_type: Option<&str>,
                                   action: Option<&str>,
                                   field: Option<&str>,
                                   old_value: Option<&str>,
                                   new_value: Option<&str>) -> IndyResult<String> {
        debug!("build_get_auth_rule_request >>> submitter_did: {:?}, auth_type: {:?}, auth_action: {:?}, field: {:?}, \
            old_value: {:?}, new_value: {:?}", submitter_did, txn_type, action, field, old_value, new_value);

        self.validate_opt_did(submitter_did)?;

        let res = self.ledger_service.build_get_auth_rule_request(submitter_did, txn_type, action, field, old_value, new_value)?;

        debug!("build_get_auth_rule_request <<< res: {:?}", res);

        Ok(res)
    }

    fn validate_opt_did(&self, did: Option<&str>) -> IndyResult<()> {
        match did {
            Some(did) => Ok(self.crypto_service.validate_did(did)?),
            None => Ok(())
        }
    }
}

enum SignatureType {
    Single,
    Multi
}
