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
