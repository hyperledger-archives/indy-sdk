use super::*;

use native::{BString, CString, Error, Handle};

extern {
    #[no_mangle]
    pub fn indy_register_wallet_storage(command_handle: Handle,
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
    #[no_mangle]
    pub fn indy_create_wallet(command_handle: Handle,
                              config: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_open_wallet(command_handle: Handle,
                            config: CString,
                            credentials: CString,
                            cb: Option<ResponseI32CB>) -> Error;
    #[no_mangle]
    pub fn indy_export_wallet(command_handle: Handle,
                              wallet_handle: Handle,
                              export_config: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_import_wallet(command_handle: Handle,
                              config: CString,
                              credentials: CString,
                              import_config: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_close_wallet(command_handle: Handle,
                             wallet_handle: Handle,
                             cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_delete_wallet(command_handle: Handle,
                              config: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;
}

pub type WalletCreate = extern fn(name: CString,
                                  config: CString,
                                  credentials_json: CString,
                                  metadata: CString) -> Error;
pub type WalletOpen = extern fn(name: CString,
                                config: CString,
                                credentials_json: CString,
                                storage_handle_p: *mut Handle) -> Error;
pub type WalletClose = extern fn(storage_handle: Handle) -> Error;
pub type WalletDelete = extern fn(name: CString,
                                  config: CString,
                                  credentials_json: CString) -> Error;
pub type WalletAddRecord = extern fn(storage_handle: Handle,
                                     type_: CString,
                                     id: CString,
                                     value: BString,
                                     value_len: usize,
                                     tags_json: CString) -> Error;
pub type WalletUpdateRecordValue = extern fn(storage_handle: Handle,
                                             type_: CString,
                                             id: CString,
                                             value: BString,
                                             value_len: usize, ) -> Error;
pub type WalletUpdateRecordTags = extern fn(storage_handle: Handle,
                                            type_: CString,
                                            id: CString,
                                            tags_json: CString) -> Error;
pub type WalletAddRecordTags = extern fn(storage_handle: Handle,
                                         type_: CString,
                                         id: CString,
                                         tags_json: CString) -> Error;
pub type WalletDeleteRecordTags = extern fn(storage_handle: Handle,
                                            type_: CString,
                                            id: CString,
                                            tag_names_json: CString) -> Error;
pub type WalletDeleteRecord = extern fn(storage_handle: Handle,
                                        type_: CString,
                                        id: CString) -> Error;
pub type WalletGetRecord = extern fn(storage_handle: Handle,
                                     type_: CString,
                                     id: CString,
                                     options_json: CString,
                                     record_handle_p: *mut Handle) -> Error;
pub type WalletGetRecordId = extern fn(storage_handle: Handle,
                                       record_handle: Handle,
                                       record_id_p: *mut CString) -> Error;
pub type WalletGetRecordType = extern fn(storage_handle: Handle,
                                         record_handle: Handle,
                                         record_type_p: *mut CString) -> Error;
pub type WalletGetRecordValue = extern fn(storage_handle: Handle,
                                          record_handle: Handle,
                                          record_value_p: *mut BString,
                                          record_value_len_p: *mut usize) -> Error;
pub type WalletGetRecordTags = extern fn(storage_handle: Handle,
                                         record_handle: Handle,
                                         record_tags_p: *mut CString) -> Error;
pub type WalletFreeRecord = extern fn(storage_handle: Handle,
                                      record_handle: Handle) -> Error;
pub type WalletGetStorageMetadata = extern fn(storage_handle: Handle,
                                              metadata_p: *mut CString,
                                              metadata_handle: *mut Handle) -> Error;
pub type WalletSetStorageMetadata = extern fn(storage_handle: Handle,
                                              metadata_p: CString) -> Error;
pub type WalletFreeStorageMetadata = extern fn(storage_handle: Handle,
                                               metadata_handle: Handle) -> Error;
pub type WalletSearchRecords = extern fn(storage_handle: Handle,
                                         type_: CString,
                                         query_json: CString,
                                         options_json: CString,
                                         search_handle_p: *mut Handle) -> Error;
pub type WalletSearchAllRecords = extern fn(storage_handle: Handle,
                                            search_handle_p: *mut Handle) -> Error;
pub type WalletGetSearchTotalCount = extern fn(storage_handle: Handle,
                                               search_handle: Handle,
                                               total_count_p: *mut usize) -> Error;
pub type WalletFetchSearchNextRecord = extern fn(storage_handle: Handle,
                                                 search_handle: Handle,
                                                 record_handle_p: *mut Handle) -> Error;
pub type WalletFreeSearch = extern fn(storage_handle: Handle,
                                      search_handle: Handle) -> Error;
