use messages::thread::Thread;
use v3::messages::{MessageType, MessageId, A2AMessage, A2AMessageKinds};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack {
    #[serde(rename = "@type")]
    msg_type: MessageType,
    #[serde(rename = "@id")]
    id: MessageId,
    status: AckStatus,
    #[serde(rename = "~thread")]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AckStatus {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "PENDING")]
    Pending
}

impl Ack {
    pub fn create() -> Ack {
        Ack {
            msg_type: MessageType::build(A2AMessageKinds::Ack),
            id: MessageId::new(),
            status: AckStatus::Ok,
            thread: Thread::new(),
        }
    }

    pub fn set_status(mut self, status: AckStatus) -> Ack {
        self.status = status;
        self
    }

    pub fn set_thread(mut self, thread: Thread) -> Ack {
        self.thread = thread;
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Ack(self.clone()) // TODO: THINK how to avoid clone
    }
}