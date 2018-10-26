extern crate libc;
use utils::logger::{ EnabledCB, FlushCB, LibvcxLogger, LibvcxDefaultLogger, LogCB };
use utils::cstring::CStringUtils;
use self::libc::{c_char, c_void};

use utils::error::{ INVALID_CONFIGURATION, SUCCESS };
#[no_mangle]
pub extern fn vcx_set_default_logger(pattern: *const c_char) -> u32 {
    check_useful_opt_c_str!(pattern, INVALID_CONFIGURATION.code_num);
    LibvcxDefaultLogger::init(pattern)
}

#[no_mangle]
pub extern fn vcx_set_logger(context: *const c_void,
                             enabled: Option<EnabledCB>,
                             log: Option<LogCB>,
                             flush: Option<FlushCB>) -> u32 {
    println!("vcx_set_logger");
    trace!("vcx_set_logger( context: {:?}, enabled: {:?}, log: {:?}, flush: {:?}",
           context, enabled, log, flush);
    check_useful_c_callback!(log, SUCCESS.code_num);
    LibvcxLogger::init(context, enabled, log, flush);
    println!("vcx_set_logger -> initited");
    0
}




