use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use domain::wallet::Tags;
use errors::prelude::*;
use services::wallet::{RecordOptions, SearchOptions, WalletRecord, WalletSearch, WalletService};
use utils::sequence;
use api::{WalletHandle, PoolHandle};
use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;

const CRED_DEF_CACHE: &str = "cred_def_cache";
const SCHEMA_CACHE: &str = "schema_cache";

pub enum NonSecretsCommand {
    AddRecord(WalletHandle,
              String, // type
              String, // id
              String, // value
              Option<Tags>, //tags
              Box<Fn(IndyResult<()>) + Send>),
    UpdateRecordValue(WalletHandle,
                      String, // type
                      String, // id
                      String, // value
                      Box<Fn(IndyResult<()>) + Send>),
    UpdateRecordTags(WalletHandle,
                     String, // type
                     String, // id
                     Tags, //tags
                     Box<Fn(IndyResult<()>) + Send>),
    AddRecordTags(WalletHandle,
                  String, // type
                  String, // id
                  Tags, //tags
                  Box<Fn(IndyResult<()>) + Send>),
    DeleteRecordTags(WalletHandle,
                     String, // type
                     String, // id
                     String, //tag names json
                     Box<Fn(IndyResult<()>) + Send>),
    DeleteRecord(WalletHandle,
                 String, // type
                 String, // id
                 Box<Fn(IndyResult<()>) + Send>),
    GetRecord(WalletHandle,
              String, // type
              String, // id
              String, // options json
              Box<Fn(IndyResult<String>) + Send>),
    OpenSearch(WalletHandle,
               String, // type
               String, // query json
               String, // options json
               Box<Fn(IndyResult<i32>) + Send>),
    FetchSearchNextRecords(WalletHandle,
                           i32, // wallet search handle
                           usize, // count
                           Box<Fn(IndyResult<String>) + Send>),
    CloseSearch(i32, // wallet search handle
                Box<Fn(IndyResult<()>) + Send>),
    GetSchema(PoolHandle,
              WalletHandle,
              String, // submitter_did
              String, // id
              String, // options_json
              Box<Fn(IndyResult<String>) + Send>),
    GetSchemaContinue(
        WalletHandle,
        IndyResult<(String, String)>, // ledger_response
        GetCacheOptions,              // options
        i32,                          // cb_id
    ),
    GetCredDef(PoolHandle,
               WalletHandle,
               String, // submitter_did
               String, // id
               String, // options_json
               Box<Fn(IndyResult<String>) + Send>),
    GetCredDefContinue(
        WalletHandle,
        IndyResult<(String, String)>, // ledger_response
        GetCacheOptions,              // options
        i32,                          // cb_id
    ),
    PurgeSchemaCache(WalletHandle,
                     String, // options json
                     Box<Fn(IndyResult<()>) + Send>),
    PurgeCredDefCache(WalletHandle,
                      String, // options json
                      Box<Fn(IndyResult<()>) + Send>),
}

pub struct NonSecretsCommandExecutor {
    wallet_service: Rc<WalletService>,
    searches: RefCell<HashMap<i32, Box<WalletSearch>>>,

    pending_callbacks: RefCell<HashMap<i32, Box<Fn(IndyResult<String>)>>>,
}

impl NonSecretsCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> NonSecretsCommandExecutor {
        NonSecretsCommandExecutor {
            wallet_service,
            searches: RefCell::new(HashMap::new()),
            pending_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: NonSecretsCommand) {
        match command {
            NonSecretsCommand::AddRecord(handle, type_, id, value, tags, cb) => {
                info!(target: "non_secrets_command_executor", "AddRecord command received");
                cb(self.add_record(handle, &type_, &id, &value, tags.as_ref()));
            }
            NonSecretsCommand::UpdateRecordValue(handle, type_, id, value, cb) => {
                info!(target: "non_secrets_command_executor", "UpdateRecordValue command received");
                cb(self.update_record_value(handle, &type_, &id, &value));
            }
            NonSecretsCommand::UpdateRecordTags(handle, type_, id, tags, cb) => {
                info!(target: "non_secrets_command_executor", "UpdateRecordTags command received");
                cb(self.update_record_tags(handle, &type_, &id, &tags));
            }
            NonSecretsCommand::AddRecordTags(handle, type_, id, tags, cb) => {
                info!(target: "non_secrets_command_executor", "AddRecordTags command received");
                cb(self.add_record_tags(handle, &type_, &id, &tags));
            }
            NonSecretsCommand::DeleteRecordTags(handle, type_, id, tags_names_json, cb) => {
                info!(target: "non_secrets_command_executor", "DeleteRecordTags command received");
                cb(self.delete_record_tags(handle, &type_, &id, &tags_names_json));
            }
            NonSecretsCommand::DeleteRecord(handle, type_, id, cb) => {
                info!(target: "non_secrets_command_executor", "DeleteRecord command received");
                cb(self.delete_record(handle, &type_, &id));
            }
            NonSecretsCommand::GetRecord(handle, type_, id, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "GetRecord command received");
                cb(self.get_record(handle, &type_, &id, &options_json));
            }
            NonSecretsCommand::OpenSearch(handle, type_, query_json, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "OpenSearch command received");
                cb(self.open_search(handle, &type_, &query_json, &options_json));
            }
            NonSecretsCommand::FetchSearchNextRecords(wallet_handle, wallet_search_handle, count, cb) => {
                info!(target: "non_secrets_command_executor", "SearchNextRecords command received");
                cb(self.fetch_search_next_records(wallet_handle, wallet_search_handle, count));
            }
            NonSecretsCommand::CloseSearch(wallet_search_handle, cb) => {
                info!(target: "non_secrets_command_executor", "CloseSearch command received");
                cb(self.close_search(wallet_search_handle));
            }
            NonSecretsCommand::GetSchema(pool_handle, wallet_handle, submitter_did, id, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "GetSchema command received");
                self.get_schema(pool_handle, wallet_handle, &submitter_did, &id, &options_json, cb);
            }
            NonSecretsCommand::GetSchemaContinue(wallet_handle, ledger_response, options, cb_id) => {
                info!(target: "non_secrets_command_executor", "GetSchemaContinue command received");
                self._get_schema_continue(wallet_handle, ledger_response, options, cb_id);
            }
            NonSecretsCommand::GetCredDef(pool_handle, wallet_handle, submitter_did, id, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "GetCredDef command received");
                self.get_cred_def(pool_handle, wallet_handle, &submitter_did, &id, &options_json, cb);
            }
            NonSecretsCommand::GetCredDefContinue(wallet_handle, ledger_response, options, cb_id) => {
                info!(target: "non_secrets_command_executor", "GetCredDefContinue command received");
                self._get_cred_def_continue(wallet_handle, ledger_response, options, cb_id);
            }
            NonSecretsCommand::PurgeSchemaCache(wallet_handle, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "PurgeSchemaCache command received");
                cb(self.purge_schema_cache(wallet_handle, &options_json));
            }
            NonSecretsCommand::PurgeCredDefCache(wallet_handle, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "PurgeCredDefCache command received");
                cb(self.purge_cred_def_cache(wallet_handle, &options_json));
            }
        };
    }

    fn add_record(&self,
                  wallet_handle: WalletHandle,
                  type_: &str,
                  id: &str,
                  value: &str,
                  tags: Option<&Tags>) -> IndyResult<()> {
        trace!("add_record >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}, tags: {:?}", wallet_handle, type_, id, value, tags);

        self._check_type(type_)?;

        let res = self.wallet_service.add_record(wallet_handle, type_, id, value, tags.unwrap_or(&Tags::new()))?;

        trace!("add_record <<< res: {:?}", res);

        Ok(res)
    }

    fn update_record_value(&self,
                           wallet_handle: WalletHandle,
                           type_: &str,
                           id: &str,
                           value: &str) -> IndyResult<()> {
        trace!("update_record_value >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}", wallet_handle, type_, id, value);

        self._check_type(type_)?;

        let res = self.wallet_service.update_record_value(wallet_handle, type_, id, value)?;

        trace!("update_record_value <<< res: {:?}", res);

        Ok(res)
    }

    fn update_record_tags(&self,
                          wallet_handle: WalletHandle,
                          type_: &str,
                          id: &str,
                          tags: &Tags) -> IndyResult<()> {
        trace!("update_record_tags >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags: {:?}", wallet_handle, type_, id, tags);

        self._check_type(type_)?;

        let res = self.wallet_service.update_record_tags(wallet_handle, type_, id, &tags)?;

        trace!("update_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn add_record_tags(&self,
                       wallet_handle: WalletHandle,
                       type_: &str,
                       id: &str,
                       tags: &Tags) -> IndyResult<()> {
        trace!("add_record_tags >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags: {:?}", wallet_handle, type_, id, tags);

        self._check_type(type_)?;

        let res = self.wallet_service.add_record_tags(wallet_handle, type_, id, &tags)?;

        trace!("add_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn delete_record_tags(&self,
                          wallet_handle: WalletHandle,
                          type_: &str,
                          id: &str,
                          tag_names_json: &str) -> IndyResult<()> {
        trace!("delete_record_tags >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tag_names_json: {:?}", wallet_handle, type_, id, tag_names_json);

        self._check_type(type_)?;

        let tag_names: Vec<&str> = serde_json::from_str(tag_names_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize tag names")?;

        let res = self.wallet_service.delete_record_tags(wallet_handle, type_, id, &tag_names)?;

        trace!("delete_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn delete_record(&self,
                     wallet_handle: WalletHandle,
                     type_: &str,
                     id: &str) -> IndyResult<()> {
        trace!("delete_record >>> wallet_handle: {:?}, type_: {:?}, id: {:?}", wallet_handle, type_, id);

        self._check_type(type_)?;

        let res = self.wallet_service.delete_record(wallet_handle, type_, id)?;

        trace!("delete_record <<< res: {:?}", res);

        Ok(res)
    }

    fn get_record(&self,
                  wallet_handle: WalletHandle,
                  type_: &str,
                  id: &str,
                  options_json: &str) -> IndyResult<String> {
        trace!("get_record >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, options_json: {:?}", wallet_handle, type_, id, options_json);

        self._check_type(type_)?;

        serde_json::from_str::<RecordOptions>(options_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize options")?;

        let record = self.wallet_service.get_record(wallet_handle, type_, id, &options_json)?;

        let res = serde_json::to_string(&record)
            .to_indy(IndyErrorKind::InvalidStructure, "Cannot serialize WalletRecord")?;

        trace!("get_record <<< res: {:?}", res);

        Ok(res)
    }

    fn open_search(&self,
                   wallet_handle: WalletHandle,
                   type_: &str,
                   query_json: &str,
                   options_json: &str) -> IndyResult<i32> {
        trace!("open_search >>> wallet_handle: {:?}, type_: {:?}, query_json: {:?}, options_json: {:?}", wallet_handle, type_, query_json, options_json);

        self._check_type(type_)?;

        serde_json::from_str::<SearchOptions>(options_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize options")?;

        let search = self.wallet_service.search_records(wallet_handle, type_, query_json, &options_json)?;

        let search_handle = sequence::get_next_id();

        self.searches.borrow_mut().insert(search_handle, Box::new(search));

        trace!("open_search <<< res: {:?}", search_handle);

        Ok(search_handle)
    }

    fn fetch_search_next_records(&self,
                                 wallet_handle: WalletHandle,
                                 wallet_search_handle: i32,
                                 count: usize) -> IndyResult<String> {
        trace!("fetch_search_next_records >>> wallet_handle: {:?}, wallet_search_handle: {:?}, count: {:?}", wallet_handle, wallet_search_handle, count);

        let mut searches = self.searches.borrow_mut();
        let search = searches.get_mut(&wallet_search_handle)
            .ok_or(err_msg(IndyErrorKind::InvalidWalletHandle, format!("Unknown WalletSearch handle: {}", wallet_search_handle)))?;

        let mut records: Vec<WalletRecord> = Vec::new();
        for _ in 0..count {
            match search.fetch_next_record()? {
                Some(record) => records.push(record),
                None => break
            }
        }

        let search_result = SearchRecords {
            total_count: search.get_total_count()?,
            records: if records.is_empty() { None } else { Some(records) }
        };

        let res = serde_json::to_string(&search_result)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize SearchRecords")?;

        trace!("fetch_search_next_records <<< res: {:?}", res);
        Ok(res)
    }

    fn close_search(&self,
                    wallet_search_handle: i32) -> IndyResult<()> {
        trace!("close_search >>> wallet_search_handle: {:?}", wallet_search_handle);

        let res = match self.searches.borrow_mut().remove(&wallet_search_handle) {
            Some(_) => Ok(()),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, format!("Wallet Search Handle is invalid: {}", wallet_search_handle)))
        }?;

        trace!("close_search <<< res: {:?}", res);

        Ok(res)
    }

    fn get_schema(&self,
                  pool_handle: PoolHandle,
                  wallet_handle: WalletHandle,
                  submitter_did: &str,
                  id: &str,
                  options_json: &str,
                  cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("get_schema >>> pool_handle: {:?}, wallet_handle: {:?}, submitter_did: {:?}, id: {:?}, options_json: {:?}",
               pool_handle, wallet_handle, submitter_did, id, options_json);

        let options = try_cb!(serde_json::from_str::<GetCacheOptions>(options_json).to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize options"), cb);

        let cache = if !options.no_cache.unwrap_or(false) {
            let options_json = json!({
                "retrieveType": false,
                "retrieveValue": true,
                "retrieveTags": true,
            }).to_string();
            match self.wallet_service.get_record(wallet_handle, SCHEMA_CACHE, id, &options_json) {
                Ok(record) => Ok(Some(record)),
                Err(err) => if err.kind() == IndyErrorKind::WalletItemNotFound {Ok(None)} else {Err(err)}
            }
        } else {Ok(None)};
        let cache = try_cb!(cache, cb);

        if cache.is_some() {
            let cache = cache.unwrap();
            let min_fresh = options.min_fresh.unwrap_or(-1);
            if min_fresh >= 0 {
                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
                if ts - min_fresh <= cache.get_tags().unwrap_or(&Tags::new()).get("timestamp").unwrap_or(&"-1".to_string()).parse().unwrap_or(-1) {
                    return cb(Ok(cache.get_value().unwrap_or("").to_string()))
                }
            }
            else {
                return cb(Ok(cache.get_value().unwrap_or("").to_string()))
            }
        }

        if options.no_update.unwrap_or(false) {
            return cb(Err(IndyError::from(IndyErrorKind::LedgerItemNotFound)));
        }

        let cb_id = ::utils::sequence::get_next_id();
        self.pending_callbacks.borrow_mut().insert(cb_id, cb);

        CommandExecutor::instance().send(
            Command::Ledger(
                LedgerCommand::GetSchema(
                    pool_handle,
                    Some(submitter_did.to_string()),
                    id.to_string(),
                    Box::new(move |ledger_response|{
                        CommandExecutor::instance().send(
                            Command::NonSecrets(
                                NonSecretsCommand::GetSchemaContinue(
                                    wallet_handle,
                                    ledger_response,
                                    options.clone(),
                                    cb_id,
                                )
                            )
                        ).unwrap();
                    })
                )
            )
        ).unwrap();
    }

    fn _get_schema_continue(&self, wallet_handle: WalletHandle, ledger_response: IndyResult<(String, String)>, options: GetCacheOptions, cb_id: i32) {
        let cb = self.pending_callbacks.borrow_mut().remove(&cb_id).expect("FIXME INVALID STATE");

        let (schema_id, schema_json) = try_cb!(ledger_response, cb);

        if !options.no_store.unwrap_or(false) {
            let mut tags = Tags::new();
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
            tags.insert("timestamp".to_string(), ts.to_string());
            let _ = self.wallet_service.delete_record(wallet_handle, SCHEMA_CACHE, &schema_id);
            let _ = self.wallet_service.add_record(wallet_handle, SCHEMA_CACHE, &schema_id, &schema_json, &tags);
        }

        cb(Ok(schema_json));
    }

    fn get_cred_def(&self,
                    pool_handle: PoolHandle,
                    wallet_handle: WalletHandle,
                    submitter_did: &str,
                    id: &str,
                    options_json: &str,
                    cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("get_cred_def >>> pool_handle: {:?}, wallet_handle: {:?}, submitter_did: {:?}, id: {:?}, options_json: {:?}",
               pool_handle, wallet_handle, submitter_did, id, options_json);

        let options = try_cb!(serde_json::from_str::<GetCacheOptions>(options_json).to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize options"), cb);

        let cache = if !options.no_cache.unwrap_or(false) {
            let options_json = json!({
                "retrieveType": false,
                "retrieveValue": true,
                "retrieveTags": true,
            }).to_string();
            match self.wallet_service.get_record(wallet_handle, CRED_DEF_CACHE, id, &options_json) {
                Ok(record) => Ok(Some(record)),
                Err(err) => if err.kind() == IndyErrorKind::WalletItemNotFound {Ok(None)} else {Err(err)}
            }
        } else {Ok(None)};
        let cache = try_cb!(cache, cb);

        if cache.is_some() {
            let cache = cache.unwrap();
            let min_fresh = options.min_fresh.unwrap_or(-1);
            if min_fresh >= 0 {
                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
                if ts - min_fresh <= cache.get_tags().unwrap_or(&Tags::new()).get("timestamp").unwrap_or(&"-1".to_string()).parse().unwrap_or(-1) {
                    return cb(Ok(cache.get_value().unwrap_or("").to_string()))
                }
            }
            else {
                return cb(Ok(cache.get_value().unwrap_or("").to_string()))
            }
        }

        if options.no_update.unwrap_or(false) {
            return cb(Err(IndyError::from(IndyErrorKind::LedgerItemNotFound)));
        }

        let cb_id = ::utils::sequence::get_next_id();
        self.pending_callbacks.borrow_mut().insert(cb_id, cb);

        CommandExecutor::instance().send(
            Command::Ledger(
                LedgerCommand::GetCredDef(
                    pool_handle,
                    Some(submitter_did.to_string()),
                    id.to_string(),
                    Box::new(move |ledger_response|{
                        CommandExecutor::instance().send(
                            Command::NonSecrets(
                                NonSecretsCommand::GetCredDefContinue(
                                    wallet_handle,
                                    ledger_response,
                                    options.clone(),
                                    cb_id,
                                )
                            )
                        ).unwrap();
                    })
                )
            )
        ).unwrap();
    }

    fn _get_cred_def_continue(&self, wallet_handle: WalletHandle, ledger_response: IndyResult<(String, String)>, options: GetCacheOptions, cb_id: i32) {
        let cb = self.pending_callbacks.borrow_mut().remove(&cb_id).expect("FIXME INVALID STATE");

        let (cred_def_id, cred_def_json) = try_cb!(ledger_response, cb);

        if !options.no_store.unwrap_or(false) {
            let mut tags = Tags::new();
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
            tags.insert("timestamp".to_string(), ts.to_string());
            let _ = self.wallet_service.delete_record(wallet_handle, CRED_DEF_CACHE, &cred_def_id);
            let _ = self.wallet_service.add_record(wallet_handle, CRED_DEF_CACHE, &cred_def_id, &cred_def_json, &tags);
        }

        cb(Ok(cred_def_json));
    }

    fn purge_schema_cache(&self,
                          wallet_handle: WalletHandle,
                          options_json: &str) -> IndyResult<()> {
        trace!("purge_schema_cache >>> wallet_handle: {:?}, options_json: {:?}", wallet_handle, options_json);

        let options = serde_json::from_str::<PurgeOptions>(options_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize options")?;

        let max_age = options.max_age.unwrap_or(-1);
        let query_json = if max_age >= 0 {
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
            json!({"timestamp": {"$lt": ts - max_age}}).to_string()
        } else {
            "{}".to_string()
        };

        let options_json = json!({
            "retrieveType": false,
            "retrieveValue": false,
            "retrieveTags": false,
        }).to_string();

        let mut search = self.wallet_service.search_records(
            wallet_handle,
            SCHEMA_CACHE,
            &query_json,
            &options_json,
        )?;

        while let Some(record) = search.fetch_next_record()? {
            self.wallet_service.delete_record(wallet_handle, SCHEMA_CACHE, record.get_id())?;
        }

        let res = ();

        trace!("purge_schema_cache <<< res: {:?}", res);

        Ok(res)
    }

    fn purge_cred_def_cache(&self,
                            wallet_handle: WalletHandle,
                            options_json: &str) -> IndyResult<()> {
        trace!("purge_cred_def_cache >>> wallet_handle: {:?}, options_json: {:?}", wallet_handle, options_json);

        let options = serde_json::from_str::<PurgeOptions>(options_json)
            .to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize options")?;

        let max_age = options.max_age.unwrap_or(-1);
        let query_json = if max_age >= 0 {
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
            json!({"timestamp": {"$lt": ts - max_age}}).to_string()
        } else {
            "{}".to_string()
        };

        let options_json = json!({
            "retrieveType": false,
            "retrieveValue": false,
            "retrieveTags": false,
        }).to_string();

        let mut search = self.wallet_service.search_records(
            wallet_handle,
            CRED_DEF_CACHE,
            &query_json,
            &options_json,
        )?;

        while let Some(record) = search.fetch_next_record()? {
            self.wallet_service.delete_record(wallet_handle, CRED_DEF_CACHE, record.get_id())?;
        }

        let res = ();

        trace!("purge_cred_def_cache <<< res: {:?}", res);

        Ok(res)
    }

    fn _check_type(&self, type_: &str) -> IndyResult<()> {
        if type_.starts_with(WalletService::PREFIX) {
            return Err(err_msg(IndyErrorKind::WalletAccessFailed, format!("Record of type \"{}\" is not available for fetching", type_)));
        }
        Ok(())
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRecords {
    pub total_count: Option<usize>,
    pub records: Option<Vec<WalletRecord>>
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
struct PurgeOptions {
    pub max_age: Option<i32>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetCacheOptions {
    pub no_cache: Option<bool>,     // Skip usage of cache,
    pub no_update: Option<bool>,    // Use only cached data, do not try to update.
    pub no_store: Option<bool>,     // Skip storing fresh data if updated
    pub min_fresh: Option<i32>,     // Return cached data if not older than this many seconds. -1 means do not check age.
}
