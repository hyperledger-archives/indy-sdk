use futures::*;
use futures::sync::oneshot;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;
use super::IndyError;
use utils::futures::*;
use utils::sequence;

pub fn create_wallet(config: &str, credentials: &str) -> Box<Future<Item=(), Error=IndyError>> {
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
        indy_create_wallet(command_handle,
                           c_str!(config).as_ptr(),
                           c_str!(credentials).as_ptr(),
                           Some(callback))
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

pub fn open_wallet(config: &str, credentials: &str) -> Box<Future<Item=i32, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<i32, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, wallet_handle: i32) {
        let tx = {
            let mut callbacks = CALLBACKS.lock().unwrap();
            callbacks.remove(&command_handle).unwrap()
        };

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(wallet_handle)
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
        indy_open_wallet(command_handle, c_str!(config).as_ptr(), c_str!(credentials).as_ptr(), Some(callback))
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

#[allow(unused)] // TODO: FIXME:
pub fn close_wallet(wallet_handle: i32) -> Box<Future<Item=(), Error=IndyError>> {
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
        indy_close_wallet(command_handle,
                           wallet_handle,
                           Some(callback))
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
    fn indy_create_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    #[no_mangle]
    fn indy_open_wallet(command_handle: i32,
                        config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<extern fn(xcommand_handle: i32, err: i32, handle: i32)>) -> i32;

    #[allow(unused)] // FIXME: Use!
    #[no_mangle]
    fn indy_close_wallet(command_handle: i32,
                         wallet_handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;
}