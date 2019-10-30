use v3::messages::{MessageId, MessageType, A2AMessage, A2AMessageKinds};
use v3::messages::error::ProblemReport;
use messages::thread::Thread;
use v3::messages::attachment::{
    Attachment,
    Json,
    ENCODING_BASE64
};

use error::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Presentation {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename = "presentations~attach")]
    pub presentations_attach: Attachment,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

impl Presentation {
    pub fn create() -> Self {
        Presentation {
            msg_type: MessageType::build(A2AMessageKinds::Presentation),
            id: MessageId::new(),
            comment: String::new(),
            presentations_attach: Attachment::Blank,
            thread: Thread::new(),
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_presentations_attach(mut self, presentations: String) -> VcxResult<Presentation> {
        let json: Json = Json::new(
            ::serde_json::from_str(&presentations)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Invalid Presentations: {:?}", err)))?,
            ENCODING_BASE64
        )?;
        self.presentations_attach = Attachment::JSON(json);
        Ok(self)
    }

    pub fn set_thread(mut self, thread: Thread) -> Self {
        self.thread = thread;
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Presentation(self.clone()) // TODO: THINK how to avoid clone
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresentationStatus {
    Undefined,
    Verified,
    Invalid(ProblemReport),
}

impl PresentationStatus {
    pub fn status(&self) -> u32 {
        match self {
            PresentationStatus::Undefined => 0,
            PresentationStatus::Verified => 1,
            PresentationStatus::Invalid(_) => 2,
        }
    }
}