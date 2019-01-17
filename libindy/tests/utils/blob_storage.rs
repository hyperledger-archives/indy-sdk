extern crate futures;

use indy::ErrorCode;
use indy::blob_storage;

use self::futures::Future;

pub fn open_reader(type_: &str, config_json: &str) -> Result<i32, ErrorCode> {
    blob_storage::open_reader(type_, config_json).wait()
}

pub fn open_writer(type_: &str, config_json: &str) -> Result<i32, ErrorCode> {
    blob_storage::open_writer(type_, config_json).wait()
}