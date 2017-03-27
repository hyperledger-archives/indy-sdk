pub enum SovrinRole {
    None,
    User,
    TrustAnchor,
    Steward,
    Trustee
}

pub struct SovrinIdentity {
    did: String,
    verkey: Option<String>,
    private_key: Option<String>
}

impl SovrinIdentity {
    pub fn new(did: &str, verkey: Option<&str>, private_key: Option<&str>) -> SovrinIdentity {
        SovrinIdentity {
            did: did.to_string(),
            verkey: verkey.map(String::from),
            private_key: private_key.map(String::from)
        }
    }
}