extern crate libc;

use self::libc::c_char;

use api::ErrorCode;

pub type WalletCreate = extern fn(name: *const c_char,
                                  config: *const c_char,
                                  credentials: *const c_char) -> ErrorCode;

pub type WalletOpen = extern fn(name: *const c_char,
                                config: *const c_char,
                                runtime_config: *const c_char,
                                credentials: *const c_char,
                                storage_handle_p: *mut i32) -> ErrorCode;

pub type WalletClose = extern fn(handle: i32) -> ErrorCode;

pub type WalletDelete = extern fn(name: *const c_char,
                                  config: *const c_char,
                                  credentials: *const c_char) -> ErrorCode;

pub type WalletAddRecord = extern fn(storage_handle: i32,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     value: *const u8,
                                     value_len: usize,
                                     tags_json: *const c_char) -> ErrorCode;

pub type WalletUpdateRecordValue = extern fn(storage_handle: i32,
                                             type_: *const c_char,
                                             id: *const c_char,
                                             value: *const u8,
                                             value_len: usize,) -> ErrorCode;

pub type WalletUpdateRecordTags = extern fn(storage_handle: i32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            tags_json: *const c_char) -> ErrorCode;

pub type WalletAddRecordTags = extern fn(storage_handle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tags_json: *const c_char) -> ErrorCode;

pub type WalletDeleteRecordTags = extern fn(storage_handle: i32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            tag_names_json: *const c_char) -> ErrorCode;

pub type WalletDeleteRecord = extern fn(storage_handle: i32,
                                        type_: *const c_char,
                                        id: *const c_char) -> ErrorCode;

pub type WalletGetRecord = extern fn(storage_handle: i32,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     options_json: *const c_char,
                                     record_handle_p: *mut u32) -> ErrorCode;

pub type WalletGetRecordId = extern fn(storage_handle: i32,
                                       record_handle: u32,
                                       record_id_p: *mut *const c_char) -> ErrorCode;

pub type WalletGetRecordValue = extern fn(storage_handle: i32,
                                          record_handle: u32,
                                          record_value_p: *mut *const u8,
                                          record_value_len_p: *mut usize) -> ErrorCode;

pub type WalletGetRecordTags = extern fn(storage_handle: i32,
                                         record_handle: u32,
                                         record_tags_p: *mut *const c_char) -> ErrorCode;

pub type WalletFreeRecord = extern fn(storage_handle: i32,
                                      record_handle: u32) -> ErrorCode;

pub type WalletSearchRecords = extern fn(storage_handle: i32,
                                         type_: *const c_char,
                                         query_json: *const c_char,
                                         options_json: *const c_char,
                                         search_handle_p: *mut u32) -> ErrorCode;

pub type WalletGetSearchTotalCount = extern fn(storage_handle: i32,
                                               search_handle: u32,
                                               total_count_p: *mut u32) -> ErrorCode;

pub type WalletFetchSearchNextRecord = extern fn(storage_handle: i32,
                                                 search_handle: u32,
                                                 record_handle_p: *mut i32) -> ErrorCode;

pub type WalletFreeSearch = extern fn(storage_handle: i32,
                                      search_handle: u32) -> ErrorCode;
