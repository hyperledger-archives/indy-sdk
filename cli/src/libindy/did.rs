use super::{ErrorCode, IndyHandle};

use utils::timeout::TimeoutUtils;

use libc::c_char;
use std::ffi::CString;
use std::sync::mpsc::channel;


pub struct Did {}

impl Did {
    pub fn new(wallet_handle: IndyHandle, my_did_json: &str) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, did, verkey| {
            sender.send((err, did, verkey)).unwrap();
        });

        let (command_handle, cb) = Did::closure_to_create_and_store_my_did_cb(cb);

        let my_did_json = CString::new(my_did_json).unwrap();

        let err = unsafe {
            indy_create_and_store_my_did(command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, my_did, my_verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((my_did, my_verkey))
    }

    pub fn set_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });
        let (command_handle, callback) = Did::closure_to_store_did_metadata_cb(cb);

        let did = CString::new(did).unwrap();
        let metadata = CString::new(metadata).unwrap();

        let err = unsafe {
            indy_set_did_metadata(command_handle,
                                  wallet_handle,
                                  did.as_ptr(),
                                  metadata.as_ptr(),
                                  callback)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }
        let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(())
    }

    pub fn list_dids_with_meta(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, dids| {
            sender.send((err, dids)).unwrap();
        });

        let (command_handle, cb) = Did::closure_to_list_dids_with_meta(cb);

        let err = unsafe {
            indy_list_my_dids_with_meta(command_handle, wallet_handle, cb)
        };

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, dids) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(dids)
    }

    pub fn closure_to_create_and_store_my_did_cb(closure: Box<FnMut(ErrorCode, String, String) + Send>) -> (i32,
                                                                                                            Option<extern fn(command_handle: i32,
                                                                                                                             err: ErrorCode,
                                                                                                                             did: *const c_char,
                                                                                                                             verkey: *const c_char)>) {
        super::callbacks::_closure_to_cb_ec_string_string(closure)
    }

    pub fn closure_to_store_did_metadata_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        super::callbacks::_closure_to_cb_ec(closure)
    }

    fn closure_to_list_dids_with_meta(closure: Box<FnMut(ErrorCode, String) + Send>)
                                      -> (i32,
                                          Option<extern fn(command_handle: i32, err: ErrorCode,
                                                           dids: *const c_char)>) {
        super::callbacks::_closure_to_cb_ec_string(closure)
    }
}

extern {
    #[no_mangle]
    pub fn indy_create_and_store_my_did(command_handle: i32,
                                        wallet_handle: i32,
                                        did_json: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             did: *const c_char,
                                                             verkey: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_set_did_metadata(command_handle: i32,
                             wallet_handle: i32,
                             did: *const c_char,
                             metadata: *const c_char,
                             cb: Option<extern fn(command_handle_: i32,
                                                  err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_list_my_dids_with_meta(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        dids: *const c_char)>) -> ErrorCode;
}