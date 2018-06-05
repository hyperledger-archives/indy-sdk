extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::GET_TXN;

use self::indy_crypto::utils::json::JsonEncodable;


#[derive(Serialize, PartialEq, Debug)]
pub struct GetTxnOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: i32
}

impl GetTxnOperation {
    pub fn new(data: i32) -> GetTxnOperation {
        GetTxnOperation {
            _type: GET_TXN.to_string(),
            data
        }
    }
}

impl JsonEncodable for GetTxnOperation {}