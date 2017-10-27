use utils::json::{JsonEncodable, JsonDecodable};

#[derive(Serialize, Deserialize)]
pub struct KeyInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

impl KeyInfo {
    pub fn new(seed: Option<String>, crypto_type: Option<String>) -> KeyInfo {
        KeyInfo {
            seed: seed,
            crypto_type: crypto_type
        }
    }
}

impl JsonEncodable for KeyInfo {}

impl<'a> JsonDecodable<'a> for KeyInfo {}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyDidInfo {
    pub did: Option<String>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>,
    pub cid: Option<bool>
}

impl MyDidInfo {
    pub fn new(did: Option<String>, seed: Option<String>,
               crypto_type: Option<String>, cid: Option<bool>) -> MyDidInfo {
        MyDidInfo {
            did: did,
            seed: seed,
            crypto_type: crypto_type,
            cid: cid
        }
    }
}

impl JsonEncodable for MyDidInfo {}

impl<'a> JsonDecodable<'a> for MyDidInfo {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheirDidInfo {
    pub did: String,
    pub verkey: Option<String>
}

impl TheirDidInfo {
    pub fn new(did: String, verkey: Option<String>) -> TheirDidInfo {
        TheirDidInfo {
            did: did,
            verkey: verkey
        }
    }
}

impl JsonEncodable for TheirDidInfo {}

impl<'a> JsonDecodable<'a> for TheirDidInfo {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Key {
    pub verkey: String,
    pub signkey: String
}

impl Key {
    pub fn new(verkey: String, signkey: String) -> Key {
        Key {
            verkey: verkey,
            signkey: signkey
        }
    }
}

impl JsonEncodable for Key {}

impl<'a> JsonDecodable<'a> for Key {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Did {
    pub did: String,
    pub verkey: String
}

impl Did {
    pub fn new(did: String, verkey: String) -> Did {
        Did {
            did: did,
            verkey: verkey
        }
    }
}

impl JsonEncodable for Did {}

impl<'a> JsonDecodable<'a> for Did {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Endpoint {
    pub ha: String,
    pub verkey: String
}

impl Endpoint {
    pub fn new(ha: String, verkey: String) -> Endpoint {
        Endpoint {
            ha: ha,
            verkey: verkey
        }
    }
}

impl JsonEncodable for Endpoint {}

impl<'a> JsonDecodable<'a> for Endpoint {}