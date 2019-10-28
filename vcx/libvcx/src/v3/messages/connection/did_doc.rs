use error::prelude::*;
use url::Url;

pub const CONTEXT: &str = "https://w3id.org/did/v1";
pub const KEY_TYPE: &str = "Ed25519VerificationKey2018";
pub const KEY_AUTHENTICATION_TYPE: &str = "Ed25519SignatureAuthentication2018";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DidDoc {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "publicKey")]
    pub public_key: Vec<PublicKey>,
    pub authentication: Vec<Authentication>,
    pub service: Vec<Service>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PublicKey {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub controller: String,
    #[serde(rename = "publicKeyBase58")]
    pub public_key_base_58: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Authentication {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub priority: u32,
    #[serde(rename = "recipientKeys")]
    pub recipient_keys: Vec<String>,
    #[serde(default)]
    #[serde(rename = "routingKeys")]
    pub routing_keys: Vec<String>,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

impl Default for DidDoc {
    fn default() -> DidDoc {
        DidDoc {
            context: String::from(CONTEXT),
            id: String::new(),
            public_key: vec![],
            authentication: vec![],
            service: vec![Service {
                // TODO: FIXME Several services????
                id: String::from("did:example:123456789abcdefghi;did-communication"),
                type_: String::from("did-communication"),
                priority: 0,
                service_endpoint: String::new(),
                recipient_keys: Vec::new(),
                routing_keys: Vec::new(),
            }],
        }
    }
}

impl DidDoc {
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_service_endpoint(&mut self, service_endpoint: String) {
        self.service.get_mut(0)
            .map(|service| {
                service.service_endpoint = service_endpoint;
                service
            });
    }

    pub fn set_keys(&mut self, recipient_keys: Vec<String>, routing_keys: Vec<String>) {
        let mut id = 0;

        recipient_keys
            .iter()
            .for_each(|key| {
                id += 1;

                let key_id = id.to_string();
                let key_reference = DidDoc::_build_key_reference(&self.id, &key_id);

                self.public_key.push(
                    PublicKey {
                        id: key_id,
                        type_: String::from(KEY_TYPE),
                        controller: self.id.clone(),
                        public_key_base_58: key.clone(),
                    });

                self.authentication.push(
                    Authentication {
                        type_: String::from(KEY_TYPE),
                        public_key: key_reference.clone()
                    });


                self.service.get_mut(0)
                    .map(|service| {
                        service.recipient_keys.push(key_reference);
                        service
                    });
            });

        routing_keys
            .iter()
            .for_each(|key| {
                id += 1;

                let key_id = id.to_string();
                let key_reference = DidDoc::_build_key_reference(&self.id, &key_id);

                self.public_key.push(
                    PublicKey {
                        id: key_id,
                        type_: String::from(KEY_TYPE),
                        controller: self.id.clone(),
                        public_key_base_58: key.clone(),
                    });

                self.service.get_mut(0)
                    .map(|service| {
                        service.routing_keys.push(key_reference);
                        service
                    });
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
        let id = DidDoc::_parse_key_reference(target_key);

        let key = self.public_key.iter().find(|key_| key_.id == id.to_string() || key_.public_key_base_58 == id.to_string())
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Cannot find PublicKey definition for key: {:?}", id)))?;

        if key.type_ != KEY_TYPE {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Unsupported PublicKey type: {:?}", key.type_)));
        }

        Ok(())
    }

    fn validate_authentication(&self, target_key: &str) -> VcxResult<()> {
        let key = self.authentication.iter().find(|key_| key_.public_key == target_key.to_string())
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Cannot find Authentication section for key: {:?}", target_key)))?;

        if key.type_ != KEY_TYPE {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("DIDDoc validation failed: Unsupported Authentication type: {:?}", key.type_)));
        }

        Ok(())
    }

    pub fn resolve_keys(&self) -> (Vec<String>, Vec<String>) {
        let service: Service = match self.service.get(0).cloned() {
            Some(service) => service,
            None => return (Vec::new(), Vec::new())
        };

        let recipient_keys: Vec<String> =
            service.recipient_keys
                .iter()
                .map(|key| self.key_for_reference(key))
                .collect();

        let routing_keys: Vec<String> =
            service.routing_keys
                .iter()
                .map(|key| self.key_for_reference(key))
                .collect();

        (recipient_keys, routing_keys)
    }

    pub fn get_endpoint(&self) -> String {
        match self.service.get(0) {
            Some(service) => service.service_endpoint.to_string(),
            None => String::new()
        }
    }

    fn key_for_reference(&self, key_reference: &str) -> String {
        let id = DidDoc::_parse_key_reference(key_reference);

        self.public_key.iter().find(|key_| key_.id == id.to_string() || key_.public_key_base_58 == id.to_string())
            .map(|key| key.public_key_base_58.clone())
            .unwrap_or_default()
    }

    fn _build_key_reference(did: &str, id: &str) -> String {
        format!("{}#{}", did, id)
    }

    fn _parse_key_reference(key_reference: &str) -> String {
        let pars: Vec<&str> = key_reference.split("#").collect();
        pars.get(1).or(pars.get(0)).map(|s| s.to_string()).unwrap_or_default()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn _key_1() -> String {
        String::from("GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL")
    }

    pub fn _key_2() -> String {
        String::from("Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR")
    }

    pub fn _key_3() -> String {
        String::from("3LYuxJBJkngDbvJj4zjx13DBUdZ2P96eNybwd2n9L9AU")
    }

    pub fn _id() -> String {
        String::from("VsKV7grR1BUE29mG2Fm2kX")
    }

    pub fn _service_endpoint() -> String {
        String::from("http://localhost:8080")
    }

    pub fn _recipient_keys() -> Vec<String> {
        vec![_key_1()]
    }

    pub fn _routing_keys() -> Vec<String> {
        vec![_key_2(), _key_3()]
    }

    pub fn _key_reference_1() -> String {
        DidDoc::_build_key_reference(&_id(), "1")
    }

    pub fn _key_reference_2() -> String {
        DidDoc::_build_key_reference(&_id(), "2")
    }

    pub fn _key_reference_3() -> String {
        DidDoc::_build_key_reference(&_id(), "3")
    }

    pub fn _label() -> String {
        String::from("test")
    }

    pub fn _did_doc() -> DidDoc {
        DidDoc {
            context: String::from(CONTEXT),
            id: _id(),
            public_key: vec![
                PublicKey { id: "1".to_string(), type_: KEY_TYPE.to_string(), controller: _id(), public_key_base_58: _key_1() },
                PublicKey { id: "2".to_string(), type_: KEY_TYPE.to_string(), controller: _id(), public_key_base_58: _key_2() },
                PublicKey { id: "3".to_string(), type_: KEY_TYPE.to_string(), controller: _id(), public_key_base_58: _key_3() }
            ],
            authentication: vec![
                Authentication { type_: KEY_TYPE.to_string(), public_key: _key_reference_1() }
            ],
            service: vec![Service {
                // TODO: FIXME Several services????
                id: String::from("did:example:123456789abcdefghi;did-communication"),
                type_: String::from("did-communication"),
                priority: 0,
                service_endpoint: _service_endpoint(),
                recipient_keys: vec![_key_reference_1()],
                routing_keys: vec![_key_reference_2(), _key_reference_3()],
            }],
        }
    }

    #[test]
    fn test_did_doc_build_works() {
        let mut did_doc: DidDoc = DidDoc::default();
        did_doc.set_id(_id());
        did_doc.set_service_endpoint(_service_endpoint());
        did_doc.set_keys(_recipient_keys(), _routing_keys());

        assert_eq!(_did_doc(), did_doc);
    }

    #[test]
    fn test_did_doc_validate_works() {
        _did_doc().validate().unwrap()
    }

    #[test]
    fn test_did_doc_key_for_reference_works() {
        assert_eq!(_key_1(), _did_doc().key_for_reference(&_key_reference_1()));
    }

    #[test]
    fn test_did_doc_resolve_keys_works() {
        let (recipient_keys, routing_keys) = _did_doc().resolve_keys();
        assert_eq!(_recipient_keys(), recipient_keys);
        assert_eq!(_routing_keys(), routing_keys);
    }

    #[test]
    fn test_did_doc_build_key_reference_works() {
        assert_eq!(_key_reference_1(), DidDoc::_build_key_reference(&_id(), "1"));
    }

    #[test]
    fn test_did_doc_parse_key_reference_works() {
        assert_eq!(String::from("1"), DidDoc::_parse_key_reference(&_key_reference_1()));
        assert_eq!(_key_1(), DidDoc::_parse_key_reference(&_key_1()));
    }
}
