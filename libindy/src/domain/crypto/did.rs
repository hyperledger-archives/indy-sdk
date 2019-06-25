use named_type::NamedType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyDidInfo {
    pub did: Option<String>,
    pub seed: Option<String>,
    pub crypto_type: Option<String>,
    pub cid: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheirDidInfo {
    pub did: String,
    pub verkey: Option<String>
}

impl TheirDidInfo {
    pub fn new(did: String, verkey: Option<String>) -> TheirDidInfo {
        TheirDidInfo {
            did,
            verkey
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, NamedType)]
pub struct Did {
    pub did: String,
    pub verkey: String
}

impl Did {
    pub fn new(did: String, verkey: String) -> Did {
        Did {
            did,
            verkey
        }
    }
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct DidMetadata {
    pub value: String
}

#[derive(Serialize, Clone, Debug, NamedType)]
#[serde(rename_all = "camelCase")]
pub struct DidWithMeta {
    pub did: String,
    pub verkey: String,
    pub temp_verkey: Option<String>,
    pub metadata: Option<String>
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TheirDid {
    pub did: String,
    pub verkey: String
}

#[derive(Serialize, Deserialize, Debug, NamedType)]
pub struct TemporaryDid {
    pub did: String,
    pub verkey: String
}

impl From<TemporaryDid> for Did {
    fn from(temp_did: TemporaryDid) -> Self {
        Did {
            did: temp_did.did,
            verkey: temp_did.verkey
        }
    }
}

