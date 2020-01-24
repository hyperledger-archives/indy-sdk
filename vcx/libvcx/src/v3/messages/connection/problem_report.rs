use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::localization::Localization;
use messages::thread::Thread;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProblemReport {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(rename = "problem-code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_code: Option<ProblemCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain: Option<String>,
    #[serde(rename = "~l10n")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Localization>,
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

impl ProblemReport {
    pub fn create() -> ProblemReport {
        ProblemReport::default()
    }
}

impl ProblemReport {
    pub fn set_problem_code(mut self, problem_code: ProblemCode) -> ProblemReport {
        self.problem_code = Some(problem_code);
        self
    }

    pub fn set_explain(mut self, explain: String) -> ProblemReport {
        self.explain = Some(explain);
        self
    }
}

threadlike!(ProblemReport);
a2a_message!(ProblemReport, ConnectionProblemReport);

impl Default for ProblemCode {
    fn default() -> ProblemCode {
        ProblemCode::Empty
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::response::tests::*;

    fn _problem_code() -> ProblemCode {
        ProblemCode::ResponseProcessingError
    }

    fn _explain() -> String {
        String::from("test explanation")
    }

    pub fn _problem_report() -> ProblemReport {
        ProblemReport {
            id: MessageId::id(),
            problem_code: Some(_problem_code()),
            explain: Some(_explain()),
            localization: None,
            thread: _thread(),
        }
    }

    #[test]
    fn test_problem_report_build_works() {
        let report: ProblemReport = ProblemReport::default()
            .set_problem_code(_problem_code())
            .set_explain(_explain())
            .set_thread_id(&_thread_id());

        assert_eq!(_problem_report(), report);
    }
}
