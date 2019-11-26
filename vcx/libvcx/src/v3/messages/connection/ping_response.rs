use messages::thread::Thread;
use v3::messages::a2a::{MessageId, A2AMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PingResponse {
    #[serde(rename = "@id")]
    pub id: MessageId,
    comment: Option<String>,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

impl PingResponse {
    pub fn create() -> PingResponse {
        PingResponse::default()
    }

    pub fn set_thread_id(mut self, id: String) -> Self {
        self.thread.thid = Some(id);
        self
    }

    pub fn set_comment(mut self, comment: String) -> PingResponse {
        self.comment = Some(comment);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::PingResponse(self.clone()) // TODO: THINK how to avoid clone
    }
}

impl Default for PingResponse {
    fn default() -> PingResponse {
        PingResponse {
            id: MessageId::new(),
            comment: None,
            thread: Thread::new(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _comment() -> String {
        String::from("comment")
    }

    pub fn _ping_response() -> PingResponse {
        PingResponse {
            id: MessageId::id(),
            thread: _thread(),
            comment: Some(_comment()),
        }
    }

    #[test]
    fn test_ping_response_build_works() {
        let ping_response: PingResponse = PingResponse::default()
            .set_comment(_comment())
            .set_thread_id(_thread_id());

        assert_eq!(_ping_response(), ping_response);
    }
}