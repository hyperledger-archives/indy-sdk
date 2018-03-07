extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::did::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::ledger::LedgerUtils;

pub struct AuthzUtils {}

impl AuthzUtils {
    pub fn create_and_store_policy_address(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), ErrorCode> {
        let (create_and_store_my_did_sender, create_and_store_my_did_receiver) = channel();
        let create_and_store_my_did_cb = Box::new(move |err, did, verkey| {
            create_and_store_my_did_sender.send((err, did, verkey)).unwrap();
        });
        let (create_and_store_my_did_command_handle, create_and_store_my_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_my_did_cb);

        let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));

        let my_did_json = CString::new(my_did_json).unwrap();

        let err =
            indy_create_and_store_my_did(create_and_store_my_did_command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         create_and_store_my_did_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, my_did, my_verkey) = create_and_store_my_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok((my_did, my_verkey))
    }
}