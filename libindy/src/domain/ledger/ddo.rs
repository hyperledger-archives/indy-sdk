extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::GET_DDO;

use self::indy_crypto::utils::json::JsonEncodable;

#[derive(Serialize, PartialEq, Debug)]
pub struct GetDdoOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String
}

impl GetDdoOperation {
    pub fn new(dest: String) -> GetDdoOperation {
        GetDdoOperation {
            _type: GET_DDO.to_string(),
            dest
        }
    }
}

impl JsonEncodable for GetDdoOperation {}