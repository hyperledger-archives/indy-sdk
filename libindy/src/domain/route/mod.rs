#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct EncHeader {
    pub from: String,
    pub cek_nonce: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AuthRecipient {
    pub enc_header : String,
    pub cek: String,
    pub to : String
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AnonRecipient {
    pub cek: String,
    pub to: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AuthAMES {
    pub recipients: Vec<AuthRecipient>,
    pub ver: String,
    pub enc: String,
    pub ciphertext: String,
    pub iv: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AnonAMES {
    pub recipients: Vec<AnonRecipient>,
    pub ver: String,
    pub enc: String,
    pub ciphertext: String,
    pub iv: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum AMES {
    Auth(AuthAMES),
    Anon(AnonAMES)
}