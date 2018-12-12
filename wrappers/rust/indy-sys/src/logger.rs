use {CString, CVoid, Error};

extern {

    #[no_mangle]
    pub fn indy_set_logger(context: *const CVoid,
                           enabled: Option<EnabledCB>,
                           log: Option<LogCB>,
                           flush: Option<FlushCB>) -> Error;

    #[no_mangle]
    pub fn indy_set_default_logger(pattern: CString) -> Error;

    #[no_mangle]
    pub fn indy_get_logger(context_p: *mut CVoid,
                           enabled_cb_p: *mut Option<EnabledCB>,
                           log_cb_p: *mut Option<LogCB>,
                           flush_cb_p: *mut Option<FlushCB>) -> Error;
}

pub type EnabledCB = extern fn(context: *const CVoid,
                               level: u32,
                               target: CString) -> bool;

pub type LogCB = extern fn(context: *const CVoid,
                           level: u32,
                           target: CString,
                           message: CString,
                           module_path: CString,
                           file: CString,
                           line: u32);

pub type FlushCB = extern fn(context: *const CVoid);

