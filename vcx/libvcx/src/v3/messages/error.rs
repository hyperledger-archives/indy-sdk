use v3::messages::a2a::{MessageId, A2AMessage};
use messages::thread::Thread;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemReport {
    #[serde(rename = "@id")]
    id: MessageId,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
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
    pub problem_items: Option<HashMap<String, String>>,
    pub comment: Option<String>
}

impl ProblemReport {
    pub fn create() -> Self {
        ProblemReport::default()
    }

    pub fn set_description(mut self, code: u32) -> Self {
        self.description = Some(Description {
            en: None,
            code
        });
        self
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_thread_id(mut self, id: String) -> Self {
        self.thread.thid = Some(id);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::CommonProblemReport(self.clone()) // TODO: THINK how to avoid clone
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

impl Default for ProblemReport {
    fn default() -> ProblemReport {
        ProblemReport {
            id: MessageId::new(),
            thread: Thread::new(),
            description: None,
            who_retries: None,
            tracking_uri: None,
            escalation_uri: None,
            fix_hint: None,
            impact: None,
            noticed_time: None,
            location: None,
            problem_items: None,
            comment: None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _code() -> u32 { 0 }

    fn _comment() -> String {
        String::from("test comment")
    }

    pub fn _problem_report() -> ProblemReport {
        ProblemReport {
            id: MessageId::id(),
            thread: _thread(),
            description: Some(Description { en: None, code: _code() }),
            who_retries: None,
            tracking_uri: None,
            escalation_uri: None,
            fix_hint: None,
            impact: None,
            noticed_time: None,
            location: None,
            problem_items: None,
            comment: Some(_comment()),
        }
    }

    #[test]
    fn test_problem_report_build_works() {
        let report: ProblemReport = ProblemReport::default()
            .set_comment(_comment())
            .set_thread_id(_thread_id())
            .set_description(_code());

        assert_eq!(_problem_report(), report);
    }
}