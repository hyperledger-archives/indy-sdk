/* test isn't ready until > libindy 1.0.1
extern crate libc;

use self::libc::c_char;
use std::sync::mpsc::channel;
use std::ffi::CString;
use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

extern {
fn indy_prep_msg(command_handle: i32,
                 wallet_handle: i32,
                 sender_vk: *const c_char,
                 recipient_vk: *const c_char,
                 msg_data: *const u8,
                 msg_len: u32,
                 cb: Option<extern fn(command_handle_: i32, err: i32, encrypted_msg: *const u8, encrypted_len: u32)>) -> i32;

fn indy_prep_anonymous_msg(command_handle: i32,
                           recipient_vk: *const c_char,
                           msg_data: *const u8,
                           msg_len: u32,
                           cb: Option<extern fn(command_handle_: i32, err: i32, encrypted_msg: *const u8, encrypted_len: u32)>) -> i32;

fn indy_parse_msg(command_handle: i32,
                  wallet_handle: i32,
                  recipient_vk: *const c_char,
                  encrypted_msg: *const u8,
                  encrypted_len: u32,
                  cb: Option<extern fn(command_handle_: i32, err: i32, sender_vk: *const c_char, msg_data: *const u8, msg_len: u32)>) -> i32;
}

pub fn prep_msg(wallet_handle: i32, sender_vk: &str, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, i32> {
let (sender, receiver) = channel();

let cb = Box::new(move |err, encrypted_msg| {
    sender.send((err, encrypted_msg)).unwrap();
});

let (command_handle, cb) = CallbackUtils::closure_to_prep_msg_cb(cb);

let sender_vk = CString::new(sender_vk).unwrap();
let recipient_vk = CString::new(recipient_vk).unwrap();

unsafe {
    let err = indy_prep_msg(command_handle,
                            wallet_handle,
                            sender_vk.as_ptr(),
                            recipient_vk.as_ptr(),
                            msg.as_ptr() as *const u8,
                            msg.len() as u32,
                            cb);

    if err != 0 {
        return Err(err);
    }

    let (err, encrypted_msg) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    if err != 0 {
        return Err(err);
    }

    Ok(encrypted_msg)
}
}

pub fn prep_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, i32> {
let (sender, receiver) = channel();

let cb = Box::new(move |err, encrypted_msg| {
    sender.send((err, encrypted_msg)).unwrap();
});

let (command_handle, cb) = CallbackUtils::closure_to_prep_anonymous_msg_cb(cb);

let recipient_vk = CString::new(recipient_vk).unwrap();

unsafe {
    let err = indy_prep_anonymous_msg(command_handle,
                                      recipient_vk.as_ptr(),
                                      msg.as_ptr() as *const u8,
                                      msg.len() as u32,
                                      cb);

    if err != 0 {
        return Err(err);
    }

    let (err, encrypted_msg) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    if err != 0 {
        return Err(err);
    }

    Ok(encrypted_msg)
}
}

pub fn parse_msg(wallet_handle: i32, recipient_vk: &str, msg: &[u8]) -> Result<(Option<String>, Vec<u8>), i32> {
let (sender, receiver) = channel();

let cb = Box::new(move |err, verkey, msg| {
    sender.send((err, verkey, msg)).unwrap();
});

let (command_handle, cb) = CallbackUtils::closure_to_parse_msg_cb(cb);

let recipient_vk = CString::new(recipient_vk).unwrap();

unsafe {
    let err = indy_parse_msg(command_handle,
                             wallet_handle,
                             recipient_vk.as_ptr(),
                             msg.as_ptr() as *const u8,
                             msg.len() as u32,
                             cb);

    if err != 0 {
        return Err(err);
    }

    let (err, verkey, msg) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    if err != 0 {
        return Err(err);
    }

    Ok((verkey, msg))
}
}


#[cfg(test)]
pub mod tests {

use super::*;
use utils::wallet;
use utils::signus::SignusUtils;
use utils::constants::*;

#[test]
fn test_send_msg() {
    let my_wallet = wallet::init_wallet("test_send_msg_my_wallet",POOL, "Default").unwrap();
    let their_wallet = wallet::init_wallet("test_send_msg_their_wallet",POOL, "Default").unwrap();

    let (my_did, my_vk) = SignusUtils::create_and_store_my_did(my_wallet, Some(MY1_SEED)).unwrap();
    let (their_did, their_vk) = SignusUtils::create_and_store_my_did(their_wallet, Some(MY1_SEED)).unwrap();

    SignusUtils::store_their_did_from_parts(my_wallet, their_did.as_ref(), their_vk.as_ref()).unwrap();
    SignusUtils::store_their_did_from_parts(their_wallet, my_did.as_ref(), my_vk.as_ref()).unwrap();

    let message = "this is a test message for encryption";
    let encrypted_message = prep_msg(my_wallet, my_vk.as_ref(), their_vk.as_ref(),message.as_bytes()).unwrap();
    let (_, decrypted_message) = parse_msg(their_wallet,their_vk.as_ref(),&encrypted_message[..]).unwrap();

    assert_eq!(message.as_bytes().to_vec(), decrypted_message);

}
}
*/

