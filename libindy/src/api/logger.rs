extern crate libc;
extern crate time;
extern crate log;

use self::libc::{c_void, c_char};

use api::ErrorCode;
use errors::ToErrorCode;

use utils::logger;
use utils::logger::{EnabledCB, LogCB, FlushCB, IndyLogger, IndyDefaultLogger};
use utils::cstring::CStringUtils;

/// Set custom logger implementation.
///
/// Allows library user to provide custom logger implementation as set of handlers.
///
/// #Params
/// context: logger context
/// enabled: "enabled" operation handler (false positive if not specified)
/// log: "log" operation handler
/// flush: "flush" operation handler
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_set_logger(context: *const c_void,
                              enabled: Option<EnabledCB>,
                              log: Option<LogCB>,
                              flush: Option<FlushCB>) -> ErrorCode {
    trace!("indy_set_logger >>> context: {:?}, enabled: {:?}, log: {:?}, flush: {:?}", context, enabled, log, flush);

    check_useful_c_callback!(log, ErrorCode::CommonInvalidParam3);

    let result = IndyLogger::init(context, enabled, log, flush);

    let res = result_to_err_code!(result);

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

    let result = IndyDefaultLogger::init(level);

    let res = result_to_err_code!(result);

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
pub extern fn indy_get_logger(context_p: *mut *const c_void,
                              enabled_cb_p: *mut Option<EnabledCB>,
                              log_cb_p: *mut Option<LogCB>,
                              flush_cb_p: *mut Option<FlushCB>) -> ErrorCode {
    trace!("indy_get_logger >>> context_p: {:?}, enabled_cb_p: {:?}, log_cb_p: {:?}, flush_cb_p: {:?}", context_p, enabled_cb_p, log_cb_p, flush_cb_p);

    //    let logger = get_indy_logger();

    unsafe {
        match logger::LOGGER_STATE {
            logger::LoggerState::Default => {
                *enabled_cb_p = Some(logger::IndyDefaultLogger::enabled);
                *log_cb_p = Some(logger::IndyDefaultLogger::log);
                *flush_cb_p = Some(logger::IndyDefaultLogger::flush);
            }
            logger::LoggerState::Custom => {
                *context_p = logger::CONTEXT;
                *enabled_cb_p = logger::ENABLED_CB;
                *log_cb_p = logger::LOG_CB;
                *flush_cb_p = logger::FLUSH_CB;
            }
        }
    };


    let res = ErrorCode::Success;

    trace!("indy_get_logger: <<< res: {:?}", res);

    res
}