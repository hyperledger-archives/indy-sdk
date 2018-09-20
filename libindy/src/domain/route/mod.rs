pub struct Payload {
    pub iv: Vec<u8>,
    pub tag: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub sym_key: Vec<u8>
}

pub struct AMESData {
    pub header: Header,
    pub cek: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub iv: Vec<u8>,
    pub tag: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AMESJson {
    pub recipients: Vec<Recipient>,
    pub ciphertext: String,
    pub iv: String,
    pub tag: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipient {
    pub header : Header,
    pub cek: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub typ : String,
    pub alg : String,
    pub enc : String,
    pub kid : String,
    pub jwk : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AMESCompact {
    pub header : Header,
    pub cek : String,
    pub iv : String,
    pub ciphertext : String,
    pub tag : String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JWM {
    JWMFull(AMESJson),
    JWMCompact(AMESCompact)
}

impl Header {
    pub fn new_authcrypt_header(recipient_vk: &str, sender_vk: &str) -> Header {
        Header {
            typ: String::from("x-b64nacl"),
            alg: String::from("x-auth"),
            enc: String::from("xsalsa20poly1305"),
            kid: String::from(recipient_vk),
            jwk: Some(String::from(sender_vk)),
        }
    }

    pub fn new_anoncrypt_header(recipient_vk: &str) -> Header {
        Header {
            typ: String::from("x-b64nacl"),
            alg: String::from("x-anon"),
            enc: String::from("xsalsa20poly1305"),
            kid: String::from(recipient_vk),
            jwk: Some(String::from("")),
        }
    }
}

impl Recipient {
    pub fn new(header : Header, cek: String) -> Recipient {
        Recipient {
            header,
            cek: cek
        }
    }
}