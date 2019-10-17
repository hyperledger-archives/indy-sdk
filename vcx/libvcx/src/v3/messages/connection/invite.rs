use v3::messages::{MessageType, A2AMessageKinds};

#[derive(Debug, Deserialize, Serialize)]
pub struct Invitation {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: String,
    pub label: String,
    #[serde(rename = "recipientKeys")]
    pub recipient_keys: Vec<String>,
    #[serde(rename = "routingKeys")]
    pub routing_keys: Vec<String>,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

impl Invitation {
    pub fn create() -> Invitation {
        Invitation {
            msg_type: MessageType::build(A2AMessageKinds::Invitation),
            id: String::new(),
            label: String::new(),
            service_endpoint: String::new(),
            recipient_keys: Vec::new(),
            routing_keys: Vec::new(),
        }
    }

    pub fn set_label(mut self, label: String) -> Invitation {
        self.label = label;
        self
    }

    pub fn set_service_endpoint(mut self, service_endpoint: String) -> Invitation {
        self.service_endpoint = service_endpoint;
        self
    }

    pub fn set_recipient_keys(mut self, recipient_keys: Vec<String>) -> Invitation {
        self.recipient_keys = recipient_keys;
        self
    }

    pub fn set_routing_keys(mut self, routing_keys: Vec<String>) -> Invitation {
        self.routing_keys = routing_keys;
        self
    }
}