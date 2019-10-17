use error::prelude::*;
use v3::messages::{A2AMessageKinds, MessageType};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Forward {
    #[serde(rename = "@type")]
    msg_type: MessageType,
    to: String,
    #[serde(rename = "msg")]
    msg: ::serde_json::Value,
}

impl Forward {
    pub fn new(to: String, msg: Vec<u8>) -> VcxResult<Forward> {
        let msg = ::serde_json::from_slice(msg.as_slice())
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidState, err))?;

        Ok(Forward {
            msg_type: MessageType::build(A2AMessageKinds::Forward),
            to,
            msg,
        })
    }
}