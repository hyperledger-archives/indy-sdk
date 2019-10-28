use v3::messages::A2AMessage;
use v3::messages::connection::did_doc::*;
use v3::messages::{MessageType, MessageId, A2AMessageKinds};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Request {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub label: String,
    pub connection: ConnectionData
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ConnectionData {
    pub did: String,
    pub did_doc: DidDoc,
}

impl Default for Request {
    fn default() -> Request {
        Request {
            msg_type: MessageType::build(A2AMessageKinds::Request),
            id: MessageId::new(),
            label: String::new(),
            connection: ConnectionData {
                did: String::new(),
                did_doc: DidDoc::default()
            }
        }
    }
}

impl Request {
    pub fn create() -> Request {
        Request::default()
    }

    pub fn set_did(mut self, did: String) -> Request {
        self.connection.did = did.clone();
        self.connection.did_doc.set_id(did);
        self
    }

    pub fn set_id(mut self, id: MessageId) -> Request {
        self.id = id;
        self
    }

    pub fn set_label(mut self, label: String) -> Request {
        self.label = label;
        self
    }

    pub fn set_service_endpoint(mut self, service_endpoint: String) -> Request {
        self.connection.did_doc.set_service_endpoint(service_endpoint);
        self
    }

    pub fn set_keys(mut self, recipient_keys: Vec<String>, routing_keys: Vec<String>) -> Request {
        self.connection.did_doc.set_keys(recipient_keys, routing_keys);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::ConnectionRequest(self.clone()) // TODO: THINK how to avoid clone
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::did_doc::tests::*;

    fn _did() -> String {
        String::from("VsKV7grR1BUE29mG2Fm2kX")
    }

    fn _id() -> MessageId {
        MessageId(String::from("testid"))
    }

    pub fn _request() -> Request {
        Request {
            msg_type: MessageType::build(A2AMessageKinds::Request),
            id: _id(),
            label: _label(),
            connection: ConnectionData {
                did: _did(),
                did_doc: _did_doc()
            },
        }
    }

    #[test]
    fn test_request_build_works() {
        let request: Request = Request::default()
            .set_id(_id())
            .set_did(_did())
            .set_label(_label())
            .set_service_endpoint(_service_endpoint())
            .set_keys(_recipient_keys(), _routing_keys());

        assert_eq!(_request(), request);
    }
}