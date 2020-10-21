use super::*;

use {CString, Error, CommandHandle, StorageHandle};

extern {
    #[no_mangle]
    pub fn indy_collect_metrics(command_handle: CommandHandle,
                                cb: Option<ResponseStringCB>) -> Error;
}
