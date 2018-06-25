extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::GET_NYM;
use super::response::{GetReplyResultV0, GetReplyResultV1};

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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetNymReplyResult {
    GetNymReplyResultV0(GetReplyResultV0<String>),
    GetNymReplyResultV1(GetReplyResultV1<GetNymResultDataV1>)
}

impl<'a> JsonDecodable<'a> for GetNymReplyResult {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct GetNymResultDataV0 {
    pub identifier: Option<String>,
    pub dest: String,
    pub role: Option<String>,
    pub verkey: Option<String>
}

impl<'a> JsonDecodable<'a> for GetNymResultDataV0 {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct GetNymResultDataV1 {
    pub ver: String,
    pub id: String,
    pub did: String,
    pub verkey: Option<String>,
    pub role: Option<String>
}

impl<'a> JsonDecodable<'a> for GetNymResultDataV1 {}
