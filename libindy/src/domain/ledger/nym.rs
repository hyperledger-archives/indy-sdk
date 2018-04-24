extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::GET_NYM;

use self::indy_crypto::utils::json::{JsonEncodable, JsonDecodable};

#[derive(Serialize, PartialEq, Debug)]
pub struct GetNymOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String
}

impl GetNymOperation {
    pub fn new(dest: String) -> GetNymOperation {
        GetNymOperation {
            _type: GET_NYM.to_string(),
            dest
        }
    }
}

impl JsonEncodable for GetNymOperation {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNymReplyResult {
    pub identifier: String,
    pub req_id: u64,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: String,
    pub dest: String
}

impl<'a> JsonDecodable<'a> for GetNymReplyResult {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNymResultData {
    pub identifier: Option<String>,
    pub dest: String,
    pub role: Option<String>,
    pub verkey: Option<String>
}

impl<'a> JsonDecodable<'a> for GetNymResultData {}
