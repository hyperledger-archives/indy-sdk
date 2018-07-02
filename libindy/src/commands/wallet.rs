extern crate libc;
extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use services::wallet::WalletService;
use api::wallet::*;
use std::rc::Rc;

use std::result;

type Result<T> = result::Result<T, IndyError>;

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
                       WalletGetStorageMetadata, // get storage metadata
                       WalletSetStorageMetadata, // set storage metadata
                       WalletFreeStorageMetadata, // free storage metadata
                       WalletSearchRecords, // search records
                       WalletSearchAllRecords, // search all records
                       WalletGetSearchTotalCount, // get search total count
                       WalletFetchSearchNextRecord, // fetch search next record
                       WalletFreeSearch, // free search
                       Box<Fn(Result<()>) + Send>),
    Create(String, // pool name
           String, // wallet name
           Option<String>, // storage type
           Option<String>, // config
           String, // credentials
           Box<Fn(Result<()>) + Send>),
    Open(String, // wallet name
         Option<String>, // wallet runtime config
         String, // wallet credentials
         Box<Fn(Result<i32>) + Send>),
    Close(i32, // handle
          Box<Fn(Result<()>) + Send>),
    ListWallets(Box<Fn(Result<String>) + Send>),
    Delete(String, // name
           String, // wallet credentials
           Box<Fn(Result<()>) + Send>),
    Export(i32, // wallet_handle
           String, // export config_json
           Box<Fn(Result<()>) + Send>),
    Import(String, // pool name
           String, // wallet name
           Option<String>, // storage type
           Option<String>, // config
           String, // credentials
           String, // import_config_json
           Box<Fn(Result<()>) + Send>),
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
                                              delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                                              get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                                              free_storage_metadata, search_records, search_all_records, get_search_total_count,
                                              fetch_search_next_record, free_search, cb) => {
                info!(target: "wallet_command_executor", "RegisterWalletType command received");
                cb(self.register_type(&type_, create, open, close, delete, add_record,
                                      update_record_value, update_record_tags, add_record_tags,
                                      delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                                      get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                                      free_storage_metadata, search_records, search_all_records, get_search_total_count,
                                      fetch_search_next_record, free_search));
            }
            WalletCommand::Create(pool_name, name, storage_type, config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Create command received");
                cb(self.create(&pool_name, &name, storage_type.as_ref().map(String::as_str),
                               config.as_ref().map(String::as_str), &credentials));
            }
            WalletCommand::Open(name, runtime_config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Open command received");
                cb(self.open(&name, runtime_config.as_ref().map(String::as_str), &credentials));
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
                cb(self.delete(&name, &credentials));
            }
            WalletCommand::Export(wallet_handle, export_config_json, cb) => {
                info!(target: "wallet_command_executor", "Export command received");
                cb(self.export(wallet_handle, &export_config_json));
            }
            WalletCommand::Import(pool_name, name, storage_type, config, credentials, import_config, cb) => {
                info!(target: "wallet_command_executor", "Import command received");
                cb(self.import(&pool_name, &name, storage_type.as_ref().map(String::as_str),
                               config.as_ref().map(String::as_str), &credentials, &import_config));
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
                     get_storage_metadata: WalletGetStorageMetadata,
                     set_storage_metadata: WalletSetStorageMetadata,
                     free_storage_metadata: WalletFreeStorageMetadata,
                     search_records: WalletSearchRecords,
                     search_all_records: WalletSearchAllRecords,
                     get_search_total_count: WalletGetSearchTotalCount,
                     fetch_search_next_record: WalletFetchSearchNextRecord,
                     free_search: WalletFreeSearch) -> Result<()> {
        info!("register_type >>>");

        let res = self
            .wallet_service
            .register_wallet_storage(
                type_, create, open, close, delete, add_record, update_record_value, update_record_tags,
                add_record_tags, delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                free_storage_metadata, search_records, search_all_records,
                get_search_total_count, fetch_search_next_record, free_search)?;

        info!("register_type <<< res: {:?}", res);

        Ok(res)
    }

    fn create(&self,
              pool_name: &str,
              name: &str,
              storage_type: Option<&str>,
              config: Option<&str>,
              credentials: &str) -> Result<()> {
        debug!("create >>> pool_name: {:?}, name: {:?}, storage_type: {:?}, config: {:?}, credentials: {:?}",
               pool_name, name, storage_type, config, credentials);

        let res = self.wallet_service.create_wallet(pool_name, name, storage_type, config, credentials)?;

        debug!("create <<< res: {:?}", res);

        Ok(res)
    }

    fn open(&self,
            name: &str,
            runtime_config: Option<&str>,
            credentials: &str) -> Result<i32> {
        debug!("open >>> name: {:?}, runtime_config: {:?}, credentials: {:?}", name, runtime_config, credentials);

        let res = self.wallet_service.open_wallet(name, runtime_config, credentials)?;

        debug!("open <<< res: {:?}", res);

        Ok(res)
    }

    fn close(&self,
             handle: i32) -> Result<()> {
        debug!("close >>> handle: {:?}", handle);

        let res = self.wallet_service.close_wallet(handle)?;

        debug!("close <<< res: {:?}", res);

        Ok(res)
    }

    fn list_wallets(&self) -> Result<String> {
        debug!("list_wallets >>>");

        let res = self.wallet_service.list_wallets()
            .and_then(|wallets|
                serde_json::to_string(&wallets)
                    .map_err(|err|
                        WalletError::CommonError(CommonError::InvalidState(format!("Can't serialize wallets list {}", err)))))?;

        debug!("list_wallets << res: {:?}", res);

        Ok(res)
    }

    fn delete(&self,
              name: &str,
              credentials: &str) -> Result<()> {
        debug!("delete >>> name: {:?}, credentials: {:?}", name, credentials);

        let res = self.wallet_service.delete_wallet(name, credentials)?;

        debug!("delete <<< res: {:?}", res);

        Ok(res)
    }

    fn export(&self,
              wallet_handle: i32,
              export_config_json: &str) -> Result<()> {
        debug!("export >>> handle: {:?}, export_config_json: {:?}", wallet_handle, export_config_json);

        // TODO - later add proper versioning
        let res = self.wallet_service.export_wallet(wallet_handle, export_config_json, 0)?;

        debug!("export <<< res: {:?}", res);

        Ok(res)
    }

    fn import(&self,
              pool_name: &str,
              name: &str,
              storage_type: Option<&str>,
              config: Option<&str>,
              credentials: &str,
              import_config: &str) -> Result<()> {
        debug!("import >>> pool_name: {:?}, name: {:?}, storage_type: {:?}, config: {:?}, credentials: {:?}, import_config: {:?}",
               pool_name, name, storage_type, config, credentials, import_config);

        let res = self.wallet_service.import_wallet(pool_name, name, storage_type, config, credentials, import_config)?;

        debug!("import <<< res: {:?}", res);

        Ok(res)
    }
}
