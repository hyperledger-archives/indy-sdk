extern crate libc;
extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use services::wallet::{WalletService, WalletRecord, WalletSearch};
use api::wallet::*;
use std::rc::Rc;
use std::collections::HashMap;
use utils::sequence::SequenceUtils;
use std::cell::RefCell;

use self::indy_crypto::utils::json::JsonEncodable;

pub enum WalletCommand {
    RegisterWalletType(String, // type_
                       WalletCreate, // create
                       WalletOpen, // open
                       WalletClose, // close
                       WalletDelete, // delete
                       WalletAddRecord, // add record
                       WalletUpdateRecordValue, // update record value
                       WalletUpdateRecordTags, // update record value
                       WalletAddRecordTags, // add record tags
                       WalletDeleteRecordTags, // delete record tags
                       WalletDeleteRecord, // delete record
                       WalletGetRecord, // get record
                       WalletGetRecordId, // get record id
                       WalletGetRecordType, // get record id
                       WalletGetRecordValue, // get record value
                       WalletGetRecordTags, // get record tags
                       WalletFreeRecord, // free record
                       WalletSearchRecords, // search records
                       WalletSearchAllRecords, // search all records
                       WalletGetSearchTotalCount, // get search total count
                       WalletFetchSearchNextRecord, // fetch search next record
                       WalletFreeSearch, // free search
                       Box<Fn(Result<(), IndyError>) + Send>),
    Create(String, // pool name
           String, // wallet name
           Option<String>, // wallet type
           Option<String>, // wallet config
           Option<String>, // wallet credentials
           Box<Fn(Result<(), IndyError>) + Send>),
    Open(String, // wallet name
         Option<String>, // wallet runtime config
         Option<String>, // wallet credentials
         Box<Fn(Result<i32, IndyError>) + Send>),
    Close(i32, // handle
          Box<Fn(Result<(), IndyError>) + Send>),
    ListWallets(Box<Fn(Result<String, IndyError>) + Send>),
    Delete(String, // name
           Option<String>, // wallet credentials
           Box<Fn(Result<(), IndyError>) + Send>),
    AddRecord(i32, // handle
              String, // type
              String, // id
              String, // value
              Option<String>, //tags json
              Box<Fn(Result<(), IndyError>) + Send>),
    UpdateRecordValue(i32, // handle
                      String, // type
                      String, // id
                      String, // value
                      Box<Fn(Result<(), IndyError>) + Send>),
    UpdateRecordTags(i32, // handle
                     String, // type
                     String, // id
                     String, //tags json
                     Box<Fn(Result<(), IndyError>) + Send>),
    AddRecordTags(i32, // handle
                  String, // type
                  String, // id
                  String, //tags json
                  Box<Fn(Result<(), IndyError>) + Send>),
    DeleteRecordTags(i32, // handle
                     String, // type
                     String, // id
                     String, //tag names json
                     Box<Fn(Result<(), IndyError>) + Send>),
    DeleteRecord(i32, // handle
                 String, // type
                 String, // id
                 Box<Fn(Result<(), IndyError>) + Send>),
    GetRecord(i32, // handle
              String, // type
              String, // id
              String, // options json
              Box<Fn(Result<String, IndyError>) + Send>),
    OpenSearch(i32, // handle
               String, // type
               String, // query json
               String, // options json
               Box<Fn(Result<i32, IndyError>) + Send>),
    FetchSearchNextRecords(i32, // wallet handle
                           i32, // search handle
                           usize, // count
                           Box<Fn(Result<String, IndyError>) + Send>)
}

pub struct WalletCommandExecutor {
    wallet_service: Rc<WalletService>,
    searches: RefCell<HashMap<i32, Box<WalletSearch>>>
}

impl WalletCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> WalletCommandExecutor {
        WalletCommandExecutor {
            wallet_service,
            searches: RefCell::new(HashMap::new())
        }
    }

    pub fn execute(&self, command: WalletCommand) {
        match command {
            WalletCommand::RegisterWalletType(type_, create, open, close, delete, add_record,
                                              update_record_value, update_record_tags, add_record_tags,
                                              delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                                              get_record_value, get_record_tags, free_record,
                                              search_records, search_all_records, get_search_total_count,
                                              fetch_search_next_record, free_search, cb) => {
                info!(target: "wallet_command_executor", "RegisterWalletType command received");
                cb(self.register_type(&type_, create, open, close, delete, add_record,
                                      update_record_value, update_record_tags, add_record_tags,
                                      delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                                      get_record_value, get_record_tags, free_record,
                                      search_records, search_all_records, get_search_total_count,
                                      fetch_search_next_record, free_search));
            }
            WalletCommand::Create(pool_name, name, xtype, config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Create command received");
                cb(self.create(&pool_name, &name, xtype.as_ref().map(String::as_str),
                               config.as_ref().map(String::as_str),
                               credentials.as_ref().map(String::as_str)));
            }
            WalletCommand::Open(name, runtime_config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Open command received");
                cb(self.open(&name, runtime_config.as_ref().map(String::as_str),
                             credentials.as_ref().map(String::as_str)));
            }
            WalletCommand::Close(handle, cb) => {
                info!(target: "wallet_command_executor", "Close command received");
                cb(self.close(handle));
            }
            WalletCommand::ListWallets(cb) => {
                info!(target: "wallet_command_executor", "ListWallets command received");
                cb(self.list_wallets());
            }
            WalletCommand::Delete(name, credentials, cb) => {
                info!(target: "wallet_command_executor", "Delete command received");
                cb(self.delete(&name, credentials.as_ref().map(String::as_str)));
            }
            WalletCommand::AddRecord(handle, type_, id, value, tags_json, cb) => {
                info!(target: "wallet_command_executor", "AddRecord command received");
                cb(self.add_record(handle, &type_, &id, &value, tags_json.as_ref().map(String::as_str)));
            }
            WalletCommand::UpdateRecordValue(handle, type_, id, value, cb) => {
                info!(target: "wallet_command_executor", "UpdateRecordValue command received");
                cb(self.update_record_value(handle, &type_, &id, &value));
            }
            WalletCommand::UpdateRecordTags(handle, type_, id, tags_json, cb) => {
                info!(target: "wallet_command_executor", "UpdateRecordTags command received");
                cb(self.update_record_tags(handle, &type_, &id, &tags_json));
            }
            WalletCommand::AddRecordTags(handle, type_, id, tags_json, cb) => {
                info!(target: "wallet_command_executor", "AddRecordTags command received");
                cb(self.add_record_tags(handle, &type_, &id, &tags_json));
            }
            WalletCommand::DeleteRecordTags(handle, type_, id, tags_names_json, cb) => {
                info!(target: "wallet_command_executor", "DeleteRecordTags command received");
                cb(self.delete_record_tags(handle, &type_, &id, &tags_names_json));
            }
            WalletCommand::DeleteRecord(handle, type_, id, cb) => {
                info!(target: "wallet_command_executor", "DeleteRecord command received");
                cb(self.delete_record(handle, &type_, &id));
            }
            WalletCommand::GetRecord(handle, type_, id, options_json, cb) => {
                info!(target: "wallet_command_executor", "GetRecord command received");
                cb(self.get_record(handle, &type_, &id, &options_json));
            }
            WalletCommand::OpenSearch(handle, type_, query_json, options_json, cb) => {
                info!(target: "wallet_command_executor", "OpenSearch command received");
                cb(self.open_search(handle, &type_, &query_json, &options_json));
            }
            WalletCommand::FetchSearchNextRecords(wallet_handle, wallet_search_handle, count, cb) => {
                info!(target: "wallet_command_executor", "SearchNextRecords command received");
                cb(self.fetch_search_next_records(wallet_handle, wallet_search_handle, count));
            }
        };
    }

    fn register_type(&self,
                     type_: &str,
                     create: WalletCreate,
                     open: WalletOpen,
                     close: WalletClose,
                     delete: WalletDelete,
                     add_record: WalletAddRecord,
                     update_record_value: WalletUpdateRecordValue,
                     update_record_tags: WalletUpdateRecordTags,
                     add_record_tags: WalletAddRecordTags,
                     delete_record_tags: WalletDeleteRecordTags,
                     delete_record: WalletDeleteRecord,
                     get_record: WalletGetRecord,
                     get_record_id: WalletGetRecordId,
                     get_record_type: WalletGetRecordType,
                     get_record_value: WalletGetRecordValue,
                     get_record_tags: WalletGetRecordTags,
                     free_record: WalletFreeRecord,
                     search_records: WalletSearchRecords,
                     search_all_records: WalletSearchAllRecords,
                     get_search_total_count: WalletGetSearchTotalCount,
                     fetch_search_next_record: WalletFetchSearchNextRecord,
                     free_search: WalletFreeSearch) -> Result<(), IndyError> {
        self
            .wallet_service
            .register_wallet_storage(
                type_, create, open, close, delete, add_record, update_record_value, update_record_tags,
                add_record_tags, delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                get_record_value, get_record_tags, free_record, search_records, search_all_records,
                get_search_total_count, fetch_search_next_record, free_search)
            .map_err(IndyError::from)
    }

    fn create(&self,
              pool_name: &str,
              name: &str,
              xtype: Option<&str>,
              config: Option<&str>,
              credentials: Option<&str>) -> Result<(), IndyError> {
        self.wallet_service.create(pool_name, name, xtype, config, credentials)
            .map_err(|err| IndyError::WalletError(err))
    }

    fn open(&self,
            name: &str,
            runtime_config: Option<&str>,
            credentials: Option<&str>) -> Result<i32, IndyError> {
        self.wallet_service.open(name, runtime_config, credentials)
            .map_err(|err| IndyError::WalletError(err))
    }

    fn close(&self,
             handle: i32) -> Result<(), IndyError> {
        self.wallet_service.close(handle)
            .map_err(|err| IndyError::WalletError(err))
    }

    fn list_wallets(&self) -> Result<String, IndyError> {
        self.wallet_service.list_wallets()
            .and_then(|wallets|
                serde_json::to_string(&wallets)
                    .map_err(|err|
                        WalletError::CommonError(CommonError::InvalidState(format!("Can't serialize wallets list {}", err)))))
            .map_err(IndyError::from)
    }

    fn delete(&self,
              name: &str,
              credentials: Option<&str>) -> Result<(), IndyError> {
        self.wallet_service.delete(name, credentials)
            .map_err(|err| IndyError::WalletError(err))
    }

    fn add_record(&self,
                  handle: i32,
                  type_: &str,
                  id: &str,
                  value: &str,
                  tags_json: Option<&str>) -> Result<(), IndyError> {
        trace!("add_record >>> handle: {:?}, type_: {:?}, id: {:?}, value: {:?}, tags_json: {:?}", handle, type_, id, value, tags_json);

        self._check_type(type_)?;

        let res = self.wallet_service.add_record(handle, type_, id, value, tags_json.unwrap_or(""))?; //TODO: question

        trace!("add_record <<< res: {:?}", res);

        Ok(res)
    }

    fn update_record_value(&self,
                           handle: i32,
                           type_: &str,
                           id: &str,
                           value: &str) -> Result<(), IndyError> {
        trace!("update_record_value >>> handle: {:?}, type_: {:?}, id: {:?}, value: {:?}", handle, type_, id, value);

        self._check_type(type_)?;

        let res = self.wallet_service.update_record_value(handle, type_, id, value)?;

        trace!("update_record_value <<< res: {:?}", res);

        Ok(res)
    }

    fn update_record_tags(&self,
                          handle: i32,
                          type_: &str,
                          id: &str,
                          tags_json: &str) -> Result<(), IndyError> {
        trace!("update_record_tags >>> handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", handle, type_, id, tags_json);

        self._check_type(type_)?;

        let res = self.wallet_service.update_record_tags(handle, type_, id, tags_json)?;

        trace!("update_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn add_record_tags(&self,
                       handle: i32,
                       type_: &str,
                       id: &str,
                       tags_json: &str) -> Result<(), IndyError> {
        trace!("add_record_tags >>> handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", handle, type_, id, tags_json);

        self._check_type(type_)?;

        let res = self.wallet_service.add_record_tags(handle, type_, id, tags_json)?;

        trace!("add_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn delete_record_tags(&self,
                          handle: i32,
                          type_: &str,
                          id: &str,
                          tags_names_json: &str) -> Result<(), IndyError> {
        trace!("delete_record_tags >>> handle: {:?}, type_: {:?}, id: {:?}, tags_names_json: {:?}", handle, type_, id, tags_names_json);

        self._check_type(type_)?;

        let res = self.wallet_service.delete_record_tags(handle, type_, id, tags_names_json)?;

        trace!("delete_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn delete_record(&self,
                     handle: i32,
                     type_: &str,
                     id: &str) -> Result<(), IndyError> {
        trace!("delete_record >>> handle: {:?}, type_: {:?}, id: {:?}", handle, type_, id);

        self._check_type(type_)?;

        let res = self.wallet_service.delete_record(handle, type_, id)?;

        trace!("delete_record <<< res: {:?}", res);

        Ok(res)
    }

    fn get_record(&self,
                  handle: i32,
                  type_: &str,
                  id: &str,
                  options_json: &str) -> Result<String, IndyError> {
        trace!("get_record >>> handle: {:?}, type_: {:?}, id: {:?}, options_json: {:?}", handle, type_, id, options_json);

        self._check_type(type_)?;

        let record = self.wallet_service.get_record(handle, type_, id, options_json)?;

        let res = record.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize WalletRecord: {:?}", err)))?;

        trace!("get_record <<< res: {:?}", res);

        Ok(res)
    }

    fn open_search(&self,
                   handle: i32,
                   type_: &str,
                   query_json: &str,
                   options_json: &str) -> Result<i32, IndyError> {
        trace!("open_search >>> handle: {:?}, type_: {:?}, query_json: {:?}, options_json: {:?}", handle, type_, query_json, options_json);

        self._check_type(type_)?;

        let search = self.wallet_service.search_records(handle, type_, query_json, options_json)?;

        let search_handle = SequenceUtils::get_next_id();

        let mut searches = self.searches.borrow_mut();
        searches.insert(search_handle, Box::new(search));

        trace!("open_search <<< res: {:?}", search_handle);

        Ok(search_handle)
    }

    fn fetch_search_next_records(&self,
                                 handle: i32,
                                 search_handle: i32,
                                 count: usize) -> Result<String, IndyError> {
        trace!("fetch_search_next_records >>> handle: {:?}, search_handle: {:?}, count: {:?}", handle, search_handle, count);

        let mut searches = self.searches.borrow_mut();

        let search = searches.get_mut(&search_handle)
            .ok_or(CommonError::InvalidStructure(format!("Unknown WalletSearch handle: {}", search_handle)))?;

        let mut records: Vec<WalletRecord> = Vec::new();

        for _ in 0..count {
            if let Some(record) = search.fetch_next_record()? {
                records.push(record);
            }
        }

        let mut wallet_search_result: serde_json::Value = serde_json::Value::Object(serde_json::map::Map::new());
        wallet_search_result["records"] = json!(records);

        if let Some(total_count) = search.get_total_count()? {
            wallet_search_result["totalCount"] = json!(total_count);
        }

        let res = serde_json::to_string(&wallet_search_result)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot serialize WalletSearchResult: {:?}", err)))?;

        trace!("fetch_search_next_records <<< res: {:?}", res);

        Ok(res)
    }

    fn _check_type(&self, type_: &str) -> Result<(), IndyError> {
        if type_.starts_with(WalletService::PREFIX) {
            return Err(IndyError::WalletError(WalletError::AccessFailed(format!("Record Type is available: {}", type_))));
        }
        Ok(())
    }
}