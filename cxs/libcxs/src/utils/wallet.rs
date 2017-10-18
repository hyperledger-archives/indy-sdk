extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use std::ptr::null;
use utils::generate_command_handle;
use utils::init::indy_error_to_cxs_error_code;

pub static mut WALLET_HANDLE: i32 = 0;

extern {
    fn indy_create_wallet(command_handle: i32,
                                 pool_name: *const c_char,
                                 name: *const c_char,
                                 xtype: *const c_char,
                                 config: *const c_char,
                                 credentials: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_open_wallet(command_handle: i32,
                               name: *const c_char,
                               runtime_config: *const c_char,
                               credentials: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: i32, handle: i32)>) -> i32;

    fn indy_close_wallet(command_handle: i32,
                                handle: i32,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_delete_wallet(command_handle: i32,
                                 name: *const c_char,
                                 credentials: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;
}


extern "C" fn open_wallet_callback(_handle: i32, err: i32, handle: i32) {
    info!("libindy returned err: {} handle: {}", err, handle);

    if err == 0 {
        unsafe {
            WALLET_HANDLE = handle;
        }
    }
}

pub fn get_wallet_handle() -> i32 {
    unsafe {
        WALLET_HANDLE
    }
}

pub fn init_wallet<'a>(pool_name:&str, wallet_name:&str, wallet_type:&str) -> u32 {
    let handle = generate_command_handle();

    // FIXME: we don't care that "create" fails
    extern "C" fn dummy_callback(_handle: i32, _err: i32) { }

    unsafe {
        let indy_err = indy_create_wallet(handle,
                                          CStringUtils::string_to_cstring(pool_name.to_string()).as_ptr(),
                                          CStringUtils::string_to_cstring(wallet_name.to_string()).as_ptr(),
                                          CStringUtils::string_to_cstring(wallet_type.to_string()).as_ptr(),
                                          null(),
                                          null(),
                                          Some(dummy_callback));

        info!("indy_create_wallet returned {}", indy_err);

        // ignore 112 - wallet already exists
        if indy_err != 112 && indy_err != 0 {
            return indy_error_to_cxs_error_code(indy_err);
        }
    }

    unsafe {
        let indy_err = indy_open_wallet(handle,
                                        CStringUtils::string_to_cstring(wallet_name.to_string()).as_ptr(),
                                        null(),
                                        null(),
                                        Some(open_wallet_callback));

        info!("indy_open_wallet returned {}", indy_err);

        indy_error_to_cxs_error_code(indy_err)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::error;
    use std::thread;
    use std::time::Duration;

    //TODO: make boilerplate test code use same wallet?

    pub fn make_wallet(wallet_name: &str) {
        let pool_name = String::from("pool1");
        let wallet_type = String::from("default");
        assert_eq!(error::SUCCESS.code_num, init_wallet(&pool_name, &wallet_name, &wallet_type));
        thread::sleep(Duration::from_secs(2));
    }

    pub fn delete_wallet(wallet_name: &str) {
        let handle = generate_command_handle();
        extern "C" fn dummy_callback(_handle: i32, _err: i32) { }

        let wallet_handle = get_wallet_handle();

        unsafe {
            let indy_err = indy_close_wallet(handle,
                                             wallet_handle,
                                             Some(dummy_callback));
        }

        unsafe {
           let indy_err = indy_delete_wallet(handle,
                                             CStringUtils::string_to_cstring(wallet_name.to_string()).as_ptr(),
                                             null(),
                                             Some(dummy_callback));
        }
        thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn test_wallet() {
        let pool_name = String::from("pool1");
        let wallet_name = String::from("wallet1");
        let wallet_type = String::from("default");
        assert_eq!(error::SUCCESS.code_num, init_wallet(&pool_name, &wallet_name, &wallet_type));
        assert_eq!(error::UNKNOWN_ERROR.code_num, init_wallet(&String::from(""),&wallet_name, &wallet_type));

        thread::sleep(Duration::from_secs(1));
        delete_wallet("wallet1");
        let handle = get_wallet_handle();
        let wallet_name2 = String::from("wallet2");
        assert_eq!(error::SUCCESS.code_num, init_wallet(&pool_name, &wallet_name2, &wallet_type));

        thread::sleep(Duration::from_secs(1));
        assert_ne!(handle, get_wallet_handle());
        delete_wallet("wallet2");
    }
}
