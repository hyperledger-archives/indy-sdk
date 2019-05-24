use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use domain::wallet::Tags;
use errors::prelude::*;
use services::wallet::WalletService;
use api::{WalletHandle, PoolHandle};
use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;

const CRED_DEF_CACHE: &str = "cred_def_cache";
const SCHEMA_CACHE: &str = "schema_cache";

pub enum CacheCommand {
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

pub struct CacheCommandExecutor {
    wallet_service: Rc<WalletService>,

    pending_callbacks: RefCell<HashMap<i32, Box<Fn(IndyResult<String>)>>>,
}

impl CacheCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> CacheCommandExecutor {
        CacheCommandExecutor {
            wallet_service,
            pending_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: CacheCommand) {
        match command {
            CacheCommand::GetSchema(pool_handle, wallet_handle, submitter_did, id, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "GetSchema command received");
                self.get_schema(pool_handle, wallet_handle, &submitter_did, &id, &options_json, cb);
            }
            CacheCommand::GetSchemaContinue(wallet_handle, ledger_response, options, cb_id) => {
                info!(target: "non_secrets_command_executor", "GetSchemaContinue command received");
                self._get_schema_continue(wallet_handle, ledger_response, options, cb_id);
            }
            CacheCommand::GetCredDef(pool_handle, wallet_handle, submitter_did, id, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "GetCredDef command received");
                self.get_cred_def(pool_handle, wallet_handle, &submitter_did, &id, &options_json, cb);
            }
            CacheCommand::GetCredDefContinue(wallet_handle, ledger_response, options, cb_id) => {
                info!(target: "non_secrets_command_executor", "GetCredDefContinue command received");
                self._get_cred_def_continue(wallet_handle, ledger_response, options, cb_id);
            }
            CacheCommand::PurgeSchemaCache(wallet_handle, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "PurgeSchemaCache command received");
                cb(self.purge_schema_cache(wallet_handle, &options_json));
            }
            CacheCommand::PurgeCredDefCache(wallet_handle, options_json, cb) => {
                info!(target: "non_secrets_command_executor", "PurgeCredDefCache command received");
                cb(self.purge_cred_def_cache(wallet_handle, &options_json));
            }
        }
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
                Err(err) => if err.kind() == IndyErrorKind::WalletItemNotFound { Ok(None) } else { Err(err) }
            }
        } else { Ok(None) };
        let cache = try_cb!(cache, cb);

        if let Some(cache) = cache {
            let min_fresh = options.min_fresh.unwrap_or(-1);
            if min_fresh >= 0 {
                let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(ts) => ts.as_secs() as i32,
                    Err(err) => {
                        error!("Cannot get time: {:?}", err);
                        return cb(Err(IndyError::from_msg(IndyErrorKind::InvalidState, format!("Cannot get time: {:?}", err))))
                    }
                };
                if ts - min_fresh <= cache.get_tags().unwrap_or(&Tags::new()).get("timestamp").unwrap_or(&"-1".to_string()).parse().unwrap_or(-1) {
                    return cb(Ok(cache.get_value().unwrap_or("").to_string()))
                }
            } else {
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
                    Box::new(move |ledger_response| {
                        CommandExecutor::instance().send(
                            Command::Cache(
                                CacheCommand::GetSchemaContinue(
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
            let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(ts) => ts.as_secs() as i32,
                Err(err) => {
                    warn!("Cannot get time: {:?}", err);
                    0
                }
            };
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
                Err(err) => if err.kind() == IndyErrorKind::WalletItemNotFound { Ok(None) } else { Err(err) }
            }
        } else { Ok(None) };
        let cache = try_cb!(cache, cb);

        if let Some(cache) = cache {
            let min_fresh = options.min_fresh.unwrap_or(-1);
            if min_fresh >= 0 {
                let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(ts) => ts.as_secs() as i32,
                    Err(err) => {
                        error!("Cannot get time: {:?}", err);
                        return cb(Err(IndyError::from_msg(IndyErrorKind::InvalidState, format!("Cannot get time: {:?}", err))))
                    }
                };
                if ts - min_fresh <= cache.get_tags().unwrap_or(&Tags::new()).get("timestamp").unwrap_or(&"-1".to_string()).parse().unwrap_or(-1) {
                    return cb(Ok(cache.get_value().unwrap_or("").to_string()))
                }
            } else {
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
                    Box::new(move |ledger_response| {
                        CommandExecutor::instance().send(
                            Command::Cache(
                                CacheCommand::GetCredDefContinue(
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
            let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(ts) => ts.as_secs() as i32,
                Err(err) => {
                    warn!("Cannot get time: {:?}", err);
                    0
                }
            };
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
            let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(ts) => ts.as_secs() as i32,
                Err(err) => {
                    error!("Cannot get time: {:?}", err);
                    return Err(IndyError::from_msg(IndyErrorKind::InvalidState, format!("Cannot get time: {:?}", err)))
                }
            };
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
            let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(ts) => ts.as_secs() as i32,
                Err(err) => {
                    error!("Cannot get time: {:?}", err);
                    return Err(IndyError::from_msg(IndyErrorKind::InvalidState, format!("Cannot get time: {:?}", err)))
                }
            };
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
