pub const CONTEXT: &str = "https://w3id.org/did/v1";
pub const AUTHENTICATION_TYPE: &str = "Ed25519VerificationKey2018";
pub const PUBLIC_KEY_TYPE: &str = "Ed25519VerificationKey2018";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DidDoc {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "publicKey")]
    pub public_key: Vec<PublicKey>,
    pub authentication: Vec<Authentication>,
    pub service: Vec<Service>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PublicKey {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub owner: String,
    #[serde(rename = "publicKeyPem")]
    pub public_key_pem: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Authentication {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub priority: u32,
    #[serde(rename = "recipientKeys")]
    pub recipient_keys: Vec<String>,
    #[serde(rename = "routingKeys")]
    pub routing_keys: Vec<String>,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

impl DidDoc {
    pub fn set_service_endpoint(&mut self, service_endpoint: String) {
        self.service.get_mut(0)
            .map(|service| {
                service.service_endpoint = service_endpoint;
                service
            });
    }

    // TODO: use key references in DID Doc instead of copy
    pub fn set_recipient_keys(&mut self, recipient_keys: Vec<String>) {
        recipient_keys
            .iter()
            .for_each(|key| {
                self.authentication.push(
                    Authentication {
                        type_: String::from(AUTHENTICATION_TYPE),
                        public_key: key.clone()
                    });

                self.public_key.push(
                    PublicKey {
                        id: key.clone(),
                        type_: String::from(PUBLIC_KEY_TYPE),
                        owner: self.id.clone(),
                        public_key_pem: String::new(),
                        // TODO public_key_pem
                    });
            });

        self.service.get_mut(0)
            .map(|service| {
                service.recipient_keys = recipient_keys;
                service
            });
    }

    // TODO: use key references in DID Doc instead of copy
    pub fn set_routing_keys(&mut self, routing_keys: Vec<String>) {
        routing_keys
            .iter()
            .for_each(|key| {
                self.authentication.push(
                    Authentication {
                        type_: String::from(AUTHENTICATION_TYPE),
                        public_key: key.clone()
                    });
            });

        self.service.get_mut(0)
            .map(|service| {
                service.routing_keys = routing_keys;
                service
            });
    }
}
