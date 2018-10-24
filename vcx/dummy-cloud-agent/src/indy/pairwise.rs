use futures::*;
use futures::sync::oneshot;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;
use super::IndyError;
use utils::futures::*;
use utils::sequence;

#[derive(Deserialize, Debug)]
pub struct Pairwise {
    pub my_did: String,
    pub their_did: String,
    pub metadata: String,
}

#[derive(Deserialize, Debug)]
pub struct PairwiseInfo {
    pub my_did: String,
    pub metadata: String,
}

#[allow(unused)] //FIXME:
pub fn is_pairwise_exists(wallet_handle: i32, their_did: &str) -> Box<Future<Item=bool, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<bool, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, exists: bool) {
        let tx = {
            let mut callbacks = CALLBACKS.lock().unwrap();
            callbacks.remove(&command_handle).unwrap()
        };

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(exists)
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
        indy_is_pairwise_exists(command_handle, wallet_handle, c_str!(their_did).as_ptr(), Some(callback))
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

#[allow(unused)] //FIXME:
pub fn create_pairwise(wallet_handle: i32, their_did: &str, my_did: &str, metadata: &str) -> Box<Future<Item=(), Error=IndyError>> {
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
        indy_create_pairwise(
            command_handle,
            wallet_handle,
            c_str!(their_did).as_ptr(),
            c_str!(my_did).as_ptr(),
            c_str!(metadata).as_ptr(),
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

#[allow(unused)] //FIXME:
pub fn get_pairwise(wallet_handle: i32, their_did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<String, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, pairwise_info: *const c_char) {
        let tx = {
            let mut callbacks = CALLBACKS.lock().unwrap();
            callbacks.remove(&command_handle).unwrap()
        };

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_str!(pairwise_info))
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
        indy_get_pairwise(command_handle, wallet_handle, c_str!(their_did).as_ptr(), Some(callback))
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

#[allow(unused)] //FIXME:
pub fn list_pairwise(wallet_handle: i32) -> Box<Future<Item=String, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<String, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, list_pairwise: *const c_char) {
        let tx = {
            let mut callbacks = CALLBACKS.lock().unwrap();
            callbacks.remove(&command_handle).unwrap()
        };

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_str!(list_pairwise))
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
        indy_list_pairwise(command_handle, wallet_handle, Some(callback))
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

#[allow(unused)] //FIXME:
pub fn set_pairwise_metadata(wallet_handle: i32, their_did: &str, metadata: &str) -> Box<Future<Item=(), Error=IndyError>> {
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
        indy_set_pairwise_metadata(
            command_handle,
            wallet_handle,
            c_str!(their_did).as_ptr(),
            c_str!(metadata).as_ptr(),
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
    pub fn indy_is_pairwise_exists(command_handle: i32,
                                   wallet_handle: i32,
                                   their_did: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32,
                                                        err: i32, exists: bool)>) -> i32;

    #[no_mangle]
    pub fn indy_create_pairwise(command_handle: i32,
                                wallet_handle: i32,
                                their_did: *const c_char,
                                my_did: *const c_char,
                                metadata: *const c_char,
                                cb: Option<extern fn(xcommand_handle: i32,
                                                     err: i32)>) -> i32;

    #[no_mangle]
    pub fn indy_get_pairwise(command_handle: i32,
                             wallet_handle: i32,
                             their_did: *const c_char,
                             cb: Option<extern fn(xcommand_handle: i32,
                                                  err: i32,
                                                  pairwise_info: *const c_char)>) -> i32;

    #[no_mangle]
    pub fn indy_list_pairwise(command_handle: i32,
                              wallet_handle: i32,
                              cb: Option<extern fn(xcommand_handle: i32,
                                                   err: i32,
                                                   list_pairwise: *const c_char)>) -> i32;

    #[no_mangle]
    pub fn indy_set_pairwise_metadata(command_handle: i32,
                                      wallet_handle: i32,
                                      their_did: *const c_char,
                                      metadata: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32,
                                                           err: i32)>) -> i32;
}