use std::sync::mpsc::{channel};
use std::ffi::{CString};

use sovrin::api::agent::{
    sovrin_agent_connect,
};
use sovrin::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

pub struct AgentUtils {}

impl AgentUtils {
    pub fn connect(wallet_handle: i32, sender_did: &str, receiver_did: &str) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();
        let closure = Box::new(move |err, connection_handle| { sender.send((err, connection_handle)).unwrap(); });
        let (cmd_connect, cb) = CallbackUtils::closure_to_agent_connect_cb(closure);
        let msg_cb = CallbackUtils::closure_to_agent_message_cb(Box::new(move |conn_handle, err, msg| {
            println!("On connection {} received (with error {:?}) agent message {}", conn_handle, err, msg);
        })); //TODO make as parameter?

        let err = sovrin_agent_connect(cmd_connect, wallet_handle,
                                       CString::new(sender_did).unwrap().as_ptr(),
                                       CString::new(receiver_did).unwrap().as_ptr(),
                                       cb, msg_cb);
        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, conn_handle) = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(conn_handle)
    }
}