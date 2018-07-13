extern crate libc;
extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use services::wallet::{WalletService, WalletRecord, WalletSearch, RecordOptions, SearchOptions};
use std::rc::Rc;
use std::collections::HashMap;
use utils::sequence::SequenceUtils;
use std::cell::RefCell;
use self::indy_crypto::utils::json::JsonEncodable;

use std::result;

type Result<T> = result::Result<T, IndyError>;

pub enum NonSecretsCommand {
    AddRecord(i32, // handle
              String, // type
              String, // id
              String, // value
              Option<String>, //tags json
              Box<Fn(Result<()>) + Send>),
    UpdateRecordValue(i32, // handle
                      String, // type
                      String, // id
                      String, // value
                      Box<Fn(Result<()>) + Send>),
    UpdateRecordTags(i32, // handle
                     String, // type
                     String, // id
                     String, //tags json
                     Box<Fn(Result<()>) + Send>),
    AddRecordTags(i32, // handle
                  String, // type
                  String, // id
                  String, //tags json
                  Box<Fn(Result<()>) + Send>),
    DeleteRecordTags(i32, // handle
                     String, // type
                     String, // id
                     String, //tag names json
                     Box<Fn(Result<()>) + Send>),
    DeleteRecord(i32, // handle
                 String, // type
                 String, // id
                 Box<Fn(Result<()>) + Send>),
    GetRecord(i32, // handle
              String, // type
              String, // id
              String, // options json
              Box<Fn(Result<String>) + Send>),
    OpenSearch(i32, // handle
               String, // type
               String, // query json
               String, // options json
               Box<Fn(Result<i32>) + Send>),
    FetchSearchNextRecords(i32, // wallet handle
                           i32, // wallet search handle
                           usize, // count
                           Box<Fn(Result<String>) + Send>),
    CloseSearch(i32, // wallet search handle
                Box<Fn(Result<()>) + Send>)
}

pub struct NonSecretsCommandExecutor {
    wallet_service: Rc<WalletService>,
    searches: RefCell<HashMap<i32, Box<WalletSearch>>>
}

impl NonSecretsCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> NonSecretsCommandExecutor {
        NonSecretsCommandExecutor {
            wallet_service,
            searches: RefCell::new(HashMap::new())
        }
    }

    pub fn execute(&self, command: NonSecretsCommand) {
        match command {
            NonSecretsCommand::AddRecord(handle, type_, id, value, tags_json, cb) => {
                info!(target: "non_secrets_command_executor", "AddRecord command received");
                cb(self.add_record(handle, &type_, &id, &value, tags_json.as_ref().map(String::as_str)));
            }
            NonSecretsCommand::UpdateRecordValue(handle, type_, id, value, cb) => {
                info!(target: "non_secrets_command_executor", "UpdateRecordValue command received");
                cb(self.update_record_value(handle, &type_, &id, &value));
            }
            NonSecretsCommand::UpdateRecordTags(handle, type_, id, tags_json, cb) => {
                info!(target: "non_secrets_command_executor", "UpdateRecordTags command received");
                cb(self.update_record_tags(handle, &type_, &id, &tags_json));
            }
            NonSecretsCommand::AddRecordTags(handle, type_, id, tags_json, cb) => {
                info!(target: "non_secrets_command_executor", "AddRecordTags command received");
                cb(self.add_record_tags(handle, &type_, &id, &tags_json));
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
        };
    }

    fn add_record(&self,
                  wallet_handle: i32,
                  type_: &str,
                  id: &str,
                  value: &str,
                  tags_json: Option<&str>) -> Result<()> {
        trace!("add_record >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}, tags_json: {:?}", wallet_handle, type_, id, value, tags_json);

        self._check_type(type_)?;

        let tags = serde_json::from_str(tags_json.unwrap_or("{}"))
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize tags: {:?}", err)))?;
        let res = self.wallet_service.add_record(wallet_handle, type_, id, value, &tags)?;

        trace!("add_record <<< res: {:?}", res);

        Ok(res)
    }

    fn update_record_value(&self,
                           wallet_handle: i32,
                           type_: &str,
                           id: &str,
                           value: &str) -> Result<()> {
        trace!("update_record_value >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}", wallet_handle, type_, id, value);

        self._check_type(type_)?;

        let res = self.wallet_service.update_record_value(wallet_handle, type_, id, value)?;

        trace!("update_record_value <<< res: {:?}", res);

        Ok(res)
    }

    fn update_record_tags(&self,
                          wallet_handle: i32,
                          type_: &str,
                          id: &str,
                          tags_json: &str) -> Result<()> {
        trace!("update_record_tags >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", wallet_handle, type_, id, tags_json);

        self._check_type(type_)?;

        let tags = serde_json::from_str(tags_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize tags: {:?}", err)))?;
        let res = self.wallet_service.update_record_tags(wallet_handle, type_, id, &tags)?;

        trace!("update_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn add_record_tags(&self,
                       wallet_handle: i32,
                       type_: &str,
                       id: &str,
                       tags_json: &str) -> Result<()> {
        trace!("add_record_tags >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", wallet_handle, type_, id, tags_json);

        self._check_type(type_)?;

        let tags = serde_json::from_str(tags_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize tags: {:?}", err)))?;
        let res = self.wallet_service.add_record_tags(wallet_handle, type_, id, &tags)?;

        trace!("add_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn delete_record_tags(&self,
                          wallet_handle: i32,
                          type_: &str,
                          id: &str,
                          tag_names_json: &str) -> Result<()> {
        trace!("delete_record_tags >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tag_names_json: {:?}", wallet_handle, type_, id, tag_names_json);

        self._check_type(type_)?;

        let tag_names: Vec<&str> = serde_json::from_str(tag_names_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize tag names: {:?}", err)))?;

        let res = self.wallet_service.delete_record_tags(wallet_handle, type_, id, &tag_names)?;

        trace!("delete_record_tags <<< res: {:?}", res);

        Ok(res)
    }

    fn delete_record(&self,
                     wallet_handle: i32,
                     type_: &str,
                     id: &str) -> Result<()> {
        trace!("delete_record >>> wallet_handle: {:?}, type_: {:?}, id: {:?}", wallet_handle, type_, id);

        self._check_type(type_)?;

        let res = self.wallet_service.delete_record(wallet_handle, type_, id)?;

        trace!("delete_record <<< res: {:?}", res);

        Ok(res)
    }

    fn get_record(&self,
                  wallet_handle: i32,
                  type_: &str,
                  id: &str,
                  options_json: &str) -> Result<String> {
        trace!("get_record >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, options_json: {:?}", wallet_handle, type_, id, options_json);

        self._check_type(type_)?;

        serde_json::from_str::<RecordOptions>(options_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Options Json: {:?}", err)))?;

        let record = self.wallet_service.get_record(wallet_handle, type_, id, &options_json)?;

        let res = serde_json::to_string(&record)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize WalletRecord: {:?}", err)))?;

        trace!("get_record <<< res: {:?}", res);

        Ok(res)
    }

    fn open_search(&self,
                   wallet_handle: i32,
                   type_: &str,
                   query_json: &str,
                   options_json: &str) -> Result<i32> {
        trace!("open_search >>> wallet_handle: {:?}, type_: {:?}, query_json: {:?}, options_json: {:?}", wallet_handle, type_, query_json, options_json);

        self._check_type(type_)?;

        serde_json::from_str::<SearchOptions>(options_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Options Json: {:?}", err)))?;

        let search = self.wallet_service.search_records(wallet_handle, type_, query_json, &options_json)?;

        let search_handle = SequenceUtils::get_next_id();

        self.searches.borrow_mut().insert(search_handle, Box::new(search));

        trace!("open_search <<< res: {:?}", search_handle);

        Ok(search_handle)
    }

    fn fetch_search_next_records(&self,
                                 wallet_handle: i32,
                                 wallet_search_handle: i32,
                                 count: usize) -> Result<String> {
        trace!("fetch_search_next_records >>> wallet_handle: {:?}, wallet_search_handle: {:?}, count: {:?}", wallet_handle, wallet_search_handle, count);

        let mut searches = self.searches.borrow_mut();
        let search = searches.get_mut(&wallet_search_handle)
            .ok_or(WalletError::InvalidHandle(format!("Unknown WalletSearch handle: {}", wallet_search_handle)))?;

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

        let res = search_result.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize SearchRecords: {:?}", err)))?;

        trace!("fetch_search_next_records <<< res: {:?}", res);

        Ok(res)
    }

    fn close_search(&self,
                    wallet_search_handle: i32) -> Result<()> {
        trace!("close_search >>> wallet_search_handle: {:?}", wallet_search_handle);

        let res = match self.searches.borrow_mut().remove(&wallet_search_handle) {
            Some(_) => Ok(()),
            None => Err(WalletError::InvalidHandle(format!("Wallet Search Handle is invalid: {}", wallet_search_handle)))
        }?;

        trace!("close_search <<< res: {:?}", res);

        Ok(res)
    }

    fn _check_type(&self, type_: &str) -> Result<()> {
        if type_.starts_with(WalletService::PREFIX) {
            return Err(IndyError::WalletError(WalletError::AccessFailed(format!("Record of type \"{}\" is not available", type_))));
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

impl JsonEncodable for SearchRecords {}
