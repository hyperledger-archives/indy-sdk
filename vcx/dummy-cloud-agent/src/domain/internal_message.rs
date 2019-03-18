use domain::a2a::RemoteMessageType;
use domain::status::MessageStatusCode;
use domain::payload::Thread;

use utils::rand::rand_string;

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMessage {
    pub uid: String,
    pub _type: RemoteMessageType,
    pub sender_did: String,
    pub status_code: MessageStatusCode,
    pub ref_msg_id: Option<String>,
    pub payload: Option<Vec<u8>>,
    pub sending_data: HashMap<String, Option<String>>,
    pub thread: Option<Thread>
}

impl InternalMessage {
    pub fn new(uid: Option<&str>,
               mtype: RemoteMessageType,
               status_code: MessageStatusCode,
               sender_did: &str,
               ref_msg_id: Option<&str>,
               payload: Option<Vec<u8>>,
               sending_data: Option<HashMap<String, Option<String>>>,
               thread: Option<Thread>) -> InternalMessage {
        InternalMessage {
            uid: uid.map(String::from).unwrap_or(rand_string(10)),
            _type: mtype.clone(),
            sender_did: sender_did.to_string(),
            status_code,
            ref_msg_id: ref_msg_id.map(String::from),
            payload,
            sending_data: sending_data.unwrap_or(HashMap::new()),
            thread,
        }
    }
}