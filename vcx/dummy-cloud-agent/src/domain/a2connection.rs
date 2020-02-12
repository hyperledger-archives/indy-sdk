use std::convert::Into;

use crate::domain::a2a::*;

#[derive(Debug)]
pub enum A2ConnMessage {
    GetMessages(GetMessages),
    MessagesByConnection(MessagesByConnection),
    UpdateMessages(UpdateMessageStatus),
    MessageStatusUpdatedByConnection(UidByConnection),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MessagesByConnection {
    #[serde(rename = "pairwiseDID")]
    #[serde(default)]
    pub did: String,
    pub msgs: Vec<GetMessagesDetailResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UidByConnection {
    pub uids: Vec<String>,
    #[serde(rename = "pairwiseDID")]
    #[serde(default)]
    pub pairwise_did: String,
}

impl Into<MessagesByConnection> for A2ConnMessage {
    fn into(self) -> MessagesByConnection {
        match self {
            A2ConnMessage::MessagesByConnection(msg) => msg,
            _ => panic!("Cant' convert message") // TODO: FIXME
        }
    }
}

impl Into<UidByConnection> for A2ConnMessage {
    fn into(self) -> UidByConnection {
        match self {
            A2ConnMessage::MessageStatusUpdatedByConnection(msg) => msg,
            _ => panic!("Cant' convert message") // TODO: FIXME
        }
    }
}