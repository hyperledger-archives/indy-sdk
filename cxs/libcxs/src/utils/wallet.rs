extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use connection;
use settings;
use utils::error;
use utils::cstring::CStringUtils;
use std::ptr::null;
use utils::generate_command_handle;
use utils::init::indy_error_to_cxs_error_code;
use api::CxsStateType;
use std::thread;
use rand::{thread_rng, Rng};
use std::time::Duration;

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

    fn indy_create_and_store_my_did(command_handle: i32,
                                    wallet_handle: i32,
                                    did_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                         did: *const c_char,
                                                         verkey: *const c_char,
                                                         pk: *const c_char)>) -> i32;
}


extern "C" fn open_wallet_callback(_handle: i32, err: i32, handle: i32) {
    info!("open_wallet_callback handle: {} error code: {}", handle, err);

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
    if settings::test_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return error::SUCCESS.code_num;
    }

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

pub fn create_and_store_my_did(handle: u32, did_json: &str) -> Result<u32, String> {

    if settings::test_mode_enabled() {
        //TEST MODE: sleep for a few milliseconds and fire off the callback with random data
        let my_handle = handle.clone();
        warn!("using test mode create_and_store_my_did with handle {}",my_handle);
        thread::spawn(move|| {
            thread::sleep(Duration::from_millis(200));
            let did: String = "8XFh8yBzrpJQmNyZzgoTqB".to_string();
            let verkey: String = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string();
            let pk: String = thread_rng().gen_ascii_chars().take(32).collect();
            store_new_did_info_cb(my_handle as i32, 0, CString::new(did).unwrap().as_ptr(), CString::new(verkey).unwrap().as_ptr(), CString::new(pk).unwrap().as_ptr());
        });

        return Ok(0);
    }

    let wallet_handle = get_wallet_handle();

    info!("creating and storing a new DID with handle {} and wallet {}",handle,wallet_handle);
    unsafe {
        match indy_create_and_store_my_did(handle as i32, wallet_handle, CString::new(did_json).unwrap().as_ptr(), Some(store_new_did_info_cb)) {
            0 => return Ok(0),
            _ => return Err("libindy returned error".to_owned()),
        }
    }
}


extern "C" fn store_new_did_info_cb(handle: i32,
                                    err: i32,
                                    did: *const c_char,
                                    verkey: *const c_char,
                                    pk: *const c_char) {
    check_useful_c_str!(did, ());
    check_useful_c_str!(verkey, ());
    check_useful_c_str!(pk, ());
    info!("handle: {} err: {} did: {} verkey: {} pk: {}", handle as u32, err, did, verkey, pk);
    connection::set_pw_did(handle as u32, &did);
    connection::set_pw_verkey(handle as u32, &verkey);

    match connection::create_agent_pairwise(handle as u32) {
        Err(_) => error!("could not create pairwise key on agent"),
        Ok(_) => info!("created pairwise key on agent"),
    };

    match connection::update_agent_profile(handle as u32) {
        Err(_) => error!("could not update profile on agent"),
        Ok(_) => info!("updated profile on agent"),
    };

    connection::set_state(handle as u32, CxsStateType::CxsStateInitialized);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::error;
    use std::thread;

    //TODO: make boilerplate test code use same wallet?

    pub fn make_wallet(wallet_name: &str) {
        let pool_name = String::from("pool1");
        let wallet_type = String::from("default");
        assert_eq!(error::SUCCESS.code_num, init_wallet(&pool_name, &wallet_name, &wallet_type));
        thread::sleep(Duration::from_secs(1));
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
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_wallet() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
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

    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called create_cb")
    }

    #[test]
    fn test_cb_adds_verkey() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = connection::build_connection("test_cb_adds_verkey".to_owned());
        thread::sleep(Duration::from_secs(1));
        assert!(!connection::get_pw_verkey(handle).unwrap().is_empty());
    }

}
