use utils::json::{JsonEncodable, JsonDecodable};

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

#[derive(Serialize, Deserialize)]
pub struct MyKyesInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>
}

impl MyKyesInfo {
    pub fn new(seed: Option<String>, crypto_type: Option<String>) -> MyKyesInfo {
        MyKyesInfo {
            seed: seed,
            crypto_type: crypto_type
        }
    }
}

impl JsonEncodable for MyKyesInfo {}

impl<'a> JsonDecodable<'a> for MyKyesInfo {}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyDid {
    pub did: String,
    pub crypto_type: String,
    pub pk: String,
    pub sk: String,
    pub verkey: String,
    pub signkey: String
}

impl MyDid {
    pub fn new(did: String, crypto_type: String, pk: String, sk: String, verkey: String, signkey: String) -> MyDid {
        MyDid {
            did: did,
            crypto_type: crypto_type,
            pk: pk,
            sk: sk,
            verkey: verkey,
            signkey: signkey
        }
    }
}

impl JsonEncodable for MyDid {}

impl<'a> JsonDecodable<'a> for MyDid {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheirDidInfo {
    pub did: String,
    pub crypto_type: Option<String>,
    pub verkey: Option<String>
}

impl TheirDidInfo {
    pub fn new(did: String, crypto_type: Option<String>, verkey: Option<String>) -> TheirDidInfo {
        TheirDidInfo {
            did: did,
            crypto_type: crypto_type,
            verkey: verkey
        }
    }
}

impl JsonEncodable for TheirDidInfo {}

impl<'a> JsonDecodable<'a> for TheirDidInfo {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheirDid {
    pub did: String,
    pub crypto_type: String,
    pub verkey: Option<String>,
    pub pk: Option<String>
}

impl TheirDid {
    pub fn new(did: String, crypto_type: String, verkey: Option<String>, pk: Option<String>) -> TheirDid {
        TheirDid {
            did: did,
            crypto_type: crypto_type,
            verkey: verkey,
            pk: pk
        }
    }
}

impl JsonEncodable for TheirDid {}

impl<'a> JsonDecodable<'a> for TheirDid {}