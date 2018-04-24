extern crate libc;
extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use services::wallet::{WalletService, WalletRecord, WalletSearch};
use std::rc::Rc;
use std::collections::HashMap;
use utils::sequence::SequenceUtils;
use std::cell::RefCell;

use self::indy_crypto::utils::json::JsonEncodable;

pub enum NonSecretsCommand {
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
                           Box<Fn(Result<String, IndyError>) + Send>),
    CloseSearch(i32, // wallet handle
                u32, // search handle
                Box<Fn(Result<(), IndyError>) + Send>)
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
            NonSecretsCommand::CloseSearch(wallet_handle, search_handle, cb) => {
                info!(target: "non_secrets_command_executor", "CloseSearch command received");
                cb(self.close_search(wallet_handle, search_handle));
            }
        };
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

    fn close_search(&self,
                    handle: i32,
                    search_handle: u32) -> Result<(), IndyError> {
        trace!("close_search >>> handle: {:?}, search_handle: {:?}", handle, search_handle);

        match self.searches.borrow_mut().remove(&handle) {
            Some(_) => self.wallet_service.close_search(handle, search_handle),
            None => Err(WalletError::InvalidHandle(format!("Wallet Search handle is invalid: {}", handle)))
        }?;

        trace!("close_search <<< ");

        Ok(())
    }

    fn _check_type(&self, type_: &str) -> Result<(), IndyError> {
        if type_.starts_with(WalletService::PREFIX) {
            return Err(IndyError::WalletError(WalletError::AccessFailed(format!("Record Type is available: {}", type_))));
        }
        Ok(())
    }
}