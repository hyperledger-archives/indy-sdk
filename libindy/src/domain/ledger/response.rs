extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub req_id: u64,
    pub reason: String
}

impl JsonEncodable for Response {}

impl<'a> JsonDecodable<'a> for Response {}


#[derive(Deserialize, Debug)]
pub struct ReplyResult<T> {
    pub identifier: String,
    pub req_id: u64,
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub  _type: String,
    pub  data: T
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for ReplyResult<T> {}

#[derive(Deserialize, Debug)]
pub struct Reply<T> {
    pub result: T,
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for Reply<T> {}

#[serde(tag = "op")]
#[derive(Deserialize, Debug)]
pub enum Message<T> {
    #[serde(rename = "REQNACK")]
    ReqNACK(Response),
    #[serde(rename = "REPLY")]
    Reply(Reply<T>),
    #[serde(rename = "REJECT")]
    Reject(Response)
}