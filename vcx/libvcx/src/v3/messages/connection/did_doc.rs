use error::prelude::*;
use url::Url;

pub const CONTEXT: &str = "https://w3id.org/did/v1";
pub const KEY_TYPE: &str = "Ed25519VerificationKey2018";
pub const KEY_AUTHENTICATION_TYPE: &str = "Ed25519SignatureAuthentication2018";

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
    #[serde(rename = "publicKeyBase58")]
    pub public_key_base_58: String,
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
                        type_: String::from(KEY_AUTHENTICATION_TYPE),
                        public_key: key.clone()
                    });

                self.public_key.push(
                    PublicKey {
                        id: key.clone(),
                        type_: String::from(KEY_TYPE),
                        owner: self.id.clone(),
                        public_key_base_58: key.clone(),
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
                self.public_key.push(
                    PublicKey {
                        id: key.clone(),
                        type_: String::from(KEY_TYPE),
                        owner: self.id.clone(),
                        public_key_base_58: key.clone(),
                    });
            });

        self.service.get_mut(0)
            .map(|service| {
                service.routing_keys = routing_keys;
                service
            });
    }

    pub fn validate(&self) -> VcxResult<()> {
        for service in self.service.iter() {
            Url::parse(&service.service_endpoint)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Unsupported endpoint: {:?}", service.service_endpoint)))?;

            service.recipient_keys
                .iter()
                .map(|key| {
                    self.validate_public_key(key)
                        .and(self.validate_authentication(key))
                })
                .collect::<VcxResult<()>>()?;

            service.routing_keys
                .iter()
                .map(|key| {
                    self.validate_public_key(key)
                })
                .collect::<VcxResult<()>>()?;
        }

        Ok(())
    }

    fn validate_public_key(&self, target_key: &str) -> VcxResult<()> {
        let key = self.public_key.iter().find(|key_| key_.id == target_key.to_string())
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Cannot find PublicKey definition for key: {:?}", target_key)))?;

        if key.type_ != KEY_TYPE {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Unsupported PublicKey type: {:?}", key.type_)));
        }

        Ok(())
    }

    fn validate_authentication(&self, target_key: &str) -> VcxResult<()> {
        let key = self.authentication.iter().find(|key_| key_.public_key == target_key.to_string())
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Cannot find Authentication section for key: {:?}", target_key)))?;

        if key.type_ != KEY_AUTHENTICATION_TYPE {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Unsupported Authentication type: {:?}", key.type_)));
        }

        Ok(())
    }
}
