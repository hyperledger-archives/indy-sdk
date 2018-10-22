use domain::a2a::CreateMessageType;
use domain::status::MessageStatusCode;

use chrono::{DateTime, Utc};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct InternalMessage {
    pub uid: String,
    pub _type: CreateMessageType,
    pub sender_did: String,
    pub status_code: MessageStatusCode,
    pub last_update_datetime: DateTime<Utc>,
    pub ref_msg_id: Option<String>,
    pub topic_id: Option<String>,
    pub seq_no: Option<String>,
    pub payload: Option<Vec<u8>>,
}

impl InternalMessage {
    pub fn new(uid: Option<&str>,
               mtype: &CreateMessageType,
               status_code: MessageStatusCode,
               from_did: &str,
               payload: Option<Vec<u8>>,
               ) -> InternalMessage {
        trace!("InternalMessage::new >> {:?}, {:?}, {:?}, {:?}", uid, mtype, status_code, from_did);

        let uid = uid.map(String::from).unwrap_or(_rand_string(10));

        InternalMessage {
            uid,
            _type: mtype.clone(),
            sender_did: from_did.to_string(),
            status_code,
            last_update_datetime: Utc::now(),
            ref_msg_id: None,
            topic_id: None,
            seq_no: None,
            payload,
        }
    }

    pub fn update(&mut self, status: &MessageStatusCode, ref_msg_id: Option<&str>) {
        self.status_code = status.clone();
        ref_msg_id.map(|id| self.ref_msg_id = Some(id.to_string()));
        self.last_update_datetime = Utc::now();
    }
}

fn _rand_string(len: usize) -> String {
    thread_rng().gen_ascii_chars().take(len).collect()
}


