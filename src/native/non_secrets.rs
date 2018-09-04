use super::*;

use native::{Error, Handle, CString};

extern {
    #[no_mangle]
    pub fn indy_add_wallet_record(command_handle: Handle,
                                  wallet_handle: Handle,
                                  xtype: CString,
                                  id: CString,
                                  value: CString,
                                  tags_json: CString,
                                  cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_update_wallet_record_value(command_handle: Handle,
                                           wallet_handle: Handle,
                                           xtype: CString,
                                           id: CString,
                                           value: CString,
                                           cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_update_wallet_record_tags(command_handle: Handle,
                                          wallet_handle: Handle,
                                          xtype: CString,
                                          id: CString,
                                          tags_json: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_add_wallet_record_tags(command_handle: Handle,
                                       wallet_handle: Handle,
                                       xtype: CString,
                                       id: CString,
                                       tags_json: CString,
                                       cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_delete_wallet_record_tags(command_handle: Handle,
                                          wallet_handle: Handle,
                                          xtype: CString,
                                          id: CString,
                                          tag_names_json: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_delete_wallet_record(command_handle: Handle,
                                     wallet_handle: Handle,
                                     xtype: CString,
                                     id: CString,
                                     cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_wallet_record(command_handle: Handle,
                                  wallet_handle: Handle,
                                  xtype: CString,
                                  id: CString,
                                  options_json: CString,
                                  cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_open_wallet_search(command_handle: Handle,
                                   wallet_handle: Handle,
                                   xtype: CString,
                                   query_json: CString,
                                   options_json: CString,
                                   cb: Option<ResponseI32CB>) -> Error;

    #[no_mangle]
    pub fn indy_fetch_wallet_search_next_records(command_handle: Handle,
                                                 wallet_handle: Handle,
                                                 wallet_search_handle: Handle,
                                                 count: usize,
                                                 cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_close_wallet_search(command_handle: Handle,
                                    wallet_search_handle: Handle,
                                    cb: Option<ResponseEmptyCB>) -> Error;
}
