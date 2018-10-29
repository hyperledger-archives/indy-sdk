use domain::a2a::MessageType;
use domain::status::MessageStatusCode;

use utils::rand::rand_string;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InternalMessage {
    pub uid: String,
    pub _type: MessageType,
    pub sender_did: String,
    pub status_code: MessageStatusCode,
    pub ref_msg_id: Option<String>,
    pub seq_no: Option<String>, // ?
    pub payload: Option<Vec<u8>>,
    pub sending_data: HashMap<&'static str, Option<String>>
}

impl InternalMessage {
    pub fn new(uid: Option<&str>,
               mtype: MessageType,
               status_code: MessageStatusCode,
               sender_did: &str,
               ref_msg_id: Option<&str>,
               payload: Option<Vec<u8>>,
               sending_data: Option<HashMap<&'static str, Option<String>>>) -> InternalMessage {
        InternalMessage {
            uid: uid.map(String::from).unwrap_or(rand_string(10)),
            _type: mtype.clone(),
            sender_did: sender_did.to_string(),
            status_code,
            ref_msg_id: ref_msg_id.map(String::from),
            seq_no: None,
            payload,
            sending_data: sending_data.unwrap_or(HashMap::new()),
        }
    }
}



