extern crate libc;
use utils::logger::{ EnabledCB, FlushCB, LibvcxLogger, LibvcxDefaultLogger, LogCB, LOGGER_STATE };
use utils::cstring::CStringUtils;
use self::libc::{c_char, c_void};

use utils::error::{ INVALID_CONFIGURATION, SUCCESS };
#[no_mangle]
pub extern fn vcx_set_default_logger(pattern: *const c_char) -> u32 {
    check_useful_opt_c_str!(pattern, INVALID_CONFIGURATION.code_num);
    let res = LibvcxDefaultLogger::init(pattern);
    match res {
        Ok(_) => {
            debug!("Logger Successfully Initialized");
            SUCCESS.code_num
        },
        Err(ec) => {
            error!("Logger Failed To Initialize: {}", ec);
            ec
        },
    }
}

#[no_mangle]
pub extern fn vcx_set_logger(context: *const c_void,
                             enabled: Option<EnabledCB>,
                             log: Option<LogCB>,
                             flush: Option<FlushCB>) -> u32 {
    trace!("vcx_set_logger( context: {:?}, enabled: {:?}, log: {:?}, flush: {:?}",
           context, enabled, log, flush);
    check_useful_c_callback!(log, SUCCESS.code_num);
    let res = LibvcxLogger::init(context, enabled, log, flush);
    match res {
        Ok(_) => {
            debug!("Logger Successfully Initialized");
            SUCCESS.code_num
        },
        Err(ec) => {
            error!("Logger Failed To Initialize: {}", ec);
            ec
        },
    }
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
///
/// This is tested in wrapper tests (python3)
#[no_mangle]
pub extern fn vcx_get_logger(context_p: *mut *const c_void,
                              enabled_cb_p: *mut Option<EnabledCB>,
                              log_cb_p: *mut Option<LogCB>,
                              flush_cb_p: *mut Option<FlushCB>) -> u32 {
    info!("vcx_get_logger >>> context_p: {:?}, enabled_cb_p: {:?}, log_cb_p: {:?}, flush_cb_p: {:?}", context_p, enabled_cb_p, log_cb_p, flush_cb_p);

    unsafe {
        let (context, enabled_cb, log_cb, flush_cb) = LOGGER_STATE.get();

        *context_p = context;
        *enabled_cb_p = enabled_cb;
        *log_cb_p = log_cb;
        *flush_cb_p = flush_cb;
    }

    let res = SUCCESS.code_num;

    info!("vcx_get_logger: <<< res: {:?}", res);

    res
}



