extern crate libc;
extern crate serde_json;

use errors::indy::IndyError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use services::wallet::WalletService;
use api::wallet::*;
use std::rc::Rc;

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
           Box<Fn(Result<(), IndyError>) + Send>)
}

pub struct WalletCommandExecutor {
    wallet_service: Rc<WalletService>
}

impl WalletCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> WalletCommandExecutor {
        WalletCommandExecutor {
            wallet_service
        }
    }

    pub fn execute(&self, command: WalletCommand) {
        match command {
            WalletCommand::RegisterWalletType(type_, create, open, close, delete, add_record,
                                              update_record_value, update_record_tags, add_record_tags,
                                              delete_record_tags, delete_record, get_record, get_record_id,
                                              get_record_value, get_record_tags, free_record,
                                              search_records, get_search_total_count, fetch_search_next_record,
                                              free_search, cb) => {
                info!(target: "wallet_command_executor", "RegisterWalletType command received");
                cb(self.register_type(&type_, create, open, close, delete, add_record,
                                      update_record_value, update_record_tags, add_record_tags,
                                      delete_record_tags, delete_record, get_record, get_record_id,
                                      get_record_value, get_record_tags, free_record,
                                      search_records, get_search_total_count, fetch_search_next_record,
                                      free_search));
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
                     get_record_value: WalletGetRecordValue,
                     get_record_tags: WalletGetRecordTags,
                     free_record: WalletFreeRecord,
                     search_records: WalletSearchRecords,
                     get_search_total_count: WalletGetSearchTotalCount,
                     fetch_search_next_record: WalletFetchSearchNextRecord,
                     free_search: WalletFreeSearch) -> Result<(), IndyError> {
        self
            .wallet_service
            .register_wallet_storage(
                type_, create, open, close, delete, add_record, update_record_value, update_record_tags,
                add_record_tags, delete_record_tags, delete_record, get_record, get_record_id,
                get_record_value, get_record_tags, free_record, search_records, get_search_total_count,
                fetch_search_next_record, free_search)
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
              handle: &str,
              credentials: Option<&str>) -> Result<(), IndyError> {
        self.wallet_service.delete(handle, credentials)
            .map_err(|err| IndyError::WalletError(err))
    }
}