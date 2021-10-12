use super::*;

use {CString, Error, CommandHandle, WalletHandle};

extern {

    pub fn indy_add_wallet_record(command_handle: CommandHandle,
                                  wallet_handle: WalletHandle,
                                  type_: CString,
                                  id: CString,
                                  value: CString,
                                  tags_json: CString,
                                  cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_update_wallet_record_value(command_handle: CommandHandle,
                                           wallet_handle: WalletHandle,
                                           type_: CString,
                                           id: CString,
                                           value: CString,
                                           cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_update_wallet_record_tags(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          type_: CString,
                                          id: CString,
                                          tags_json: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_add_wallet_record_tags(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       type_: CString,
                                       id: CString,
                                       tags_json: CString,
                                       cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_delete_wallet_record_tags(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          type_: CString,
                                          id: CString,
                                          tag_names_json: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_delete_wallet_record(command_handle: CommandHandle,
                                     wallet_handle: WalletHandle,
                                     type_: CString,
                                     id: CString,
                                     cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_get_wallet_record(command_handle: CommandHandle,
                                  wallet_handle: WalletHandle,
                                  type_: CString,
                                  id: CString,
                                  options_json: CString,
                                  cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_open_wallet_search(command_handle: CommandHandle,
                                   wallet_handle: WalletHandle,
                                   type_: CString,
                                   query_json: CString,
                                   options_json: CString,
                                   cb: Option<ResponseI32CB>) -> Error;

    pub fn indy_fetch_wallet_search_next_records(command_handle: CommandHandle,
                                                 wallet_handle: WalletHandle,
                                                 wallet_search_handle: SearchHandle,
                                                 count: usize,
                                                 cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_close_wallet_search(command_handle: CommandHandle,
                                    wallet_search_handle: SearchHandle,
                                    cb: Option<ResponseEmptyCB>) -> Error;
}
