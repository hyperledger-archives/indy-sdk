use serde;
use time;

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ProtocolVersion {}

lazy_static! {
    pub static ref PROTOCOL_VERSION: AtomicUsize = AtomicUsize::new(1);
}

impl ProtocolVersion {
    pub fn set(version: usize) {
        PROTOCOL_VERSION.store(version, Ordering::Relaxed);
    }

    pub fn get() -> usize {
        PROTOCOL_VERSION.load(Ordering::Relaxed)
    }

    pub fn is_node_1_3() -> bool {
        ProtocolVersion::get() == 1
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T: serde::Serialize> {
    pub req_id: u64,
    pub identifier: String,
    pub operation: T,
    pub protocol_version: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>
}

impl<T: serde::Serialize> Request<T> {
    pub fn new(identifier: &str, operation: T) -> Request<T> {
        let req_id = time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64;
        let protocol_version: usize = ProtocolVersion::get();
        Request {
            req_id,
            identifier: identifier.to_string(),
            operation,
            protocol_version,
            signature: None
        }
    }
}