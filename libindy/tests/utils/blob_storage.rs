extern crate libc;

use std::ffi::CString;

use indy::api::blob_storage::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;

pub struct BlobStorageUtils {}

impl BlobStorageUtils {
    pub fn open_reader(type_: &str, config_json: &str) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_i32();

        let type_ = CString::new(type_).unwrap();
        let config_json = CString::new(config_json).unwrap();

        let err = indy_open_blob_storage_reader(command_handle,
                                                type_.as_ptr(),
                                                config_json.as_ptr(),
                                                cb);

        super::results::result_to_int(err, receiver)
    }

    pub fn open_writer(type_: &str, config_json: &str) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_i32();

        let type_ = CString::new(type_).unwrap();
        let config_json = CString::new(config_json).unwrap();

        let err = indy_open_blob_storage_writer(command_handle,
                                                type_.as_ptr(),
                                                config_json.as_ptr(),
                                                cb);

        super::results::result_to_int(err, receiver)
    }
}