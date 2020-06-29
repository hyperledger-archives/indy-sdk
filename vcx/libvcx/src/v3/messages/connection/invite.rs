use v3::messages::a2a::{A2AMessage, MessageId};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Invitation {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub label: String,
    #[serde(rename = "recipientKeys")]
    pub recipient_keys: Vec<String>,
    #[serde(default)]
    #[serde(rename = "routingKeys")]
    pub routing_keys: Vec<String>,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
    // TODO: Create two types of invitation with traits
    #[serde(rename = "did")]
    pub did: Option<String>
}

impl Invitation {
    pub fn create() -> Invitation {
        Invitation::default()
    }

    pub fn set_label(mut self, label: String) -> Invitation {
        self.label = label;
        self
    }

    pub fn set_id(mut self, id: String) -> Invitation {
        self.id = MessageId(id);
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
    
    pub fn set_public_did(mut self, public_did: Option<String>) -> Invitation {
        self.did = public_did;
        self
    }
}

a2a_message!(Invitation, ConnectionInvitation);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::did_doc::tests::*;

    pub fn _invitation() -> Invitation {
        Invitation {
            id: MessageId::id(),
            label: _label(),
            recipient_keys: _recipient_keys(),
            routing_keys: _routing_keys(),
            service_endpoint: _service_endpoint(),
            did: None
        }
    }

    pub fn _public_invitation(public_did: &str) -> Invitation {
        Invitation {
            id: MessageId::id(),
            label: _label(),
            recipient_keys: vec!["".to_string()],
            routing_keys: vec!["".to_string()],
            service_endpoint: "".to_string(),
            did: Some(public_did.to_string())
        }
    }

    pub fn _invitation_json() -> String {
        ::serde_json::to_string(&_invitation().to_a2a_message()).unwrap()
    }

    #[test]
    fn test_request_build_works() {
        let invitation: Invitation = Invitation::default()
            .set_label(_label())
            .set_service_endpoint(_service_endpoint())
            .set_recipient_keys(_recipient_keys())
            .set_routing_keys(_routing_keys());

        assert_eq!(_invitation(), invitation);
    }
}
