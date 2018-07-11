extern crate libc;
extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
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
    Create(String, // config
           String, // credentials
           Box<Fn(Result<()>) + Send>),
    Open(String, // config
         String, // credentials
         Box<Fn(Result<i32>) + Send>),
    Close(i32, // handle
          Box<Fn(Result<()>) + Send>),
    Delete(String, // config
           String, // credentials
           Box<Fn(Result<()>) + Send>),
    Export(i32, // wallet_handle
           String, // export config
           Box<Fn(Result<()>) + Send>),
    Import(String, // config
           String, // credentials
           String, // import config
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
                debug!(target: "wallet_command_executor", "RegisterWalletType command received");
                cb(self._register_type(&type_, create, open, close, delete, add_record,
                                       update_record_value, update_record_tags, add_record_tags,
                                       delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                                       get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                                       free_storage_metadata, search_records, search_all_records, get_search_total_count,
                                       fetch_search_next_record, free_search));
            }
            WalletCommand::Create(config, credentials, cb) => {
                debug!(target: "wallet_command_executor", "Create command received");
                cb(self._create(&config, &credentials));
            }
            WalletCommand::Open(config, credentials, cb) => {
                debug!(target: "wallet_command_executor", "Open command received");
                cb(self._open(&config, &credentials));
            }
            WalletCommand::Close(handle, cb) => {
                debug!(target: "wallet_command_executor", "Close command received");
                cb(self._close(handle));
            }
            WalletCommand::Delete(config, credentials, cb) => {
                debug!(target: "wallet_command_executor", "Delete command received");
                cb(self._delete(&config, &credentials));
            }
            WalletCommand::Export(wallet_handle, export_config, cb) => {
                debug!(target: "wallet_command_executor", "Export command received");
                cb(self._export(wallet_handle, &export_config));
            }
            WalletCommand::Import(config, credentials, import_config, cb) => {
                debug!(target: "wallet_command_executor", "Import command received");
                cb(self._import(&config, &credentials, &import_config));
            }
        };
    }

    fn _register_type(&self,
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
        trace!("_register_type >>> type_: {:?}", type_);

        let res = self
            .wallet_service
            .register_wallet_storage(
                type_, create, open, close, delete, add_record, update_record_value, update_record_tags,
                add_record_tags, delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                free_storage_metadata, search_records, search_all_records,
                get_search_total_count, fetch_search_next_record, free_search)?;

        trace!("_register_type <<< res: {:?}", res);
        Ok(res)
    }

    fn _create(&self,
               config: &str,
               credentials: &str) -> Result<()> {
        trace!("_create >>> config: {:?}, credentials: {:?}", config, "_"); // TODO: FIXME: Log secrets in debug

        let res = self.wallet_service.create_wallet(config, credentials)?;

        trace!("_create <<< res: {:?}", res);
        Ok(res)
    }

    fn _open(&self,
             config: &str,
             credentials: &str) -> Result<i32> {
        trace!("_open >>> config: {:?}, credentials: {:?}", config, "_"); // TODO: FIXME: Log secrets in debug

        let res = self.wallet_service.open_wallet(config, credentials)?;

        trace!("_open <<< res: {:?}", res);
        Ok(res)
    }

    fn _close(&self,
              handle: i32) -> Result<()> {
        trace!("_close >>> handle: {:?}", handle);

        let res = self.wallet_service.close_wallet(handle)?;

        trace!("_close <<< res: {:?}", res);
        Ok(res)
    }

    fn _delete(&self,
               config: &str,
               credentials: &str) -> Result<()> {
        trace!("_delete >>> config: {:?}, credentials: {:?}", config, "_"); // TODO: FIXME: Log secrets in debug

        let res = self.wallet_service.delete_wallet(config, credentials)?;

        trace!("_delete <<< res: {:?}", res);
        Ok(res)
    }

    fn _export(&self,
               wallet_handle: i32,
               export_config: &str) -> Result<()> {
        trace!("_export >>> handle: {:?}, export_config: {:?}", wallet_handle, export_config);

        // TODO - later add proper versioning
        let res = self.wallet_service.export_wallet(wallet_handle, export_config, 0)?;

        trace!("_export <<< res: {:?}", res);
        Ok(res)
    }

    fn _import(&self,
               config: &str,
               credentials: &str,
               import_config: &str) -> Result<()> {
        trace!("_import >>> config: {:?}, credentials: {:?}, import_config: {:?}",
               config, "_", import_config); // TODO: FIXME: Log credentials in debug

        let res = self.wallet_service.import_wallet(config, credentials, import_config)?;

        trace!("_import <<< res: {:?}", res);
        Ok(res)
    }
}
