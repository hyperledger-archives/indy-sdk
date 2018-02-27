extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::blob_storage::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

pub struct BlobStorageUtils {}

impl BlobStorageUtils {
    pub fn open_reader(type_: &str, config_json: &str, location: &str, hash: &str) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, handle| {
            sender.send((err, handle)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_open_reader_cb(cb);

        let type_ = CString::new(type_).unwrap();
        let config_json = CString::new(config_json).unwrap();
        let location = CString::new(location).unwrap();
        let hash = CString::new(hash).unwrap();

        let err = indy_open_reader(command_handle,
                                   type_.as_ptr(),
                                   config_json.as_ptr(),
                                   location.as_ptr(),
                                   hash.as_ptr(),
                                   cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, handle) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(handle)
    }
}