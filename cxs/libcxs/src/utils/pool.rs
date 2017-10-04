
use indy::api::pool::indy_create_pool_ledger_config;
use std::ffi::CString;
use utils::generate_command_handle;
use utils::init::indy_error_to_cxs_error_code;
use indy::api::ErrorCode;
use error;


pub fn create_pool_config<'a>(pool1:&str, config_name:&str)-> &'a error::Error {
    let pool_name = pool1;
    let config_name = config_name;
    let c_pool_name = CString::new(pool_name).unwrap();
    let c_config_name = CString::new(config_name).unwrap();
    let command_handle: i32 = generate_command_handle();

    // currently we have no call backs
    extern "C" fn f(_handle: i32, _err: ErrorCode) { }

    let indy_err = indy_create_pool_ledger_config(command_handle,
                                    c_pool_name.as_ptr(),
                                    c_config_name.as_ptr(),
                                    Some(f));

    indy_error_to_cxs_error_code(indy_err)

}





#[cfg(test)]
mod tests {
    use super::*;
    use error::SUCCESS;

    #[test]
    fn test_config() {
        let pool_name = "pool1";
        let config_name = "config1";
        assert_eq!(&SUCCESS.code_num, &create_pool_config(&pool_name, &config_name).code_num);
    }


}