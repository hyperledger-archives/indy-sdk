use v3::messages::a2a::{A2AMessage, MessageId};
use v3::messages::mime_type::MimeType;
use messages::thread::Thread;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PresentationProposal {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub presentation_proposal: PresentationPreview,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PresentationPreview {
    pub attributes: Vec<Attribute>,
    pub predicates: Vec<Predicate>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub cred_def_id: Option<String>,
    #[serde(rename = "mime-type")]
    pub mime_type: Option<MimeType>,
    pub value: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Predicate {
    pub name: String,
    pub cred_def_id: Option<String>,
    pub predicate: String,
    pub threshold: i64,
    pub filter: Vec<::serde_json::Value>
}

impl Default for PresentationProposal {
    fn default() -> PresentationProposal {
        PresentationProposal {
            id: MessageId::new(),
            comment: None,
            presentation_proposal: PresentationPreview::default(),
            thread: Thread::new(),
        }
    }
}

impl Default for PresentationPreview {
    fn default() -> PresentationPreview {
        PresentationPreview {
            attributes: Vec::new(),
            predicates: Vec::new(),
        }
    }
}

impl PresentationProposal {
    pub fn create() -> Self {
        PresentationProposal::default()
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_presentation_preview(mut self, presentation_preview: PresentationPreview) -> PresentationProposal {
        self.presentation_proposal = presentation_preview;
        self
    }

    pub fn set_thread_id(mut self, id: String) -> Self {
        self.thread.thid = Some(id);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::PresentationProposal(self.clone()) // TODO: THINK how to avoid clone
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::proof_presentation::presentation_request::tests::{thread, thread_id};

    fn _attachment() -> ::serde_json::Value {
        json!({"presentation": {}})
    }

    fn _comment() -> String {
        String::from("comment")
    }

    fn _presentation_preview() -> PresentationPreview {
        PresentationPreview {
            attributes: vec![Attribute {
                name: String::from("name"),
                cred_def_id: None,
                mime_type: None,
                value: None,
            }],
            predicates: vec![]
        }
    }

    pub fn _presentation_proposal() -> PresentationProposal {
        PresentationProposal {
            id: MessageId::id(),
            comment: Some(_comment()),
            thread: thread(),
            presentation_proposal: _presentation_preview()
        }
    }

    #[test]
    fn test_presentation_proposal_build_works() {
        let presentation_proposal: PresentationProposal = PresentationProposal::default()
            .set_comment(_comment())
            .set_thread_id(thread_id())
            .set_presentation_preview(_presentation_preview());

        assert_eq!(_presentation_proposal(), presentation_proposal);
    }
}
