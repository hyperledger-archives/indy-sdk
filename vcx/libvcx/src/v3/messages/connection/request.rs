use v3::messages::A2AMessage;
use v3::messages::connection::did_doc::*;
use v3::messages::{MessageType, A2AMessageKinds};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: String,
    pub label: String,
    pub connection: ConnectionData
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConnectionData {
    pub did: String,
    pub did_doc: DidDoc,
}

impl Request {
    pub fn create() -> Request {
        Request {
            msg_type: MessageType::build(A2AMessageKinds::Request),
            id: String::new(),
            label: String::new(),
            connection: ConnectionData {
                did: String::new(),
                did_doc: DidDoc {
                    context: String::new(),
                    id: String::new(),
                    public_key: vec![PublicKey {
                        id: String::new(),
                        type_: String::new(),
                        owner: String::new(),
                        public_key_pem: String::new(),
                    }],
                    authentication: vec![Authentication {
                        type_: String::new(),
                        public_key: String::new()
                    }],
                    service: vec![Service {
                        id: String::new(),
                        type_: String::new(),
                        priority: 0,
                        service_endpoint: String::new(),
                        recipient_keys: Vec::new(),
                        routing_keys: Vec::new(),
                    }],
                }
            }
        }
    }

    pub fn set_did(mut self, did: String) -> Request {
        self.connection.did = did;
        self
    }

    pub fn set_label(mut self, label: String) -> Request {
        self.label = label;
        self
    }

    pub fn set_service_endpoint(mut self, service_endpoint: String) -> Request {
        self.connection.did_doc.service.get_mut(0)
            .map(|service| {
                service.service_endpoint = service_endpoint;
                service
            });
        self
    }

    pub fn set_recipient_keys(mut self, recipient_keys: Vec<String>) -> Request {
        self.connection.did_doc.service.get_mut(0)
            .map(|service| {
                service.recipient_keys = recipient_keys;
                service
            });
        self
    }

    pub fn set_routing_keys(mut self, routing_keys: Vec<String>) -> Request {
        self.connection.did_doc.service.get_mut(0)
            .map(|service| {
                service.routing_keys = routing_keys;
                service
            });
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage{
        A2AMessage::ConnectionRequest(self.clone())
    }
}