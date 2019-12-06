#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum MessageFamilies {
    Routing,
    DidExchange,
    Notification,
    Signature,
    CredentialIssuance,
    ReportProblem,
    PresentProof,
    TrustPing,
    Unknown(String)
}

impl MessageFamilies {
    pub fn version(&self) -> &'static str {
        match self {
            MessageFamilies::Routing => "1.0",
            MessageFamilies::DidExchange => "1.0",
            MessageFamilies::Notification => "1.0",
            MessageFamilies::Signature => "1.0",
            MessageFamilies::CredentialIssuance => "1.0",
            MessageFamilies::ReportProblem => "1.0",
            MessageFamilies::PresentProof => "1.0",
            MessageFamilies::TrustPing => "1.0",
            MessageFamilies::Unknown(_) => "1.0"
        }
    }
}

impl From<String> for MessageFamilies {
    fn from(family: String) -> Self {
        match family.as_str() {
            "routing" => MessageFamilies::Routing,
            "connections" => MessageFamilies::DidExchange, // TODO: should be didexchange
            "signature" => MessageFamilies::Signature,
            "notification" => MessageFamilies::Notification,
            "issue-credential" => MessageFamilies::CredentialIssuance,
            "report-problem" => MessageFamilies::ReportProblem,
            "present-proof" => MessageFamilies::PresentProof,
            "trust_ping" => MessageFamilies::TrustPing,
            family @ _ => MessageFamilies::Unknown(family.to_string())
        }
    }
}

impl ::std::string::ToString for MessageFamilies {
    fn to_string(&self) -> String {
        match self {
            MessageFamilies::Routing => "routing".to_string(),
            MessageFamilies::DidExchange => "connections".to_string(), // TODO: should be didexchange
            MessageFamilies::Notification => "notification".to_string(),
            MessageFamilies::Signature => "signature".to_string(),
            MessageFamilies::CredentialIssuance => "issue-credential".to_string(),
            MessageFamilies::ReportProblem => "report-problem".to_string(),
            MessageFamilies::PresentProof => "present-proof".to_string(),
            MessageFamilies::TrustPing => "trust_ping".to_string(),
            MessageFamilies::Unknown(family) => family.to_string()
        }
    }
}