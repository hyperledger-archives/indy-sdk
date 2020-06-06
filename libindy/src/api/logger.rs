use libc::{c_void, c_char};

use indy_api_types::ErrorCode;
use indy_api_types::errors::prelude::*;

use crate::utils::logger::{EnabledCB, LogCB, FlushCB, LibindyLogger, LibindyDefaultLogger, LOGGER_STATE};
use indy_utils::ctypes;
use log::LevelFilter;

/// Set custom logger implementation.
///
/// Allows library user to provide custom logger implementation as set of handlers.
///
/// #Params
/// context: pointer to some logger context that will be available in logger handlers.
/// enabled: (optional) "enabled" operation handler - calls to determines if a log record would be logged. (false positive if not specified)
/// log: "log" operation handler - calls to logs a record.
/// flush: (optional) "flush" operation handler - calls to flushes buffered records (in case of crash or signal).
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

    let result = LibindyLogger::init(context, enabled, log, flush, None);

    let res = prepare_result!(result);

    trace!("indy_set_logger: <<< res: {:?}", res);

    res
}


/// Set custom logger implementation.
///
/// Allows library user to provide custom logger implementation as set of handlers.
///
/// # Arguments
/// * `context` - pointer to some logger context that will be available in logger handlers.
/// * `enabled` - (optional) "enabled" operation handler - calls to determines if a log record would be logged. (false positive if not specified)
/// * `log` - "log" operation handler - calls to logs a record.
/// * `flush` - (optional) "flush" operation handler - calls to flushes buffered records (in case of crash or signal).
/// * `max_lvl` - Maximum log level represented as u32.
/// Possible values are from 0 to 5 inclusive: 0 - Off, 1 - Error, 2 - Warn, 3 - Trace, 4 - Debug, 5 - Trace
///
/// # Returns
/// On success returns `ErrorCode::Success`
/// ErrorCode::CommonInvalidParam3 is returned in case of `log` callback is missed
/// ErrorCode::CommonInvalidParam5 is returned in case of `max_lvl` value is out of range [0-5]
#[no_mangle]
pub extern fn indy_set_logger_with_max_lvl(context: *const c_void,
                                           enabled: Option<EnabledCB>,
                                           log: Option<LogCB>,
                                           flush: Option<FlushCB>,
                                           max_lvl: u32) -> ErrorCode {
    trace!("indy_set_logger >>> context: {:?}, enabled: {:?}, log: {:?}, flush: {:?}, max lvl {}", context, enabled, log, flush, max_lvl);

    check_useful_c_callback!(log, ErrorCode::CommonInvalidParam3);
    check_u32_less_or_eq!(max_lvl, LevelFilter::max() as usize as u32, ErrorCode::CommonInvalidParam5);

    let result = LibindyLogger::init(context, enabled, log, flush, Some(max_lvl));

    let res = prepare_result!(result);

    trace!("indy_set_logger: <<< res: {:?}", res);

    res
}

///
/// Set maximum log level
///
/// # Arguments
/// * `max_lvl` - Maximum log level represented as u32.
/// Possible values are from 0 to 5 inclusive: 0 - Off, 1 - Error, 2 - Warn, 3 - Trace, 4 - Debug, 5 - Trace
///
/// # Return
/// On success returns `ErrorCode::Success`
/// ErrorCode::CommonInvalidParam1 is returned in case of `max_lvl` value is out of range [0-5]
#[no_mangle]
pub extern fn indy_set_log_max_lvl(max_lvl: u32) -> ErrorCode {
    trace!("indy_set_log_max_lvl >>> max_lvl: {}", max_lvl);

    check_u32_less_or_eq!(max_lvl, LevelFilter::max() as usize as u32, ErrorCode::CommonInvalidParam1);

    let result = LibindyLogger::set_max_level(max_lvl);

    let res = prepare_result!(result);

    trace!("indy_set_log_max_lvl: <<< res: {:?}", res);

    res
}


/// Set default logger implementation.
///
/// Allows library user use `env_logger` logger as default implementation.
/// More details about `env_logger` and its customization can be found here: https://crates.io/crates/env_logger
///
/// #Params
/// pattern: (optional) pattern that corresponds with the log messages to show.
///
/// NOTE: You should specify either `pattern` parameter or `RUST_LOG` environment variable to init logger.
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_set_default_logger(pattern: *const c_char) -> ErrorCode {
    trace!("indy_set_default_logger >>> pattern: {:?}", pattern);

    check_useful_opt_c_str!(pattern, ErrorCode::CommonInvalidParam1);

    trace!("indy_set_default_logger: entities >>> pattern: {:?}", pattern);

    let result = LibindyDefaultLogger::init(pattern);

    let res = prepare_result!(result);

    trace!("indy_set_default_logger: <<< res: {:?}", res);

    res
}

/// Get the currently used logger.
///
/// NOTE: if logger is not set dummy implementation would be returned.
///
/// #Params
/// `context_p` - Reference that will contain logger context.
/// `enabled_cb_p` - Reference that will contain pointer to enable operation handler.
/// `log_cb_p` - Reference that will contain pointer to log operation handler.
/// `flush_cb_p` - Reference that will contain pointer to flush operation handler.
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_get_logger(context_p: *mut *const c_void,
                              enabled_cb_p: *mut Option<EnabledCB>,
                              log_cb_p: *mut Option<LogCB>,
                              flush_cb_p: *mut Option<FlushCB>) -> ErrorCode {
    trace!("indy_get_logger >>> context_p: {:?}, enabled_cb_p: {:?}, log_cb_p: {:?}, flush_cb_p: {:?}", context_p, enabled_cb_p, log_cb_p, flush_cb_p);

    unsafe {
        let (context, enabled_cb, log_cb, flush_cb) = LOGGER_STATE.get();

        *context_p = context;
        *enabled_cb_p = enabled_cb;
        *log_cb_p = log_cb;
        *flush_cb_p = flush_cb;
    }

    let res = ErrorCode::Success;

    trace!("indy_get_logger: <<< res: {:?}", res);

    res
}