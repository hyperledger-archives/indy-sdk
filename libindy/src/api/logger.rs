extern crate libc;

use self::libc::{c_void, c_char};

use api::ErrorCode;

extern crate time;
extern crate log;

use utils::logger::{EnabledCB, LogCB, FlushCB, init_indy_logger, init_default_logger, get_indy_logger};
use utils::cstring::CStringUtils;

/// Set custom logger implementation.
///
/// Allows library user to provide custom logger implementation as set of handlers.
///
/// #Params
/// context: logger context
/// enabled: "enabled" operation handler
///     NOTE: it's ignored and is a false positive.
/// log: "log" operation handler
/// flush: "flush" operation handler
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_set_logger(context: *const c_void,
                              _enabled: Option<EnabledCB>,
                              log: Option<LogCB>,
                              flush: Option<FlushCB>) -> ErrorCode {
    trace!("indy_set_logger >>> context: {:?}, log: {:?}, flush: {:?}", context, log, flush);

    check_useful_c_callback!(log, ErrorCode::CommonInvalidParam3);

    let res = match init_indy_logger(context, log, flush) {
        Ok(()) => ErrorCode::Success,
        Err(_) => ErrorCode::CommonInvalidState
    };

    trace!("indy_set_logger: <<< res: {:?}", res);

    res
}

/// Set default logger implementation.
///
/// Allows library user use default "environment" logger implementation.
///
/// #Params
/// level: min level of message to log
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_set_default_logger(level: *const c_char) -> ErrorCode {
    trace!("indy_set_default_logger >>> level: {:?}", level);

    check_useful_opt_c_str!(level, ErrorCode::CommonInvalidParam1);

    trace!("indy_set_default_logger: entities >>> level: {:?}", level);

    let res = match init_default_logger(level) {
        Ok(()) => ErrorCode::Success,
        Err(_) => ErrorCode::CommonInvalidState
    };

    trace!("indy_set_default_logger: <<< res: {:?}", res);

    res
}

/// Get the currently used logger.
///
/// NOTE: if logger is not set dummy implementation would be returned.
///
/// #Params
/// `logger_p` - Reference that will contain logger pointer.
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_get_logger(logger_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_get_logger >>> logger_p: {:?}", logger_p);

    let logger = get_indy_logger();

    unsafe {
        *logger_p = Box::into_raw(Box::new(logger)) as *const c_void;
    };

    let res = ErrorCode::Success;

    trace!("indy_get_logger: <<< res: {:?}", res);

    res
}