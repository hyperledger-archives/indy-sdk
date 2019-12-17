use messages::thread::Thread;
use v3::messages::a2a::{MessageId, A2AMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Ping {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(default)]
    pub response_requested: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "~thread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
}

impl Ping {
    pub fn create() -> Ping {
        Ping::default()
    }

    pub fn set_thread_id(mut self, id: String) -> Self {
        self.thread = Some(Thread::new().set_thid(id));
        self
    }

    pub fn set_comment(mut self, comment: Option<String>) -> Ping {
        self.comment = comment;
        self
    }

    pub fn request_response(mut self) -> Ping {
        self.response_requested = true;
        self
    }
}

a2a_message!(Ping);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _comment() -> String {
        String::from("comment")
    }

    pub fn _ping() -> Ping {
        Ping {
            id: MessageId::id(),
            response_requested: false,
            thread: Some(_thread()),
            comment: Some(_comment()),
        }
    }

    #[test]
    fn test_ping_build_works() {
        let ping: Ping = Ping::default()
            .set_comment(Some(_comment()))
            .set_thread_id(_thread_id());

        assert_eq!(_ping(), ping);
    }
}