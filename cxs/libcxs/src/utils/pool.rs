extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use utils::cstring::CStringUtils;
use utils::generate_command_handle;
use utils::init::indy_error_to_cxs_error_code;

extern {
    fn indy_create_pool_ledger_config(command_handle: i32,
                                             config_name: *const c_char,
                                             config: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_delete_pool_ledger_config(command_handle: i32,
                                             config_name: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;
}

pub fn create_pool_config<'a>(pool1:&str, config_name:&str)-> u32 {
    let pool_name = pool1;
    let config_name = config_name;
    let c_pool_name = CString::new(pool_name).unwrap();
    let c_config_name = CString::new(config_name).unwrap();
    let command_handle: i32 = generate_command_handle();

    // currently we have no call backs
    extern "C" fn f(_handle: i32, _err: i32) { }

    unsafe {
        let indy_err = indy_create_pool_ledger_config(command_handle,
                                                      c_pool_name.as_ptr(),
                                                      c_config_name.as_ptr(),
                                                      Some(f));

        info!("indy_create_pool_ledger_config returned {}", indy_err);

        indy_error_to_cxs_error_code(indy_err)
    }
}

pub fn delete_pool_config(config: &str) {
    let command_handle: i32 = generate_command_handle();
    extern "C" fn f(_handle: i32, _err: i32) { }
    unsafe {
        let indy_err = indy_delete_pool_ledger_config(command_handle,
                                                     CStringUtils::string_to_cstring(config.to_string()).as_ptr(),
                                                     Some(f));
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use utils::error::SUCCESS;

    #[test]
    fn test_config() {
        let pool_name = String::from("pool1");
        let config_name = String::from("config1");
        assert_eq!(SUCCESS.code_num, create_pool_config(&pool_name, &config_name));
        delete_pool_config("config1");
    }


}