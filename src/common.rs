pub enum SovrinRole {
    None,
    User,
    TrustAnchor,
    Steward,
    Trustee
}

pub struct SovrinIdentity {
    did: String,
    verkey: Option(String),
    private_key: Option(String)
}