extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::{ATTRIB, GET_ATTR};

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Serialize, PartialEq, Debug)]
pub struct AttribOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enc: Option<String>
}

impl AttribOperation {
    pub fn new(dest: String, hash: Option<String>, raw: Option<String>,
               enc: Option<String>) -> AttribOperation {
        AttribOperation {
            _type: ATTRIB.to_string(),
            dest,
            hash,
            raw,
            enc,
        }
    }
}

impl JsonEncodable for AttribOperation {}


#[derive(Serialize, PartialEq, Debug)]
pub struct GetAttribOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enc: Option<String>
}

impl GetAttribOperation {
    pub fn new(dest: String, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> GetAttribOperation {
        GetAttribOperation {
            _type: GET_ATTR.to_string(),
            dest,
            raw: raw.map(String::from),
            hash: hash.map(String::from),
            enc: enc.map(String::from)
        }
    }
}

impl JsonEncodable for GetAttribOperation {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAttribReplyResult {
    pub  identifier: String,
    pub  req_id: u64,
    #[serde(rename = "type")]
    pub  _type: String,
    pub  data: String,
    pub  dest: String,
    pub  raw: String,
    pub  seq_no: Option<i32>
}

impl<'a> JsonDecodable<'a> for GetAttribReplyResult {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttribData {
    pub endpoint: Endpoint
}

impl<'a> JsonDecodable<'a> for AttribData {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Endpoint {
    pub ha: String,
    pub verkey: Option<String>
}

impl Endpoint {
    pub fn new(ha: String, verkey: Option<String>) -> Endpoint {
        Endpoint {
            ha,
            verkey
        }
    }
}

impl JsonEncodable for Endpoint {}

impl<'a> JsonDecodable<'a> for Endpoint {}