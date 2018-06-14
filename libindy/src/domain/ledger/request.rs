extern crate serde;
extern crate serde_json;
extern crate indy_crypto;
extern crate time;

use self::indy_crypto::utils::json::JsonEncodable;

use std::sync::Mutex;

lazy_static! {
    pub static ref PROTOCOL_VERSION: Mutex<u64> = Mutex::new(2);
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T: serde::Serialize> {
    pub req_id: u64,
    pub identifier: String,
    pub operation: T,
    pub protocol_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>
}

impl<T: serde::Serialize> Request<T> {
    pub fn new(req_id: u64, identifier: &str, operation: T, protocol_version: u64) -> Request<T> {
        Request {
            req_id,
            identifier: identifier.to_string(),
            operation,
            protocol_version,
            signature: None
        }
    }
}

impl<T: JsonEncodable> JsonEncodable for Request<T> {}