extern crate serde;
extern crate serde_json;
extern crate indy_crypto;
extern crate time;

use self::indy_crypto::utils::json::JsonEncodable;


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
    fn new(req_id: u64, identifier: &str, operation: T, protocol_version: u64) -> Request<T> {
        Request {
            req_id,
            identifier: identifier.to_string(),
            operation,
            protocol_version,
            signature: None
        }
    }

    pub fn build_request(identifier: &str, operation: T) -> Result<String, serde_json::Error> {
        let req_id = time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64;
        serde_json::to_string(&Request::new(req_id, identifier, operation, 1))
    }
}

impl<T: JsonEncodable> JsonEncodable for Request<T> {}