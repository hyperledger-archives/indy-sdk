extern crate libc;

use self::libc::{c_void, c_char};

use api::ErrorCode;

extern crate time;
extern crate log;

use utils::logger::{EnabledCB, LogCB, FlushCB, init_indy_logger, init_default_logger};
use utils::cstring::CStringUtils;

#[no_mangle]
pub extern fn indy_set_logger(context: *const c_void,
                              _enabled_cb: Option<EnabledCB>,
                              log_cb: Option<LogCB>,
                              flush_cb: Option<FlushCB>) -> ErrorCode {
    trace!("indy_set_logger >>> context: {:?}, log_cb: {:?}, flush_cb: {:?}", context, log_cb, flush_cb);

    check_useful_c_callback!(log_cb, ErrorCode::CommonInvalidParam3);

    let res = match init_indy_logger(context, log_cb, flush_cb) {
        Ok(()) => ErrorCode::Success,
        Err(_) => ErrorCode::CommonInvalidState
    };

    trace!("indy_set_logger: <<< res: {:?}", res);

    res
}

#[no_mangle]
pub extern fn indy_set_default_logger(level: *const c_char) -> ErrorCode {
    trace!("indy_set_default_logger >>>");

    check_useful_opt_c_str!(level, ErrorCode::CommonInvalidParam1);

    let res = match init_default_logger(level) {
        Ok(()) => ErrorCode::Success,
        Err(_) => ErrorCode::CommonInvalidState
    };

    trace!("indy_set_default_logger: <<< res: {:?}", res);

    res
}