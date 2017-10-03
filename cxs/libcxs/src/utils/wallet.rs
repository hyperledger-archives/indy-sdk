
use indy::api::wallet::indy_create_wallet;

use std::ffi::CString;
use std::ptr::null;
use utils::generate_command_handle;
use utils::init::indy_to_cxs_error_code;
use api::Errorcode;
use indy::api::ErrorCode;


pub fn create_wallet(pool_name:&str, wallet_name:&str, wallet_type: &str) -> Errorcode {
    let handle = generate_command_handle();
    let c_pool_name = CString::new(pool_name).unwrap();
    let pool_name_ptr = c_pool_name.as_ptr();
    let c_listener_wallet_name = CString::new(wallet_name).unwrap().as_ptr();
    let c_wallet_type = CString::new(wallet_type).unwrap().as_ptr();

    // currently we have no call backs
    extern "C" fn dummy_callback(_handle: i32, _err: ErrorCode) { }

    indy_to_cxs_error_code(indy_create_wallet(handle, pool_name_ptr,
                                 c_listener_wallet_name,
                                 c_wallet_type,
                                 null(),
                                 null(),
                                 Some(dummy_callback)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        let pool_name = "pool1";
        let wallet_name = "wallet1";
        let wallet_type = "default";
        assert_eq!(Errorcode::Success, create_wallet(&pool_name, &wallet_name, &wallet_type ))
    }
}
