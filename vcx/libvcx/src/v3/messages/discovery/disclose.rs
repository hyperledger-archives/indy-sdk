use messages::thread::Thread;
use settings::Actors;
use v3::messages::a2a::{MessageId, A2AMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Disclose {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub protocols: Vec<ProtocolDescriptor>,
    #[serde(rename = "~thread")]
    pub thread: Thread
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProtocolDescriptor {
    pub pid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Actors>>
}

impl Disclose {
    pub fn create() -> Disclose {
        Disclose::default()
    }

    pub fn set_protocols(mut self, protocols: Vec<ProtocolDescriptor>) -> Self {
        self.protocols = protocols;
        self
    }

    pub fn add_protocol(&mut self, protocol: ProtocolDescriptor) {
        self.protocols.push(protocol);
    }

    pub fn set_thread_id(mut self, id: String) -> Self {
        self.thread.thid = Some(id);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Disclose(self.clone()) // TODO: THINK how to avoid clone
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _protocol_descriptor() -> ProtocolDescriptor {
        ProtocolDescriptor { pid: String::from("did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/"), roles: None }
    }

    pub fn _disclose() -> Disclose {
        Disclose {
            id: MessageId::id(),
            protocols: vec![_protocol_descriptor()],
            thread: _thread(),
        }
    }

    #[test]
    fn test_disclose_build_works() {
        let mut disclose: Disclose = Disclose::default()
            .set_thread_id(_thread_id());

        disclose.add_protocol(_protocol_descriptor());

        assert_eq!(_disclose(), disclose);
    }
}