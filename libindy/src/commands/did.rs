use errors::common::CommonError;
use errors::did::DidError;
use errors::wallet::WalletError;
use errors::indy::IndyError;
use domain::crypto::key::KeyInfo;
use domain::crypto::did::{MyDidInfo, Did, TheirDidInfo, TheirDid, TemporaryDid, DidWithMeta};
use domain::ledger::response::Reply;
use domain::ledger::nym::{GetNymReplyResult, GetNymResultDataV0};
use domain::ledger::attrib::{GetAttrReplyResult, AttribData, Endpoint};
use services::wallet::{WalletService, RecordOptions, SearchOptions};
use services::crypto::CryptoService;
use services::ledger::LedgerService;

use serde_json;
use std::error::Error;
use std::rc::Rc;
use std::str;
use std::cell::RefCell;

use commands::ledger::LedgerCommand;
use commands::{Command, CommandExecutor};
use std::collections::HashMap;
use utils::sequence::SequenceUtils;
use utils::crypto::base58;

pub enum DidCommand {
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
    GetMyDidWithMeta(
        i32, // wallet handle
        String, // my did
        Box<Fn(Result<String, IndyError>) + Send>),
    ListMyDidsWithMeta(
        i32, // wallet handle
        Box<Fn(Result<String, IndyError>) + Send>),
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
        Box<Fn(Result<(String, Option<String>), IndyError>) + Send>),
    SetDidMetadata(
        i32, // wallet handle
        String, // did
        String, // metadata
        Box<Fn(Result<(), IndyError>) + Send>),
    GetDidMetadata(
        i32, // wallet handle
        String, // did
        Box<Fn(Result<String, IndyError>) + Send>),
    AbbreviateVerkey(
        String, // did
        String, // verkey
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

macro_rules! ensure_their_did {
    ($self_:ident, $wallet_handle:ident, $pool_handle:ident, $their_did:ident, $deferred_cmd:expr, $cb:ident) => (match $self_._wallet_get_their_did($wallet_handle, &$their_did) {
          Ok(val) => val,
          Err(WalletError::ItemNotFound) => {
              // No their their_did present in the wallet. Defer this command until it is fetched from ledger.
              return $self_._fetch_their_did_from_ledger($wallet_handle, $pool_handle, &$their_did, $deferred_cmd);
            }
            Err(err) => return $cb(Err(IndyError::from(err)))
        });
}

pub struct DidCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    ledger_service: Rc<LedgerService>,
    deferred_commands: RefCell<HashMap<i32, DidCommand>>,
}

impl DidCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
               crypto_service: Rc<CryptoService>,
               ledger_service: Rc<LedgerService>) -> DidCommandExecutor {
        DidCommandExecutor {
            wallet_service,
            crypto_service,
            ledger_service,
            deferred_commands: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: DidCommand) {
        match command {
            DidCommand::CreateAndStoreMyDid(wallet_handle, my_did_info_json, cb) => {
                info!("CreateAndStoreMyDid command received");
                cb(self.create_and_store_my_did(wallet_handle, &my_did_info_json));
            }
            DidCommand::ReplaceKeysStart(wallet_handle, key_info_json, did, cb) => {
                info!("ReplaceKeysStart command received");
                cb(self.replace_keys_start(wallet_handle, &key_info_json, &did));
            }
            DidCommand::ReplaceKeysApply(wallet_handle, did, cb) => {
                info!("ReplaceKeysApply command received");
                cb(self.replace_keys_apply(wallet_handle, &did));
            }
            DidCommand::StoreTheirDid(wallet_handle, identity_json, cb) => {
                info!("StoreTheirDid command received");
                cb(self.store_their_did(wallet_handle, &identity_json));
            }
            DidCommand::GetMyDidWithMeta(wallet_handle, my_did, cb) => {
                info!("GetMyDidWithMeta command received");
                cb(self.get_my_did_with_meta(wallet_handle, my_did))
            }
            DidCommand::ListMyDidsWithMeta(wallet_handle, cb) => {
                info!("ListMyDidsWithMeta command received");
                cb(self.list_my_dids_with_meta(wallet_handle));
            }
            DidCommand::KeyForDid(pool_handle, wallet_handle, did, cb) => {
                info!("KeyForDid command received");
                self.key_for_did(pool_handle, wallet_handle, did, cb);
            }
            DidCommand::KeyForLocalDid(wallet_handle, did, cb) => {
                info!("KeyForLocalDid command received");
                cb(self.key_for_local_did(wallet_handle, did));
            }
            DidCommand::SetEndpointForDid(wallet_handle, did, address, transport_key, cb) => {
                info!("SetEndpointForDid command received");
                cb(self.set_endpoint_for_did(wallet_handle, did, address, transport_key));
            }
            DidCommand::GetEndpointForDid(wallet_handle, pool_handle, did, cb) => {
                info!("GetEndpointForDid command received");
                self.get_endpoint_for_did(wallet_handle, pool_handle, did, cb);
            }
            DidCommand::SetDidMetadata(wallet_handle, did, metadata, cb) => {
                info!("SetDidMetadata command received");
                cb(self.set_did_metadata(wallet_handle, did, metadata));
            }
            DidCommand::GetDidMetadata(wallet_handle, did, cb) => {
                info!("GetDidMetadata command received");
                cb(self.get_did_metadata(wallet_handle, did));
            }
            DidCommand::AbbreviateVerkey(did, verkey, cb) => {
                info!("AbbreviateVerkey command received");
                cb(self.abbreviate_verkey(did, verkey));
            }
            DidCommand::GetNymAck(wallet_handle, result, deferred_cmd_id) => {
                info!("GetNymAck command received");
                self.get_nym_ack(wallet_handle, result, deferred_cmd_id);
            }
            DidCommand::GetAttribAck(wallet_handle, result, deferred_cmd_id) => {
                info!("GetAttribAck command received");
                self.get_attrib_ack(wallet_handle, result, deferred_cmd_id);
            }
        };
    }

    fn create_and_store_my_did(&self,
                               wallet_handle: i32,
                               my_did_info_json: &str) -> Result<(String, String), IndyError> {
        debug!("create_and_store_my_did >>> wallet_handle: {:?}, my_did_info_json: {:?}", wallet_handle, my_did_info_json);

        let my_did_info: MyDidInfo = serde_json::from_str(&my_did_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid MyDidInfo json: {:?}", err)))?;

        let (did, key) = self.crypto_service.create_my_did(&my_did_info)?;

        if self.wallet_service.record_exists::<Did>(wallet_handle, &did.did)? {
            return Err(IndyError::DidError(DidError::AlreadyExistsError(did.did)));
        };

        self.wallet_service.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new())?;
        self.wallet_service.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())?;

        let res = (did.did, did.verkey);

        debug!("create_and_store_my_did <<< res: {:?}", res);

        Ok(res)
    }

    fn replace_keys_start(&self,
                          wallet_handle: i32,
                          key_info_json: &str,
                          my_did: &str) -> Result<String, IndyError> {
        debug!("replace_keys_start >>> wallet_handle: {:?}, key_info_json: {:?}, my_did: {:?}", wallet_handle, key_info_json, my_did);

        self.crypto_service.validate_did(my_did)?;

        let key_info: KeyInfo = serde_json::from_str(key_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(format!("Invalid KeyInfo json: {}", err.description())))?;

        let my_did = self._wallet_get_my_did(wallet_handle, my_did)?;

        let temporary_key = self.crypto_service.create_key(&key_info)?;
        let my_temporary_did = TemporaryDid { did: my_did.did, verkey: temporary_key.verkey.clone() };

        self.wallet_service.add_indy_object(wallet_handle, &temporary_key.verkey, &temporary_key, &HashMap::new())?;
        self.wallet_service.add_indy_object(wallet_handle, &my_temporary_did.did, &my_temporary_did, &HashMap::new())?;

        let res = my_temporary_did.verkey;

        debug!("replace_keys_start <<< res: {:?}", res);

        Ok(res)
    }

    fn replace_keys_apply(&self,
                          wallet_handle: i32,
                          my_did: &str) -> Result<(), IndyError> {
        debug!("replace_keys_apply >>> wallet_handle: {:?}, my_did: {:?}", wallet_handle, my_did);

        self.crypto_service.validate_did(my_did)?;

        let my_did = self._wallet_get_my_did(wallet_handle, my_did)?;
        let my_temporary_did: TemporaryDid =
            self.wallet_service.get_indy_object(wallet_handle, &my_did.did, &RecordOptions::id_value(), &mut String::new())?;

        let my_did = Did::from(my_temporary_did);

        self.wallet_service.update_indy_object(wallet_handle, &my_did.did, &my_did)?;
        self.wallet_service.delete_indy_record::<TemporaryDid>(wallet_handle, &my_did.did)?;

        debug!("replace_keys_apply <<<");

        Ok(())
    }

    fn store_their_did(&self,
                       wallet_handle: i32,
                       their_did_info_json: &str) -> Result<(), IndyError> {
        debug!("store_their_did >>> wallet_handle: {:?}, their_did_info_json: {:?}", wallet_handle, their_did_info_json);

        let their_did_info: TheirDidInfo = serde_json::from_str(their_did_info_json)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(format!("Invalid TheirDidInfo json: {}", err.description())))?;

        let their_did = self.crypto_service.create_their_did(&their_did_info)?;

        self.wallet_service.add_indy_object(wallet_handle, &their_did.did, &their_did, &HashMap::new())?;

        debug!("store_their_did <<<");

        Ok(())
    }

    fn get_my_did_with_meta(&self, wallet_handle: i32, my_did: String) -> Result<String, IndyError> {
        debug!("get_my_did_with_meta >>> wallet_handle: {:?}, my_did: {:?}", wallet_handle, my_did);

        self.crypto_service.validate_did(&my_did)?;

        let did_record = self.wallet_service.get_indy_record::<Did>(wallet_handle, &my_did, &RecordOptions::full())?;

        let did: Did = did_record.get_value()
            .and_then(|tags_json| serde_json::from_str(&tags_json).ok())
            .ok_or(CommonError::InvalidStructure(format!("Cannot deserialize Did: {:?}", my_did)))?;

        let meta: Option<String> = did_record.get_tags()
            .and_then(|tags| tags.get("metadata").cloned());

        let did_with_meta = DidWithMeta {
            did: did.did,
            verkey: did.verkey,
            metadata: meta
        };

        let res = serde_json::to_string(&did_with_meta)
            .map_err(|err|
                IndyError::CommonError(CommonError::InvalidState(format!("Can't serialize DID {}", err))))?;

        debug!("get_my_did_with_meta <<< res: {:?}", res);

        Ok(res)
    }

    fn list_my_dids_with_meta(&self, wallet_handle: i32) -> Result<String, IndyError> {
        debug!("list_my_dids_with_meta >>> wallet_handle: {:?}", wallet_handle);

        let mut did_search =
            self.wallet_service.search_indy_records::<Did>(wallet_handle, "{}", &SearchOptions::full())?;

        let mut dids: Vec<DidWithMeta> = Vec::new();

        while let Some(did_record) = did_search.fetch_next_record()? {
            let did_id = did_record.get_id();

            let did: Did = did_record.get_value()
                .and_then(|tags_json| serde_json::from_str(&tags_json).ok())
                .ok_or(CommonError::InvalidStructure(format!("Cannot deserialize Did: {:?}", did_id)))?;

            let meta: Option<String> = did_record.get_tags()
                .and_then(|tags| tags.get("metadata").cloned());

            let did_with_meta = DidWithMeta {
                did: did.did,
                verkey: did.verkey,
                metadata: meta
            };

            dids.push(did_with_meta);
        }

        let res = serde_json::to_string(&dids)
            .map_err(|err|
                IndyError::CommonError(CommonError::InvalidState(format!("Can't serialize DIDs list {}", err))))?;

        debug!("list_my_dids_with_meta <<< res: {:?}", res);

        Ok(res)
    }

    fn key_for_did(&self,
                   pool_handle: i32,
                   wallet_handle: i32,
                   did: String,
                   cb: Box<Fn(Result<String, IndyError>) + Send>) {
        debug!("key_for_did >>> pool_handle: {:?}, wallet_handle: {:?}, did: {:?}", pool_handle, wallet_handle, did);

        try_cb!(self.crypto_service.validate_did(&did), cb);

        // Look to my did
        match self._wallet_get_my_did(wallet_handle, &did) {
            Ok(my_did) => return cb(Ok(my_did.verkey)),
            Err(WalletError::ItemNotFound) => {}
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        // look to their did
        let their_did = ensure_their_did!(self,
                                          wallet_handle,
                                          pool_handle,
                                          did,
                                          DidCommand::KeyForDid(
                                              pool_handle,
                                              wallet_handle,
                                              did.clone(),
                                              cb),
                                           cb);

        let res = their_did.verkey;

        debug!("key_for_did <<< res: {:?}", res);

        cb(Ok(res))
    }

    fn key_for_local_did(&self,
                         wallet_handle: i32,
                         did: String) -> Result<String, IndyError> {
        info!("key_for_local_did >>> wallet_handle: {:?}, did: {:?}", wallet_handle, did);

        self.crypto_service.validate_did(&did)?;

        // Look to my did
        match self._wallet_get_my_did(wallet_handle, &did) {
            Ok(my_did) => return Ok(my_did.verkey),
            Err(WalletError::ItemNotFound) => {}
            Err(err) => return Err(IndyError::from(err))
        };

        // look to their did
        let their_did = self._wallet_get_their_did(wallet_handle, &did)?;

        let res = their_did.verkey;

        info!("key_for_local_did <<< res: {:?}", res);

        Ok(res)
    }

    fn set_endpoint_for_did(&self,
                            wallet_handle: i32,
                            did: String,
                            address: String,
                            transport_key: String) -> Result<(), IndyError> {
        debug!("set_endpoint_for_did >>> wallet_handle: {:?}, did: {:?}, address: {:?}, transport_key: {:?}", wallet_handle, did, address, transport_key);

        self.crypto_service.validate_did(&did)?;
        self.crypto_service.validate_key(&transport_key)?;

        let endpoint = Endpoint::new(address.to_string(), Some(transport_key.to_string()));

        self.wallet_service.upsert_indy_object(wallet_handle, &did, &endpoint)?;

        debug!("set_endpoint_for_did <<<");
        Ok(())
    }

    fn get_endpoint_for_did(&self,
                            wallet_handle: i32,
                            pool_handle: i32,
                            did: String,
                            cb: Box<Fn(Result<(String, Option<String>), IndyError>) + Send>) {
        debug!("get_endpoint_for_did >>> wallet_handle: {:?}, pool_handle: {:?}, did: {:?}", wallet_handle, pool_handle, did);

        try_cb!(self.crypto_service.validate_did(&did), cb);

        let endpoint =
            self.wallet_service.get_indy_object::<Endpoint>(wallet_handle, &did, &RecordOptions::id_value(), &mut String::new());

        match endpoint {
            Ok(endpoint) => cb(Ok((endpoint.ha, endpoint.verkey))),
            Err(WalletError::ItemNotFound) => {
                return self._fetch_attrib_from_ledger(wallet_handle,
                                                      pool_handle,
                                                      &did,
                                                      DidCommand::GetEndpointForDid(
                                                          wallet_handle,
                                                          pool_handle,
                                                          did.clone(),
                                                          cb));
            }
            Err(err) => cb(Err(IndyError::from(err)))
        };
    }

    fn set_did_metadata(&self,
                        wallet_handle: i32,
                        did: String,
                        metadata: String) -> Result<(), IndyError> {
        debug!("set_did_metadata >>> wallet_handle: {:?}, did: {:?}, metadata: {:?}", wallet_handle, did, metadata);

        self.crypto_service.validate_did(&did)?;

        self.wallet_service.get_indy_record::<Did>(wallet_handle, &did, &RecordOptions::id())?;

        let mut tags = HashMap::new();
        tags.insert(String::from("metadata"), metadata);

        let res = self.wallet_service.add_indy_record_tags::<Did>(wallet_handle, &did, &tags)?;

        debug!("set_did_metadata >>> res: {:?}", res);

        Ok(res)
    }

    fn get_did_metadata(&self,
                        wallet_handle: i32,
                        did: String) -> Result<String, IndyError> {
        debug!("get_did_metadata >>> wallet_handle: {:?}, did: {:?}", wallet_handle, did);

        self.crypto_service.validate_did(&did)?;

        let res = self.wallet_service.get_indy_record::<Did>(wallet_handle, &did, &RecordOptions::full())?
            .get_tags()
            .and_then(|tags| tags.get("metadata").cloned())
            .ok_or(WalletError::ItemNotFound)?;

        debug!("get_did_metadata <<< res: {:?}", res);

        Ok(res)
    }

    fn abbreviate_verkey(&self,
                         did: String,
                         verkey: String) -> Result<String, IndyError> {
        info!("abbreviate_verkey >>> did: {:?}, verkey: {:?}", did, verkey);

        self.crypto_service.validate_did(&did)?;
        self.crypto_service.validate_key(&verkey)?;

        let did = base58::decode(&did)?;
        let dverkey = base58::decode(&verkey)?;

        let (first_part, second_part) = dverkey.split_at(16);

        let res = if first_part.eq(did.as_slice()) {
            format!("~{}", base58::encode(second_part))
        } else {
            verkey
        };

        debug!("abbreviate_verkey <<< res: {:?}", res);

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
        trace!("_get_nym_ack >>> wallet_handle: {:?}, get_nym_reply_result: {:?}", wallet_handle, get_nym_reply_result);

        let get_nym_reply = get_nym_reply_result?;

        let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_reply)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid GetNymReplyResult json: {:?}", err)))?;

        let their_did_info = match get_nym_response.result() {
            GetNymReplyResult::GetNymReplyResultV0(res) => {
                if let Some(data) = &res.data {
                    let gen_nym_result_data: GetNymResultDataV0 = serde_json::from_str(data)
                        .map_err(map_err_trace!())
                        .map_err(|_| CommonError::InvalidState("Invalid GetNymResultData json".to_string()))?;

                    TheirDidInfo::new(gen_nym_result_data.dest, gen_nym_result_data.verkey)
                } else {
                    return Err(WalletError::ItemNotFound.into()); //TODO FIXME use separate error
                }
            }
            GetNymReplyResult::GetNymReplyResultV1(res) => TheirDidInfo::new(res.txn.data.did, res.txn.data.verkey)
        };

        let their_did = self.crypto_service.create_their_did(&their_did_info)?;

        self.wallet_service.add_indy_object(wallet_handle, &their_did.did, &their_did, &HashMap::new())?;

        trace!("_get_nym_ack <<<");

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
        trace!("_get_attrib_ack >>> wallet_handle: {:?}, get_attrib_reply_result: {:?}", wallet_handle, get_attrib_reply_result);

        let get_attrib_reply = get_attrib_reply_result?;

        let get_attrib_reply: Reply<GetAttrReplyResult> = serde_json::from_str(&get_attrib_reply)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid GetAttrReplyResult json {:?}", err)))?;

        let (raw, did) = match get_attrib_reply.result() {
            GetAttrReplyResult::GetAttrReplyResultV0(res) => (res.data, res.dest),
            GetAttrReplyResult::GetAttrReplyResultV1(res) => (res.txn.data.raw, res.txn.data.did)
        };

        let attrib_data: AttribData = serde_json::from_str(&raw)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid GetAttReply json: {:?}", err)))?;

        let endpoint = Endpoint::new(attrib_data.endpoint.ha, attrib_data.endpoint.verkey);

        self.wallet_service.add_indy_object(wallet_handle, &did, &endpoint, &HashMap::new())?;

        trace!("_get_attrib_ack <<<");

        Ok(())
    }

    fn _defer_command(&self, cmd: DidCommand) -> i32 {
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

    fn _call_error_cb(&self, command: DidCommand, err: IndyError) {
        match command {
            DidCommand::CreateAndStoreMyDid(_, _, cb) => {
                return cb(Err(err));
            }
            DidCommand::ReplaceKeysStart(_, _, _, cb) => {
                return cb(Err(err));
            }
            DidCommand::ReplaceKeysApply(_, _, cb) => {
                return cb(Err(err));
            }
            DidCommand::StoreTheirDid(_, _, cb) => {
                return cb(Err(err));
            }
            DidCommand::KeyForDid(_, _, _, cb) => {
                return cb(Err(err));
            }
            DidCommand::GetEndpointForDid(_, _, _, cb) => {
                return cb(Err(err));
            }
            _ => {}
        }
    }

    fn _fetch_their_did_from_ledger(&self,
                                    wallet_handle: i32, pool_handle: i32,
                                    did: &str, deferred_cmd: DidCommand) {
        // Defer this command until their did is fetched from ledger.
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
                        .send(Command::Did(DidCommand::GetNymAck(
                            wallet_handle,
                            result,
                            deferred_cmd_id
                        ))).unwrap();
                })
            ))).unwrap();
    }

    fn _fetch_attrib_from_ledger(&self,
                                 wallet_handle: i32, pool_handle: i32,
                                 did: &str, deferred_cmd: DidCommand) {
        // Defer this command until their did is fetched from ledger.
        let deferred_cmd_id = self._defer_command(deferred_cmd);

        // TODO we need passing of my_did as identifier
        let get_attrib_request = self.ledger_service.build_get_attrib_request(did, did, Some("endpoint"), None, None)
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
                        .send(Command::Did(DidCommand::GetAttribAck(
                            wallet_handle,
                            result,
                            deferred_cmd_id
                        ))).unwrap();
                })
            ))).unwrap();
    }

    fn _wallet_get_my_did(&self, wallet_handle: i32, my_did: &str) -> Result<Did, WalletError> {
        self.wallet_service.get_indy_object(wallet_handle, &my_did, &RecordOptions::id_value(), &mut String::new())
    }

    fn _wallet_get_their_did(&self, wallet_handle: i32, their_did: &str) -> Result<TheirDid, WalletError> {
        self.wallet_service.get_indy_object(wallet_handle, &their_did, &RecordOptions::id_value(), &mut String::new())
    }

    fn _wallet_get_did_metadata(&self, wallet_handle: i32, did: &str) -> Option<String> {
        self.wallet_service.get_indy_record::<Did>(wallet_handle, did, &RecordOptions::full()).ok()
            .and_then(|rec|
                rec.get_tags()
                    .and_then(|tags| tags.get("metadata").cloned())
            )
    }
}
