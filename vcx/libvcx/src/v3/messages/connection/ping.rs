use messages::thread::Thread;
use v3::messages::a2a::{MessageId, A2AMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ping {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(default)]
    pub response_requested: bool,
    comment: Option<String>,
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>,
}

impl Ping {
    pub fn create() -> Ping {
        Ping::default()
    }

    pub fn set_id(mut self, id: MessageId) -> Ping {
        self.id = id;
        self
    }

    pub fn set_thread(mut self, thread: Thread) -> Ping {
        self.thread = Some(thread);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Ping(self.clone()) // TODO: THINK how to avoid clone
    }
}

impl Default for Ping {
    fn default() -> Ping {
        Ping {
            id: MessageId::new(),
            response_requested: false,
            comment: None,
            thread: None,
        }
    }
}