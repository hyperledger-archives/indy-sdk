#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct AuthRecipient {
    pub enc_from : String,
    pub e_cek: String,
    pub cek_nonce: String,
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct JWE {
    pub protected: Protected,
    pub recipients: Vec<Recipient>,
    pub aad: String,
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
    pub sender: String,
    pub kid: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Protected {
    pub enc: String,
    pub typ: String,
    pub aad_hash_alg: String,
    pub cek_enc: String
}