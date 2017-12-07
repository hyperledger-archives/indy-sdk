use super::ErrorCode;

use utils::timeout::TimeoutUtils;

use libc::c_char;
use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;

pub struct Pool {}

impl Pool {
    pub fn create_pool_ledger_config(pool_name: &str, pool_config: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = Pool::closure_to_create_pool_ledger_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();
        let pool_config_str = CString::new(pool_config).unwrap();

        let err = unsafe {
            indy_create_pool_ledger_config(command_handle,
                                           pool_name.as_ptr(),
                                           pool_config_str.as_ptr(),
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

    pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();


        let cb = Box::new(move |err, pool_handle| {
            sender.send((err, pool_handle)).unwrap();
        });

        let (command_handle, cb) = Pool::closure_to_open_pool_ledger_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_open_pool_ledger(command_handle,
                                  pool_name.as_ptr(),
                                  if config.is_some() { config_str.as_ptr() } else { null() },
                                  cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, pool_handle) = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(pool_handle)
    }

    #[allow(dead_code)] //TODO add refresh pool command or remove this code
    pub fn refresh(pool_handle: i32) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let (command_handle, cb) = Pool::closure_to_refresh_pool_ledger_cb(
            Box::new(move |res| sender.send(res).unwrap()));

        let res = unsafe { indy_refresh_pool_ledger(command_handle, pool_handle, cb) };
        if res != ErrorCode::Success {
            return Err(res);
        }
        let res = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if res != ErrorCode::Success {
            return Err(res);
        }

        Ok(())
    }

    pub fn list() -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, pools| {
            sender.send((err, pools)).unwrap();
        });

        let (command_handle, cb) = Pool::_closure_to_list_pools_cb(cb);

        let err = unsafe {
            indy_list_pools(command_handle, cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, pools) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(pools)
    }

    pub fn close(pool_handle: i32) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let (command_handle, cb) = Pool::closure_to_close_pool_ledger_cb(
            Box::new(move |res| sender.send(res).unwrap()));

        let res = unsafe { indy_close_pool_ledger(command_handle, pool_handle, cb) };
        if res != ErrorCode::Success {
            return Err(res);
        }
        let res = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if res != ErrorCode::Success {
            return Err(res);
        }

        Ok(())
    }

    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let (cmd_id, cb) = Pool::closure_to_delete_pool_ledger_config_cb(Box::new(
            move |res| sender.send(res).unwrap()));

        let pool_name = CString::new(pool_name).unwrap();

        let res = unsafe { indy_delete_pool_ledger_config(cmd_id, pool_name.as_ptr(), cb) };
        if res != ErrorCode::Success {
            return Err(res);
        }
        let res = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if res != ErrorCode::Success {
            return Err(res);
        }
        Ok(())
    }

    pub fn closure_to_create_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        super::callbacks::_closure_to_cb_ec(closure)
    }

    pub fn closure_to_open_pool_ledger_cb(closure: Box<FnMut(ErrorCode, i32) + Send>)
                                          -> (i32,
                                              Option<extern fn(command_handle: i32, err: ErrorCode,
                                                               pool_handle: i32)>) {
        super::callbacks::_closure_to_cb_ec_i32(closure)
    }

    pub fn closure_to_refresh_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                        Option<extern fn(command_handle: i32,
                                                                                                         err: ErrorCode)>) {
        super::callbacks::_closure_to_cb_ec(closure)
    }

    fn _closure_to_list_pools_cb(closure: Box<FnMut(ErrorCode, String) + Send>)
                                 -> (i32,
                                     Option<extern fn(command_handle: i32, err: ErrorCode,
                                                      pools: *const c_char)>) {
        super::callbacks::_closure_to_cb_ec_string(closure)
    }

    pub fn closure_to_close_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                      Option<extern fn(command_handle: i32,
                                                                                                       err: ErrorCode)>) {
        super::callbacks::_closure_to_cb_ec(closure)
    }

    pub fn closure_to_delete_pool_ledger_config_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                              Option<extern fn(command_handle: i32,
                                                                                                               err: ErrorCode)>) {
        super::callbacks::_closure_to_cb_ec(closure)
    }
}

extern {
    #[no_mangle]
    fn indy_create_pool_ledger_config(command_handle: i32,
                                      config_name: *const c_char,
                                      config: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_open_pool_ledger(command_handle: i32,
                                 config_name: *const c_char,
                                 config: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, pool_handle: i32)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_refresh_pool_ledger(command_handle: i32,
                                    handle: i32,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_pools(command_handle: i32,
                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                pools: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_close_pool_ledger(command_handle: i32,
                                  handle: i32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_delete_pool_ledger_config(command_handle: i32,
                                          config_name: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;
}