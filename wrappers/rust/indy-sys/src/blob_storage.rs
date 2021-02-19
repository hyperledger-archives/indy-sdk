use super::*;

use {CString, CommandHandle, Error};

extern "C" {

    pub fn indy_open_blob_storage_reader(
        command_handle: CommandHandle,
        type_: CString,
        config_json: CString,
        cb: Option<ResponseI32CB>,
    ) -> Error;

    pub fn indy_open_blob_storage_writer(
        command_handle: CommandHandle,
        type_: CString,
        config_json: CString,
        cb: Option<ResponseI32CB>,
    ) -> Error;
}
