use v3::messages::MessageId;
use v3::messages::MessageType;
use messages::thread::Thread;
use std::collections::HashMap;
use v3::messages::A2AMessageKinds;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub location: Option<String>,
    pub problem_items: Option<HashMap<String, String>>
}

impl ProblemReport {
    pub fn create() -> Self {
        ProblemReport {
            msg_type: MessageType::build(A2AMessageKinds::ProblemReport),
            id: MessageId::new(),
            thread: None,
            description: Description {
                en: None,
                code: 0
            },
            who_retries: None,
            tracking_uri: None,
            escalation_uri: None,
            fix_hint: None,
            impact: None,
            noticed_time: None,
            location: None,
            problem_items: None
        }
    }

    pub fn set_description(mut self, code: u32) -> Self {
        self.description = Description {
            en: None,
            code
        };
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Description {
    pub en: Option<String>,
    pub code: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixHint {
    en: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Impact {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "thread")]
    Thread,
    #[serde(rename = "connection")]
    Connection
}
