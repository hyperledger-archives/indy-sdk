use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::agent::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

pub struct AgentUtils {}

impl AgentUtils {
    pub fn prep_msg(wallet_handle: i32, sender_vk: &str, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, encrypted_msg| {
            sender.send((err, encrypted_msg)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prep_msg_cb(cb);

        let sender_vk = CString::new(sender_vk).unwrap();
        let recipient_vk = CString::new(recipient_vk).unwrap();

        let err = indy_prep_msg(command_handle,
                                wallet_handle,
                                sender_vk.as_ptr(),
                                recipient_vk.as_ptr(),
                                msg.as_ptr() as *const u8,
                                msg.len() as u32,
                                cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, encrypted_msg) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(encrypted_msg)
    }

    pub fn prep_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, encrypted_msg| {
            sender.send((err, encrypted_msg)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prep_anonymous_msg_cb(cb);

        let recipient_vk = CString::new(recipient_vk).unwrap();

        let err = indy_prep_anonymous_msg(command_handle,
                                          recipient_vk.as_ptr(),
                                          msg.as_ptr() as *const u8,
                                          msg.len() as u32,
                                          cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, encrypted_msg) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(encrypted_msg)
    }

    pub fn parse_msg(wallet_handle: i32, recipient_vk: &str, msg: &[u8]) -> Result<(Option<String>, Vec<u8>), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, verkey, msg| {
            sender.send((err, verkey, msg)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_parse_msg_cb(cb);

        let recipient_vk = CString::new(recipient_vk).unwrap();

        let err = indy_parse_msg(command_handle,
                                 wallet_handle,
                                 recipient_vk.as_ptr(),
                                 msg.as_ptr() as *const u8,
                                 msg.len() as u32,
                                 cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, verkey, msg) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((verkey, msg))
    }
}