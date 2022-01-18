use super::*;

use {BString, CString, Error, CommandHandle, StorageHandle};

extern {

    pub fn indy_register_wallet_storage(command_handle: CommandHandle,
                                        type_: CString,
                                        create: Option<WalletCreate>,
                                        open: Option<WalletOpen>,
                                        close: Option<WalletClose>,
                                        delete: Option<WalletDelete>,
                                        add_record: Option<WalletAddRecord>,
                                        update_record_value: Option<WalletUpdateRecordValue>,
                                        update_record_tags: Option<WalletUpdateRecordTags>,
                                        add_record_tags: Option<WalletAddRecordTags>,
                                        delete_record_tags: Option<WalletDeleteRecordTags>,
                                        delete_record: Option<WalletDeleteRecord>,
                                        get_record: Option<WalletGetRecord>,
                                        get_record_id: Option<WalletGetRecordId>,
                                        get_record_type: Option<WalletGetRecordType>,
                                        get_record_value: Option<WalletGetRecordValue>,
                                        get_record_tags: Option<WalletGetRecordTags>,
                                        free_record: Option<WalletFreeRecord>,
                                        get_storage_metadata: Option<WalletGetStorageMetadata>,
                                        set_storage_metadata: Option<WalletSetStorageMetadata>,
                                        free_storage_metadata: Option<WalletFreeStorageMetadata>,
                                        search_records: Option<WalletSearchRecords>,
                                        search_all_records: Option<WalletSearchAllRecords>,
                                        get_search_total_count: Option<WalletGetSearchTotalCount>,
                                        fetch_search_next_record: Option<WalletFetchSearchNextRecord>,
                                        free_search: Option<WalletFreeSearch>,
                                        cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_create_wallet(command_handle: CommandHandle,
                              config: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_open_wallet(command_handle: CommandHandle,
                            config: CString,
                            credentials: CString,
                            cb: Option<ResponseWalletHandleCB>) -> Error;

    pub fn indy_export_wallet(command_handle: CommandHandle,
                              wallet_handle: WalletHandle,
                              export_config: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_import_wallet(command_handle: CommandHandle,
                              config: CString,
                              credentials: CString,
                              import_config: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_close_wallet(command_handle: CommandHandle,
                             wallet_handle: WalletHandle,
                             cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_delete_wallet(command_handle: CommandHandle,
                              config: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_generate_wallet_key(command_handle: CommandHandle,
                                    config: CString,
                                    cb: Option<ResponseStringCB>) -> Error;
}

pub type WalletCreate = extern fn(name: CString,
                                  config: CString,
                                  credentials_json: CString,
                                  metadata: CString) -> Error;
pub type WalletOpen = extern fn(name: CString,
                                config: CString,
                                credentials_json: CString,
                                storage_handle_p: *mut StorageHandle) -> Error;
pub type WalletClose = extern fn(storage_handle: StorageHandle) -> Error;
pub type WalletDelete = extern fn(name: CString,
                                  config: CString,
                                  credentials_json: CString) -> Error;
pub type WalletAddRecord = extern fn(storage_handle: StorageHandle,
                                     type_: CString,
                                     id: CString,
                                     value: BString,
                                     value_len: usize,
                                     tags_json: CString) -> Error;
pub type WalletUpdateRecordValue = extern fn(storage_handle: StorageHandle,
                                             type_: CString,
                                             id: CString,
                                             value: BString,
                                             value_len: usize, ) -> Error;
pub type WalletUpdateRecordTags = extern fn(storage_handle: StorageHandle,
                                            type_: CString,
                                            id: CString,
                                            tags_json: CString) -> Error;
pub type WalletAddRecordTags = extern fn(storage_handle: StorageHandle,
                                         type_: CString,
                                         id: CString,
                                         tags_json: CString) -> Error;
pub type WalletDeleteRecordTags = extern fn(storage_handle: StorageHandle,
                                            type_: CString,
                                            id: CString,
                                            tag_names_json: CString) -> Error;
pub type WalletDeleteRecord = extern fn(storage_handle: StorageHandle,
                                        type_: CString,
                                        id: CString) -> Error;
pub type WalletGetRecord = extern fn(storage_handle: StorageHandle,
                                     type_: CString,
                                     id: CString,
                                     options_json: CString,
                                     record_handle_p: *mut RecordHandle) -> Error;
pub type WalletGetRecordId = extern fn(storage_handle: StorageHandle,
                                       record_handle: RecordHandle,
                                       record_id_p: *mut CString) -> Error;
pub type WalletGetRecordType = extern fn(storage_handle: StorageHandle,
                                         record_handle: RecordHandle,
                                         record_type_p: *mut CString) -> Error;
pub type WalletGetRecordValue = extern fn(storage_handle: StorageHandle,
                                          record_handle: RecordHandle,
                                          record_value_p: *mut BString,
                                          record_value_len_p: *mut usize) -> Error;
pub type WalletGetRecordTags = extern fn(storage_handle: StorageHandle,
                                         record_handle: RecordHandle,
                                         record_tags_p: *mut CString) -> Error;
pub type WalletFreeRecord = extern fn(storage_handle: StorageHandle,
                                      record_handle: RecordHandle) -> Error;
pub type WalletGetStorageMetadata = extern fn(storage_handle: StorageHandle,
                                              metadata_p: *mut CString,
                                              metadata_handle: *mut MetadataHandle) -> Error;
pub type WalletSetStorageMetadata = extern fn(storage_handle: StorageHandle,
                                              metadata_p: CString) -> Error;
pub type WalletFreeStorageMetadata = extern fn(storage_handle: StorageHandle,
                                               metadata_handle: MetadataHandle) -> Error;
pub type WalletSearchRecords = extern fn(storage_handle: StorageHandle,
                                         type_: CString,
                                         query_json: CString,
                                         options_json: CString,
                                         search_handle_p: *mut SearchHandle) -> Error;
pub type WalletSearchAllRecords = extern fn(storage_handle: StorageHandle,
                                            search_handle_p: *mut SearchHandle) -> Error;
pub type WalletGetSearchTotalCount = extern fn(storage_handle: StorageHandle,
                                               search_handle: SearchHandle,
                                               total_count_p: *mut usize) -> Error;
pub type WalletFetchSearchNextRecord = extern fn(storage_handle: StorageHandle,
                                                 search_handle: SearchHandle,
                                                 record_handle_p: *mut RecordHandle) -> Error;
pub type WalletFreeSearch = extern fn(storage_handle: StorageHandle,
                                      search_handle: SearchHandle) -> Error;
