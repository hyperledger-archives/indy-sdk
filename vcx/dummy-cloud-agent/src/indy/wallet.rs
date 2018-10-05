use super::errors::*;
use futures::*;
use futures::sync::oneshot;
use utils::sequence;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;

pub fn create_wallet(config: &str, credentials: &str) -> Box<Future<Item=(), Error=Error>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<()>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(ErrorKind::from_err_code(err).into())
        } else {
            Ok(())
        };

        tx.send(res).unwrap();
    }

    let config = c_str!(config);
    let credentials = c_str!(credentials);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_create_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&0).unwrap();
        Box::new(done(Err(ErrorKind::from_err_code(err).into())))
    } else {
        Box::new(rx
            .map_err(|_| "channel error!".into())
            .and_then(|res| done(res)))
    }
}

pub fn open_wallet(config: &str, credentials: &str) -> Box<Future<Item=i32, Error=Error>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<i32>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, wallet_handle: i32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(ErrorKind::from_err_code(err).into())
        } else {
            Ok(wallet_handle)
        };

        tx.send(res).unwrap();
    }

    let config = c_str!(config);
    let credentials = c_str!(credentials);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_open_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&0).unwrap();
        Box::new(done(Err(ErrorKind::from_err_code(err).into())))
    } else {
        Box::new(rx
            .map_err(|_| "channel error!".into())
            .and_then(|res| done(res)))
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
}