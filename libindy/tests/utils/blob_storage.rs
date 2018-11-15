extern crate libc;
extern crate byteorder;
extern crate serde_json;
extern crate rmp_serde;
extern crate time;
extern crate futures;
extern crate indyrs as indy;

use self::indy::ErrorCode;
use self::indy::blob_storage::Blob;

use self::futures::Future;

pub fn open_reader(type_: &str, config_json: &str) -> Result<i32, ErrorCode> {
    Blob::open_reader(type_, config_json).wait()
}

pub fn open_writer(type_: &str, config_json: &str) -> Result<i32, ErrorCode> {
    Blob::open_writer(type_, config_json).wait()
}