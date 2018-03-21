extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;
use std::ptr::null;

use indy::api::sss::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

pub struct SSSUtils {}

impl SSSUtils {
    pub fn shard_msg_with_secret_and_store_shards(wallet_handle: i32, m: u8,
                                                  n: u8, msg: Option<&str>, verkey: &str) -> Result<String, ErrorCode> {
        let (store_shards, store_shards_receiver) = channel();
        let cb = Box::new(move |err, verkey| {
            store_shards.send((err, verkey)).unwrap();
        });
        let (create_and_store_policy_command_handle, create_and_store_policy_callback) = CallbackUtils::closure_to_store_shards_cb(cb);

        let msg_str = msg.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let verkey = CString::new(verkey).unwrap();

        let err =
            indy_shard_msg_with_secret_and_store_shards(create_and_store_policy_command_handle,
                                             wallet_handle,
                                             m, n, if msg.is_some() { msg_str.as_ptr() } else { null() }, verkey.as_ptr(),
                                             create_and_store_policy_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, vk) = store_shards_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(vk)
    }

    pub fn get_shards_of_verkey(wallet_handle: i32, verkey: &str) -> Result<String, ErrorCode> {
        let (get_shards, get_shards_receiver) = channel();
        let cb = Box::new(move |err, shards_json| {
            get_shards.send((err, shards_json)).unwrap();
        });
        let (get_shards_command_handle, get_shards_callback) = CallbackUtils::closure_to_get_shards_cb(cb);

        let verkey = CString::new(verkey).unwrap();

        let err =
            indy_get_shards_of_verkey(get_shards_command_handle,
                                                        wallet_handle,
                                        verkey.as_ptr(),
                                                        get_shards_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, shards_json) = get_shards_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(shards_json)
    }

    pub fn get_shard_of_verkey(wallet_handle: i32, verkey: &str, shard_number: u8) -> Result<String, ErrorCode> {
        let (get_shard, get_shard_receiver) = channel();
        let cb = Box::new(move |err, shard| {
            get_shard.send((err, shard)).unwrap();
        });
        let (get_shard_command_handle, get_shard_callback) = CallbackUtils::closure_to_get_shard_cb(cb);

        let verkey = CString::new(verkey).unwrap();

        let err =
            indy_get_shard_of_verkey(get_shard_command_handle,
                                      wallet_handle,
                                      verkey.as_ptr(),
                                      shard_number,
                                      get_shard_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, shard) = get_shard_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(shard)
    }

    pub fn get_recover_secret_from_shards(shards_json: &str) -> Result<String, ErrorCode> {
        let (recover_secret, recover_secret_receiver) = channel();
        let cb = Box::new(move |err, shards_json| {
            recover_secret.send((err, shards_json)).unwrap();
        });
        let (recover_secret_command_handle, recover_secret_callback) = CallbackUtils::closure_to_recover_secret_cb(cb);

        let shards_json = CString::new(shards_json).unwrap();

        let err =
            indy_recover_secret_from_shards(recover_secret_command_handle,
                                            shards_json.as_ptr(),
                                            recover_secret_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, secret) = recover_secret_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(secret)
    }
}