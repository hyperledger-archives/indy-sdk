use std::sync::mpsc::{channel};
use std::ffi::{CString};

use sovrin::api::agent::{
    sovrin_agent_connect,
    sovrin_agent_listen,
    sovrin_agent_send,
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

    pub fn listen(wallet_handle: i32, endpoint: &str) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();
        let on_msg = Box::new(|conn_handle, err, msg| {
            info!("On connection {} received (with error {:?}) agent message {}", conn_handle, err, msg);
        }); //TODO make as parameter?
        let on_msg = CallbackUtils::closure_to_agent_message_cb(on_msg);

        let on_connect = Box::new(|listener_handle, err, conn_handle, sender_did, receiver_did| {
            info!("New connection {} on listener {}, err {:?}, sender DID {}, receiver DID {}", conn_handle, listener_handle, err, sender_did, receiver_did);
        });
        let on_connect = CallbackUtils::closure_to_agent_connected_cb(on_connect);

        let cb = Box::new(move |err, listener_handle| sender.send((err, listener_handle)).unwrap());
        let (cmd_id, cb) = CallbackUtils::closure_to_agent_listen_cb(cb);

        let res = sovrin_agent_listen(cmd_id, wallet_handle, CString::new(endpoint).unwrap().as_ptr(), cb, on_connect, on_msg);

        if res != ErrorCode::Success {
            return Err(res);
        }

        let (res, listener_handle) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if res != ErrorCode::Success {
            return Err(res);
        }

        Ok(listener_handle)
    }

    pub fn send(conn_handle: i32, msg: &str) -> Result<(), ErrorCode> {
        let (send_sender, send_receiver) = channel();
        let (send_cmd_id, send_cb) = CallbackUtils::closure_to_agent_send_cb(
            Box::new(move |err_code| send_sender.send(err_code).unwrap())
        );
        sovrin_agent_send(send_cmd_id, conn_handle, CString::new(msg).unwrap().as_ptr(), send_cb);
        let send_result = send_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if send_result != ErrorCode::Success {
            return Err(send_result)
        }
        Ok(())
    }
}