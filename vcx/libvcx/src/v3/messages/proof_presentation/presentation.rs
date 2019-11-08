use v3::messages::{MessageId, A2AMessage};
use messages::thread::Thread;
use v3::messages::attachment::{
    Attachments,
    Attachment,
    Json,
    AttachmentEncoding
};
use messages::proofs::proof_message::ProofMessage;
use std::convert::TryInto;

use error::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Presentation {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "presentations~attach")]
    pub presentations_attach: Attachments,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

impl Presentation {
    pub fn create() -> Self {
        Presentation {
            id: MessageId::new(),
            comment: None,
            presentations_attach: Attachments::new(),
            thread: Thread::new(),
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_presentations_attach(mut self, presentations: String) -> VcxResult<Presentation> {
        let json: Json = Json::new(::serde_json::Value::String(presentations), AttachmentEncoding::Base64)?;
        self.presentations_attach.add(Attachment::JSON(json));
        Ok(self)
    }

    pub fn set_thread(mut self, thread: Thread) -> Self {
        self.thread = thread;
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Presentation(self.clone()) // TODO: THINK how to avoid clone
    }

    pub fn to_json(&self) -> VcxResult<String> {
        serde_json::to_string(self)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize Presentation: {}", err)))
    }
}

impl TryInto<ProofMessage> for Presentation {
    type Error = VcxError;

    fn try_into(self) -> Result<ProofMessage, Self::Error> {
        let mut proof = ProofMessage::new();
        proof.libindy_proof = self.presentations_attach.content().unwrap();
        Ok(proof)
    }
}