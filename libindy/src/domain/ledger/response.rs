extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use self::indy_crypto::utils::json::JsonDecodable;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub req_id: u64,
    pub reason: String
}

impl<'a> JsonDecodable<'a> for Response {}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Reply<T> {
    ReplyV0(ReplyV0<T>),
    ReplyV1(ReplyV1<T>)
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for Reply<T> {}

impl<T> Reply<T> {
    pub fn result(self) -> T {
        match self {
            Reply::ReplyV0(reply) => reply.result,
            Reply::ReplyV1(mut reply) => reply.data.result.remove(0).result
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReplyV0<T> {
    pub result: T
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for ReplyV0<T> {}

#[derive(Debug, Deserialize)]
pub struct ReplyV1<T> {
    pub data: ReplyDataV1<T>
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for ReplyV1<T> {}

#[derive(Debug, Deserialize)]
pub struct ReplyDataV1<T> {
    pub  result: Vec<ReplyV0<T>>
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for ReplyDataV1<T> {}


#[derive(Debug, Deserialize)]
pub struct GetReplyResultV0<T> {
    pub  data: Option<T>
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for GetReplyResultV0<T> {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReplyResultV1<T> {
    pub  txn: GetReplyTxnV1<T>,
    pub  txn_metadata: TxnMetadata,
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for GetReplyResultV1<T> {}

#[derive(Debug, Deserialize)]
pub struct GetReplyTxnV1<T> {
    pub data: T,
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for GetReplyTxnV1<T> {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TxnMetadata {
    pub seq_no: u32,
    pub creation_time: u64,
}

impl<'a> JsonDecodable<'a> for TxnMetadata {}

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