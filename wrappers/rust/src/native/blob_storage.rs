use super::*;

use native::{Error, Handle, CString};

extern {
    #[no_mangle]
    pub fn indy_open_blob_storage_reader(command_handle: Handle,
                                         type_: CString,
                                         config_json: CString,
                                         cb: Option<ResponseI32CB>) -> Error;

    #[no_mangle]
    pub fn indy_open_blob_storage_writer(command_handle: Handle,
                                         type_: CString,
                                         config_json: CString,
                                         cb: Option<ResponseI32CB>) -> Error;
}
