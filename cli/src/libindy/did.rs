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

    pub fn closure_to_create_and_store_my_did_cb(closure: Box<FnMut(ErrorCode, String, String) + Send>) -> (i32,
                                                                                                            Option<extern fn(command_handle: i32,
                                                                                                                             err: ErrorCode,
                                                                                                                             did: *const c_char,
                                                                                                                             verkey: *const c_char)>) {
        super::callbacks::_closure_to_cb_ec_string_string(closure)
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
}