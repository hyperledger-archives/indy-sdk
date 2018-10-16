use futures::*;
use futures::sync::oneshot;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;
use super::IndyError;
use utils::futures::*;
use utils::sequence;

pub fn create_and_store_my_did(wallet_handle: i32, did_info: &str) -> Box<Future<Item=(String, String), Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<(String, String), IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, str1: *const c_char, str2: *const c_char) {
        let tx = {
            let mut callbacks = CALLBACKS.lock().unwrap();
            callbacks.remove(&command_handle).unwrap()
        };

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok((rust_str!(str1), rust_str!(str2)))
        };

        tx.send(res).unwrap();
    }

    let (rx, command_handle) = {
        let (tx, rx) = oneshot::channel();
        let command_handle = sequence::get_next_id();
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.insert(command_handle, tx);
        (rx, command_handle)
    };

    let err = unsafe {
        indy_create_and_store_my_did(command_handle, wallet_handle, c_str!(did_info).as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&command_handle).unwrap();
        future::err(IndyError::from_err_code(err)).into_box()
    } else {
        rx
            .map_err(|_| panic!("channel error!"))
            .and_then(|res| res)
            .into_box()
    }
}

pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<String, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, str1: *const c_char) {
        let tx = {
            let mut callbacks = CALLBACKS.lock().unwrap();
            callbacks.remove(&command_handle).unwrap()
        };

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_str!(str1))
        };

        tx.send(res).unwrap();
    }

    let (rx, command_handle) = {
        let (tx, rx) = oneshot::channel();
        let command_handle = sequence::get_next_id();
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.insert(command_handle, tx);
        (rx, command_handle)
    };

    let err = unsafe {
        indy_key_for_local_did(command_handle, wallet_handle, c_str!(did).as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&command_handle).unwrap();
        future::err(IndyError::from_err_code(err)).into_box()
    } else {
        rx
            .map_err(|_| panic!("channel error!"))
            .and_then(|res| res)
            .into_box()
    }
}

pub fn store_their_did(wallet_handle: i32, did_info: &str) -> Box<Future<Item=(), Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<(), IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32) {
        let tx = {
            let mut callbacks = CALLBACKS.lock().unwrap();
            callbacks.remove(&command_handle).unwrap()
        };

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(())
        };

        tx.send(res).unwrap();
    }

    let (rx, command_handle) = {
        let (tx, rx) = oneshot::channel();
        let command_handle = sequence::get_next_id();
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.insert(command_handle, tx);
        (rx, command_handle)
    };

    let err = unsafe {
        indy_store_their_did(command_handle, wallet_handle, c_str!(did_info).as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&command_handle).unwrap();
        future::err(IndyError::from_err_code(err)).into_box()
    } else {
        rx
            .map_err(|_| panic!("channel error!"))
            .and_then(|res| res)
            .into_box()
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

    #[no_mangle]
    pub fn indy_store_their_did(command_handle: i32,
                                        wallet_handle: i32,
                                        did_info: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32,
                                                             err: i32)>) -> i32;
}