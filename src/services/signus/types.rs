use utils::json::{JsonEncodable, JsonDecodable};

#[derive(Serialize, Deserialize, Clone)]
pub struct MyDidInfo {
    pub did: Option<String>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

impl MyDidInfo {
    pub fn new(did: Option<String>, seed: Option<String>, crypto_type: Option<String>) -> MyDidInfo {
        MyDidInfo {
            did: did,
            seed: seed,
            crypto_type: crypto_type
        }
    }
}

impl JsonEncodable for MyDidInfo {}

impl<'a> JsonDecodable<'a> for MyDidInfo {}

#[derive(Serialize, Deserialize)]
pub struct MyIdentityInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

impl MyIdentityInfo {
    pub fn new(seed: Option<String>, crypto_type: Option<String>) -> MyIdentityInfo {
        MyIdentityInfo {
            seed: seed,
            crypto_type: crypto_type
        }
    }
}

impl JsonEncodable for MyIdentityInfo {}

impl<'a> JsonDecodable<'a> for MyIdentityInfo {}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyDid {
    pub did: String,
    pub crypto_type: String,
    pub public_key: String,
    pub secret_key: String,
    pub ver_key: String,
    pub sign_key: String
}

impl MyDid {
    pub fn new(did: String, crypto_type: String, public_key: String, secret_key: String, ver_key: String, sign_key: String) -> MyDid {
        MyDid {
            did: did,
            crypto_type: crypto_type,
            public_key: public_key,
            secret_key: secret_key,
            ver_key: ver_key,
            sign_key: sign_key
        }
    }
}

impl JsonEncodable for MyDid {}

impl<'a> JsonDecodable<'a> for MyDid {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheirDid {
    pub did: String,
    pub crypto_type: Option<String>,
    pub pk: Option<String>,
    pub verkey: Option<String>,
    pub endpoint: Option<String>,
}

impl TheirDid {
    pub fn new(did: String, crypto_type: Option<String>, pk: Option<String>, verkey: Option<String>, endpoint: Option<String>) -> TheirDid {
        TheirDid {
            did: did,
            crypto_type: crypto_type,
            pk: pk,
            verkey: verkey,
            endpoint: endpoint,
        }
    }
}

impl JsonEncodable for TheirDid {}

impl<'a> JsonDecodable<'a> for TheirDid {}