use v3::messages::connection::did_doc::DidDoc;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(default)]
    pub recipient_keys: Vec<String>,
    pub routing_keys: Option<Vec<String>>,
    pub service_endpoint: String,
}

impl Service {
    pub fn create() -> Self {
        Service::default()
    }

    pub fn set_service_endpoint(mut self, service_endpoint: String) -> Self {
        self.service_endpoint = service_endpoint;
        self
    }

    pub fn set_routing_keys(mut self, routing_keys: Vec<String>) -> Self {
        self.routing_keys = Some(routing_keys);
        self
    }

    pub fn set_recipient_keys(mut self, recipient_keys: Vec<String>) -> Self {
        self.recipient_keys = recipient_keys;
        self
    }
}

impl Into<DidDoc> for Service {
    fn into(self) -> DidDoc {
        let mut did_doc: DidDoc = DidDoc::default();
        did_doc.set_service_endpoint(self.service_endpoint.clone());
        did_doc.set_keys(self.recipient_keys, self.routing_keys.unwrap_or_default());
        did_doc
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::did_doc::tests::{_service_endpoint, _recipient_keys, _routing_keys};

    pub fn _service() -> Service {
        Service {
            recipient_keys: _recipient_keys(),
            routing_keys: Some(_routing_keys()),
            service_endpoint: _service_endpoint(),
        }
    }

    #[test]
    fn test_service_build_works() {
        let service: Service = Service::default()
            .set_service_endpoint(_service_endpoint())
            .set_recipient_keys(_recipient_keys())
            .set_routing_keys(_routing_keys());

        assert_eq!(_service(), service);
    }
}