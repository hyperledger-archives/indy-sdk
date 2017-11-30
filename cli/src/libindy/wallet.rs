use super::ErrorCode;

use utils::timeout::TimeoutUtils;
use utils::sequence::SequenceUtils;

use libc::c_char;

use std::collections::HashMap;
use std::ffi::CString;
use std::ptr::null;
use std::sync::Mutex;
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
        lazy_static! {
            static ref CREATE_WALLET_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn create_wallet_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CREATE_WALLET_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CREATE_WALLET_CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_wallet_callback))
    }

    pub fn _closure_to_open_wallet_cb(closure: Box<FnMut(ErrorCode, i32) + Send>)
                                      -> (i32,
                                          Option<extern fn(command_handle: i32, err: ErrorCode,
                                                           handle: i32)>) {
        lazy_static! {
            static ref OPEN_WALLET_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
        }

        extern "C" fn open_wallet_callback(command_handle: i32, err: ErrorCode, handle: i32) {
            let mut callbacks = OPEN_WALLET_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, handle)
        }

        let mut callbacks = OPEN_WALLET_CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(open_wallet_callback))
    }

    fn _closure_to_delete_wallet_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                               Option<extern fn(command_handle: i32,
                                                                                                err: ErrorCode)>) {
        lazy_static! {
            static ref DELETE_WALLET_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn delete_wallet_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = DELETE_WALLET_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = DELETE_WALLET_CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(delete_wallet_callback))
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
    fn indy_close_wallet(command_handle: i32,
                         handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_delete_wallet(command_handle: i32,
                          name: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;
}