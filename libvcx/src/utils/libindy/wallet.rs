extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use settings;
use std::ptr::null;
use utils::libindy::{indy_function_eval};
use utils::libindy::return_types::{ Return_I32, Return_I32_I32, receive};
use utils::libindy::error_codes::{map_indy_error_code, map_string_error};
use utils::timeout::TimeoutUtils;
use utils::error;

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

    fn indy_store_their_did(command_handle: i32,
                            wallet_handle: i32,
                            identity_json: *const c_char,
                            cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;
}

pub fn get_wallet_handle() -> i32 { unsafe { WALLET_HANDLE } }

pub fn open_wallet(wallet_name: &str) -> Result<i32, u32> {
    if settings::test_indy_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return Ok(1);
    }

    let mut use_key = false;
    let credentials = match settings::get_wallet_credentials() {
        Some(x) => {debug!("using key for indy wallet"); use_key = true; CString::new(x).map_err(map_string_error)? },
        None => CString::new("").map_err(map_string_error)?,
    };

    let open_obj = Return_I32_I32::new()?;
    let wallet_name = CString::new(wallet_name).map_err(map_string_error)?;

    unsafe {
         // Open Wallet
        let err = indy_open_wallet(open_obj.command_handle,
                               wallet_name.as_ptr(),
                               null(),
                               if use_key { credentials.as_ptr() } else { null() },
                               Some(open_obj.get_callback()));

        if err != 206 && err != 0 {
            warn!("libindy open wallet returned: {}", err);
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
        }

        let wallet_handle = match receive(&open_obj.receiver, TimeoutUtils::some_long()) {
            Ok((err, handle)) => {
                if err != 206 && err != 0 {
                    warn!("libindy open wallet returned: {}", err);
                    return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
                }
                handle
            },
            Err(err) => return Err(error::UNKNOWN_LIBINDY_ERROR.code_num),
        };

        WALLET_HANDLE = wallet_handle;
        Ok(wallet_handle)
    }
}

pub fn init_wallet(wallet_name: &str) -> Result<i32, u32> {
    if settings::test_indy_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return Ok(1);
    }

    let pool_name = match settings::get_config_value(settings::CONFIG_POOL_NAME) {
        Ok(x) => x,
        Err(_) => "pool1".to_owned(),
    };

    let wallet_type = match settings::get_config_value(settings::CONFIG_WALLET_TYPE) {
        Ok(x) => x,
        Err(_) => "default".to_owned(),
    };
    let mut use_key = false;
    let credentials = match settings::get_wallet_credentials() {
        Some(x) => {debug!("using key for indy wallet"); use_key = true; CString::new(x).map_err(map_string_error)? },
        None => CString::new("").map_err(map_string_error)?,
    };

    let open_obj = Return_I32_I32::new()?;
    let create_obj = Return_I32::new()?;
    let pool_name = CString::new(pool_name).map_err(map_string_error)?;
    let xtype = CString::new("default").map_err(map_string_error)?;
    let c_wallet_name = CString::new(wallet_name).map_err(map_string_error)?;

    unsafe {
        let err = indy_create_wallet(create_obj.command_handle,
                                     pool_name.as_ptr(),
                                     c_wallet_name.as_ptr(),
                                     xtype.as_ptr(),
                                     null(),
                                     if use_key { credentials.as_ptr() } else { null() },
                                     Some(create_obj.get_callback()));

        // ignore 203 - wallet already exists
        if err != 203 && err != 0 {
            warn!("libindy create wallet returned: {}", err);
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
        }
        match receive(&create_obj.receiver, TimeoutUtils::some_long()) {
            Ok(_) => {
                if err != 203 && err != 0 {
                    warn!("libindy open wallet returned: {}", err);
                    return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
                }
            },
            Err(err) => return Err(error::UNKNOWN_LIBINDY_ERROR.code_num),
        };
    }

    open_wallet(wallet_name)
}

pub fn close_wallet() -> Result<(), u32> {
    let rtn_obj = Return_I32::new()?;

    unsafe {
        indy_function_eval(
            indy_close_wallet(rtn_obj.command_handle,
                              WALLET_HANDLE,
                             Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
        WALLET_HANDLE = 0;
    }

    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn delete_wallet(wallet_name: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() {
        unsafe { WALLET_HANDLE = 0;}
        return Ok(())
    }

    let rtn_obj = Return_I32::new()?;
    let wallet_name = CString::new(wallet_name).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_delete_wallet(rtn_obj.command_handle,
                              wallet_name.as_ptr(),
                              null(),
                              Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }
    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn store_their_did(identity_json: &str) -> Result<(), u32> {

    let identity_json = CString::new(identity_json.to_string()).map_err(map_string_error)?;
    let wallet_handle = get_wallet_handle();

    let rtn_obj = Return_I32::new()?;

    unsafe {
        indy_function_eval(
            indy_store_their_did(rtn_obj.command_handle,
                                 wallet_handle,
                                 identity_json.as_ptr(),
                                 Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }
    rtn_obj.receive(TimeoutUtils::some_long())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::error;
    use std::thread;
    use std::time::Duration;
    use utils::libindy::signus::SignusUtils;

    #[test]
    fn test_wallet() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_name = String::from("walletUnique");
        let mut wallet_handle = init_wallet(&wallet_name).unwrap();
        assert!( wallet_handle > 0);
        assert_eq!(error::UNKNOWN_LIBINDY_ERROR.code_num, init_wallet(&String::from("")).unwrap_err());

        thread::sleep(Duration::from_secs(1));
        delete_wallet("walletUnique").unwrap();
        let handle = get_wallet_handle();
        let wallet_name2 = String::from("wallet2");
        wallet_handle = init_wallet(&wallet_name2).unwrap();
        assert!(wallet_handle > 0);

        thread::sleep(Duration::from_secs(1));
        assert_ne!(handle, get_wallet_handle());
        delete_wallet("wallet2").unwrap();
    }

    #[test]
    fn test_wallet_with_credentials() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY,"pass");

        let handle = init_wallet("password_wallet").unwrap();

        SignusUtils::create_and_store_my_did(handle,None).unwrap();
        delete_wallet("password_wallet").unwrap();
    }
}