use super::IndyError;
use futures::*;
use futures::sync::oneshot;
use utils::sequence;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;

pub fn create_and_store_my_did(wallet_handle: i32, did_info: &str) -> Box<Future<Item=(String, String), Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<(String, String), IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, str1: *const c_char, str2: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok((rust_str!(str1), rust_str!(str2)))
        };

        tx.send(res).unwrap();
    }

    let did_info = c_str!(did_info);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_create_and_store_my_did(command_handle, wallet_handle, did_info.as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&0).unwrap();
        Box::new(done(Err(IndyError::from_err_code(err))))
    } else {
        Box::new(rx
            .map_err(|_| panic!("channel error!"))
            .and_then(|res| done(res)))
    }
}

pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<String, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, str1: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_str!(str1))
        };

        tx.send(res).unwrap();
    }

    let did = c_str!(did);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_key_for_local_did(command_handle, wallet_handle, did.as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&0).unwrap();
        Box::new(done(Err(IndyError::from_err_code(err))))
    } else {
        Box::new(rx
            .map_err(|_| panic!("channel error!"))
            .and_then(|res| done(res)))
    }
}

extern {
    #[no_mangle]
    fn indy_create_and_store_my_did(command_handle: i32,
                                        wallet_handle: i32,
                                        did_info: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: i32, str1: *const c_char, str2: *const c_char)>) -> i32;

    #[no_mangle]
    fn indy_key_for_local_did(command_handle: i32,
                                  wallet_handle: i32,
                                  did: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32, err: i32, str1: *const c_char)>) -> i32;
}