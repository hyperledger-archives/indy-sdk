use indy_sys::logger::indy_set_default_logger;

pub fn set_default_indy_logger() {
    unsafe { indy_set_default_logger(::std::ptr::null()); }
}