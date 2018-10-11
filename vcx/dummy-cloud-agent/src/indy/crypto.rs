use futures::*;
use futures::sync::oneshot;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;
use super::IndyError;
use utils::sequence;

#[allow(unused)] // FIXME: Use!
pub fn create_key(wallet_handle: i32,
                  key_info: &str) -> Box<Future<Item=String, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<String, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, verkey: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_str!(verkey))
        };

        tx.send(res).unwrap();
    }

    let key_info = c_str!(key_info);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_create_key(command_handle, wallet_handle, key_info.as_ptr(), Some(callback))
    };

    if err != 0 {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.remove(&0).unwrap();
        Box::new(done(Err(IndyError::from_err_code(err))))
    } else {
        Box::new(rx
            .map_err(|_| panic!("channel i32!"))
            .and_then(|res| done(res)))
    }
}

#[allow(unused)] // FIXME: Use!
pub fn auth_crypt(wallet_handle: i32,
                  sender_vk: &str,
                  recipient_vk: &str,
                  message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<Vec<u8>, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, encrypted_message_raw: *const u8, encrypted_message_len: u32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_slice!(encrypted_message_raw, encrypted_message_len).to_owned())
        };

        tx.send(res).unwrap();
    }

    let sender_vk = c_str!(sender_vk);
    let recipient_vk = c_str!(recipient_vk);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_crypto_auth_crypt(command_handle,
                               wallet_handle,
                               sender_vk.as_ptr(),
                               recipient_vk.as_ptr(),
                               message.as_ptr() as *const u8,
                               message.len() as u32,
                               Some(callback))
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

#[allow(unused)] // FIXME: Use!
pub fn auth_decrypt(wallet_handle: i32,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<Future<Item=(String, Vec<u8>), Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<(String, Vec<u8>), IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, sender_vk: *const c_char, message_raw: *const u8, message_len: u32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok((rust_str!(sender_vk), rust_slice!(message_raw, message_len).to_owned()))
        };

        tx.send(res).unwrap();
    }

    let recipient_vk = c_str!(recipient_vk);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_crypto_auth_decrypt(command_handle,
                                 wallet_handle,
                                 recipient_vk.as_ptr(),
                                 encrypted_message.as_ptr() as *const u8,
                                 encrypted_message.len() as u32,
                                 Some(callback))
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

#[allow(unused)] // FIXME: Use!
pub fn anon_crypt(recipient_vk: &str,
                  message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<Vec<u8>, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, encrypted_message_raw: *const u8, encrypted_message_len: u32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_slice!(encrypted_message_raw, encrypted_message_len).to_owned())
        };

        tx.send(res).unwrap();
    }

    let recipient_vk = c_str!(recipient_vk);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_crypto_anon_crypt(command_handle,
                               recipient_vk.as_ptr(),
                               message.as_ptr() as *const u8,
                               message.len() as u32,
                               Some(callback))
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

#[allow(unused)] // FIXME: Use!
pub fn anon_decrypt(wallet_handle: i32,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, oneshot::Sender<Result<Vec<u8>, IndyError>>>> = Default::default();
    }

    extern fn callback(command_handle: i32, err: i32, message_raw: *const u8, message_len: u32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let tx = callbacks.remove(&command_handle).unwrap();

        let res = if err != 0 {
            Err(IndyError::from_err_code(err))
        } else {
            Ok(rust_slice!(message_raw, message_len).to_owned())
        };

        tx.send(res).unwrap();
    }

    let recipient_vk = c_str!(recipient_vk);

    let (tx, rx) = oneshot::channel();
    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::get_next_id();
    callbacks.insert(command_handle, tx);

    let err = unsafe {
        indy_crypto_anon_decrypt(command_handle,
                                 wallet_handle,
                                 recipient_vk.as_ptr(),
                                 encrypted_message.as_ptr() as *const u8,
                                 encrypted_message.len() as u32,
                                 Some(callback))
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
    fn indy_create_key(command_handle: i32,
                       wallet_handle: i32,
                       key_info: *const c_char,
                       cb: Option<extern fn(xcommand_handle: i32, err: i32, verkey: *const c_char)>) -> i32;


    #[no_mangle]
    fn indy_crypto_auth_crypt(command_handle: i32,
                              wallet_handle: i32,
                              sender_vk: *const c_char,
                              recipient_vk: *const c_char,
                              message_raw: *const u8,
                              message_len: u32,
                              cb: Option<extern fn(xcommand_handle: i32, err: i32, encrypted_msg: *const u8, encrypted_msg_len: u32)>) -> i32;

    #[no_mangle]
    fn indy_crypto_auth_decrypt(command_i32: i32,
                                wallet_handle: i32,
                                recipient_vk: *const c_char,
                                encrypted_msg_raw: *const u8,
                                encrypted_msg_len: u32,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32, sender_vk: *const c_char, message: *const u8, message_len: u32)>) -> i32;

    #[no_mangle]
    fn indy_crypto_anon_crypt(command_i32: i32,
                              recipient_vk: *const c_char,
                              message_raw: *const u8,
                              message_len: u32,
                              cb: Option<extern fn(xcommand_handle: i32, err: i32, msg: *const u8, msg_len: u32)>) -> i32;

    #[no_mangle]
    fn indy_crypto_anon_decrypt(command_i32: i32,
                                wallet_i32: i32,
                                recipient_vk: *const c_char,
                                encrypted_msg_raw: *const u8,
                                encrypted_msg_len: u32,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32, msg: *const u8, msg_len: u32)>) -> i32;
}