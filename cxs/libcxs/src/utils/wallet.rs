extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use settings;
use utils::error;
use std::ptr::null;
use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use std::sync::mpsc::channel;

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

pub fn get_wallet_handle() -> i32 { unsafe { WALLET_HANDLE } }

pub fn init_wallet(wallet_name: &str, pool_name: &str, wallet_type: &str) -> Result<i32, u32> {
    if settings::test_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return Ok(1);
    }

    let (sender, receiver) = channel();
    let (open_sender, open_receiver) = channel();

    let cb = Box::new(move |err| {
        sender.send(err).unwrap();
    });
    let open_cb = Box::new(move |err, handle| {
        open_sender.send((err, handle)).unwrap();
    });

    let (command_handle, cb) = CallbackUtils::closure_to_create_wallet_cb(cb);
    let (open_command_handle, open_cb) = CallbackUtils::closure_to_open_wallet_cb(open_cb);

    let pool_name = CString::new(pool_name).unwrap();
    let xtype = CString::new("default").unwrap();
    let wallet_name = CString::new(wallet_name).unwrap();

    unsafe {
        let err =
            indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               xtype.as_ptr(),
                               null(),
                               null(),
                               cb);

        // ignore 203 - wallet already exists
        if err != 203 && err != 0 {
            return Err(error::UNKNOWN_ERROR.code_num);
        }

        let err = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != 203 && err != 0 {
            return Err(error::UNKNOWN_ERROR.code_num);
        }

        let err =
            indy_open_wallet(open_command_handle,
                             wallet_name.as_ptr(),
                             null(),
                             null(),
                             open_cb);

        if err != 206 && err != 0 {
            return Err(err as u32);
        }

        let (err, wallet_handle) = open_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != 206 && err != 0 {
            return Err(err as u32);
        }

        WALLET_HANDLE = wallet_handle;
        Ok(wallet_handle)
    }
}

pub fn delete_wallet(wallet_name: &str) -> Result<(), i32> {
    let (sender, receiver) = channel();

    let cb = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    let (command_handle, cb) = CallbackUtils::closure_to_delete_wallet_cb(cb);

    let wallet_name = CString::new(wallet_name).unwrap();

    unsafe {
        let err =
            indy_delete_wallet(command_handle,
                               wallet_name.as_ptr(),
                               null(),
                               cb);

        if err != 0 {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != 0 {
            return Err(err);
        }

        WALLET_HANDLE = 0;

        Ok(())
    }
}

pub fn close_wallet(wallet_handle: i32) -> Result<(), i32> {
    let (sender, receiver) = channel();

    let cb = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    let (command_handle, cb) = CallbackUtils::closure_to_delete_wallet_cb(cb);

    unsafe {
        let err =
            indy_close_wallet(command_handle,
                              wallet_handle,
                              cb);

        if err != 0 {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != 0 {
            return Err(err);
        }

        WALLET_HANDLE = 0;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::error;
    use std::thread;
    use utils::generate_command_handle;
    use std::time::Duration;
    use utils::cstring::CStringUtils;

    //TODO: make boilerplate test code use same wallet?

    pub fn make_wallet(wallet_name: &str) {
        let pool_name = String::from("pool1");
        let wallet_type = String::from("default");
        assert!(init_wallet(&wallet_name, &pool_name, &wallet_type).unwrap() >  0);
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
        unsafe { WALLET_HANDLE = 0; }
    }

    #[test]
    fn test_wallet() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let pool_name = String::from("pool1");
        let wallet_name = String::from("wallet1");
        let wallet_type = String::from("default");
        assert!(init_wallet(&wallet_name, &pool_name, &wallet_type).unwrap() > 0);
        assert_eq!(error::UNKNOWN_ERROR.code_num, init_wallet(&String::from(""),&pool_name, &wallet_type).unwrap_err());

        thread::sleep(Duration::from_secs(1));
        delete_wallet("wallet1");
        let handle = get_wallet_handle();
        let wallet_name2 = String::from("wallet2");
        assert!(init_wallet(&wallet_name2, &pool_name, &wallet_type).unwrap() > 0);

        thread::sleep(Duration::from_secs(1));
        assert_ne!(handle, get_wallet_handle());
        delete_wallet("wallet2");
    }
}
