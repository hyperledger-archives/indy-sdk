use messages::thread::Thread;
use v3::messages::a2a::{MessageId, A2AMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PingResponse {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

impl PingResponse {
    pub fn create() -> PingResponse {
        PingResponse::default()
    }

    pub fn set_comment(mut self, comment: String) -> PingResponse {
        self.comment = Some(comment);
        self
    }
}

threadlike!(PingResponse);
a2a_message!(PingResponse);

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
            .set_thread_id(&_thread_id());

        assert_eq!(_ping_response(), ping_response);
    }
}