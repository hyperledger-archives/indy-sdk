use messages::thread::Thread;
use v3::messages::a2a::{MessageId, A2AMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ack {
    #[serde(rename = "@id")]
    pub id: MessageId,
    status: AckStatus,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        Ack::default()
    }

    pub fn set_status(mut self, status: AckStatus) -> Ack {
        self.status = status;
        self
    }

    pub fn set_thread_id(mut self, id: String) -> Self {
        self.thread.thid = Some(id);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Ack(self.clone()) // TODO: THINK how to avoid clone
    }
}

impl Default for Ack {
    fn default() -> Ack {
        Ack {
            id: MessageId::new(),
            status: AckStatus::Ok,
            thread: Thread::new(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    pub fn _ack() -> Ack {
        Ack {
            id: MessageId::id(),
            status: AckStatus::Fail,
            thread: _thread(),
        }
    }

    #[test]
    fn test_ack_build_works() {
        let ack: Ack = Ack::default()
            .set_status(AckStatus::Fail)
            .set_thread_id(_thread_id());

        assert_eq!(_ack(), ack);
    }
}