use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::attachment::{Attachments, Attachment, Json, AttachmentEncoding};
use error::VcxResult;
use messages::thread::Thread;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialRequest {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "requests~attach")]
    pub requests_attach: Attachments,
    #[serde(rename = "~thread")]
    pub thread: Thread
}

impl CredentialRequest {
    pub fn create() -> Self {
        CredentialRequest {
            id: MessageId::new(),
            comment: None,
            requests_attach: Attachments::new(),
            thread: Thread::new(),
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_requests_attach(mut self, credential_request: String) -> VcxResult<CredentialRequest> {
        let json: Json = Json::new(::serde_json::Value::String(credential_request), AttachmentEncoding::Base64)?;
        self.requests_attach.add(Attachment::JSON(json));
        Ok(self)
    }

    pub fn set_thread(mut self, thread: Thread) -> Self {
        self.thread = thread;
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::CredentialRequest(self.clone()) // TODO: THINK how to avoid clone
    }
}