#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct JWE {
    pub protected: String,
    pub iv: String,
    pub ciphertext: String,
    pub tag: String

}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Recipient {
    pub encrypted_key: String,
    pub header: Header
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Header {
    pub kid: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Protected {
    pub enc: String,
    pub typ: String,
    pub alg: String,
    pub recipients: Vec<Recipient>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UnpackMessage {
    pub message: String,
    pub recipient_verkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_verkey: Option<String>
}