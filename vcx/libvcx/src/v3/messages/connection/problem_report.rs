use v3::messages::{MessageType, MessageId, A2AMessage, A2AMessageKinds};
use messages::thread::Thread;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemReport {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(rename = "problem-code")]
    pub problem_code: ProblemCode,
    pub explain: String,
    #[serde(rename = "~l10n")]
    pub localization: Localization,
    #[serde(rename = "~thread")]
    pub thread: Thread
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProblemCode {
    Empty,
    #[serde(rename = "request_not_accepted")]
    RequestNotAccepted,
    #[serde(rename = "request_processing_error")]
    RequestProcessingError,
    #[serde(rename = "response_not_accepted")]
    ResponseNotAccepted,
    #[serde(rename = "response_processing_error")]
    ResponseProcessingError
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Localization {
    locale: Locales
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Locales {
    #[serde(rename = "en")]
    En,
}

impl ProblemReport {
    pub fn create() -> ProblemReport {
        ProblemReport::default()
    }
}

impl ProblemReport {
    pub fn set_id(mut self, id: MessageId) -> ProblemReport {
        self.id = id;
        self
    }

    pub fn set_problem_code(mut self, problem_code: ProblemCode) -> ProblemReport {
        self.problem_code = problem_code;
        self
    }

    pub fn set_explain(mut self, explain: String) -> ProblemReport {
        self.explain = explain;
        self
    }

    pub fn set_thread(mut self, thread: Thread) -> ProblemReport {
        self.thread = thread;
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::ConnectionProblemReport(self.clone()) // TODO: THINK how to avoid clone
    }
}

impl Default for ProblemReport {
    fn default() -> ProblemReport {
        ProblemReport {
            msg_type: MessageType::build(A2AMessageKinds::ConnectionProblemReport),
            id: MessageId::new(),
            problem_code: ProblemCode::Empty,
            explain: String::new(),
            localization: Localization::default(),
            thread: Thread::new(),
        }
    }
}

impl Default for Localization {
    fn default() -> Localization {
        Localization { locale: Locales::En }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _id() -> MessageId {
        MessageId(String::from("testid"))
    }

    fn _problem_code() -> ProblemCode {
        ProblemCode::ResponseProcessingError
    }

    fn _explain() -> String {
        String::from("test explanation")
    }

    fn _problem_report() -> ProblemReport {
        ProblemReport {
            msg_type: MessageType::build(A2AMessageKinds::ConnectionProblemReport),
            id:  _id(),
            problem_code: _problem_code(),
            explain: _explain(),
            localization: Localization::default(),
            thread: _thread(),
        }
    }

    #[test]
    fn test_problem_report_build_works() {
        let report: ProblemReport = ProblemReport::default()
            .set_id(_id())
            .set_problem_code(_problem_code())
            .set_explain(_explain())
            .set_thread(_thread());

        assert_eq!(_problem_report(), report);
    }
}