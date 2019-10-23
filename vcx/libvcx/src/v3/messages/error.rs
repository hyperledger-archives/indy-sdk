use v3::messages::MessageId;
use v3::messages::MessageType;
use messages::thread::Thread;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemReport {
    #[serde(rename = "@type")]
    msg_type: MessageType,
    #[serde(rename = "@id")]
    id: MessageId,
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>,
    pub description: Description,
    pub who_retries: Option<WhoRetries>,
    #[serde(rename = "tracking-uri")]
    pub tracking_uri: Option<String>,
    #[serde(rename = "escalation-uri")]
    pub escalation_uri: Option<String>,
    #[serde(rename = "fix-hint")]
    pub fix_hint: Option<FixHint>,
    pub impact: Option<Impact>,
    pub noticed_time: Option<String>,
    #[serde(rename = "where")]
    pub location: Option<Location>,
    pub problem_items: Option<HashMap<String, String>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub en: Option<String>,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WhoRetries {
    #[serde(rename = "me")]
    Me,
    #[serde(rename = "you")]
    You,
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "none")]
    None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixHint {
    en: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Impact {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "thread")]
    Thread,
    #[serde(rename = "connection")]
    Connection
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
    You(SpecificLocation),
    Me(SpecificLocation),
    Other(SpecificLocation)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecificLocation {
    Cloud,
    Edge,
    Wire,
    Agency
}

// TODO: deserializer for Location:
// A string that describes where the error happened, from the perspective of the reporter,
// and that uses the “you” or “me” or “other” prefix,
// followed by a suffix like “cloud”, “edge”, “wire”, “agency”, etc.