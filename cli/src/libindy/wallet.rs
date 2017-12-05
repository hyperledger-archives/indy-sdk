use super::ErrorCode;

use utils::timeout::TimeoutUtils;

use libc::c_char;

use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;

pub struct Wallet {}

impl Wallet {
    pub fn create_wallet(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = Wallet::_closure_to_create_wallet_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();
        let wallet_name = CString::new(wallet_name).unwrap();
        let xtype_str = xtype.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               if xtype.is_some() { xtype_str.as_ptr() } else { null() },
                               if config.is_some() { config_str.as_ptr() } else { null() },
                               null(),
                               cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn open_wallet(wallet_name: &str, config: Option<&str>) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, handle| {
            sender.send((err, handle)).unwrap();
        });

        let (command_handle, cb) = Wallet::_closure_to_open_wallet_cb(cb);

        let wallet_name = CString::new(wallet_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_open_wallet(command_handle,
                             wallet_name.as_ptr(),
                             if config.is_some() { config_str.as_ptr() } else { null() },
                             null(),
                             cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, wallet_handle) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(wallet_handle)
    }

    pub fn list_wallets() -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, wallets| {
            sender.send((err, wallets)).unwrap();
        });

        let (command_handle, cb) = Wallet::_closure_to_list_wallets_cb(cb);

        let err = unsafe {
            indy_list_wallets(command_handle, cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, wallets) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(wallets)
    }

    pub fn delete_wallet(wallet_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = Wallet::_closure_to_delete_wallet_cb(cb);

        let wallet_name = CString::new(wallet_name).unwrap();

        let err = unsafe {
            indy_delete_wallet(command_handle,
                               wallet_name.as_ptr(),
                               null(),
                               cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = Wallet::_closure_to_delete_wallet_cb(cb);


        let err = unsafe {
            indy_close_wallet(command_handle,
                              wallet_handle,
                              cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    fn _closure_to_create_wallet_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                               Option<extern fn(command_handle: i32,
                                                                                                err: ErrorCode)>) {
        super::callbacks::_closure_to_cb_ec(closure)
    }

    fn _closure_to_open_wallet_cb(closure: Box<FnMut(ErrorCode, i32) + Send>)
                                  -> (i32,
                                      Option<extern fn(command_handle: i32, err: ErrorCode,
                                                       handle: i32)>) {
        super::callbacks::_closure_to_cb_ec_i32(closure)
    }

    fn _closure_to_list_wallets_cb(closure: Box<FnMut(ErrorCode, String) + Send>)
                                   -> (i32,
                                       Option<extern fn(command_handle: i32, err: ErrorCode,
                                                        wallets: *const c_char)>) {
        super::callbacks::_closure_to_cb_ec_string(closure)
    }


    fn _closure_to_delete_wallet_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                               Option<extern fn(command_handle: i32,
                                                                                                err: ErrorCode)>) {
        super::callbacks::_closure_to_cb_ec(closure)
    }
}

extern {
    #[no_mangle]
    fn indy_create_wallet(command_handle: i32,
                          pool_name: *const c_char,
                          name: *const c_char,
                          xtype: *const c_char,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_open_wallet(command_handle: i32,
                        name: *const c_char,
                        runtime_config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, handle: i32)>) -> ErrorCode;

    #[no_mangle]
    fn indy_list_wallets(command_handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                              wallets: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_close_wallet(command_handle: i32,
                         handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_delete_wallet(command_handle: i32,
                          name: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;
}