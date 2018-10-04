use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;

use native::blob_storage;
use native::ResponseI32CB;

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

pub struct Blob {}

impl Blob {
    pub fn open_reader(xtype: &str, config_json: &str) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Blob::_open_reader(command_handle, xtype, config_json, cb);

        ResultHandler::one(err, receiver)
    }

    pub fn open_reader_timeout(xtype: &str, config_json: &str, timeout: Duration) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Blob::_open_reader(command_handle, xtype, config_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    pub fn open_reader_async<F: 'static>(xtype: &str, config_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, IndyHandle) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Blob::_open_reader(command_handle, xtype, config_json, cb)
    }

    fn _open_reader(command_handle: IndyHandle, xtype: &str, config_json: &str, cb: Option<ResponseI32CB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let config_json = c_str!(config_json);

        ErrorCode::from(unsafe { blob_storage::indy_open_blob_storage_reader(command_handle, xtype.as_ptr(), config_json.as_ptr(), cb) })
    }

    pub fn open_writer(xtype: &str, config_json: &str) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Blob::_open_writer(command_handle, xtype, config_json, cb);

        ResultHandler::one(err, receiver)
    }

    pub fn open_writer_timeout(xtype: &str, config_json: &str, timeout: Duration) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Blob::_open_writer(command_handle, xtype, config_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    pub fn open_writer_async<F: 'static>(xtype: &str, config_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, IndyHandle) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Blob::_open_writer(command_handle, xtype, config_json, cb)
    }

    fn _open_writer(command_handle: IndyHandle, xtype: &str, config_json: &str, cb: Option<ResponseI32CB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let config_json = c_str!(config_json);

        ErrorCode::from(unsafe { blob_storage::indy_open_blob_storage_writer(command_handle, xtype.as_ptr(), config_json.as_ptr(), cb) })
    }
}
