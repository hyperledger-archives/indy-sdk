extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use self::indy_crypto::utils::json::JsonEncodable;
use super::constants::GET_VALIDATOR_INFO;

#[derive(Serialize, PartialEq, Debug)]
pub struct GetValidatorInfoOperation {
    #[serde(rename = "type")]
    pub _type: String,
}

impl GetValidatorInfoOperation {
    pub fn new() -> GetValidatorInfoOperation {
        GetValidatorInfoOperation {
            _type: GET_VALIDATOR_INFO.to_string(),
        }
    }
}

impl JsonEncodable for GetValidatorInfoOperation {}

